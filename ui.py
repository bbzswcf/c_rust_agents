import gradio as gr
import os
import json
import logging
import time
from queue import Queue, Empty
from threading import Thread
from c_rust import (
    setup_logging,
    setup_projects,
    process_files,
    cleanup_projects,
    code_preprocess
)

# 创建一个队列来存储日志消息
log_queue = Queue()
conversion_status = "idle"

class QueueHandler(logging.Handler):
    def emit(self, record):
        log_entry = self.format(record)
        log_queue.put(log_entry)

def setup_queue_logger():
    """设置将日志输出到队列的处理器"""
    logger = logging.getLogger()
    queue_handler = QueueHandler()
    formatter = logging.Formatter('%(message)s')  # 简化格式，只显示消息内容
    queue_handler.setFormatter(formatter)
    logger.addHandler(queue_handler)
    logger.setLevel(logging.INFO)  # 确保设置了正确的日志级别
    return logger

def load_metadata():
    """加载元数据文件"""
    try:
        with open('tool/c_metadata_all.json', 'r', encoding='utf-8') as f:
            return json.load(f)
    except Exception as e:
        return None

def get_available_files(input_dir):
    """获取可用的C文件列表"""
    available_files = []
    try:
        for root, _, files in os.walk(input_dir):
            for file in files:
                if file.endswith(('.c', '.h', '.cpp')):
                    # 使用os.path.join和os.path.relpath来正确处理路径
                    full_path = os.path.join(root, file)
                    rel_path = os.path.relpath(full_path, input_dir)
                    available_files.append(rel_path)
        return sorted(available_files)
    except Exception as e:
        logging.error(f"读取文件列表时出错: {str(e)}")
        return []

def get_conversion_progress():
    """获取转换进度信息"""
    metadata = load_metadata()
    if not metadata:
        return "未开始转换"
    
    total_funcs = 0
    converted_funcs = 0
    for file_info in metadata.values():
        for func in file_info['functions']:
            total_funcs += 1
            if func.get('rust_code'):
                converted_funcs += 1
    
    return f"已转换: {converted_funcs}/{total_funcs} 函数"

def get_file_list():
    """获取文件和函数列表"""
    metadata = load_metadata()
    if not metadata:
        return "未找到文件列表"
    
    result = []
    for file_path, file_info in metadata.items():
        file_str = f"\n📄 {file_path}:"
        for func in file_info['functions']:
            if func.get('rust_code'):
                status = "✅ 已转换"
            elif func.get('conversion_failed'):
                status = "❌ 转换失败"
            else:
                status = "⏳ 未转换"
            file_str += f"\n  {status} {func['name']}"
        result.append(file_str)
    return "\n".join(result)

def convert_selected_files(input_dir, output_dir, selected_files, log_output=None, progress=gr.Progress()):
    """转换选定的文件"""
    global conversion_status
    conversion_status = "running"
    
    try:
        # 清空日志队列
        while not log_queue.empty():
            log_queue.get()
        
        # 预处理代码
        if selected_files:
            # 将选定的文件路径写入 translation_order.txt
            with open('tool/translation_order.txt', 'w', encoding='utf-8') as f:
                for file in selected_files:
                    f.write(file + '\n')
            logging.info(f"开始转换以下文件：\n{', '.join(selected_files)}")
        
        # 执行预处理
        code_preprocess(input_dir)
        
        # 执行转换流程
        setup_projects()
        process_files()
        cleanup_projects()
        
        conversion_status = "completed"
        # 返回更新后的状态、进度和文件列表
        return "转换完成！", get_conversion_progress(), "", get_file_list()
    
    except Exception as e:
        conversion_status = "failed"
        error_msg = f"转换失败: {str(e)}"
        logging.error(error_msg)
        return error_msg, "转换失败", "", get_file_list()

def update_logs(history):
    """更新日志显示"""
    if conversion_status == "running":
        logs = []
        while not log_queue.empty():
            log = log_queue.get()
            # 特殊处理Agent输出
            if "专家输出：" in log:
                logs.append("\n" + "="*50)
                logs.append(log)
                logs.append("="*50 + "\n")
            else:
                logs.append(log)
        if logs:
            new_history = (history or "") + "\n" + "\n".join(logs)
            return new_history
    return history

def periodic_update(log_output):
    """周期性更新日志"""
    logs = []
    while not log_queue.empty():
        log = log_queue.get_nowait()
        # 特殊处理Agent输出
        if any(x in log for x in ["语法专家输出：", "修复专家输出：", "修复规划专家输出："]):
            logs.append("\n" + "="*50)
            logs.append(log)
            logs.append("="*50 + "\n")
        else:
            logs.append(log)
    if logs:
        new_text = "\n".join(logs)
        return new_text
    return log_output

def start_conversion(input_dir, output_dir, selected_files, log_output):
    """开始转换并启动日志更新线程"""
    global conversion_status
    conversion_status = "running"
    
    try:
        # 清空日志队列
        while not log_queue.empty():
            log_queue.get()
        
        # 设置日志处理器
        logger = setup_queue_logger()
        
        # 创建一个事件来同步日志更新
        log_update_event = Queue()
        
        # 启动日志更新线程
        def update_thread():
            while conversion_status == "running":
                try:
                    logs = []
                    while not log_queue.empty():
                        log = log_queue.get_nowait()
                        # 特殊处理Agent输出
                        if any(x in log for x in ["语法专家输出：", "修复专家输出：", "修复规划专家输出："]):
                            logs.append("\n" + "="*50)
                            logs.append(log)
                            logs.append("="*50 + "\n")
                        else:
                            logs.append(log)
                    if logs:
                        new_text = "\n".join(logs)
                        log_update_event.put(new_text)
                except Exception as e:
                    print(f"更新日志失败: {str(e)}")
                time.sleep(0.1)
        
        # 启动更新线程
        Thread(target=update_thread, daemon=True).start()
        
        # 执行转换
        conversion_thread = Thread(target=lambda: convert_selected_files(input_dir, output_dir, selected_files))
        conversion_thread.start()
        
        # 主线程负责更新UI
        current_log = ""
        final_status = None
        final_progress = None
        
        while conversion_status == "running":
            try:
                # 非阻塞方式获取新日志
                try:
                    new_log = log_update_event.get(timeout=0.1)
                    if new_log:
                        current_log += new_log + "\n"
                        yield (
                            gr.update(value="转换中..."),
                            gr.update(value=get_conversion_progress()),
                            gr.update(value=current_log)
                        )
                except Empty:
                    # 即使没有新日志，也定期更新进度
                    yield (
                        gr.update(value="转换中..."),
                        gr.update(value=get_conversion_progress()),
                        gr.update(value=current_log)
                    )
            except Exception as e:
                print(f"更新UI失败: {str(e)}")
                time.sleep(0.1)
        
        # 等待转换线程完成
        conversion_thread.join()
        
        # 获取最终状态
        if conversion_status == "completed":
            final_status = "转换完成！"
            final_progress = get_conversion_progress()
        else:
            final_status = "转换失败"
            final_progress = "转换失败"
        
        # 返回最终状态
        yield (
            gr.update(value=final_status),
            gr.update(value=final_progress),
            gr.update(value=current_log)
        )
        
    except Exception as e:
        conversion_status = "failed"
        error_msg = f"转换失败: {str(e)}"
        logging.error(error_msg)
        return error_msg, "转换失败", log_output

def create_ui():
    """创建Gradio界面"""
    # 自定义CSS样式
    custom_css = """
    #component-0 {
        max-width: 100% !important;  /* 使用全宽 */
        margin: 0 !important;
        padding: 20px;
        font-family: 'Segoe UI', system-ui, -apple-system, sans-serif;
    }
    .main-header {
        text-align: center;
        color: #2c3e50;
        margin-bottom: 30px;
        font-size: 2.8em;
        font-weight: 700;
        text-shadow: 2px 2px 4px rgba(0,0,0,0.1);
        animation: fadeIn 1s ease-in;
        padding: 20px 0;
        background: linear-gradient(120deg, #a1c4fd 0%, #c2e9fb 100%);
        border-radius: 15px;
        margin: 20px 0;
    }
    .main-container {
        display: flex;
        gap: 20px;
        padding: 0;
        width: 100%;
    }
    .input-column {
        background: linear-gradient(145deg, #f8f9fa, #ffffff);
        padding: 25px;
        border-radius: 15px;
        box-shadow: 0 4px 15px rgba(0,0,0,0.1);
        transition: all 0.3s ease;
        width: 25%;  /* 固定宽度比例 */
        min-width: 300px;
    }
    .output-column {
        background: linear-gradient(145deg, #ffffff, #f8f9fa);
        padding: 25px;
        border-radius: 15px;
        box-shadow: 0 4px 15px rgba(0,0,0,0.1);
        transition: all 0.3s ease;
        width: 75%;  /* 固定宽度比例 */
    }
    .log-area, .file-list {
        background: #f8f9fa;
        color: #2c3e50;
        font-family: 'Consolas', 'Monaco', monospace;
        padding: 15px;
        border-radius: 10px;
        height: 600px;  /* 统一高度 */
        overflow-y: auto;
        font-size: 14px;
        line-height: 1.6;
        width: 100%;
        border: 1px solid #e0e0e0;
    }
    .log-area::-webkit-scrollbar,
    .file-list::-webkit-scrollbar {
        width: 8px;
    }
    .log-area::-webkit-scrollbar-track,
    .file-list::-webkit-scrollbar-track {
        background: #f1f1f1;
        border-radius: 4px;
    }
    .log-area::-webkit-scrollbar-thumb,
    .file-list::-webkit-scrollbar-thumb {
        background: #c1c1c1;
        border-radius: 4px;
    }
    .log-area::-webkit-scrollbar-thumb:hover,
    .file-list::-webkit-scrollbar-thumb:hover {
        background: #a8a8a8;
    }
    .section-title {
        font-size: 1.2em;
        font-weight: 600;
        color: #2c3e50;
        margin: 15px 0;
        padding-bottom: 8px;
        border-bottom: 2px solid #3498db;
    }
    .status-container {
        display: flex;
        gap: 10px;
        margin: 20px 0;
    }
    .status-label {
        font-weight: 600;
        color: #2c3e50;
        padding: 12px 20px;
        border-radius: 10px;
        background: rgba(44,62,80,0.1);
        flex: 1;
        text-align: center;
        font-size: 14px;
    }
    .button-container {
        display: flex;
        gap: 10px;
        margin: 20px 0;
    }
    .convert-btn, .refresh-btn {
        flex: 1;
        background: linear-gradient(145deg, #2ecc71, #27ae60) !important;
        border: none !important;
        padding: 12px 24px !important;
        border-radius: 8px !important;
        font-weight: 600 !important;
        letter-spacing: 0.5px !important;
        transition: all 0.3s ease !important;
        text-transform: uppercase !important;
        font-size: 14px !important;
        color: white !important;
    }
    .refresh-btn {
        background: linear-gradient(145deg, #3498db, #2980b9) !important;
    }
    """

    with gr.Blocks(title="C到Rust代码转换工具", theme=gr.themes.Soft(), css=custom_css) as app:
        gr.Markdown("# 🚀 C到Rust代码转换工具", elem_classes=["main-header"])
        
        with gr.Row(equal_height=True, elem_classes=["main-container"]):
            with gr.Column(elem_classes=["input-column"]):
                gr.Markdown("### 📂 输入设置", elem_classes=["section-title"])
                input_dir = gr.Textbox(
                    label="输入目录",
                    value="./Input/01-Primary",
                    info="C代码所在目录"
                )
                output_dir = gr.Textbox(
                    label="输出目录",
                    value="./Output/primary",
                    info="Rust代码输出目录"
                )
                
                gr.Markdown("### 📄 文件选择", elem_classes=["section-title"])
                initial_files = get_available_files("./Input/01-Primary")
                file_selector = gr.Dropdown(
                    label="选择要转换的文件",
                    choices=initial_files,
                    multiselect=True,
                    info="可以选择多个文件"
                )
                
                with gr.Row(elem_classes=["button-container"]):
                    refresh_files_btn = gr.Button(
                        "🔄 刷新文件列表", 
                        elem_classes=["refresh-btn"]
                    )
                    convert_btn = gr.Button(
                        "🚀 开始转换", 
                        variant="primary", 
                        elem_classes=["convert-btn"]
                    )
                
                with gr.Row(elem_classes=["status-container"]):
                    status_label = gr.Label(
                        label="📊 转换状态", 
                        elem_classes=["status-label"]
                    )
                    progress_label = gr.Label(
                        label="📈 转换进度", 
                        value=get_conversion_progress(), 
                        elem_classes=["status-label"]
                    )
            
            with gr.Column(elem_classes=["output-column"]):
                with gr.Row():
                    with gr.Column(scale=2):
                        gr.Markdown("### 📝 转换日志", elem_classes=["section-title"])
                        log_output = gr.TextArea(
                            label="实时日志",
                            interactive=False,
                            autoscroll=True,
                            lines=30,
                            elem_classes=["log-area"],
                            every=0.1  # 添加自动更新间隔
                        )
                    
                    with gr.Column(scale=1):
                        gr.Markdown("### 📋 文件和函数列表", elem_classes=["section-title"])
                        file_list = gr.TextArea(
                            label="详细信息",
                            value=get_file_list(),
                            interactive=False,
                            elem_classes=["file-list"]
                        )
                        refresh_list_btn = gr.Button(
                            "🔄 刷新列表",
                            elem_classes=["refresh-btn"]
                        )

        # 转换按钮点击事件
        convert_btn.click(
            fn=start_conversion,  # 使用start_conversion而不是convert_selected_files
            inputs=[input_dir, output_dir, file_selector, log_output],
            outputs=[status_label, progress_label, log_output],
            queue=True
        ).then(  # 转换完成后更新文件列表
            fn=get_file_list,
            outputs=[file_list]
        )

        # 添加日志自动更新
        log_output.change(
            fn=update_logs,
            inputs=[log_output],
            outputs=[log_output],
            queue=False
        )

        # 刷新按钮点击事件
        refresh_files_btn.click(
            fn=lambda x: gr.Dropdown(choices=get_available_files(x)),
            inputs=[input_dir],
            outputs=[file_selector]
        )

        # 刷新列表按钮点击事件
        refresh_list_btn.click(
            fn=get_file_list,
            outputs=[file_list]
        )
    
    return app

if __name__ == "__main__":
    app = create_ui()
    app.queue()
    app.launch(
        server_name="127.0.0.1",
        server_port=7860,
        share=False  # 禁用分享功能
    ) 
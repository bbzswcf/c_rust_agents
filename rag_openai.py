import os
from langchain_community.vectorstores import Chroma
from langchain_community.document_loaders import TextLoader
from langchain.docstore.document import Document
from tenacity import retry, stop_after_attempt, wait_exponential
import hashlib
import pickle
from langchain_openai import OpenAIEmbeddings
from LLM_config import gpt4_key, gpt_base_url



class OpenAICustomEmbeddings:
    def __init__(self, api_key, base_url=None, cache_dir="cache"):
        self.embeddings = OpenAIEmbeddings(
            api_key=api_key,
            base_url=base_url,
            model="text-embedding-ada-002"
        )
        self.cache_dir = cache_dir
        os.makedirs(cache_dir, exist_ok=True)
    
    def _get_cache_key(self, text):
        return hashlib.md5(text.encode()).hexdigest()

    @retry(
        stop=stop_after_attempt(3),
        wait=wait_exponential(multiplier=1, min=4, max=10)
    )
    def _embed_single_text(self, text):
        # 检查缓存
        cache_key = self._get_cache_key(text)
        cache_path = os.path.join(self.cache_dir, f"{cache_key}.pkl")
        
        if os.path.exists(cache_path):
            try:
                with open(cache_path, 'rb') as f:
                    print("使用缓存的嵌入结果")
                    return pickle.load(f)
            except Exception as e:
                print(f"读取缓存失败: {str(e)}")

        # 使用 OpenAI 嵌入
        try:
            embedding = self.embeddings.embed_query(text)
            
            # 保存到缓存
            try:
                with open(cache_path, 'wb') as f:
                    pickle.dump(embedding, f)
                print("已缓存新的嵌入结果")
            except Exception as e:
                print(f"保存缓存失败: {str(e)}")
            
            return embedding
            
        except Exception as e:
            print(f"嵌入请求失败: {str(e)}")
            raise

    def embed_documents(self, texts):
        embeddings = []
        texts = [str(text).strip() for text in texts if text]
        
        for i, text in enumerate(texts):
            try:
                embedding = self._embed_single_text(text)
                embeddings.append(embedding)
                print(f"已处理 {i + 1}/{len(texts)} 个文本")
                
            except Exception as e:
                print(f"处理第 {i + 1} 个文本时出错: {str(e)}")
                print(f"文本内容: {text[:100]}...")
                raise
                
        return embeddings

    def embed_query(self, text):
        return self._embed_single_text(text)

def create_knowledge_base(rust_code_dir):
    documents = []
    skipped_files = []  # 记录被跳过的文件
    embedding_errors = []  # 记录嵌入错误的文件
    
    # 遍历所有的 .rs 文件
    total_files = sum(1 for _, _, files in os.walk(rust_code_dir) 
                     for file in files if file.endswith('.rs'))
    current_file = 0
    
    for root, dirs, files in os.walk(rust_code_dir):
        for file in files:
            if file.endswith(".rs"):
                current_file += 1
                rust_file_path = os.path.join(root, file)
                try:
                    rust_loader = TextLoader(rust_file_path, encoding='utf-8')
                    rust_code = rust_loader.load()[0].page_content
                    
                    documents.append(Document(
                        page_content=rust_code,
                        metadata={"file_path": rust_file_path}
                    ))
                    print(f"正在处理第 {current_file}/{total_files} 个文件")
                except Exception as e:
                    print(f"正在处理第 {current_file}/{total_files} 个文件")
                    print(f"处理文件时出错: {str(e)}")
                    skipped_files.append(rust_file_path)
    
    if skipped_files:
        print(f"\n总共跳过了 {len(skipped_files)} 个文件")
    
    print(f"\n开始进行文档嵌入...")
    
    embeddings = OpenAICustomEmbeddings(api_key=gpt4_key, base_url=gpt_base_url)
    successful_documents = []
    
    # 创建一个临时的向量存储
    vectorstore = None
    batch_size = 5000  # 每批处理的文档数量
    
    for i in range(0, len(documents), batch_size):
        batch_documents = documents[i:i + batch_size]
        batch_successful = []
        
        # 处理当前批次的文档
        for j, doc in enumerate(batch_documents, 1):
            try:
                print(f"正在嵌入第 {i + j}/{len(documents)} 个文档")
                embeddings.embed_documents([doc.page_content])
                batch_successful.append(doc)
            except Exception as e:
                print(f"嵌入文档时出错: {str(e)}")
                embedding_errors.append(doc.metadata['file_path'])
                continue
        
        # 如果这个批次有成功处理的文档
        if batch_successful:
            try:
                print(f"\n处理第 {i//batch_size + 1} 批文档...")
                if vectorstore is None:
                    # 创建新的向量存储
                    vectorstore = Chroma.from_documents(
                        batch_successful,
                        embeddings,
                        persist_directory="./rag_db_test"
                    )
                else:
                    # 添加到现有的向量存储
                    vectorstore.add_documents(batch_successful)
                
                successful_documents.extend(batch_successful)
                print(f"成功添加 {len(batch_successful)} 个文档到向量存储")
                
                # 每批次后保存
                vectorstore.persist()
                
            except Exception as e:
                print(f"处理批次时出错: {str(e)}")
                continue
    
    print(f"\n总共成功嵌入了 {len(successful_documents)}/{len(documents)} 个文档")
    
    if not successful_documents:
        print("警告：没有成功处理任何文件")
        return None
    
    return vectorstore

def find_similar_example(query_code, vectorstore):
    similar_docs = vectorstore.similarity_search(query_code, k=1)
    if similar_docs:
        return {
            "rust_code": similar_docs[0].page_content,
            "file_path": similar_docs[0].metadata["file_path"]
        }
    return None

def main():
    rust_code_dir = "修改后/Rust_test"  # Rust 代码目录
    
    # 创建知识库
    vectorstore = create_knowledge_base(rust_code_dir)
    
    # 添加空值检查
    if vectorstore is None:
        print("无法创建知识库，程序退出")
        return
        
    # 在使用完成后添加显式持久化
    vectorstore.persist()
    # 示例：查找相似的代码
    query_code = """
pub fn string_nocase_hash(vlocation: CStr) -> u32 {
    let mut result: u32 = 5381;
    let mut p: CStr;
    p = vlocation;
    while *p != '\0' as u8 {        
        result = (result << 5) + result + c_tolower!(*p) as u32;
        p += 1;
    }
    return result 
}
    """
    
    similar_example = find_similar_example(query_code, vectorstore)
    if similar_example:
        print("\n找到相似的Rust代码:")
        print(f"文件路径: {similar_example['file_path']}")
        print("代码内容:")
        print(similar_example['rust_code'])
    else:
        print("未找到相似的代码示例")

if __name__ == "__main__":
    main()

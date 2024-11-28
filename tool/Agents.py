# 设置Agent
from Agent_prompt import *
from openai import OpenAI
from LLM_config import *
import time


client = OpenAI(api_key=deepseek_key, base_url=deepseek_base_url)

class Agent:
    def __init__(self, role: str, prompt: str, temperature: float, top_p: float):
        self.role = role
        self.prompt = prompt
        self.temperature = temperature
        self.top_p = top_p

    def generate_response(self, user_input: str, example_input: list = [], example_output: list = []) -> str:
        max_attempts = 5
        attempt = 0
        message_list = [
            {"role": "system", "content": self.prompt}
        ]
        for i in range(len(example_input)):
            message_list.append({"role": "user", "content": example_input[i]})
            message_list.append({"role": "assistant", "content": example_output[i]})
        message_list.append({"role": "user", "content": user_input})
        
        while attempt < max_attempts:
            attempt += 1
            print(f"{self.role} 正在回答 (尝试 {attempt}/{max_attempts}):")
            
            try:
                response = client.chat.completions.create(
                    model="deepseek-chat",
                    messages=message_list,
                    temperature=self.temperature,
                    top_p=self.top_p,
                    stream=True
                )
                
                full_response = ""
                for chunk in response:
                    if chunk.choices and chunk.choices[0].delta.content is not None:
                        content = chunk.choices[0].delta.content
                        print(content, end='', flush=True)
                        full_response += content
                print("\n")
                
                # 检查响应是否有效
                full_response = full_response.strip().encode('utf-8', errors='ignore').decode('utf-8')
                if full_response:
                    return full_response
                else:
                    print(f"获得空响应，重试中...")
                    time.sleep(1)
                    continue
                    
            except Exception as e:
                print(f"生成响应时发生错误：{str(e)}")
                if attempt < max_attempts:
                    print("重试中...")
                    time.sleep(1)
                    continue
                else:
                    return ""
        
        print(f"达到最大重试次数 ({max_attempts})，返回空响应")
        return ""

# 创建不同的Agent

syntax_agent = Agent(
    role="Syntax Conversion Expert",
    prompt=Syntax_system_prompt,
    temperature=0,
    top_p=1
)
    # temperature=0.2,
    # top_p=0.9


feedback_agent = Agent(
    role="Feedback Expert",
    prompt=Feedback_prompt_new,
    temperature=0,
    top_p=1
)
optimize_agent = Agent(
    role="optimize Expert",
    prompt=Optimize_prompt,
    temperature=0,
    top_p=1
)
optimize_agent_2 = Agent(
    role="optimize Expert",
    prompt=Optimize_prompt_2,
    temperature=0,
    top_p=1)
    # temperature=0.2,
    # top_p=0.95

# fix_agent = Agent(
#     role="fix Expert",
#     prompt="You are a proficient C and Rust advanced developer.",
#     temperature=0.2,
#     top_p=0.9)

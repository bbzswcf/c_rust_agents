# 设置Agent
import os
from Agent_prompt_simple import *
from openai import OpenAI
from LLM_config import *

# siliconflow_key = "sk-ewfzqneuiyrzxbkautzrjsfulfxtyxwlunettvibwfxjbust"
# siliconflow_key = siliconflow_key

siliconflow_base_url = "https://api.siliconflow.cn/v1"


client = OpenAI(api_key=siliconflow_key, base_url=siliconflow_base_url)

class Agent:
    def __init__(self, role: str, prompt: str, temperature: float, top_p: float):
        self.role = role
        self.prompt = prompt
        self.temperature = temperature
        self.top_p = top_p

    def generate_response(self, user_input: str) -> str:
        max_attempts = 5
        attempt = 0
        
        while attempt < max_attempts:
            attempt += 1
            print(f"{self.role} 正在回答 (尝试 {attempt}/{max_attempts}):")
            
            try:
                response = client.chat.completions.create(
                    model="deepseek-ai/DeepSeek-V2.5",
                    messages=[
                        {"role": "system", "content": self.prompt},
                        {"role": "user", "content": user_input}
                    ],
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
                if full_response.strip():
                    return full_response
                else:
                    print(f"获得空响应，重试中...")
                    continue
                    
            except Exception as e:
                print(f"生成响应时发生错误：{str(e)}")
                if attempt < max_attempts:
                    print("重试中...")
                    continue
                else:
                    return ""
        
        print(f"达到最大重试次数 ({max_attempts})，返回空响应")
        return ""

# 创建不同的Agent

api_agent = Agent(
    role="API Conversion Expert",
    prompt=API_prompt,
    temperature=0.2,
    top_p=0.9
)

syntax_agent = Agent(
    role="Syntax Conversion Expert",
    prompt=Syntax_prompt,
    temperature=0.2,
    top_p=0.9
)

syntax_agent_2 = Agent(
    role="Syntax Conversion Expert",
    prompt=Syntax_prompt_2,
    temperature=0.2,
    top_p=0.9
)

feedback_agent = Agent(
    role="Feedback Expert",
    prompt=Feedback_prompt,
    temperature=0.3,
    top_p=0.85)
optimize_agent = Agent(
    role="optimize Expert",
    prompt=Optimize_prompt,
    temperature=0.2,
    top_p=0.9)
optimize_agent_2 = Agent(
    role="optimize Expert",
    prompt=Optimize_prompt_2,
    temperature=0.2,
    top_p=0.9)

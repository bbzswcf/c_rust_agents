# 设置Agent
from Agent_prompt_complex import *
# from NO_API_Prompt import *  # w/o api agent
from openai import OpenAI

deepseek_key = "sk-764372d3e899489480a9a0deda637953"
deepseek_base_url = "https://api.deepseek.com/beta"
client = OpenAI(api_key=deepseek_key, base_url=deepseek_base_url)

class Agent:
    def __init__(self, role: str, prompt: str, temperature: float, top_p: float):
        self.role = role
        self.prompt = prompt
        self.temperature = temperature
        self.top_p = top_p

    def generate_response(self, user_input: str) -> str:
        print(f"{self.role} 正在回答:")
        try:
            response = client.chat.completions.create(
                model="deepseek-coder",

                messages=[
                    {"role": "system", "content": self.prompt},
                    {"role": "user", "content": user_input}
                ],
                temperature=self.temperature,
                top_p= self.top_p,
                stream=True
            )
            full_response = ""
            for chunk in response:
                if chunk.choices and chunk.choices[0].delta.content is not None:
                    content = chunk.choices[0].delta.content
                    print(content, end='', flush=True)
                    full_response += content
            print("\n")
            return full_response
        except Exception as e:
            print(f"生成响应时发生错误：{str(e)}")
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

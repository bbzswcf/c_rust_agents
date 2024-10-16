# 简单的构建一个
import os
from langchain_community.vectorstores import Chroma
from langchain_openai import OpenAIEmbeddings
from langchain.text_splitter import CharacterTextSplitter
from langchain_community.document_loaders import TextLoader
from langchain.docstore.document import Document

gpt4_key = "sk-******"
gpt_base_url = "******"
def create_knowledge_base(c_code_dir, rust_code_dir):
    documents = []
    
    for problem_folder in os.listdir(c_code_dir):
        c_file_path = os.path.join(c_code_dir, problem_folder, "main.c")
        rust_file_path = os.path.join(rust_code_dir, f"{problem_folder}.rs")
        
        if os.path.exists(c_file_path) and os.path.exists(rust_file_path):
            c_loader = TextLoader(c_file_path)
            rust_loader = TextLoader(rust_file_path)
            
            c_code = c_loader.load()[0].page_content
            rust_code = rust_loader.load()[0].page_content
            
            documents.append(Document(
                page_content=c_code,
                metadata={"rust_code": rust_code, "problem": problem_folder}
            ))
    
    text_splitter = CharacterTextSplitter(chunk_size=1000, chunk_overlap=0)
    texts = text_splitter.split_documents(documents)
    
    embeddings = OpenAIEmbeddings(api_key=gpt4_key, base_url=gpt_base_url)
    vectorstore = Chroma.from_documents(texts, embeddings, persist_directory="./chroma_db")
    vectorstore.persist()
    
    print(f"知识库创建完成，共 {len(texts)} 个代码片段")
    return vectorstore

def find_similar_example(query_code, vectorstore):
    similar_docs = vectorstore.similarity_search(query_code, k=1)
    if similar_docs:
        return {
            "c_code": similar_docs[0].page_content,
            "rust_code": similar_docs[0].metadata["rust_code"],
            "problem": similar_docs[0].metadata["problem"]
        }
    return None

def main():
    c_code_dir = "test1/c_codes"
    rust_code_dir = "test1/succeed"
    
    # 创建知识库
    vectorstore = create_knowledge_base(c_code_dir, rust_code_dir)
    
    # 示例：查找相似的代码
    query_code = """
    #include <stdio.h>
    int main() {
        int a, b;
        scanf("%d %d", &a, &b);
        printf("%d\n", a + b);
        return 0;
    }
    """
    
    similar_example = find_similar_example(query_code, vectorstore)
    if similar_example:
        print(f"找到的相似C代码示例")
        print(similar_example['c_code'])
        print("\n对应的Rust代码:")
        print(similar_example['rust_code'])
    else:
        print("未找到相似的代码示例")

if __name__ == "__main__":
    main()

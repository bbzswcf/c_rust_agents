FROM rust:latest
LABEL author="yz&wcf"

# 使用清华镜像源加速
ENV RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
ENV RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup

# 安装Rust组件
RUN rustup default stable && \
    rustup component add clippy && \
    rustup component add rustfmt

# 创建必要的目录
RUN mkdir -p /root/.cargo

# 添加apt源配置和cargo配置
COPY sources.list /etc/apt/
COPY config /root/.cargo/

# 添加GPG密钥并安装软件
RUN DEBIAN_FRONTEND=noninteractive apt-get update -y && \
    apt-get install -y \
    build-essential \
    python3 \
    python3-pip && \
    apt-get clean -y

# 添加requirements.txt并安装Python依赖
COPY requirements.txt /tmp/
RUN mv /usr/lib/python3.11/EXTERNALLY-MANAGED /usr/lib/python3.11/EXTERNALLY-MANAGED.bk
RUN pip3 install -r /tmp/requirements.txt -i https://pypi.tuna.tsinghua.edu.cn/simple

CMD ["bash"]
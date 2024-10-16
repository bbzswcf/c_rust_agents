from setuptools import find_packages, setup

setup(
    name="c_rust_agents",
    version="1.0",
    author="",
    license="",
    python_requires=">=3.9",
    packages=find_packages(exclude=[]),
    install_requires=[
        "openai==1.51.2",
        "chardet==5.2.0",
    ],
)

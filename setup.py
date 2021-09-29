import sys

from cx_Freeze import Executable, setup

target = Executable(script="src/anvil.py", base= None, icon="src/icon.ico")
packages = ["multiprocessing"]

setup(
    name="Anvil",
    version="0.3.0",
    description="Anvil is a lightweight CLI Forge Profile Manager.",
    executables=[target],
    options={"build.exe": {"packages":packages}}
)

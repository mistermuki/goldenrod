import sys

from cx_Freeze import Executable, setup

# base="Win32GUI" should be used only for Windows GUI app
base = "Console"

target = Executable(script="src/anvil.py", base=base, icon="src/icon.ico")

setup(
    name="Anvil",
    version="0.2.0",
    description="Anvil is a lightweight CLI Forge Profile Manager.",
    executables=[target],
    options={"build.exe": {}},
)

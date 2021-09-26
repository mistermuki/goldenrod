import platform
import subprocess

import isort

subprocess.run(["pip3", "install", "-r", "requirements.txt"], shell=False)

# Sorts all the src files.
isort.code("src/*")
print("All source files have been sorted successfully.")

# Stylechecks all the src files.
subprocess.run(["python3", "-m", "black", "src/"])


if platform.system() == "Linux":
    print("Will not compile the exe because we are on Linux.")
    exit()

# Compiles Anvil into an .exe for Windows.
subprocess.run(
    ["pyinstaller", "--onefile", "--clean", "--name", "Anvil", "src/main.py"]
)

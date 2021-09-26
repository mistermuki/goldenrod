# Installs the requirements needed to build.
import subprocess

subprocess.run(["pip3", "install", "-r", "requirements.txt"], shell=False)

# Sorts all the src files.
subprocess.run(["isort", "src/*"])
print("All source files have been sorted successfully.")

# Stylechecks all the src files.
subprocess.run(["black", "src/*"])

# Compiles Anvil into an .exe for Windows.
subprocess.run(
    ["pyinstaller", "--onefile", "--clean", "--name", "Anvil", "src/main.py"]
)

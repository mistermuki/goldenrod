import platform
import subprocess

subprocess.run(["pip3", "install", "-r", "requirements.txt"], shell=False)

# Sorts all the src files.
try:
    import isort

    isort.code("src/*")
except:
    if platform.system() == "Linux":
        subprocess.run(["python3", "-m", "isort", "src/"])
    else:
        subprocess.run(["python", "-m", "isort", "src/"])


print("All source files have been sorted successfully.")
# Stylechecks all the src files.

if platform.system() == "Linux":
    subprocess.run(["python3", "-m", "black", "src/"])
else:
    subprocess.run(["python", "-m", "black", "src/"])

# Compiles Anvil into an executable.
if platform.system() == "Linux":
    subprocess.run(["python3", "setup.py", "build"])
else:
    subprocess.run(["python", "setup.py", "build"])

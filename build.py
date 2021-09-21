import os

# Installs the requirements needed to build.
os.system("pip3 install -r requirements.txt")

# Sorts all the src files.
os.system("isort src/*")
print("All source files have been sorted successfully.")

# Stylechecks all the src files.
os.system("black src/*")

# Compiles Anvil into an .exe for Windows.
os.system("pyinstaller --onefile --clean --name Anvil src/main.py")

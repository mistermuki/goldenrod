import json
import os
import shutil
import sys
import time

import requests


def installMods(_dir):
    """Handles installing mods from an Anvil profile to a client."""
    for i in data["modList"]:
        if not os.path.exists("./jarCache/"):
            os.mkdir("./jarCache/")
            print("Creating JAR Cache")

        if not os.path.exists("./jarCache/" + str(data["shortName"])):
            os.mkdir("./jarCache/" + str(data["shortName"]))
            print("Creating JAR Cache:" + "./jarCache/" + str(data["shortName"]))
            time.sleep(0.3)

        temp_mod = requests.get(data["modList"][i][0]["link"])
        print("Requesting: " + i + " from: " + data["modList"][i][0]["link"])
        cache_directory = "./jarCache/" + str(data["shortName"]) + "/" + str(i) + ".jar"
        time.sleep(0.3)

        with open(cache_directory, "wb") as f:
            print("Saving " + i + "...")
            f.write(temp_mod.content)
            print("Successfully saved: " + i)
            time.sleep(0.1)

    print("Installing mods to mods Directory...")
    mods_dir = _dir + r"/mods"

    if not os.path.exists(mods_dir):
        os.mkdir(mods_dir)
        print("Creating mods Directory: " + mods_dir)
        time.sleep(1)
    elif os.path.isdir(mods_dir):
        print("Cleaning mods Directory...")
        time.sleep(1)
        shutil.rmtree(mods_dir)
        os.mkdir(mods_dir)
        print("Creating mods Directory: " + mods_dir)
        time.sleep(1)

    print("Loading files into mods folder...")
    for x in os.listdir("./jarCache/" + data["shortName"]):
        if x.endswith(".jar"):
            print("Moving: " + x)
            shutil.move("./jarCache/" + data["shortName"] + "/" + x, mods_dir + "/" + x)
            print("Succesfully moved: " + x)
            time.sleep(1)

    print("Cleaning up jarCache...")
    shutil.rmtree("./jarCache/" + str(data["shortName"]))
    print("Cleaned up jarCache!")
    print("Thank you for using Anvil.")
    time.sleep(5)
    sys.exit(0)


def serverInstall(_dire):
    """Handles installing mods from an Anvil profile onto a server."""
    for i in data["modList"]:
        if not os.path.exists("./jarCache/"):
            os.mkdir("./jarCache/")
            print("Creating JAR Cache")

        if not os.path.exists("./jarCache/" + str(data["shortName"])):
            os.mkdir("./jarCache/" + str(data["shortName"]))
            print("Creating JAR Cache:" + "./jarCache/" + str(data["shortName"]))
            time.sleep(0.3)

        temp_mod = requests.get(data["modList"][i][0]["link"])
        print("Requesting: " + i + " from: " + data["modList"][i][0]["link"])
        cache_directory = "./jarCache/" + str(data["shortName"]) + "/" + str(i) + ".jar"
        time.sleep(0.3)

        with open(cache_directory, "wb") as f:
            print("Saving " + i + "...")
            f.write(temp_mod.content)
            print("Successfully saved: " + i)
            time.sleep(0.1)

    print("Loading files into mods folder...")
    for x in os.listdir("./jarCache/" + data["shortName"]):
        if x.endswith(".jar"):
            print("Moving: " + x)
            shutil.move("./jarCache/" + data["shortName"] + "/" + x, _dire + "/" + x)
            print("Succesfully moved: " + x)
            time.sleep(1)

    print("Cleaning up jarCache...")
    shutil.rmtree("./jarCache/" + str(data["shortName"]))
    print("Cleaned up jarCache!")
    print("Thank you for using Anvil.")
    time.sleep(5)
    sys.exit(0)


print("\n")
print(r"  /$$$$$$  /$$   /$$ /$$    /$$ /$$$$$$ /$$ ")
print(r" /$$__  $$| $$$ | $$| $$   | $$|_  $$_/| $$ ")
print(r"| $$  \ $$| $$$$| $$| $$   | $$  | $$  | $$ ")
print(r"| $$$$$$$$| $$ $$ $$|  $$ / $$/  | $$  | $$ ")
print(r"| $$__  $$| $$  $$$$ \  $$ $$/   | $$  | $$ ")
print(r"| $$  | $$| $$\  $$$  \  $$$/    | $$  | $$ ")
print(r"| $$  | $$| $$ \  $$   \  $/    /$$$$$$| $$$$$$$$")
print(r"|__/  |__/|__/  \__/    \_/    |______/|________/")
print(r"Version: 0.2.1")
print("\n")
print(r"Anvil has successfully launched.")
print(r"You can use the command, help for a list of commands")

while True:
    commandIn = input()
    command = commandIn.split(" ")

    if command[0] == "help":
        print("help - Gives you this handy list.")
        print("download [link] - downloads a server profile locally.")
        print("load [profile] [directory] - loads & installs a Forge Modpack Profile.")
        print(
            "server [profile] [directory] - for quick downloading Modpacks for Server Use."
        )
        continue

    if command[0] == "download":
        try:
            if not os.path.exists("./profiles/"):
                os.mkdir("./profiles")
        except FileNotFoundError:
            continue

        try:
            temp = requests.get(command[1])
        except FileNotFoundError:
            print("That download link is not valid.")

        print("Downloading from: " + command[1])
        with open("./tempProfile.json", "wb") as f1:
            print("Extracting Profile...")
            f1.write(temp.content)
        f2 = open("./tempProfile.json")
        downloadData = json.load(f2)
        f2.close()
        shutil.move(
            "./tempProfile.json", "./profiles/" + downloadData["shortName"] + ".json"
        )
        print(
            "Your profile, "
            + downloadData["shortName"]
            + ".json has been saved! You can load it using the load command."
        )

    if command[0] == "server":
        print("Server.")
        if os.path.exists("./profiles/" + command[1] + ".json"):
            print("Server.")
            f1 = open("./profiles/" + command[1] + ".json")
            print("Server.")
            data = json.load(f1)
            print("Server.")
            if command[2]:
                print("Server.")
                if not os.path.exists(command[2]):
                    print("This directory does not exist.")
                else:
                    serverInstall(command[2])

    if command[0] == "load":
        if os.path.exists("./profiles/" + command[1] + ".json"):
            f1 = open("./profiles/" + command[1] + ".json")
            data = json.load(f1)
            try:
                if command[2] and not os.path.exists(command[2]):
                    print("This directory does not exist.")
                    continue
            except Exception:
                appData = os.getenv("APPDATA")
                mcDir = appData + r"/.minecraft"
                installMods(mcDir)

        elif not os.path.exists("./profiles/" + command[1] + ".json"):
            print("This profile does not exist. Try again.")
            continue

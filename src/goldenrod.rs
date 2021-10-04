#![feature(str_split_whitespace_as_str)]
extern crate dirs;
extern crate reqwest;
extern crate stringr;
use std::{env, time};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::exit;
use std::string::ToString;
use std::thread::sleep;

fn process_input() {
    let mut command_text = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut command_text)
        .expect("Did not enter a correct string");
    if let Some('\n') = command_text.chars().next_back() {
        command_text.pop();
    }
    if let Some('\r') = command_text.chars().next_back() {
        command_text.pop();
    }

    let first_command = command_text.split_whitespace().next().unwrap_or("");
    let mut input_iter = command_text.split_whitespace();

    match first_command {
        "download" => {
            input_iter.next();
            println!("Attempting to download from: {}", input_iter.as_str());
            download_jsonprofile(input_iter.as_str().to_string());
        }
        "load" => {
            input_iter.next();
            let profile_name = input_iter.as_str();
            println!("Attempting to load: {}.json", profile_name);
            if Path::new(&format!("./profiles/{}.json", profile_name)).exists() {
                println!(
                    "Goldenrod Profile Loader: Installing for {}...",
                    env::consts::OS
                );
                load_profile(env::consts::OS, profile_name.to_string())
            } else {
                println!("The Goldenrod Profile, {}, does not exist.", profile_name);
                process_input();
            }
        }
        "server" => {}
        "help" => {}
        "quit" => {}
        _ => {
            println!("'{}' is not a valid command.", input_iter.as_str());
            process_input();
        }
    }
}

fn download_jsonprofile(url: String) {
    if !Path::new("./profiles").exists(){
        println!("Goldenrod JSON Retriever: Creating Profiles Folder..");
        fs::create_dir("./profiles")
            .expect("Failed to create profiles folder.")
    }
    let resp = reqwest::blocking::get(url).expect("JSON Download Request Failed");
    println!("Goldenrod JSON Retriever: Download Request Succeeded");
    let body = resp.text().expect("Request Body Invalid");
    println!("Goldenrod JSON Retriever: Response Body Valid");
    let mut out = File::create("./profiles/tempName.json").expect("Failed to create file");
    println!("Goldenrod JSON Retriever: Successfully created JSON Temp File.");
    io::copy(&mut body.as_bytes(), &mut out).expect("Failed to copy content");
    println!("Goldenrod JSON Retriever: Successfully filled JSON Temp File.");
    let file =
        fs::File::open("./profiles/tempName.json").expect("The JSON profile should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("The JSON profile should be proper JSON");
    let mut short_name: String = json
        .get("shortName")
        .expect("The JSON profile should have shortName key")
        .to_string();
    short_name = short_name.replace(r#"""#, "");
    println!(
        "Goldenrod JSON Retriever: Successfully downloaded {}.json",
        short_name
    );
    fs::rename(
        "./profiles/tempName.json",
        format!("./profiles/{}.json", short_name),
    )
    .expect("Not able to rename tempName.json.");
    println!(
        "Use the 'load' command to install a profile! (ex. load {})",
        short_name
    );
    process_input();
}

fn load_profile(os: &str, profile: String) {
    match os {
        "windows" => {
            println!(
                "Goldenrod Profile Loader: Checking if mods folder exists in default location..."
            );
            if Path::new(&format!(
                "{}/.minecraft/mods",
                dirs::data_dir().unwrap().display()
            ))
            .exists()
            {
                println!("Goldenrod Profile Loader: Mods folder exists...");
                println!("Goldenrod Profile Loader: Beginning Mod Requests");
                let file = fs::File::open(format!("./profiles/{}.json", profile))
                    .expect("The JSON profile should open read only");
                let json: serde_json::Value =
                    serde_json::from_reader(file).expect("The JSON profile should be proper JSON");

                for (key, value) in json["modList"].as_object().unwrap() {
                    println!("Goldenrod Mod Installer: Downloading {}...", key);
                    let link : String = value[0]["link"].to_string().replace(r#"""#, "");

                    let resp = reqwest::blocking::get(link).expect("JAR Download Request Failed");
                    let body = resp.text().expect("Request Body Invalid");
                    let mut out = File::create(format!("{}/.minecraft/mods/{}.jar", dirs::data_dir().unwrap().display(), key)).expect("Failed to create file");
                    io::copy(&mut body.as_bytes(), &mut out).expect("Failed to copy content");
                    println!("Goldenrod JAR Retriever: Successfully downloaded {}.", key);
                    println!("Goldenrod Mod Installer: Successfully downloaded & installed {}", key);
                }

                println!("{} has been successfully installed.", profile);
                println!("Thank you for using Goldenrod.");
                sleep(time::Duration::from_secs(5));
                exit(1);

            } else {
                println!("Goldenrod Profile Loader: Mods folder does not exist.");
                println!("Goldenrod Profile Loader: If you have installed Minecraft in it's default location...");
                println!("Goldenrod Profile Loader: ...double-check if you have Forge installed.");
                println!("If you have Minecraft and Forge installed somewhere else, please use the server command.");
            }
        }
        "linux" => {
            println!(
                "Goldenrod Profile Loader: Checking if mods folder exists in default location..."
            );
            if Path::new(&format!(
                "{}/.minecraft/mods",
                dirs::home_dir().unwrap().display()
            ))
                .exists()
            {
                println!("Goldenrod Profile Loader: Mods folder exists...");
                println!("Goldenrod Profile Loader: Beginning Mod Requests");
                let file = fs::File::open(format!("./profiles/{}.json", profile))
                    .expect("The JSON profile should open read only");
                let json: serde_json::Value =
                    serde_json::from_reader(file).expect("The JSON profile should be proper JSON");

                for (key, value) in json["modList"].as_object().unwrap() {
                    println!("Goldenrod Mod Installer: Downloading {}...", key);
                    let link: String = value[0]["link"].to_string().replace(r#"""#, "");

                    let resp = reqwest::blocking::get(link).expect("JAR Download Request Failed");
                    let body = resp.text().expect("Request Body Invalid");
                    let mut out = File::create(format!("{}/.minecraft/mods/{}.jar", dirs::home_dir().unwrap().display(), key)).expect("Failed to create file");
                    io::copy(&mut body.as_bytes(), &mut out).expect("Failed to copy content");
                    println!("Goldenrod JAR Retriever: Successfully downloaded {}.", key);
                    println!("Goldenrod Mod Installer: Successfully downloaded & installed {}", key);
                }

                println!("{} has been successfully installed.", profile);
                println!("Thank you for using Goldenrod.");
                sleep(time::Duration::from_secs(5));
                exit(1);
            } else {
                println!("Goldenrod Profile Loader: Mods folder does not exist.");
                println!("Goldenrod Profile Loader: If you have installed Minecraft in it's default location...");
                println!("Goldenrod Profile Loader: ...double-check if you have Forge installed.");
                println!("If you have Minecraft and Forge installed somewhere else, please use the server command.");
            }
        }
        _ => {
            println!("This case is impossible to reach. If you have reached here, something terrible has happened")
        }
    }
}

fn main() {
    println!(
        r#"                       ,dPYb,         8I                                                         8I"#
    );
    println!(
        r#"                       IP'`Yb         8I                                                         8I"#
    );
    println!(
        r#"                       I8  8I         8I                                                         8I"#
    );
    println!(
        r#"                       I8  8'         8I                                                         8I"#
    );
    println!(
        r#",gggg,gg    ,ggggg,    I8 dP    ,gggg,8I   ,ggg,    ,ggg,,ggg,    ,gggggg,    ,ggggg,      ,gggg,8I"#
    );
    println!(
        r#"dP"  "Y8I   dP"  "Y8ggg I8dP    dP"  "Y8I  i8" "8i  ,8" "8P" "8,   dP""""8I   dP"  "Y8ggg  dP"  "Y8I"#
    );
    println!(
        r#"i8'    ,8I  i8'    ,8I   I8P    i8'    ,8I  I8, ,8I  I8   8I   8I  ,8'    8I  i8'    ,8I   i8'    ,8I"#
    );
    println!(
        r#",d8,   ,d8I ,d8,   ,d8'  ,d8b,_ ,d8,   ,d8b, `YbadP' ,dP   8I   Yb,,dP     Y8,,d8,   ,d8'  ,d8,   ,d8b,"#
    );
    println!(
        r#"P"Y8888P"888P"Y8888P"    8P'"Y88P"Y8888P"`Y8888P"Y8888P'   8I   `Y88P      `Y8P"Y8888P"    P"Y8888P"`Y8"#
    );
    println!(r#"   ,d8I'"#);
    println!(r#" ,dP'8I                  VERSION 0.4.0"#);
    println!(r#",8"  8I"#);
    println!(r#"I8   8I"#);
    println!(r#"`8, ,8I"#);
    println!(r#"` Y8P""#);
    println!("\nEnter your commands on the line below.");
    println!("Feel free to use the 'help' command!");
    process_input();
}

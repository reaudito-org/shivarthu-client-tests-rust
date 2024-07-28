use crate::config::constants::WEBDRIVER_URL;
use std::process::{Command, Stdio};
use std::time::Duration;
use thirtyfour::prelude::*;

pub async fn initialize_driver() -> WebDriver {
    let mut caps = DesiredCapabilities::firefox();
    let _ = caps.add_arg("-profile");
    let _ =
        caps.add_arg("/home/amiya/snap/firefox/common/.mozilla/firefox/0m6bcq5b.shivarthu");

    let driver = WebDriver::new(WEBDRIVER_URL, caps).await.unwrap();
    driver
}

pub fn remove_port() {
    let status = Command::new("fuser")
        .arg("-n")
        .arg("tcp")
        .arg("-k")
        .arg("4444")
        .status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("fuser command executed successfully");
            } else {
                eprintln!("fuser command failed: {:?}", exit_status);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute fuser command: {}", e);
        }
    }
}

pub async fn run_webdriver() {
    let geckodriver_path = "geckodriver/geckodriver";
    let status = Command::new(geckodriver_path)
        .arg("--port")
        .arg("4444")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    // Check if the command was executed successfully
    match status {
        Ok(_) => {
            // Give some time for geckodriver to initialize
            tokio::time::sleep(Duration::from_secs(2)).await;

            println!("geckodriver started successfully on port 4444");
        }
        Err(e) => {
            eprintln!("Failed to execute geckodriver: {}", e);
        }
    }
}

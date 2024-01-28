use crate::config::constants::{WEBDRIVER_URL, WEBPAGE_URL};
use std::time::Duration;
use std::{thread, time};
use thirtyfour::extensions::query::conditions;
use thirtyfour::prelude::*;
use thirtyfour::Key;
use tokio::time::sleep;

// Define a struct to hold the WebDriver instance.
pub struct AccountHandle {
    driver: WebDriver,
}

impl AccountHandle {
    // Implement methods for the WebDriver struct.

    pub async fn new(driver: WebDriver) -> Result<Self, WebDriverError> {
        // Initialize WebDriverSession here.
        // You need to provide the appropriate WebDriver URL (e.g., "http://localhost:4444/wd/hub").
        Ok(AccountHandle { driver })
    }

    pub async fn sign_in(&self, account: &serde_json::Value) -> WebDriverResult<()> {
        // Load the Google homepage
        self.driver.goto(WEBPAGE_URL).await?;
        thread::sleep(time::Duration::from_secs(2));
    
        // Find the button by its ID
        let sign_in_button_id = "sign-in-button";
        let button = self.driver.find(By::Id(sign_in_button_id)).await?;
    
        // Click the button
        button.click().await?;
        thread::sleep(time::Duration::from_secs(3));
    
        // Find seed input by XPath
        let seed_input_xpath = "//*[contains(@name, 'seed')]";
        let seed_input = self.driver.find(By::XPath(seed_input_xpath)).await?;
        
        // Send keys to seed input
        seed_input.send_keys(account["seed"].as_str().unwrap()).await?;
        seed_input.send_keys(Key::Return.to_string()).await?;
        
        // Find password input by XPath
        let password_input_xpath = "//*[contains(@name, 'password')]";
        let password_input = self.driver.find(By::XPath(password_input_xpath)).await?;
        
        // Send keys to password input
        password_input.send_keys(account["password"].as_str().unwrap()).await?;
        password_input.send_keys(Key::Return.to_string()).await?;
        
        thread::sleep(time::Duration::from_secs(5));
    
        // Find seed submit button by ID
        let seed_submit_button_id = "seed-submit-button";
        let seed_submit_button = self.driver.find(By::Id(seed_submit_button_id)).await?;
        
        // Submit the seed submit button
        seed_submit_button.click().await?;
        
        thread::sleep(time::Duration::from_secs(5));
    
        Ok(())
    }
    

    // Implement other methods...
}

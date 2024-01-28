use crate::config::constants::WEBPAGE_URL;
use crate::config::delay_time::delay_for;
use std::time::Duration;
use std::{thread, time};
use thirtyfour::prelude::*;
use thirtyfour::Key;

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
        seed_input
            .send_keys(account["seed"].as_str().unwrap())
            .await?;
        seed_input.send_keys(Key::Return.to_string()).await?;

        // Find password input by XPath
        let password_input_xpath = "//*[contains(@name, 'password')]";
        let password_input = self.driver.find(By::XPath(password_input_xpath)).await?;

        // Send keys to password input
        password_input
            .send_keys(account["password"].as_str().unwrap())
            .await?;
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

    pub async fn sign_in_contract(&self, account: &serde_json::Value) -> WebDriverResult<()> {
        delay_for(Duration::from_secs(50)).await;
        let input_password = self.driver.find(By::Id("input-password")).await?;
        input_password
            .send_keys(account["password"].as_str().unwrap())
            .await?;

        // Sleep for 5 seconds
        delay_for(Duration::from_secs(5)).await;

        // Send RETURN key
        input_password.send_keys(Key::Return.to_string()).await?;

        // Find the submit button by XPath and submit
        let submit_button = self
            .driver
            .find(By::XPath("//*[contains(@type, 'submit')]"))
            .await?;
        submit_button.click().await?;

        // Sleep for 20 seconds
        thread::sleep(Duration::from_secs(20));

        Ok(())
    }

    pub async fn add_balance_transfer(
        &self,
        to_account: &serde_json::Value,
        amount: u64,
    ) -> WebDriverResult<()> {
        self.driver
            .goto(format!("{}/transfer-balance", WEBPAGE_URL))
            .await?;

        // Sleep for 5 seconds
        delay_for(Duration::from_secs(5)).await;

        // Find the destination account input and send keys
        let destination_account = self
            .driver
            .find(By::XPath("//*[contains(@name, 'destination_account')]"))
            .await?;
        destination_account
            .send_keys(to_account["public_key"].as_str().unwrap())
            .await?;
        
        delay_for(Duration::from_secs(2)).await;

        // Find the transfer balance input and send keys
        let transfer_balance = self
            .driver
            .find(By::XPath("//*[contains(@name, 'transfer_balance')]"))
            .await?;
        transfer_balance.send_keys(amount.to_string()).await?;


        delay_for(Duration::from_secs(2)).await;

        // Find seed submit button by ID
        let submit_button_id = "tranfer-balance-submit";
        let submit_button = self.driver.find(By::Id(submit_button_id)).await?;

        delay_for(Duration::from_secs(2)).await;

        submit_button.click().await?;
        Ok(())
    }

    // Implement other methods...
}

use crate::config::constants::WEBPAGE_URL;
use std::{thread, time};
use thirtyfour::prelude::*;

// Define a struct to hold the WebDriver instance.
pub struct PolkadotjsStruct {
    driver: WebDriver,
}

impl PolkadotjsStruct {
    // Implement methods for the WebDriver struct.

    pub async fn new(driver: WebDriver) -> Result<Self, WebDriverError> {
        // Initialize WebDriverSession here.
        // You need to provide the appropriate WebDriver URL (e.g., "http://localhost:4444/wd/hub").
        Ok(PolkadotjsStruct { driver })
    }

    pub async fn got_page(&self) -> WebDriverResult<()> {
        self.driver
            .goto(format!("{}/polkadotjs", WEBPAGE_URL))
            .await?;
        thread::sleep(time::Duration::from_secs(3));
        Ok(())
    }
}

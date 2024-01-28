use crate::accounts_handle::accounts_function::AccountHandle;
use crate::config::accounts::get_account_info;
use crate::config::constants::{WEBDRIVER_URL, WEBPAGE_URL};
use std::time::Duration;
use std::{thread, time};
use thirtyfour::extensions::query::conditions;
use thirtyfour::prelude::*;
use thirtyfour::Key;
use tokio::time::sleep;

// Define a struct to hold the WebDriver instance.
pub struct BalanceTransfer {
    driver: WebDriver,
}

impl BalanceTransfer {
    pub async fn new(driver: WebDriver) -> Result<Self, WebDriverError> {
        // Initialize WebDriverSession here.
        // You need to provide the appropriate WebDriver URL (e.g., "http://localhost:4444/wd/hub").
        Ok(BalanceTransfer { driver })
    }

    pub async fn transfer_balance(&self) -> WebDriverResult<()> {
        let accounts_info = get_account_info();

        let accounts_handle = AccountHandle::new(self.driver.clone()).await?;
        accounts_handle.sign_in(&accounts_info["bob"]).await?;
        let (to_account, amount) = (&accounts_info["charlie"], 1000000000000000);
        accounts_handle
            .add_balance_transfer(to_account, amount)
            .await?;
        accounts_handle
            .sign_in_contract(&accounts_info["bob"])
            .await?;

        Ok(())
    }

    // Implement other methods...
}

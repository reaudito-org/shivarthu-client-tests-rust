use crate::config::accounts::get_accounts_from_ext;
use crate::config::constants::WEBPAGE_URL;
use std::env;
use std::time::Duration;
use std::{thread, time};
use thirtyfour::extensions::query::conditions;
use thirtyfour::prelude::*;
use thirtyfour::Key;
use tokio::time::sleep;

// Define a struct to hold the WebDriver instance.
pub struct PositiveExternalityStruct {
    driver: WebDriver,
}

impl PositiveExternalityStruct {
    // Implement methods for the WebDriver struct.

    pub async fn new(driver: WebDriver) -> Result<Self, WebDriverError> {
        // Initialize WebDriverSession here.
        // You need to provide the appropriate WebDriver URL (e.g., "http://localhost:4444/wd/hub").
        Ok(PositiveExternalityStruct { driver })
    }

    pub async fn create_post(&self) -> WebDriverResult<()> {
        self.driver
            .goto(format!("{}/positive-externality/create-post", WEBPAGE_URL))
            .await?;
        thread::sleep(time::Duration::from_secs(3));

        // Find details_input element and send keys.
        let details_input = self
            .driver
            .find(By::XPath(
                "//*[contains(@name, 'positive-externality-post')]",
            ))
            .await?;
        let details_data =
            r#""Alice in Wonderland" is a timeless literary classic written by Lewis Carroll."#;
        details_input.send_keys(details_data).await?;

        sleep(Duration::from_secs(20)).await;

        // Find and submit the submit_button element.
        let submit_button = self
            .driver
            .find(By::XPath("//*[contains(@type, 'submit')]"))
            .await?;
        submit_button.click().await?;

        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();

        // println!("{:?}", address);

        // Sleep for 5 seconds (similar to the time.sleep(5) in Python code).
        sleep(Duration::from_secs(10)).await;

        let select_account_button = self.driver.find(By::Id("select-account")).await?;
        select_account_button.click().await?;

        let select_account = self.driver.find(By::Id(address)).await?;
        select_account.click().await?;

        Ok(())
    }

    pub async fn apply_staking_period(&self) -> WebDriverResult<()> {
        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();
        self.driver
            .goto(format!(
                "{}/positive-externality/apply-staking-period/{}",
                WEBPAGE_URL, address
            ))
            .await?;

        sleep(Duration::from_secs(15)).await;

        let submit_button = self
            .driver
            .find(By::XPath("//*[contains(@type, 'submit')]"))
            .await?;
        submit_button.click().await?;

        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();

        // println!("{:?}", address);

        // Sleep for 5 seconds (similar to the time.sleep(5) in Python code).
        sleep(Duration::from_secs(10)).await;

        let select_account_button = self.driver.find(By::Id("select-account")).await?;
        select_account_button.click().await?;

        let select_account = self.driver.find(By::Id(address)).await?;
        select_account.click().await?;

        Ok(())
    }
    pub async fn add_profile_stake(&self, stake: String, account_cut: &str) -> WebDriverResult<()> {
        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();
        self.driver
            .goto(format!("{}/add-profile-stake/{}", WEBPAGE_URL, address))
            .await?;
        sleep(Duration::from_secs(5)).await;
        let profile_stake = self.driver.find(By::Id("profile-stake")).await?;
        profile_stake.send_keys(stake).await?;
        sleep(Duration::from_secs(5)).await;
        let submit_button = self
            .driver
            .find(By::XPath("//*[contains(@type, 'submit')]"))
            .await?;
        submit_button.click().await?;
        sleep(Duration::from_secs(5)).await;

        let select_account_button = self.driver.find(By::Id("select-account")).await?;
        select_account_button.click().await?;

        let select_account = self.driver.find(By::Id(account_cut)).await?;
        select_account.click().await?;

        Ok(())
    }
    pub async fn challenge_evidence(&self, account_cut: &str) -> WebDriverResult<()> {
        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();
        self.driver
            .goto(format!(
                "{}/positive-externality/schelling-game/{}",
                WEBPAGE_URL, address
            ))
            .await?;

        sleep(Duration::from_secs(15)).await;
        let details_input = self
            .driver
            .find(By::XPath("//*[contains(@name,  'challenge-details')]"))
            .await?;
        let details_data = r#"Profile is invalid"#;
        details_input.send_keys(details_data).await?;

        sleep(Duration::from_secs(5)).await;

        // Find and submit the submit_button element.
        let submit_button = self
            .driver
            .find(By::Id("challenge-evidence-submit"))
            .await?;
        submit_button.click().await?;
        sleep(Duration::from_secs(15)).await;

        let select_account_button = self.driver.find(By::Id("select-account")).await?;
        select_account_button.click().await?;

        let select_account = self.driver.find(By::Id(account_cut)).await?;
        select_account.click().await?;

        Ok(())
    }

    pub async fn schelling_game_page(&self) -> WebDriverResult<()> {
        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();
        self.driver
            .goto(format!(
                "{}/positive-externality/schelling-game/{}",
                WEBPAGE_URL, address
            ))
            .await?;
        Ok(())
    }
    pub async fn apply_juror(&self, account_stake: &str, stake: &str) -> WebDriverResult<()> {
        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();
        self.driver
            .goto(format!(
                "{}/positive-externality/schelling-game/{}",
                WEBPAGE_URL, address
            ))
            .await?;

        sleep(Duration::from_secs(5)).await;
        let stake_element = self.driver.find(By::Id("juror-stake")).await?;
        stake_element.send_keys(stake).await?;

        sleep(Duration::from_secs(5)).await;

        // Find and submit the submit_button element.
        let submit_button = self.driver.find(By::Id("apply-juror-submit")).await?;
        submit_button.click().await?;
        sleep(Duration::from_secs(15)).await;

        let select_account_button = self.driver.find(By::Id("select-account")).await?;
        select_account_button.click().await?;

        let select_account = self.driver.find(By::Id(account_stake)).await?;
        select_account.click().await?;

        Ok(())
    }

    pub async fn change_period(&self, account: &str) -> WebDriverResult<()> {
        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();
        self.driver
            .goto(format!(
                "{}/positive-externality/schelling-game/{}",
                WEBPAGE_URL, address
            ))
            .await?;

        sleep(Duration::from_secs(5)).await;
        let submit_button = self.driver.find(By::Id("change-period-submit")).await?;
        submit_button.click().await?;
        sleep(Duration::from_secs(15)).await;

        let select_account_button = self.driver.find(By::Id("select-account")).await?;
        select_account_button.click().await?;

        let select_account = self.driver.find(By::Id(account)).await?;
        select_account.click().await?;

        Ok(())
    }

    pub async fn draw_jurors(
        &self,
        draw_number: &str,
        account_for_draw: &str,
    ) -> WebDriverResult<()> {
        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();
        self.driver
            .goto(format!(
                "{}/positive-externality/schelling-game/{}",
                WEBPAGE_URL, address
            ))
            .await?;

        sleep(Duration::from_secs(5)).await;
        let draw_number_element = self.driver.find(By::Id("iterations")).await?;
        draw_number_element.send_keys(draw_number).await?;

        sleep(Duration::from_secs(5)).await;

        // Find and submit the submit_button element.
        let submit_button = self.driver.find(By::Id("draw-jurors-submit")).await?;
        submit_button.click().await?;
        sleep(Duration::from_secs(15)).await;

        let select_account_button = self.driver.find(By::Id("select-account")).await?;
        select_account_button.click().await?;

        let select_account = self.driver.find(By::Id(account_for_draw)).await?;
        select_account.click().await?;

        Ok(())
    }

    pub async fn juror_selected_check(&self, account_to_check: &str) -> WebDriverResult<()> {
        let accounts_info = get_accounts_from_ext();
        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();
        self.driver
            .goto(format!(
                "{}/positive-externality-juror-selected/{}",
                WEBPAGE_URL, address
            ))
            .await?;

        sleep(Duration::from_secs(5)).await;
        let juror_address_element = self.driver.find(By::Id("juror-address-checking")).await?;
        juror_address_element.send_keys(account_to_check).await?;

        sleep(Duration::from_secs(5)).await;

        Ok(())
    }

    pub async fn commit_vote(
        &self,
        vote_string: &str,
        account_for_commit: &str,
    ) -> WebDriverResult<()> {
        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();
        self.driver
            .goto(format!(
                "{}/positive-externality/schelling-game/{}",
                WEBPAGE_URL, address
            ))
            .await?;

        sleep(Duration::from_secs(5)).await;
        let commit_vote_element = self.driver.find(By::Id("commit-vote")).await?;
        commit_vote_element.send_keys(vote_string).await?;

        sleep(Duration::from_secs(5)).await;

        // Find and submit the submit_button element.
        let submit_button = self.driver.find(By::Id("commit-vote-submit")).await?;
        submit_button.click().await?;
        sleep(Duration::from_secs(15)).await;

        let select_account_button = self.driver.find(By::Id("select-account")).await?;
        select_account_button.click().await?;

        let select_account = self.driver.find(By::Id(account_for_commit)).await?;
        select_account.click().await?;

        Ok(())
    }

    pub async fn reveal_vote(
        &self,
        choice: &str,
        salt: &str,
        account_for_reveal: &str,
    ) -> WebDriverResult<()> {
        let accounts_info = get_accounts_from_ext();

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();
        self.driver
            .goto(format!(
                "{}/positive-externality/schelling-game/{}",
                WEBPAGE_URL, address
            ))
            .await?;

        sleep(Duration::from_secs(5)).await;
        let choice_element = self.driver.find(By::Id("choice")).await?;
        choice_element.send_keys(choice).await?;
        let salt_element = self.driver.find(By::Id("salt")).await?;
        salt_element.send_keys(salt).await?;

        sleep(Duration::from_secs(5)).await;

        // Find and submit the submit_button element.
        let submit_button = self.driver.find(By::Id("reveal-vote-submit")).await?;
        submit_button.click().await?;
        sleep(Duration::from_secs(15)).await;

        let select_account_button = self.driver.find(By::Id("select-account")).await?;
        select_account_button.click().await?;

        let select_account = self.driver.find(By::Id(account_for_reveal)).await?;
        select_account.click().await?;

        Ok(())
    }
}

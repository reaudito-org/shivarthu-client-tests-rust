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
pub struct ProfileValidationStruct {
    driver: WebDriver,
}

impl ProfileValidationStruct {
    // Implement methods for the WebDriver struct.

    pub async fn new(driver: WebDriver) -> Result<Self, WebDriverError> {
        // Initialize WebDriverSession here.
        // You need to provide the appropriate WebDriver URL (e.g., "http://localhost:4444/wd/hub").
        Ok(ProfileValidationStruct { driver })
    }

    pub async fn add_profile(&self) -> WebDriverResult<()> {
        self.driver
            .goto(format!("{}/add-profile", WEBPAGE_URL))
            .await?;
        thread::sleep(time::Duration::from_secs(3));

        // Find name_input element and send keys.
        let name_input = self.driver.find(By::Id("profile-name")).await?;
        name_input.send_keys("Alice in Wonderland").await?;
        // name_input.send_keys(Key::Tab.to_string()).await?;

        // Find details_input element and send keys.
        let details_input = self
            .driver
            .find(By::XPath("//*[contains(@name, 'profile-details')]"))
            .await?;
        let details_data =
            r#""Alice in Wonderland" is a timeless literary classic written by Lewis Carroll."#;
        details_input.send_keys(details_data).await?;
        // details_input.send_keys(Key::Tab.to_string()).await?;

        // Find file_input element and send file data.
        let file_input = self.driver.find(By::Id("file-upload")).await?;
        let mut project_root = String::new(); // Replace with your actual project root
        if let Ok(current_dir) = env::current_dir() {
            project_root = format!("{}", current_dir.display());
        }
        let file_data = format!("{}/files/movie.mp4", project_root);
        println!("Project Root: {}", project_root);
        println!("File Data: {}", file_data);
        file_input.send_keys(file_data).await?;

        // file_input.send_keys(Key::Tab.to_string()).await?;

        // let elem = self
        //     .driver
        //     .query(By::Id("profile-video-load"))
        //     .first()
        //     .await?;

        // // Wait for the element to be displayed
        // elem.wait_until().displayed().await?;
        // Wait until the element with ID "profile-video-load" becomes clickable.

        // Sleep for 3 seconds (similar to the time.sleep(3) in Python code).
        sleep(Duration::from_secs(20)).await;

        // Find and submit the submit_button element.
        let submit_button = self
            .driver
            .find(By::XPath("//*[contains(@type, 'submit')]"))
            .await?;
        submit_button.click().await?;

        let accounts_info = get_accounts_from_ext(); 

        let address = accounts_info["account1"]["ss58_address"].as_str().unwrap();        
        
        println!("{:?}", address);

        // Sleep for 5 seconds (similar to the time.sleep(5) in Python code).
        sleep(Duration::from_secs(5)).await;

        Ok(())
    }

    // Implement other methods...
}

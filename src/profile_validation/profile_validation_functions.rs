use crate::config::constants::{WEBDRIVER_URL, WEBPAGE_URL};
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
        thread::sleep(time::Duration::from_secs(2));

        // Find name_input element and send keys.
        let name_input = self
            .driver
            .find(By::XPath("//*[contains(@name, 'name')]"))
            .await?;
        name_input.send_keys("Alice in Wonderland").await?;
        name_input.send_keys(Key::Return.to_string()).await?;

        // Find details_input element and send keys.
        let details_input = self
            .driver
            .find(By::XPath("//*[contains(@name, 'details')]"))
            .await?;
        let details_data =
            r#""Alice in Wonderland" is a timeless literary classic written by Lewis Carroll."#;
        details_input.send_keys(details_data).await?;
        details_input.send_keys(Key::Return.to_string()).await?;
        // Execute JavaScript to blur the details_input.
        self.driver
            .execute_async(r#"arguments[0].blur();"#, vec![details_input.to_json()?])
            .await?;

        // Find file_input element and send file data.
        let file_input = self.driver.find(By::Id("file-upload")).await?;
        let project_root = "your_project_root_here"; // Replace with your actual project root
        let file_data = format!("{}/files/movie.mp4", project_root);
        println!("Project Root: {}", project_root);
        println!("File Data: {}", file_data);
        file_input.send_keys(file_data).await?;

        // Find the element
        let elem = self.driver.find(By::Id("profile-video-load")).await?;

        // Wait for the element to be displayed
        elem.wait_until()
            .conditions(vec![conditions::element_is_displayed(true)])
            .await?;

        // Wait until the element with ID "profile-video-load" becomes clickable.

        // Sleep for 3 seconds (similar to the time.sleep(3) in Python code).
        sleep(Duration::from_secs(3)).await;

        // Find and submit the submit_button element.
        let submit_button = self
            .driver
            .find(By::XPath("//*[contains(@type, 'submit')]"))
            .await?;
        submit_button.click().await?;

        // Sleep for 5 seconds (similar to the time.sleep(5) in Python code).
        sleep(Duration::from_secs(5)).await;

        Ok(())
    }

    // Implement other methods...
}

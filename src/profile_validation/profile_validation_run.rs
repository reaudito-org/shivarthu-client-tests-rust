use crate::common::prompt::prompt;
use crate::profile_validation::profile_validation_functions::ProfileValidationStruct;
use thirtyfour::prelude::*;
pub async fn profile_validation_run(driver: WebDriver) -> WebDriverResult<()> {
    let input = prompt()?;
    if input == "n" {
        println!("You entered 'n'.");
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation.add_profile().await?;
    } else {
        panic!("You have not entered n");
    }

    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
    } else {
        panic!("You have not entered n");
    }

    Ok(())
}

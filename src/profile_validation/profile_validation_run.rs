use crate::common::prompt::prompt;
use crate::config::accounts::get_accounts_from_ext;
use crate::profile_validation::profile_validation_functions::ProfileValidationStruct;
use thirtyfour::prelude::*;
pub async fn profile_validation_run(driver: WebDriver) -> WebDriverResult<()> {
    // Add profile

    let input = prompt()?;
    if input == "n" {
        println!("You entered 'n'.");
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation.add_profile().await?;
    } else {
        panic!("You have not entered n");
    }

    // View Profile

    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation.view_profile().await?;
    } else {
        panic!("You have not entered n");
    }

    // Add profile stake 1

    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
        let accounts_info = get_accounts_from_ext();
        let account_cut = accounts_info["account2"]["ss58_address"].as_str().unwrap();
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation
            .add_profile_stake("500".to_string(), account_cut)
            .await?;
    } else {
        panic!("You have not entered n");
    }

    // Add profile stake 2

    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
        let accounts_info = get_accounts_from_ext();
        let account_cut = accounts_info["account3"]["ss58_address"].as_str().unwrap();
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation
            .add_profile_stake("500".to_string(), account_cut)
            .await?;
    } else {
        panic!("You have not entered n");
    }

    Ok(())
}

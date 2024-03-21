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

    // Challenge evidence

    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
        let accounts_info = get_accounts_from_ext();
        let account_cut = accounts_info["account4"]["ss58_address"].as_str().unwrap();
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation.challenge_evidence(account_cut).await?;
    } else {
        panic!("You have not entered n");
    }

    // Juror stake 1
    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
        let accounts_info = get_accounts_from_ext();
        let account_stake = accounts_info["account5"]["ss58_address"].as_str().unwrap();
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation.juror_stake(account_stake, "500").await?;
    } else {
        panic!("You have not entered n");
    }

    // Juror stake 2
    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
        let accounts_info = get_accounts_from_ext();
        let account_stake = accounts_info["account6"]["ss58_address"].as_str().unwrap();
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation.juror_stake(account_stake, "800").await?;
    } else {
        panic!("You have not entered n");
    }

    // Juror stake 3
    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
        let accounts_info = get_accounts_from_ext();
        let account_stake = accounts_info["account7"]["ss58_address"].as_str().unwrap();
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation
            .juror_stake(account_stake, "1500")
            .await?;
    } else {
        panic!("You have not entered n");
    }

    // Juror stake 4
    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
        let accounts_info = get_accounts_from_ext();
        let account_stake = accounts_info["account8"]["ss58_address"].as_str().unwrap();
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation
            .juror_stake(account_stake, "2000")
            .await?;
    } else {
        panic!("You have not entered n");
    }

    // Juror stake 5
    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
        let accounts_info = get_accounts_from_ext();
        let account_stake = accounts_info["account9"]["ss58_address"].as_str().unwrap();
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation
            .juror_stake(account_stake, "2500")
            .await?;
    } else {
        panic!("You have not entered n");
    }

    // Juror stake 6
    let input = prompt()?;

    if input == "n" {
        println!("You entered 'n'.");
        let accounts_info = get_accounts_from_ext();
        let account_stake = accounts_info["account10"]["ss58_address"].as_str().unwrap();
        let profile_validation = ProfileValidationStruct::new(driver.clone()).await?;
        profile_validation
            .juror_stake(account_stake, "2800")
            .await?;
    } else {
        panic!("You have not entered n");
    }
    Ok(())
}

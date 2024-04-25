use crate::common::prompt::prompt;
use crate::config::accounts::get_accounts_from_ext;
use crate::positive_externality::positive_externality_functions::PositiveExternalityStruct;
use thirtyfour::prelude::*;
pub async fn positive_externality_run(driver: WebDriver) -> WebDriverResult<()> {
    loop {
        let input = prompt("Enter the number")?;
        let trimmed_input = input.trim();

        if trimmed_input.to_lowercase() == "q" {
            println!("Exiting...");
            break;
        }

        let number: u32 = match trimmed_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number or 'q' to quit!");
                continue;
            }
        };

        match number {
            // Add profile
            1 => {
                println!("You entered '1'.");
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality.create_post().await?;
            }

            2 => {
                // View Profile
                println!("You entered '2'.");
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality.view_profile().await?;
            }

            3 => {
                // Profile stake 1
                println!("You entered '3'.");
                let accounts_info = get_accounts_from_ext();
                let account_cut = accounts_info["account2"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .add_profile_stake("500".to_string(), account_cut)
                    .await?;
            }
            4 => {
                // Profile stake 2
                println!("You entered '4'.");
                let accounts_info = get_accounts_from_ext();
                let account_cut = accounts_info["account3"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .add_profile_stake("500".to_string(), account_cut)
                    .await?;
            }
            5 => {
                println!("You entered '5'.");
                let accounts_info = get_accounts_from_ext();
                let account_cut = accounts_info["account4"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality.challenge_evidence(account_cut).await?;
            }

            6 => {
                // Staking page
                println!("You entered '6'.");
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality.schelling_game_page().await?;
            }

            7 => {
                // Juror stake 1
                println!("You entered '7'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account5"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality.apply_juror(account_stake, "500").await?;
            }
            8 => {
                // Juror stake 2
                println!("You entered '8'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account6"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality.apply_juror(account_stake, "800").await?;
            }
            9 => {
                // Juror stake 3
                println!("You entered '9'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account7"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .apply_juror(account_stake, "1500")
                    .await?;
            }
            10 => {
                // Juror stake 4
                println!("You entered '10'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account8"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .apply_juror(account_stake, "2000")
                    .await?;
            }
            11 => {
                // Juror stake 5
                println!("You entered '11'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account9"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .apply_juror(account_stake, "2500")
                    .await?;
            }
            12 => {
                // Juror stake 6
                println!("You entered '12'.");
                let accounts_info = get_accounts_from_ext();
                let account_stake = accounts_info["account10"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .apply_juror(account_stake, "2800")
                    .await?;
            }

            13 => {
                // Change Period
                println!("You entered '13'");
                let accounts_info = get_accounts_from_ext();
                let account = accounts_info["account11"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality.change_period(account).await?;
            }
            14 => {
                // Change Period
                println!("You entered '14'");
                let accounts_info = get_accounts_from_ext();
                let account_for_draw = accounts_info["account11"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .draw_jurors("3", account_for_draw)
                    .await?;
            }
            15 => {
                // Change Period
                println!("You entered '15'");
                let accounts_info = get_accounts_from_ext();
                let account_for_draw = accounts_info["account11"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .draw_jurors("2", account_for_draw)
                    .await?;
            }
            16 => {
                // Change Period
                println!("You entered '16'");
                let accounts_info = get_accounts_from_ext();
                let account = accounts_info["account11"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality.change_period(account).await?;
            }
            17 => {
                // Change Period
                println!("You entered '17'");
                let accounts_info = get_accounts_from_ext();
                let account_to_check = accounts_info["account5"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .juror_selected_check(account_to_check)
                    .await?;
                let input = prompt("Selected as juror y/n")?;
                let trimmed_input = input.trim();

                if trimmed_input.to_lowercase() == "y" {
                    println!("Selected as juror");
                    let vote_string = "0account5";
                    positive_externality
                        .commit_vote(vote_string, account_to_check)
                        .await?;
                } else {
                    println!("Not selected as juror");
                }
            }
            18 => {
                // Change Period
                println!("You entered '18'");
                let accounts_info = get_accounts_from_ext();
                let account_to_check = accounts_info["account6"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .juror_selected_check(account_to_check)
                    .await?;
                let input = prompt("Selected as juror y/n")?;
                let trimmed_input = input.trim();

                if trimmed_input.to_lowercase() == "y" {
                    println!("Selected as juror");
                    let vote_string = "0account6";
                    positive_externality
                        .commit_vote(vote_string, account_to_check)
                        .await?;
                } else {
                    println!("Not selected as juror");
                }
            }
            19 => {
                // Change Period
                println!("You entered '19'");
                let accounts_info = get_accounts_from_ext();
                let account_to_check = accounts_info["account7"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .juror_selected_check(account_to_check)
                    .await?;
                let input = prompt("Selected as juror y/n")?;
                let trimmed_input = input.trim();

                if trimmed_input.to_lowercase() == "y" {
                    println!("Selected as juror");
                    let vote_string = "0account7";
                    positive_externality
                        .commit_vote(vote_string, account_to_check)
                        .await?;
                } else {
                    println!("Not selected as juror");
                }
            }
            20 => {
                // Change Period
                println!("You entered '20'");
                let accounts_info = get_accounts_from_ext();
                let account_to_check = accounts_info["account8"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .juror_selected_check(account_to_check)
                    .await?;
                let input = prompt("Selected as juror y/n")?;
                let trimmed_input = input.trim();

                if trimmed_input.to_lowercase() == "y" {
                    println!("Selected as juror");
                    let vote_string = "1account8";
                    positive_externality
                        .commit_vote(vote_string, account_to_check)
                        .await?;
                } else {
                    println!("Not selected as juror");
                }
            }
            21 => {
                // Change Period
                println!("You entered '21'");
                let accounts_info = get_accounts_from_ext();
                let account_to_check = accounts_info["account9"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .juror_selected_check(account_to_check)
                    .await?;
                let input = prompt("Selected as juror y/n")?;
                let trimmed_input = input.trim();

                if trimmed_input.to_lowercase() == "y" {
                    println!("Selected as juror");
                    let vote_string = "1account9";
                    positive_externality
                        .commit_vote(vote_string, account_to_check)
                        .await?;
                } else {
                    println!("Not selected as juror");
                }
            }
            22 => {
                // Change Period
                println!("You entered '22'");
                let accounts_info = get_accounts_from_ext();
                let account_to_check = accounts_info["account10"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                positive_externality
                    .juror_selected_check(account_to_check)
                    .await?;
                let input = prompt("Selected as juror y/n")?;
                let trimmed_input = input.trim();

                if trimmed_input.to_lowercase() == "y" {
                    println!("Selected as juror");
                    let vote_string = "1account10";
                    positive_externality
                        .commit_vote(vote_string, account_to_check)
                        .await?;
                } else {
                    println!("Not selected as juror");
                }
            }
            23 => {
                println!("You entered '23'");
                let accounts_info = get_accounts_from_ext();
                let account_for_reveal =
                    accounts_info["account5"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                let choice = "0";
                let salt = "account5";
                positive_externality
                    .reveal_vote(choice, salt, account_for_reveal)
                    .await?;
            }
            24 => {
                println!("You entered '24'");
                let accounts_info = get_accounts_from_ext();
                let account_for_reveal =
                    accounts_info["account6"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                let choice = "0";
                let salt = "account6";
                positive_externality
                    .reveal_vote(choice, salt, account_for_reveal)
                    .await?;
            }
            25 => {
                println!("You entered '25'");
                let accounts_info = get_accounts_from_ext();
                let account_for_reveal =
                    accounts_info["account7"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                let choice = "0";
                let salt = "account7";
                positive_externality
                    .reveal_vote(choice, salt, account_for_reveal)
                    .await?;
            }
            26 => {
                println!("You entered '26'");
                let accounts_info = get_accounts_from_ext();
                let account_for_reveal =
                    accounts_info["account8"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                let choice = "1";
                let salt = "account8";
                positive_externality
                    .reveal_vote(choice, salt, account_for_reveal)
                    .await?;
            }
            27 => {
                println!("You entered '27'");
                let accounts_info = get_accounts_from_ext();
                let account_for_reveal =
                    accounts_info["account9"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                let choice = "1";
                let salt = "account9";
                positive_externality
                    .reveal_vote(choice, salt, account_for_reveal)
                    .await?;
            }
            28 => {
                println!("You entered '28'");
                let accounts_info = get_accounts_from_ext();
                let account_for_reveal =
                    accounts_info["account10"]["ss58_address"].as_str().unwrap();
                let positive_externality = PositiveExternalityStruct::new(driver.clone()).await?;
                let choice = "1";
                let salt = "account10";
                positive_externality
                    .reveal_vote(choice, salt, account_for_reveal)
                    .await?;
            }
            _ => {
                println!("Please enter a valid number");
            }
        }
    }

    Ok(())
}

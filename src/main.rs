// use shivarthu_client_test::final_run::balance_transfers::BalanceTransfer;
use shivarthu_client_test::config::accounts::get_accounts_from_ext;
use shivarthu_client_test::polkadotjs::polkadotjs_test::PolkadotjsStruct;
use shivarthu_client_test::profile_validation::profile_validation_functions::ProfileValidationStruct;
use shivarthu_client_test::webdriver_run::webdriver_function::{
    initialize_driver, remove_port, run_webdriver,
};

use shivarthu_client_test::positive_externality::positive_externality_run::positive_externality_run;
use shivarthu_client_test::profile_validation::profile_validation_run::profile_validation_run;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    remove_port();
    run_webdriver().await;
    let driver = initialize_driver().await;
    let polkadotjs = PolkadotjsStruct::new(driver.clone()).await?;
    polkadotjs.got_page().await?;
    // profile_validation_run(driver.clone()).await?;

    positive_externality_run(driver.clone()).await?;

    // println!("{:?}", accounts_info);
    remove_port();

    Ok(())
}

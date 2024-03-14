// use shivarthu_client_test::final_run::balance_transfers::BalanceTransfer;
use shivarthu_client_test::polkadotjs::polkadotjs_test::PolkadotjsStruct;
use shivarthu_client_test::profile_validation::profile_validation_functions::ProfileValidationStruct;
use shivarthu_client_test::webdriver_run::webdriver_function::{
    initialize_driver, remove_port, run_webdriver,
};

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    remove_port();
    run_webdriver().await;
    let driver = initialize_driver().await;
    let polkadotjs = PolkadotjsStruct::new(driver.clone()).await?;
    polkadotjs.got_page().await?;

    let profile_validation = ProfileValidationStruct::new(driver).await?;
    profile_validation.add_profile().await?;
    remove_port();

    Ok(())
}

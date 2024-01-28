use shivarthu_client_test::final_run::balance_transfers::BalanceTransfer;
use shivarthu_client_test::webdriver_run::webdriver_function::{initialize_driver, remove_port, run_webdriver};

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    remove_port();
    run_webdriver().await;
    let driver = initialize_driver().await;

    let balance_tranfer_driver = BalanceTransfer::new(driver).await?;
    balance_tranfer_driver.transfer_balance().await?;
    remove_port();

    Ok(())
}

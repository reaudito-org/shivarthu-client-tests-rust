use shivarthu_client_test::config::constants::WEBDRIVER_URL;
use shivarthu_client_test::final_run::balance_transfers::BalanceTransfer;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::firefox();
    // let _ = caps.add_firefox_option(
    //     "profile",
    //     "/home/amiya/snap/firefox/common/.mozilla/firefox/0m6bcq5b.shivarthu",
    // );

    let driver = WebDriver::new(WEBDRIVER_URL, caps).await?;

    let balance_tranfer_driver = BalanceTransfer::new(driver).await?;
    balance_tranfer_driver.transfer_balance().await?;

    Ok(())
}

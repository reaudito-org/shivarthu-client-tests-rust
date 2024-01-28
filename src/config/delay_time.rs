
use std::time::Duration;
use thirtyfour::support::sleep;

pub async fn delay_for(duration: Duration) {
    sleep(duration).await;
}
use clap::{Parser};
use tokio;
use mmove::mouse;
use mmove::config;

const TO_CLICK_MINUTES: u64 = 28;
const BETWEEN_CLICK_SECONDS: u64 = 5;
const ROUND_PROGRESS_MILLIS: u64 = 50;

#[tokio::main]
async fn main() {
    let config = config::Config::parse();

    loop {
        let ctoken = tokio_util::sync::CancellationToken::new();
        let rmm_ctoken = ctoken.clone(); // cancellation token for the Round Mouse Mover task

        // spawn the Round Mouse Mover task
        let rmm_task = tokio::task::spawn(async move {
            let mut rmm_mover = mouse::mover::RoundMover::new();
            while !rmm_ctoken.is_cancelled() {
                rmm_mover.round_move();
                tokio::time::sleep(std::time::Duration::from_millis(ROUND_PROGRESS_MILLIS)).await;
            }
        });

        // spawn a task to kill the Round Mouse Mover task after TO_CLICK_MINUTES
        _ = tokio::task::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(TO_CLICK_MINUTES * 60)).await;
            ctoken.cancel();
        });

        // wait for the Round Mouse Mover task to finish
        rmm_task.await.unwrap();

        // spawn the Clicker task
        let clicker_task = tokio::task::spawn(async move {
            let mut clicker = mouse::clicker::OffsetClicker::new(
                config.x, config.y, config.v_offset
            );
            clicker.click();
            tokio::time::sleep(std::time::Duration::from_secs(BETWEEN_CLICK_SECONDS)).await;
            clicker.move_and_click();
            tokio::time::sleep(std::time::Duration::from_secs(BETWEEN_CLICK_SECONDS)).await;
        });

        // wait for the Clicker task to finish
        clicker_task.await.unwrap();
    }
}

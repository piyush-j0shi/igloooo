use anyhow::Result;
use std::time::Duration;
use trpl;

async fn fetch_user(_id: u32) -> Result<String> {
    trpl::sleep(Duration::from_secs(1)).await;
    Ok("user_fetched".to_string())
}

async fn fetch_orders(_user: &str) -> Result<Vec<String>> {
    trpl::sleep(Duration::from_secs(1)).await;
    Ok(vec!["order_1".to_string(), "order_2".to_string()])
}

async fn fetch_order_details(order: String) -> Result<String> {
    trpl::sleep(Duration::from_secs(1)).await;
    Ok(format!("{} fetched", order))
}

fn main() -> Result<()> {
    trpl::block_on(async {
        let user = fetch_user(1).await?;
        let user_orders = fetch_orders(&user).await?;
        let fetch_order_details = fetch_order_details(user).await?;
        println!(
            "user_order : {:?}, user_order_details : {}",
            user_orders, fetch_order_details
        );
        Ok(())
    })
}

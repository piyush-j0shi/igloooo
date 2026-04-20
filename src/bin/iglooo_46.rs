use anyhow::Result;
use std::time::Duration;
use trpl;

async fn slow_task(name: &str, steps: u32) -> Result<String> {
    for i in 0..steps {
        trpl::sleep(Duration::from_millis(500)).await;
        println!("{}: step {}/{}", name, i + 1, steps);
    }
    Ok(format!("{name} done"))
}

fn main() -> Result<()> {
    trpl::block_on(async {
        let results = trpl::join_all(vec![
            slow_task("task_1", 3),
            slow_task("task_2", 2),
            slow_task("task_3", 4),
        ])
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

        println!("results are : {:?}", results);

        Ok(())
    })
}

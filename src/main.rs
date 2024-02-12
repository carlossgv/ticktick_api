mod clients;
use crate::clients::ticktick;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let username = env::var("USERNAME").unwrap_or_else(|_| panic!("USERNAME must be set in ENV"));
    let password = env::var("PASSWORD").unwrap_or_else(|_| panic!("PASSWORD must be set in ENV"));

    // ticktick::login(&username, &password).await?;
    // ticktick::get_user_info().await?;
    let tasks: Vec<ticktick::Task> = vec![
        ticktick::Task {
            title: "Task 1 inbox".to_string(),
            project_id: None,
            id: None,
        },
        ticktick::Task {
            title: "Task 2 inbox".to_string(),
            project_id: None,
            id: None,
        },
    ];

    ticktick::handle_tasks(tasks, ticktick::Action::Add).await?;
    Ok(())
}

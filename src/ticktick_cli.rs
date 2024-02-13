use crate::{clients::ticktick_client::Action, ticktick_client};

pub async fn login(username: String, password: String) -> Result<(), Box<dyn std::error::Error>> {
    ticktick_client::login(username, password).await
}

pub async fn add_tasks(
    title: String,
    project_id: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let task = ticktick_client::TaskBody {
        title,
        project_id,
        id: None,
    };

    ticktick_client::handle_tasks(vec![task], Action::Add).await
}

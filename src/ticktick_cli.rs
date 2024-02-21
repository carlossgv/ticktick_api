use crate::{clients::ticktick_client::Action, ticktick_client};

pub async fn login(username: String, password: String) -> Result<(), Box<dyn std::error::Error>> {
    ticktick_client::login(username, password).await
}

pub async fn update_task(
    id: String,
    title: String,
    items: Option<Vec<ticktick_client::Item>>,
    project_id: String,
    desc: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let task = ticktick_client::TaskBody {
        title,
        project_id: Some(project_id),
        id: Some(id),
        items,
        desc,
    };

    ticktick_client::handle_tasks(vec![task], Action::Update).await
}

pub async fn add_task(
    title: String,
    items: Option<Vec<ticktick_client::Item>>,
    project_id: Option<String>,
    desc: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let task = ticktick_client::TaskBody {
        title,
        project_id,
        id: None,
        items,
        desc,
    };

    ticktick_client::handle_tasks(vec![task], Action::Add).await
}

use crate::utils::parse_user_data::UserData;
use dirs::home_dir;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;

const TICK_TICK_URL: &str = "https://api.ticktick.com/api/v2";
const X_DEVICE_HEADER: &str = r#"{"platform":"web","os":"macOS 10.15.7","device":"Chrome 121.0.0.0","name":"","version":5070,"id":"65bcdf6491ea1a2e7db71fbe","channel":"website","campaign":"","websocket":""}"#;
const DATA_FILE: &str = ".ticktick_data";

fn get_file_path(filename: &str) -> String {
    if let Some(home_dir) = home_dir() {
        return home_dir.join(filename).to_string_lossy().to_string();
    } else {
        panic!("Unable to determine the home directory.");
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginBody {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorLoginResponse {
    error_id: String,
    error_code: String,
    error_message: String,
    data: ErrorLoginResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorLoginResponseData {
    remainder_times: i32,
}

pub async fn login(username: String, password: String) -> Result<(), Box<dyn std::error::Error>> {
    let body = LoginBody { username, password };

    let client = reqwest::Client::new();

    let response = client
        .post(format!(
            "{}/user/signon?wc=true&remember=true",
            TICK_TICK_URL
        ))
        .header("content-type", "application/json")
        .header("x-device", X_DEVICE_HEADER)
        .body(serde_json::to_string(&body)?)
        .send()
        .await?;

    match response.status().is_success() {
        true => {
            let absolute_file_path: String = get_file_path(DATA_FILE);
            fs::write(&absolute_file_path, "")?;

            let mut session_cookies = Vec::<String>::new();
            for cookie in response.cookies() {
                if cookie.value() == "" {
                    continue;
                }
                session_cookies.push(format!("{}={}", cookie.name(), cookie.value()));
            }

            fs::write(absolute_file_path, session_cookies.join(";"))?;
            println!("Login successful!");
        }
        false => {
            let error_response: ErrorLoginResponse = response.json().await?;
            println!(
                "Login failed! Error Code: {}, Message: {}, ID: {}",
                error_response.error_code, error_response.error_message, error_response.error_id
            );
            println!("Remainder times: {}", error_response.data.remainder_times);
        }
    }

    Ok(())
}

pub fn get_session_cookies(absolute_file_path: &str) -> Vec<String> {
    if let Ok(session_cookies_str) = fs::read_to_string(absolute_file_path) {
        let session_cookies: Vec<String> = session_cookies_str
            .split(';')
            .map(|s| s.trim().to_string()) // Convert each slice to owned String
            .collect();

        return session_cookies;
    } else {
        println!("Something went wrong reading the file");
        todo!();
    }
}

pub async fn get_user_info() -> Result<UserData, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let absolute_file_path: String = get_file_path(DATA_FILE);
    let session_cookies: Vec<String> = get_session_cookies(&absolute_file_path);
    let response = client
        .get(format!("{}/batch/check/0", TICK_TICK_URL))
        .header("content-type", "application/json")
        .header("x-device", X_DEVICE_HEADER)
        .header("Cookie", session_cookies.join(";"))
        .send()
        .await?;

    match response.status().is_success() {
        true => {
            let user_info: UserData = response.json().await?;
            println!("User Info: {:?}", user_info);
            Ok(user_info)
        }
        false => {
            let error_response: ErrorLoginResponse = response.json().await?;
            println!(
                "Get user info failed! Error Code: {}, Message: {}, ID: {}",
                error_response.error_code, error_response.error_message, error_response.error_id
            );
            println!("Remainder times: {}", error_response.data.remainder_times);
            todo!();
            // Err(Box::new("Error"))
        }
    }

    // Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct HandleTasks {
    add: Vec<TaskBody>,
    update: Vec<TaskBody>,
    delete: Vec<TaskBody>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskBody {
    pub title: String,
    pub project_id: Option<String>,
    pub id: Option<String>,
    pub items: Option<Vec<Item>>,
    pub desc: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub title: String,
    pub status: u8,
    pub id: u128,
}

pub enum Action {
    Add,
    Update,
    Delete,
}

pub async fn handle_tasks(
    tasks_list: Vec<TaskBody>,
    action: Action,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut request_body = HandleTasks {
        add: Vec::new(),
        update: Vec::new(),
        delete: Vec::new(),
    };

    match action {
        Action::Add => request_body.add = tasks_list,
        Action::Update => {
            tasks_list.iter().for_each(|task| {
                if task.id.is_none() {
                    panic!("Task id is required for update action");
                }
            });
            request_body.update = tasks_list;
        }
        Action::Delete => request_body.delete = tasks_list,
    }

    let client = reqwest::Client::new();
    let absolute_file_path: String = get_file_path(DATA_FILE);
    let session_cookies: Vec<String> = get_session_cookies(&absolute_file_path);

    let response = client
        .post(format!("{}/batch/task", TICK_TICK_URL))
        .header("content-type", "application/json")
        .header("x-device", X_DEVICE_HEADER)
        .header("Cookie", session_cookies.join(";"))
        .body(serde_json::to_string(&request_body)?)
        .send()
        .await?;

    match response.status().is_success() {
        true => {
            let user_info: serde_json::Value = response.json().await?;
            println!("User Info: {}", user_info);
        }
        false => {
            let error_response: ErrorLoginResponse = response.json().await?;
            println!(
                "Get user info failed! Error Code: {}, Message: {}, ID: {}",
                error_response.error_code, error_response.error_message, error_response.error_id
            );
            println!("Remainder times: {}", error_response.data.remainder_times);
        }
    }

    Ok(())
}

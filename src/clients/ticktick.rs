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

pub async fn login(username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let body = LoginBody {
        username: username.to_string(),
        password: password.to_string(),
    };

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
                println!("\n Cookie: {:?}", cookie);
                if cookie.value() == "" {
                    println!("Cookie is empty");
                    continue;
                }
                session_cookies.push(format!("{}={}", cookie.name(), cookie.value()));
            }

            fs::write(absolute_file_path, session_cookies.join(";"))?;
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

pub async fn get_user_info(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/batch/check/0", TICK_TICK_URL))
        .header("content-type", "application/json")
        .header("x-device", X_DEVICE_HEADER)
        .header("Authorization", format!("Bearer {}", token))
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

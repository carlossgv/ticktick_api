use reqwest;
use serde::{Deserialize, Serialize};

const TICK_TICK_URL: &str = "https://api.ticktick.com/api/v2";
const X_DEVICE_HEADER: &str = r#"{"platform":"web","os":"macOS 10.15.7","device":"Chrome 121.0.0.0","name":"","version":5070,"id":"65bcdf6491ea1a2e7db71fbe","channel":"website","campaign":"","websocket":""}"#;

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
            let login_response: LoginResponse = response.json().await?;
            println!("Token: {}", login_response.token);
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

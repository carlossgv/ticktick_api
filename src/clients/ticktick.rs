use reqwest;
use serde::Serialize;

const TICK_TICK_URL: &str = "https://api.ticktick.com/api/v2";
const X_DEVICE_HEADER: &str = r#"{"platform":"web","os":"macOS 10.15.7","device":"Chrome 121.0.0.0","name":"","version":5070,"id":"65bcdf6491ea1a2e7db71fbe","channel":"website","campaign":"","websocket":""}"#;

#[derive(Serialize)]
struct LoginBody {
    username: String,
    password: String,
}

// struct LoginResponse {
//     token: String,
// }

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

    let response: String = response.text().await?;
    println!("Token: {}", response);

    Ok(())
}

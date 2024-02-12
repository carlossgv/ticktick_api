use reqwest;
use serde::{Deserialize, Serialize};
// serde

const URL: &str = "http://localhost:3000";
// const API_KEY: &str = "2af9a32b8amsh7140e7bbfc78349p149893jsn2a19d1f3d4ca";
// const API_HOST: &str = "andruxnet-random-famous-quotes.p.rapidapi.com";

#[derive(Serialize)]
struct RequestBody {
    username: String,
    password: String,
}

// message: 'Countries',
// data: {
//     countries: [
//         {
//             name: 'United States',
//             code: 'US',
//             enabled: true,
//             iosEnabledFrom: '1.0.0',
//             andEnabledFrom: '1.0.0',
//         },
//         {
//             name: 'United Kingdom',
//             code: 'UK',
//             enabled: true,
//             iosEnabledFrom: '1.0.0',
//             andEnabledFrom: '1.0.0',
//         },
//         {
//             name: 'Canada',
//             code: 'CA',
//             enabled: true,
//             iosEnabledFrom: '1.0.0',
//             andEnabledFrom: '1.0.0',
//         },
//     ],
// },

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CountriesReponse {
    message: String,
    data: CountriesData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CountriesData {
    countries: Vec<Country>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Country {
    name: String,
    code: String,
    enabled: bool,
    ios_enabled_from: String,
    and_enabled_from: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

pub async fn get_countries(
    username: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let body = RequestBody {
        username: username.to_string(),
        password: password.to_string(),
    };

    let client = reqwest::Client::new();

    let response: CountriesReponse = client
        .post(format!("{}/api/v1/user/countries", URL))
        // .get("https://jsonplaceholder.typicode.com/posts/1")
        .header("content-type", "application/json")
        // .header("x-device", X_DEVICE_HEADER)
        // .body(serde_json::to_string(&body)?)
        .send()
        .await?
        .json()
        .await?;

    println!("Response: {:?}", response);

    // let status = response.status();
    // let body_text = response.text().await?; // Get the raw response body as text
    // println!("Raw Response Body: {:?}", body_text);

    // let cookies = response.cookies().collect::<Vec<_>>();
    // println!("Cookies: {:?}", cookies);

    // match body {
    //     LoginResponse::Token { token } => {
    //         println!("Login successful! Token: {}", token);
    //     }
    //     LoginResponse::Error {
    //         error_code,
    //         error_message,
    //         error_id,
    //         data,
    //     } => {
    //         println!(
    //             "Login failed! Error Code: {}, Message: {}, ID: {}",
    //             error_code, error_message, error_id
    //         );
    //         println!("Remainder times: {}", data.remainder_times);
    //     }
    // }

    // println!("Response: {:?}", body);

    Ok(())
}

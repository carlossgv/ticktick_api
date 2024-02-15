use rand::Rng;
mod clients;
mod ticktick_cli;
mod utils;
use crate::clients::ticktick_client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let action = &args[1];

    match action.as_str() {
        "login" => {
            let mut username = None;
            let mut password = None;

            let mut args = env::args().skip(1);

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "-u" => {
                        username = args.next();
                    }
                    "-p" => {
                        password = args.next();
                    }
                    _ => {
                        continue;
                    }
                }
            }

            match username {
                Some(username) => match password {
                    Some(password) => {
                        ticktick_cli::login(username, password).await?;
                    }
                    None => {
                        println!("Password is required");
                    }
                },
                None => {
                    println!("Username is required");
                }
            }
        }
        "add" => {
            let mut title = None;
            let mut project_id = None;
            let mut items: Option<Vec<ticktick_client::Item>> = None;
            let mut desc = None;

            let mut args = env::args().skip(1);

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "-t" => {
                        title = args.next();
                    }
                    "-d" => {
                        desc = args.next();
                    }
                    "-p" => {
                        project_id = args.next();
                    }
                    "-i" => {
                        let items_str = args.next().unwrap();
                        items = Some(
                            items_str
                                .split(",")
                                .map(|item| ticktick_client::Item {
                                    title: item.to_string().trim().to_string(),
                                    status: 0,
                                    id: generate_id(),
                                })
                                .collect(),
                        );
                    }
                    _ => {
                        continue;
                    }
                }
            }

            match title {
                Some(title) => {
                    ticktick_cli::add_tasks(title, items, project_id, desc).await?;
                }
                None => {
                    println!("Title is required");
                }
            }
        }
        _ => {
            println!("Invalid action");
        }
    }

    Ok(())
}

fn generate_id() -> u128 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

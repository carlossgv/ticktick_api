use rand::Rng;
mod clients;
mod ticktick_cli;
mod utils;
use crate::clients::ticktick_client;
use crate::utils::text_utils::parse_task;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect(); // skip program name

    if args.is_empty() {
        println!("Usage: ticktick_api [login|update] [options] OR just input task title to add");
        return Ok(());
    }

    // Check if it's a known subcommand
    let first_arg = &args[0];

    match first_arg.as_str() {
        "login" => {
            let mut username = None;
            let mut password = None;

            let mut args = args.into_iter().skip(1);

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "-u" => {
                        username = args.next();
                    }
                    "-p" => {
                        password = args.next();
                    }
                    _ => continue,
                }
            }

            match username {
                Some(username) => match password {
                    Some(password) => {
                        ticktick_cli::login(username, password).await?;
                    }
                    None => println!("Password is required"),
                },
                None => println!("Username is required"),
            }
        }

        // "update" => {
        //     let mut title = None;
        //     let mut project_id = None;
        //     let mut items: Option<Vec<ticktick_client::Item>> = None;
        //     let mut desc = None;
        //     let mut content = None;
        //     let mut id = None;
        //
        //     let mut args = args.into_iter().skip(1);
        //
        //     while let Some(arg) = args.next() {
        //         match arg.as_str() {
        //             "-t" => title = args.next(),
        //             "-c" => content = args.next(),
        //             "-d" => desc = args.next(),
        //             "-p" => project_id = args.next(),
        //             "-id" => id = args.next(),
        //             "-i" => {
        //                 if let Some(items_str) = args.next() {
        //                     items = Some(
        //                         items_str
        //                             .split(',')
        //                             .map(|item| ticktick_client::Item {
        //                                 title: item.trim().to_string(),
        //                                 status: 0,
        //                                 id: generate_id(),
        //                             })
        //                             .collect(),
        //                     );
        //                 }
        //             }
        //             _ => continue,
        //         }
        //     }
        //
        //     if id.is_none() || title.is_none() || project_id.is_none() {
        //         println!("Required fields to update: id, title, project_id");
        //         return Ok(());
        //     }
        //
        //     ticktick_cli::update_task(
        //         id.unwrap(),
        //         title.unwrap(),
        //         items,
        //         project_id.unwrap(),
        //         desc,
        //         content,
        //     )
        //     .await?;
        // }

        // DEFAULT: Treat any input as an add command
_ => {
    let mut title = None;
    let mut tags: Option<Vec<String>> = None;

    // Separate flagged args from unflagged args
    let mut args_iter = args.into_iter();
    let mut unflagged_args = Vec::new();

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-t" => title = args_iter.next(),
            _ => unflagged_args.push(arg),
        }
    }

    if title.is_none() {
        // If no -t, then require exactly one unflagged argument (quoted input)
        if unflagged_args.len() != 1 {
            println!("Error: When not using -t, the entire task must be enclosed in quotes as a single argument.");
            println!("Example: ticktick_api \"Buy milk #groceries\"");
            return Ok(());
        }

        let task_input = &unflagged_args[0];
        let parsed = parse_task(task_input);
        title = Some(parsed.title);
        tags = parsed.tags;
    }

    match title {
        Some(title) => {
            ticktick_cli::add_task(title, tags).await?;
        }
        None => {
            println!("Title is required (use -t or unflagged quoted text)");
        }
    }
}
    }

    Ok(())
}

fn generate_id() -> u128 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

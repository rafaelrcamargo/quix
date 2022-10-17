//! # VTEX Colossus API
//! The VTEX Colossus API is a REST API that allows you to connect your app to the VTEX Colossus (Logging service).
//! It allows you to receive events from the VTEX platform, and logs from your app.
//!
//! ## Endpoints
//! - `/events`: Receive events from the VTEX platform.
//! - `/logs`: Receive logs from your app.

use std::{path::PathBuf, thread};

// * Eventsource for the CLI.
use eventsource::{
    event::Event,
    reqwest::{Client as EventSourceClient, Error},
};

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{clients, commands::link::send_package, configs::Vtex};

#[derive(Deserialize)]
struct ColossusEvent {
    body: ColossusBody,
    level: Option<String>,
}

#[derive(Deserialize)]
struct ColossusBody {
    level: Option<String>,
    msg: Option<String>,
    message: Option<String>,
}

fn build(account: &str, workspace: &str, t_client: Client, it_path: PathBuf, it_client: Client) {
    let log_url = reqwest::Url::parse(&format!("https://infra.io.vtex.com/colossus/v0/{}/{}/events?onUnsubscribe=link_interrupted&sender=vtex.builder-hub&keys=build.status", account, workspace)).unwrap();

    let t_client = EventSourceClient::new_with_client(log_url, t_client);

    for event in t_client {
        match event {
            Ok(event) => {
                if event.data == "link_interrupted" {
                    error!("Link interrupted.");
                } else if event.data != "ping\n" {
                    if event.data.contains("initial_link_required") {
                        send_package(&it_path, &it_client);
                    } else if event.data.contains("generic_error") {
                        let message = serde_json::from_str::<ColossusEvent>(&event.data)
                            .unwrap()
                            .body
                            .message
                            .unwrap();

                        warn!("Generic error: {}", message);

                        drop(message)
                    } else {
                        match_event(Ok(event));
                    }
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}

fn status(account: &str, workspace: &str, t_client: Client) {
    let log_url = reqwest::Url::parse(&format!("https://infra.io.vtex.com/colossus/v0/{}/{}/events?onUnsubscribe=link_interrupted&sender=vtex.builder-hub&keys=receive.status", account, workspace)).unwrap();

    let t_client = EventSourceClient::new_with_client(log_url, t_client);

    for event in t_client {
        match_event(event);
    }
}

fn logs(account: &str, workspace: &str, t_client: Client) {
    let log_url = reqwest::Url::parse(&format!(
        "https://infra.io.vtex.com/colossus/v0/{}/{}/logs?level=debug",
        account, workspace
    ))
    .unwrap();

    let t_client = EventSourceClient::new_with_client(log_url, t_client);

    for event in t_client {
        match_event(event);
    }
}

pub fn stream(it_path: PathBuf, it_client: Client) {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    // ? Instantiate a user session.
    let session = Vtex::info();

    let s1 = session.clone();
    children.push(thread::spawn(move || {
        // ? Create a new VTEX Client.
        let t_client = clients::vtex::new(&s1.token);
        build(
            s1.account.as_str(),
            s1.workspace.as_str(),
            t_client,
            it_path,
            it_client,
        );
    }));

    let s2 = session.clone();
    children.push(thread::spawn(move || {
        // ? Create a new VTEX Client.
        let t_client = clients::vtex::new(&s2.token);
        status(s2.account.as_str(), s2.workspace.as_str(), t_client);
    }));

    let s3 = session;
    children.push(thread::spawn(move || {
        // ? Create a new VTEX Client.
        let t_client = clients::vtex::new(&s3.token);
        logs(s3.account.as_str(), s3.workspace.as_str(), t_client);
    }));

    for child in children {
        child.join().unwrap()
    }
}

fn match_event(event: Result<Event, Error>) {
    match event {
        Ok(event) => {
            if event.data != "ping\n" {
                let data = serde_json::from_str::<ColossusEvent>(&event.data).unwrap();

                let body = data.body;

                let level = match data.level {
                    Some(level) => level,
                    None => match body.level {
                        Some(level) => level,
                        None => "info".to_string(),
                    },
                };

                if level == "info" {
                    match body.message {
                        Some(message) => info!("{}", message),
                        None => {
                            if let Some(message) = body.msg {
                                info!("{}", message)
                            }
                        }
                    }
                } else if level == "warning" {
                    match body.message {
                        Some(message) => warn!("{}", message),
                        None => {
                            if let Some(message) = body.msg {
                                warn!("{}", message)
                            }
                        }
                    }
                } else if level == "error" {
                    match body.message {
                        Some(message) => error!("{}", message),
                        None => {
                            if let Some(message) = body.msg {
                                error!("{}", message)
                            }
                        }
                    }
                } else if level == "debug" || level == "trace" {
                    match body.message {
                        Some(message) => categorize_debug_level(message),
                        None => {
                            if let Some(message) = body.msg {
                                categorize_debug_level(message)
                            }
                        }
                    }
                } else {
                    trace!("{}", &event.data)
                }
            }
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}

fn categorize_debug_level(message: String) {
    if message.starts_with('[') {
        trace!("{}", message);
    } else {
        debug!("{}", message);
    }
}

//! # VTEX Builder API
//! The VTEX Builder API is a REST API that allows you to connect your app to the VTEX Builder.
//! It allows you to create and manage projects, and to link your app to the builder.
//!
//! ## Endpoints
//! - `/availability`: Check if the builder is available.
//! - `/link`: Link the app to the builder.
//! - `/relink`: Relink the app to the builder.
//! - `/clean`: Clean the builder cache.

use std::fmt;

use crate::{
    clients,
    configs::{Project, Vtex},
    constants::routes,
};
use routes::{Routes, Routes::Availability, Routes::Clean, Routes::Link, Routes::Relink};

// HTTP Client
use reqwest::{blocking::Client, Error};
use reqwest::{
    blocking::Response,
    header::{HeaderMap, HeaderValue},
};

use reqwest::header::{ACCEPT_ENCODING, CONTENT_LENGTH, CONTENT_TYPE};

/// # Clean the builder for the app.
/// This function will clean the builder for the app.
pub fn clean(client: &Client) -> Result<Response, Error> {
    client // Setup the request.
        .post(&Routes::assemble(Clean)) // Define the endpoint.
        .header(ACCEPT_ENCODING, "gzip") // More headers.
        .header(CONTENT_TYPE, "application/json") // Guess what.
        .body(r#"{"headers": {"Content-Type": "application/json"},"metric": "bh-clean"}"#) // And finally the body.
        .send() // Just wrap it up and send it.
}

/// # Link to the builder.
/// This function will link the app to the builder.
pub fn link(client: &Client, file: Vec<u8>) -> Result<Response, Error> {
    client // Setup the request.
        .post(&Routes::assemble(Link)) // Define the endpoint.
        .header(ACCEPT_ENCODING, "gzip") // More headers.
        .header(CONTENT_TYPE, "application/octet-stream") // Guess what.
        .header(CONTENT_LENGTH, file.len()) // And more headers.
        .body(file) // And finally the body.
        .send() // Just wrap it up and send it.
}

#[derive(Debug)]
pub struct RelinkBody {
    pub content: String,
    pub byte_size: usize,
    pub path: String,
}
impl fmt::Display for RelinkBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{{\"content\":\"{}\",\"byteSize\":{},\"path\":\"{}\"}}]",
            self.content, self.byte_size, self.path
        )
    }
}

/// # Re-Link to the builder.
/// This function will relink the app to the builder.
pub fn relink(client: &Client, body: RelinkBody) -> Result<Response, Error> {
    // Create a new HTTP blocking client.
    client // Setup the request.
        .put(&Routes::assemble(Relink)) // Define the endpoint.
        .header(ACCEPT_ENCODING, "gzip") // More headers.
        .header(CONTENT_TYPE, "application/json") // Guess what.
        .body(RelinkBody::to_string(&body)) // And finally the body.
        .send() // Just wrap it up and send it.
}

/// # Check the availability of the builder.
/// This function will check the availability of the builder.
/// - If the builder is up to date, it will return the new client.
/// - If the builder is not up to date, it will return the old client.
pub fn check_availability() -> Result<Client, Error> {
    // ? Create a new Project config struct.
    let project = Project::info().unwrap();

    // ? Instantiate a user session.
    let session = Vtex::info();

    let binding = Vtex::raw_info().unwrap();
    let sticky_obj = binding
        .get("apps")
        .and_then(|value| value.get(&project.vendor))
        .and_then(|value| value.get(&project.name))
        .and_then(|value| value.get("sticky-host"));

    let mut sticky_host = String::from("");

    if let Some(sticky_obj) = sticky_obj {
        sticky_host = sticky_obj.get("stickyHost").unwrap().to_string()
    } else {
        // make a post request to `/0/availability/avantivtexio.shopping-list@0.0.0` to get the sticky host, and store it in the config, the sticky host comes in the resp headers as `x-vtex-sticky-host`

        // ? Create a new VTEX Client.
        let client = clients::vtex::new(&session.token);

        // ? Create the headers to identify the availability request.
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-vtex-sticky-host",
            HeaderValue::from_str(&format!(
                "request:{}:{}:{}.{}@{}",
                &session.account,
                &session.workspace,
                &project.vendor,
                &project.name,
                &project.version
            ))
            .unwrap(),
        );

        let resp = client
            .post(&Routes::assemble(Availability))
            .headers(headers)
            .send()
            .unwrap();

        match resp.headers().get("x-vtex-sticky-host") {
            Some(host) => {
                let host = host.to_str().unwrap();
                let host = host.split(':').collect::<Vec<&str>>();
                let host = host[0];
                let host = host.to_string();

                Vtex::set_sticky_host(host.as_str());

                sticky_host = host
            }
            None => {
                help!("Error during the availability check. Have you set the correct account and workspace?");
                stringify!(&resp.text().unwrap());
                error!("Could not get the sticky host from the VTEX API.");
            }
        }
    }

    // ? Create a new VTEX Client with the Sticky Host.
    let mut headers = HeaderMap::new();

    if sticky_host.is_empty() {
        help!("Error reading the sticky host. Have you set the correct account and workspace?");
        error!("Could not get the sticky host from the VTEX API.");
    } else {
        headers.insert(
            "x-vtex-sticky-host",
            HeaderValue::from_str(&sticky_host).unwrap(),
        );
    }

    Ok(clients::vtex::new_with_headers(&session.token, &headers))
}

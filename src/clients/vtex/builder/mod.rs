//! # VTEX Builder API
//! The VTEX Builder API is a REST API that allows you to connect your app to the VTEX Builder.
//! It allows you to create and manage projects, and to link your app to the builder.
//!
//! ## Endpoints
//! - `/link`: Link the app to the builder.

use crate::constants::routes;
use routes::{Routes, Routes::Link, Routes::Relink};

// HTTP Client
use reqwest::blocking::Response;
use reqwest::Error;

use reqwest::header::{ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};

/// # Link to the builder.
/// This function will link the app to the builder.
pub fn link(file: Vec<u8>, token: &str) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new(); // Create a new HTTP blocking client.
    return client // Setup the request.
        .post(Routes::assemble(Link)) // Define the endpoint.
        .header(ACCEPT, "application/json, text/plain, */*") // Define the headers.
        .header(ACCEPT_ENCODING, "gzip") // More headers.
        .header(CONTENT_LENGTH, file.len()) // And more headers.
        .header(CONTENT_TYPE, "application/octet-stream") // Guess what.
        .header(AUTHORIZATION, format!("Bearer {}", token)) // One more.
        .body(file) // And finally the body.
        .send(); // Just wrap it up and send it.
}

#[derive(Debug)]
pub struct RelinkBody {
    pub content: String,
    pub byte_size: usize,
    pub path: String,
}

impl RelinkBody {
    pub fn to_string(&self) -> String {
        return format!(
            "{{\"content\":\"{}\",\"byteSize\":{},\"path\":\"{}\"}}",
            self.content, self.byte_size, self.path
        );
    }
}

/// # Re-Link to the builder.
/// This function will relink the app to the builder.
pub fn relink(body: RelinkBody, token: &str) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new(); // Create a new HTTP blocking client.
    return client // Setup the request.
        .put(Routes::assemble(Relink)) // Define the endpoint.
        .header(ACCEPT, "application/json, text/plain, */*") // Define the headers.
        .header(ACCEPT_ENCODING, "gzip") // More headers.
        .header(CONTENT_TYPE, "application/json") // Guess what.
        .header(AUTHORIZATION, format!("Bearer {}", token)) // One more.
        .body(RelinkBody::to_string(&body)) // And finally the body.
        .send(); // Just wrap it up and send it.
}

//! # VTEX Builder API
//! The VTEX Builder API is a REST API that allows you to connect your app to the VTEX Builder.
//! It allows you to create and manage projects, and to link your app to the builder.
//!
//! ## Endpoints
//! - `/link`: Link the app to the builder.

// HTTP Client
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;

use reqwest::header::{ACCEPT, AUTHORIZATION};

pub fn new(token: &str) -> Client {
    let mut headers = HeaderMap::new();

    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
    headers.insert(ACCEPT, "application/json, text/plain, */*".parse().unwrap());

    return reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
}

pub fn new_with_headers(token: &str, headers: &HeaderMap) -> Client {
    let mut headers = headers.clone();
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
    headers.insert(ACCEPT, "application/json, text/plain, */*".parse().unwrap());

    return reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
}

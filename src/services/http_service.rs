use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, StatusCode};

use crate::enums::http_method::HttpMethod;
use crate::structs::http_request::HttpRequest;

pub async fn handle_req(req: &HttpRequest) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
    match req.method {
        HttpMethod::GET => handle_get(req).await,
        HttpMethod::POST => handle_post(req).await,
        HttpMethod::DELETE => handle_delete(req).await,
        HttpMethod::PATCH => handle_patch(req).await,
        HttpMethod::PUT => handle_put(req).await,
    }
}

async fn handle_get(req: &HttpRequest) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    
    for (key, value) in &req.headers {
        let header_name = HeaderName::from_bytes(key.as_bytes())?;
        let header_value = HeaderValue::from_str(value)?;
        headers.insert(header_name, header_value);
    }

    if let Some(auth_value) = req.headers.get("Authorization") {
        headers.insert(AUTHORIZATION, HeaderValue::from_str(auth_value)?);
    }

    let response = client.get(&req.url)
        .headers(headers)
        .query(&req.query_params)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    Ok((status, body))
}

async fn handle_post(req: &HttpRequest) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    
    let mut form_data = Vec::new();
    for (key, value) in &req.body {
        form_data.push((key, value));
    }

    let response = client.post(&req.url)
        .headers(headers)
        .form(&form_data)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    Ok((status, body))
}

async fn handle_put(req: &HttpRequest) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    for (key, value) in &req.query_params {
        headers.insert(HeaderName::from_bytes(key.as_bytes())?, HeaderValue::from_str(value)?);
    }

    let response = client.put(&req.url)
        .headers(headers)
        .json(&req.body)
        .query(&req.query_params)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    Ok((status, body))
}

async fn handle_patch(req: &HttpRequest) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    for (key, value) in &req.query_params {
        headers.insert(HeaderName::from_bytes(key.as_bytes())?, HeaderValue::from_str(value)?);
    }

    let response = client.patch(&req.url)
        .headers(headers)
        .json(&req.body)
        .query(&req.query_params)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    Ok((status, body))
}

async fn handle_delete(req: &HttpRequest) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    for (key, value) in &req.query_params {
        headers.insert(HeaderName::from_bytes(key.as_bytes())?, HeaderValue::from_str(value)?);
    }

    let response = client.delete(&req.url)
        .headers(headers)
        .query(&req.query_params)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    Ok((status, body))
}

mod enums;
mod structs;
use std::{collections::HashMap, io::{self, Write}};
use enums::http_method::HttpMethod;
use reqwest::{header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, AUTHORIZATION}, Client, Error, Response, StatusCode};
use structs::http_request::HttpRequest;

#[tokio::main]
async fn main() {
    let mut request: HttpRequest = HttpRequest::default();
    println!("Hello, Welcome to Hostman");
    println!("The easy to use terminal or GUI Fetch Client");
    println!("Begin by Selecting Your Http Method");

    let method = select_http_method();
    request.method = method;
    let request_url = enter_url();
    request.url = request_url;
    request.headers = enter_headers();
    request.query_params = enter_query_params();
    request.body = enter_body();

    handle_req(&request).await.unwrap();
}

fn select_http_method() -> HttpMethod{
    println!("Select an HTTP method:");
    println!("1. POST");
    println!("2. GET");
    println!("3. PATCH");
    println!("4. DELETE");
    println!("5. PUT");
    
    loop {
        print!("Enter the number of your choice: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim() {
            "1" => return HttpMethod::POST,
            "2" => return HttpMethod::GET,
            "3" => return HttpMethod::PATCH,
            "4" => return HttpMethod::DELETE,
            "5" => return HttpMethod::PUT,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn enter_url() -> String {
    println!("Enter URL:");
    io::stdout().flush().unwrap();
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("failed to read");

    url
}

fn enter_headers() -> HashMap<String, String>{
    let mut headers = HashMap::new();
    println!("Enter headers (leave blank to finish):");
    loop {
        print!("Enter header key (or press Enter to finish): ");
        io::stdout().flush().unwrap();
        let mut key = String::new();
        io::stdin().read_line(&mut key).expect("Failed to read line");
        let key = key.trim();
        
        if key.is_empty() {
            break;
        }
        
        print!("Enter header value: ");
        io::stdout().flush().unwrap();
        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Failed to read line");
        let value = value.trim().to_string();
        
        headers.insert(key.to_string(), value);
    }
    headers
}

fn enter_query_params() -> HashMap<String, String>{
    let mut params = HashMap::new();
    println!("Enter query parameters (leave blank to finish):");
    loop {
        print!("Enter parameter key (or press Enter to finish): ");
        io::stdout().flush().unwrap();
        let mut key = String::new();
        io::stdin().read_line(&mut key).expect("Failed to read line");
        let key = key.trim();
        
        if key.is_empty() {
            break;
        }
        
        print!("Enter parameter value: ");
        io::stdout().flush().unwrap();
        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Failed to read line");
        let value = value.trim().to_string();
        
        params.insert(key.to_string(), value);
    }
    params
}

fn enter_body() -> HashMap<String, String>{
    let mut body = HashMap::new();
    println!("Enter body key-value pairs (leave blank to finish):");
    loop {
        print!("Enter body key (or press Enter to finish): ");
        io::stdout().flush().unwrap();
        let mut key = String::new();
        io::stdin().read_line(&mut key).expect("Failed to read line");
        let key = key.trim();
        
        if key.is_empty() {
            break;
        }
        
        print!("Enter body value: ");
        io::stdout().flush().unwrap();
        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Failed to read line");
        let value = value.trim().to_string();
        
        body.insert(key.to_string(), value);
    }
    body
}


async fn handle_req(req: &HttpRequest) -> Result<(), Box<dyn std::error::Error>>{
    match req.method {
        HttpMethod::GET => handle_get(req).await,
        HttpMethod::POST => handle_post(req).await,
        HttpMethod::DELETE => handle_delete(req).await,
        HttpMethod::PATCH => handle_patch(req).await,
        HttpMethod::PUT => handle_put(req).await,
        _ => handle_default(req)
    }
}

async fn handle_get(req: &HttpRequest) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    
    for (key, value) in &req.headers {
        let header_name = HeaderName::from_bytes(key.as_bytes())?;
        let header_value = HeaderValue::from_str(value)?;
        headers.insert(header_name, header_value);
    }

    // Ensure the Authorization header is set correctly
    if let Some(auth_value) = req.headers.get("Authorization") {
        headers.insert(AUTHORIZATION, HeaderValue::from_str(auth_value)?);
    }

    let response = client.get(&req.url)
        .headers(headers)
        .query(&req.query_params)
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    println!("Body: {}", response.text().await?);

    Ok(())
}

async fn handle_post(req: &HttpRequest) -> Result<(), Box<dyn std::error::Error>>{
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

    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    println!("Body: {}", response.text().await?);

    Ok(())
}

async  fn handle_put(req: &HttpRequest) -> Result<(), Box<dyn std::error::Error>>{
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

    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    println!("Body: {}", response.text().await?);

    Ok(())
}

async fn handle_patch(req: &HttpRequest) -> Result<(), Box<dyn std::error::Error>>{
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

    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    println!("Body: {}", response.text().await?);

    Ok(())
}

async fn handle_delete(req: &HttpRequest) -> Result<(), Box<dyn std::error::Error>>{
    let client: Client = Client::new();
    let mut headers : HeaderMap= HeaderMap::new();
    for (key, value) in &req.query_params {
        headers.insert(HeaderName::from_bytes(key.as_bytes())?, HeaderValue::from_str(value)?);
    }

    let response : Response = client.delete(&req.url)
        .headers(headers)
        .query(&req.query_params)
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    println!("Body: {}", response.text().await?);

    Ok(())
}

fn handle_default(req: &HttpRequest) -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Unsupported HTTP method: {:?}", req.method)
    )))
}
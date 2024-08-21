use crate::enums::http_method::HttpMethod;


use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub query_params: HashMap<String,String>,
    pub headers: HashMap<String,String>,
    pub body: HashMap<String,String>,
}

impl Default for HttpRequest {
    fn default() -> HttpRequest {
       HttpRequest{
        url: String::from(""),
        method: HttpMethod::POST,
        query_params: HashMap::new(),
        headers: HashMap::new(),
        body: HashMap::new()
       } 
    }
}
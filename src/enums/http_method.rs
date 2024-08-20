#[derive(Debug, PartialEq, Clone)]
pub enum HttpMethod {
    POST,
    GET,
    PATCH,
    DELETE,
    PUT
}
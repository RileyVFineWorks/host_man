#![allow(unused_imports)]
#[cfg(test)]
use super::*;
use egui::Context;

#[test]
fn test_hostman_new() {
    let hostman = HostMan::new();
    
    assert_eq!(hostman.request.method, HttpMethod::POST);
    assert!(hostman.request.url.is_empty());
    assert!(!hostman.is_loading);
    assert_eq!(*hostman.response.lock(), String::new());
    assert!(hostman.pending_request.is_none());
}

#[test]
fn test_send_request() {
    let mut hostman = HostMan::new();
    
    hostman.send_request();
    
    assert!(hostman.is_loading);
    assert!(hostman.pending_request.is_some());
}

#[test]
fn test_update_request() {
    let mut hostman = HostMan::new();
    
    hostman.request.method = HttpMethod::POST;
    hostman.request.url = "https://api.example.com".to_string();
    hostman.request.headers.insert("Content-Type".to_string(), "application/json".to_string());
    
    assert_eq!(hostman.request.method, HttpMethod::POST);
    assert_eq!(hostman.request.url, "https://api.example.com");
    assert_eq!(hostman.request.headers.get("Content-Type"), Some(&"application/json".to_string()));
}

#[test]
fn test_key_value_pair_editor() {
    let mut hostman = HostMan::new();
    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    
    // Simulate adding a new key-value pair
    map.insert("key2".to_string(), "value2".to_string());
    
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&"value1".to_string()));
    assert_eq!(map.get("key2"), Some(&"value2".to_string()));
}
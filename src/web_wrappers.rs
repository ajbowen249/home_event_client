extern crate hyper;
use hyper::client::Client;
use std::io::Read;

pub fn post_request(address: &String, body: &String) -> (hyper::status::StatusCode, String) {
    let client = Client::new();
    let mut result = client.post(address)
        .body(body)
        .send()
        .unwrap();

    let mut response_body = String::new();
    let _ = result.read_to_string(&mut response_body);

    return (result.status, response_body);
}

pub fn put_request(address: &String, body: &String) -> (hyper::status::StatusCode, String) {
    let client = Client::new();
    let mut result = client.put(address)
        .body(body)
        .send()
        .unwrap();

    let mut response_body = String::new();
    let _ = result.read_to_string(&mut response_body);

    return (result.status, response_body);
}

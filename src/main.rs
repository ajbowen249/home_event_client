extern crate hyper;
extern crate rustc_serialize;

use hyper::client::Client;
use std::io::Read;
use rustc_serialize::json;

static EVENT_SERVER_ADDR: &'static str = "http://pickleandmuffin.com/eventServer/api.php";

fn main() {
    println!("Starting up home client.");
    print!("Pinging server...");

    let ping_result = ping_event_server();

    match ping_result{
        true => println!("success!"),
        false =>{
            println!("failure :(");
            println!("Exiting");
            return;
        }
    }

    
}

fn ping_event_server() -> bool {
    let (status, _) = post_request(&EVENT_SERVER_ADDR.to_string(), &"{\"method\" : \"ping\"}".to_string());
    return status == hyper::Ok;
}

fn get_max_sequence_number -> i64 {

}

fn post_request(address: &String, body: &String) -> (hyper::status::StatusCode, String) {
    let client = Client::new();
    let mut result = client.post(address)
                           .body(body)
                           .send()
                           .unwrap();
    
    let mut response_body = String::new();
    let _ = result.read_to_string(&mut response_body);

    return (result.status, response_body);
}
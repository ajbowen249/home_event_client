extern crate hyper;
extern crate rustc_serialize;

use hyper::client::Client;
use std::io::Read;
use rustc_serialize::json;
use std::{thread, time};

static EVENT_SERVER_ADDR: &'static str = "http://pickleandmuffin.com/eventServer/api.php";
const POLL_THROTTLE_MS: u64 = 100;

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

    let mut sequence_number: i64 = -1;

    match get_highest_sequence_number(){
        Some(highest_num) => sequence_number = highest_num,
        None =>{
            println!("Failed to get initial sequence number :(");
            return;
        }
    }

    println!("Observing events after sequence number {}", sequence_number);

    loop{
        match get_chunk_of_events(sequence_number){
            Some(vec) =>{
                for item in &vec {
                    process_event(item);
                }

                sequence_number = vec[vec.len() - 1].sequenceNumber;
            }
            _ => {}
        }

        let poll_throttle = time::Duration::from_millis(POLL_THROTTLE_MS);
        thread::sleep(poll_throttle);
    }
}

fn process_event(event: &Event) {
    match event.eventType.as_ref() {
        "Message" => process_message_event(event),
        "HueRelay" => process_hue_relay_event(event),
        _ => {}
    }
}

fn process_message_event(event: &Event) {
    let message_event: MessageEventBody = json::decode(&event.body).unwrap();
    println!("Got message event at {}: {}", event.time, message_event.message);
}

fn process_hue_relay_event(event: &Event) {
    let hue_relay_event: HueRelayEventBody = json::decode(&event.body).unwrap();
    put_request(&hue_relay_event.url, &hue_relay_event.content);
}

fn ping_event_server() -> bool {
    let reqest_body = EventSeverRequest{ method: "ping".to_string(), parameters: "" };
    let reqest_body = json::encode(&reqest_body).unwrap();

    let (status, _) = post_request(&EVENT_SERVER_ADDR.to_string(), &reqest_body);
    return status == hyper::Ok;
}

fn get_chunk_of_events(after_sequence_number: i64) -> Option<Vec<Event>>{
    let reqest_body = EventSeverRequest{ method: "getChunkOfEvents".to_string(), parameters: GetChunkOfEventsRequest{ afterSequenceNumber: after_sequence_number }};
    let reqest_body = json::encode(&reqest_body).unwrap();

    let (status, response_body) = post_request(&EVENT_SERVER_ADDR.to_string(), &reqest_body);

    match status {
        hyper::Ok =>{
            let result: EventServerResult<Vec<Event>> = json::decode(&response_body).unwrap();
            return if result.result.len() > 0 { Some(result.result) } else { None };
        }
        _ => return None
    }
}

fn get_highest_sequence_number() -> Option<i64> {
    let reqest_body = EventSeverRequest{ method: "getHighestSequenceNumber".to_string(), parameters: "" };
    let reqest_body = json::encode(&reqest_body).unwrap();

    let (status, response_body) = post_request(&EVENT_SERVER_ADDR.to_string(), &reqest_body);

    match status {
        hyper::Ok =>{
            let result: EventServerResult<SequenceNumberContainer> = json::decode(&response_body).unwrap();
            return Some(result.result.sequenceNumber);
        }
        _ => return None
    }
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

fn put_request(address: &String, body: &String) -> (hyper::status::StatusCode, String) {
    let client = Client::new();
    let mut result = client.put(address)
                           .body(body)
                           .send()
                           .unwrap();
    
    let mut response_body = String::new();
    let _ = result.read_to_string(&mut response_body);

    return (result.status, response_body);
}

#[derive(RustcDecodable, RustcEncodable)]
struct EventSeverRequest<T> {
    method: String,
    parameters: T
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
struct GetChunkOfEventsRequest {
    afterSequenceNumber: i64
}

#[derive(RustcDecodable, RustcEncodable)]
struct EventServerResult<T> {
    result: T
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
struct Event {
    sequenceNumber: i64,
    eventType: String,
    time: String,
    body: String
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
struct SequenceNumberContainer {
    sequenceNumber: i64
}

#[derive(RustcDecodable, RustcEncodable)]
struct MessageEventBody {
    message: String
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
struct HueRelayEventBody{
    url: String,
    content: String
}
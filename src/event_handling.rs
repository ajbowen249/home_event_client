extern crate rustc_serialize;
use rustc_serialize::json;

use data_structures::*;
use web_wrappers::*;

pub fn process_event(event: &Event) {
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

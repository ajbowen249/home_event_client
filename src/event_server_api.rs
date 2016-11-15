extern crate hyper;
extern crate rustc_serialize;

use rustc_serialize::json;
use data_structures::*;
use web_wrappers::*;

static EVENT_SERVER_ADDR: &'static str = "http://pickleandmuffin.com/eventServer/api.php";

pub fn ping_event_server() -> bool {
    let reqest_body = EventSeverRequest{ method: "ping".to_string(), parameters: "" };
    let reqest_body = json::encode(&reqest_body).unwrap();

    let (status, _) = post_request(&EVENT_SERVER_ADDR.to_string(), &reqest_body);
    return status == hyper::Ok;
}

pub fn get_chunk_of_events(after_sequence_number: i64) -> Option<Vec<Event>>{
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

pub fn get_highest_sequence_number() -> Option<i64> {
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

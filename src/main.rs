extern crate hyper;
extern crate rustc_serialize;

use std::{thread, time};

mod data_structures;
mod web_wrappers;

mod event_server_api;
use event_server_api::*;

mod event_handling;
use event_handling::*;

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

    let mut sequence_number: i64;

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

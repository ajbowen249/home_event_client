extern crate hyper;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::io;

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

    match ping_result {
        true => println!("success!"),
        false => {
            println!("failure :(");
            println!("Exiting");
            return;
        }
    }

    let mut sequence_number: i64;

    match get_highest_sequence_number() {
        Some(highest_num) => sequence_number = highest_num,
        None => {
            println!("Failed to get initial sequence number :(");
            return;
        }
    }

    println!("Observing events after sequence number {}", sequence_number);

    let quit = Arc::new(Mutex::new(false));
    let handle: thread::JoinHandle<_>;

    {
        let quit = quit.clone();
        handle = thread::spawn(move || loop {
            match quit.lock() {
                Ok(q) => {
                    if *q {
                        return;
                    }
                }
                Err(_) => println!("Quit flag read error."),
            }

            match get_chunk_of_events(sequence_number) {
                Some(vec) => {
                    for item in &vec {
                        process_event(item);
                    }

                    sequence_number = vec[vec.len() - 1].sequenceNumber;
                }
                _ => {}
            }

            let poll_throttle = time::Duration::from_millis(POLL_THROTTLE_MS);
            thread::sleep(poll_throttle);
        });
    }

    loop {
        print!(">");
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut entry = String::new();
        match io::stdin().read_line(&mut entry) {
            Ok(_) => {
                match entry.trim() {
                    "q" | "quit" | "Q" | "QUIT" => {
                        break;
                    }
                    _ => println!("Commands:\n    q: quit"),
                }
            }
            Err(_) => println!("Console IO Error"),
        }
    }

    match quit.lock() {
        Ok(mut q) => {
            *q = true;
        }
        Err(_) => println!("Quit flag access error."),
    }

    match handle.join() {
        Ok(_) => {}
        Err(_) => println!("Error cancelling client thread."),
    }

    println!("Successfully exited.");
}

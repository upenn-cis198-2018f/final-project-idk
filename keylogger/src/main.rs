#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;
extern crate libc;

use regex::Regex;
use std::io::{self, Read};
use std::fs::File;
use std::mem;

mod keys;
mod events;
mod parser;

fn main() {

    loop {

        // we get the device file to read from
        let device_file = match get_device_file() {
            Ok(x) => x,
            Err(e) => panic!("ERROR: Device File Not Found, {}", e)
        };

        // we get the event file to read from
        let device = match File::open(device_file) {
            Ok(x) => x,
            Err(e) => panic!("ERROR: Cannot open device file, {}", e)
        };

        // we create a buffer to take in the keyboard input
        let buf: [u8; 24] = unsafe{mem::zeroed()};

        // we listen for a full string inputted by the user;
        // start and finish is indicated by '//'
        let text : String = listen_key(device, buf);

        // get parsing information and then send to calendar
        println!("{}", text);
    }

}

// function takes in device file to read from and buffer to read to,
// returns a full string based on keylogged input
fn listen_key(mut device: File, mut buf: [u8; 24]) -> String {

    let mut agg_string : String = String::new(); // aggregated string returned
    let mut agg_escape : String = String::new(); // keeps track of escape chars

    let mut start : bool = false; // true when we have started listening

    // keep looping until user is done with input
    loop {
        // read to buffer
        match device.read(&mut buf) {
            Ok(x) => (),
            Err(e) => panic!("ERROR: Cannot read from buffer, {}", e)
        };

        // we get our event
        let event : InputEvent = unsafe {mem::transmute(buf)};
        
        // we match our events to specific keys, checking for escape characters
        if event.my_type == (1 as u16) {
            if event.value == (1 as i32) {
                let key = keys::KEY_NAMES[event.code as usize];
                if key == "/" {
                    agg_escape.push_str(&"/".to_string());
                    if agg_escape == "//".to_string() {

                        if !start {
                            start = true;
                            agg_escape = "".to_string();
                        } else {
                            let final_len = agg_string.chars().count();
                            // we truncate the string to make sure it doesn't have the remaining '/' at the end
                            agg_string.truncate(final_len - 1);
                            return agg_string
                        }
                    } else if start {
                        agg_string.push_str(&key.to_string());
                    }
                } else {
                    if start {
                        if key == "<Backspace>" {
                            // need to check if there is a character to backspace before deleting
                            let curr_len = agg_string.chars().count(); 
                            if curr_len > 0 {
                                agg_string.truncate(curr_len - 1);
                            }
                        } else if key.to_string().chars().count() < 2 {
                            agg_string.push_str(&key.to_string());
                        }
                   }
                agg_escape = "".to_string(); // reset escape if not double "//"
                }
            }
        }
    }


}

// function gets device 
fn get_device_file() -> Result<String, io::Error> {
    let mut file = File::open("/proc/bus/input/devices")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    // we get our device through regex
    let expression = Regex::new("((.*\n){2})(B: EV=120013)").unwrap();
    let m = expression.find(&s).unwrap().as_str();
    let get_event = Regex::new("event[0-9]+").unwrap();
    let event = get_event.find(&m).unwrap().as_str();
    let mut filename = "/dev/input/".to_string();
    filename.push_str(event);

    Ok(filename)
}

// struct for input event
#[repr(C)]
pub struct InputEvent {
    tv_sec: isize,
    tv_usec: isize,
    pub my_type: u16,
    pub code: u16,
    pub value: i32
}
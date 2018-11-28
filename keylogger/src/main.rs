use regex::Regex;
extern crate regex;
use std::io::{self, Read};
use std::fs::File;

fn main() {


    println!("Hello, world!");
    let device_file = match get_device_file() {
    	Ok(x) => x,
    	Err(e) => panic!("ERROR: Device File Not Found, {}", e)
    };
    let device = File::open(device_file).unwrap(); // deal with unwrap

    loop {
    	let mut escape = "";
    }

}

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

    println!("{}", filename);

    Ok(filename)
}
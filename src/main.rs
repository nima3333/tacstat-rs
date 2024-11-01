use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    // Todo: read file from zip
    
    // Create a path to the desired file
    let path = Path::new("Tacview-20220630-011729-DCS-test_dmt_av8.txt.acmi");
    let display = path.display();

    // // Open the path in read-only mode, returns `io::Result<File>`
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // Read the file contents into a string, returns `io::Result<usize>`
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }

    // Hashmap containing id, name
    let mut hm = HashMap::new();
    let object_creation_pattern = Regex::new(r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+),Pilot=([\w+\- \|]+)").unwrap();

    // Parse file
    for line in read_to_string(path).unwrap().lines() {
        if let Some(caps) = object_creation_pattern.captures(line)  {
            // caps[1].to_string().unwrap()
            // caps[1].parse::<i32>().unwrap()
            hm.insert(i32::from_str_radix(&caps[1], 16).unwrap(), caps[7].to_string());
        }
        // print!("{} contains:\n", line)
        // result.push(line.to_string())
    }

    println!("\nHashMap contents:");
    for (key, value) in &hm {
        println!("{}: {}", key, value);
    }
    // `file` goes out of scope, and the "hello.txt" file gets closed
}

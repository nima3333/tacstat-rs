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

    // Hashmap containing id, name
    let mut hm = HashMap::new();
    let object_creation_pattern = Regex::new(r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+),Pilot=([\w+\- \|]+)").unwrap();

    // Parse file
    for line in read_to_string(path).unwrap().lines() {
        if let Some(caps) = object_creation_pattern.captures(line)  {
            hm.insert(i32::from_str_radix(&caps[1], 16).unwrap(), caps[7].to_string());
        }

    }

    println!("\nHashMap contents:");
    for (key, value) in &hm {
        println!("{}: {}", key, value);
    }

}

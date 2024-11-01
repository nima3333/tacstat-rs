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
    let mut current_time:f64 = 0.0;
    let mut hm: HashMap<i32, (String, String, f64, f64)> = HashMap::new();


    // Regex patterns
    let object_creation_pattern = Regex::new(r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+),Pilot=([\w+\- \|]+)").unwrap();
    let time_pattern = Regex::new(r"#([0-9.]+)$").unwrap();

    // Parse file
    for line in read_to_string(path).unwrap().lines() {
        // Creation of an object
        if let Some(caps) = object_creation_pattern.captures(line)  {
            // Parse capture group
            let (id, name, vehicle) = (
                i32::from_str_radix(&caps[1], 16).unwrap(),
                caps[7].to_string(),
                caps[6].to_string(),
            );
            // Insert element in hashmap
            hm.insert(id, (name, vehicle, current_time, 0.0));
        }
        else if let Some(caps) = time_pattern.captures(line)  {
            current_time = caps[1].parse::<f64>().unwrap();
        }

    }

    println!("\nHashMap contents:");
    for (key, (ref x, ref y, time, _last_time)) in &hm {
        println!("{}: {} in {} at {}", key, x, y, time);
    }

}

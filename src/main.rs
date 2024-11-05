use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;


fn contains_any(name: &str, list: &[&str]) -> bool {
    list.iter().any(|&item| name.contains(item))
}


fn main() {
    // Todo: read file from zip
    let start = Instant::now();

    // Create a path to the desired file
    // let path = Path::new("Tacview-20220630-011729-DCS-test_dmt_av8.txt.acmi");
    let path = Path::new("Tacview-20240924-003100-DCS-Client-Operation-Sirab_Part_2_20240923-1.zip\\Tacview-20240924-003100-DCS-Client-Operation-Sirab_Part_2_20240923-1.txt.acmi");
    let display = path.display();

    // Hashmap containing id, name
    let mut current_time:f64 = 0.0;
    let mut hm: HashMap<i32, (String, String, f64, f64)> = HashMap::new();

    // Names to whitelist
    let list_of_strings = vec!["nima3333", "Nouveau Surnom"];

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

            if contains_any(name.as_str(), &list_of_strings) {
                // Insert element in hashmap
                hm.insert(id, (name, vehicle, current_time, 0.0));
            }
        }
        else if line.starts_with("#")  {
            current_time = line.strip_prefix('#').unwrap().parse::<f64>().unwrap();
            // let caps = time_pattern.captures(line).unwrap();
            // current_time = caps[1].parse::<f64>().unwrap();
        }

    }

    println!("\nHashMap contents:");
    for (key, (ref x, ref y, time, _last_time)) in &hm {
        println!("{}: {} in {} at {}", key, x, y, time);
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

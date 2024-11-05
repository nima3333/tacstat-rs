use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

fn contains_any(name: &str, list: &[&str]) -> bool {
    list.iter().any(|&item| name.contains(item))
}

fn create_matcher(ids: &HashMap<i32, (String, String, f64, f64)>) -> Regex {
    let id_pattern = ids
        .keys()
        .map(|k| format!("{:X}", k))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"^(?:{}),T=", id_pattern)).unwrap()
}

fn main() {
    // Todo: read file from zip
    let start = Instant::now();

    // Create a path to the desired file
    // let path = Path::new("Tacview-20220630-011729-DCS-test_dmt_av8.txt.acmi");
    let path = Path::new("Tacview-20240924-003100-DCS-Client-Operation-Sirab_Part_2_20240923-1.zip\\Tacview-20240924-003100-DCS-Client-Operation-Sirab_Part_2_20240923-1.txt.acmi");
    let display = path.display();

    // Time
    let mut current_time: f64 = 0.0;
    // Hashmap containing id -> name, vehicle, time_creation, time_deletion
    let mut id_main: HashMap<i32, (String, String, f64, f64)> = HashMap::new();
    // Hashmap containing id -> current_pos (lat/long/alt)
    let mut id_to_coord: HashMap<i32, (f32, f32, f32)> = HashMap::new();

    // Names to whitelist
    let list_of_strings = vec!["nima3333", "Nouveau Surnom"];

    // Regex patterns
    let object_creation_pattern = Regex::new(r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+),Pilot=([\w+\- \|]+)").unwrap();
    let mut update_coord_pattern = create_matcher(&id_main);
    // Parse file
    for line in read_to_string(path).unwrap().lines() {
        // Creation of an object
        if line.contains("Pilot=") {
            if let Some(caps) = object_creation_pattern.captures(line) {
                // Parse capture group
                let (id, name, vehicle, lat, long, alt) = (
                    i32::from_str_radix(&caps[1], 16).unwrap(),
                    caps[7].to_string(),
                    caps[6].to_string(),
                    caps[2].parse::<f32>().unwrap(),
                    caps[3].parse::<f32>().unwrap(),
                    caps[4].parse::<f32>().unwrap(),
                );

                if contains_any(name.as_str(), &list_of_strings) {
                    // Insert element in hashmaps
                    id_main.insert(id, (name, vehicle, current_time, 0.0));
                    id_to_coord.insert(id, (lat, long, alt));
                    // Update regex
                    update_coord_pattern = create_matcher(&id_main);
                    // println!("{}", update_coord_pattern.as_str());
                }
            }
        }
        // Time tracking
        else if line.starts_with("#") {
            current_time = line.strip_prefix('#').unwrap().parse::<f64>().unwrap();
        }
        // Coordinates update
        else if line.contains("T=") {
            if update_coord_pattern.is_match(&line) {
                // println!("{}", line);
            }
        }
    }

    println!("\nHashMap contents:");
    for (key, (ref x, ref y, time, _last_time)) in &id_main {
        println!("{}: {} in {} at {}", key, x, y, time);
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

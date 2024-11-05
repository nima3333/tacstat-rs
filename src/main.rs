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
    let mut id_coords: HashMap<i32, (f32, f32, f32)> = HashMap::new();

    // Names to whitelist
    let whitelist = vec!["nima3333", "Nouveau Surnom"];

    // Regex patterns
    let object_creation_pattern = Regex::new(r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+),Pilot=([\w+\- \|]+)").unwrap();
    let mut coord_pattern = create_matcher(&id_main);
    // Parse file
    for line in read_to_string(&path).expect("Failed to read the file").lines() {
        match line {
            line if line.contains("Pilot=") => {
                if let Some(caps) = object_creation_pattern.captures(line) {
                    let id = i32::from_str_radix(&caps[1], 16).expect("Invalid ID");
                    let name = caps[7].to_owned();
                    let vehicle = caps[6].to_owned();
                    let lat = caps[2].parse::<f32>().expect("Invalid latitude");
                    let long = caps[3].parse::<f32>().expect("Invalid longitude");
                    let alt = caps[4].parse::<f32>().expect("Invalid altitude");

                    if contains_any(&name, &whitelist) {
                        // Insert into hashmaps
                        id_main.insert(id, (name, vehicle, current_time, 0.0));
                        id_coords.insert(id, (lat, long, alt));
                        // Update regex matcher
                        coord_pattern = create_matcher(&id_main);
                    }
                }
            }
            line if line.starts_with('#') => {
                current_time = line.strip_prefix('#').unwrap().parse::<f64>().expect("Invalid time format");
            }
            line if line.contains("T=") => {
                if coord_pattern.is_match(line) {
                    // Handle the matched line as needed
                }
            }
            _ => {} // Ignore lines that don't match any condition
        }
    }

    println!("\nHashMap contents:");
    for (key, (ref x, ref y, time, _last_time)) in &id_main {
        println!("{}: {} in {} at {}", key, x, y, time);
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

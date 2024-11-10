mod computation;

use computation::haversine_distance;
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
    Regex::new(&format!(
        r"^({}),T=([0-9\.]*)\|([0-9\.]*)\|([0-9\.]*)",
        id_pattern
    ))
    .unwrap()
}

fn increment_weapon_counter(
    map: &mut HashMap<i32, HashMap<String, i32>>,
    id: i32,
    weapon_name: &str,
) {
    // Access the inner hashmap for the given id, or insert a new empty one if it doesn't exist
    let weapon_map = map.entry(id).or_insert_with(HashMap::new);

    // Increment the counter for the weapon, starting from 1 if it doesn’t exist
    let counter = weapon_map.entry(weapon_name.to_string()).or_insert(0);
    *counter += 1;
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
    // Hashmap containing id -> (weapon name, nb fired)
    let mut id_weapon: HashMap<i32, HashMap<String, i32>> = HashMap::new();

    // Names to whitelist
    let whitelist = vec!["nima3333", "Nouveau Surnom"];
    //
    let mut bool_watch = true;

    // Regex patterns
    let pilot_creation_pattern = Regex::new(r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+),Pilot=([\w+\- \|]+)").unwrap();
    let weapon_creation_pattern = Regex::new(r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+)").unwrap();

    let mut coord_pattern = create_matcher(&id_main);
    // Parse file
    for line in read_to_string(&path)
        .expect("Failed to read the file")
        .lines()
    {
        match line {
            line if line.contains("Pilot=") => {
                if let Some(caps) = pilot_creation_pattern.captures(line) {
                    let id = i32::from_str_radix(&caps[1], 16).expect("Invalid ID");
                    let name: String = caps[7].to_owned();
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
                current_time = line
                    .strip_prefix('#')
                    .unwrap()
                    .parse::<f64>()
                    .expect("Invalid time format");
                bool_watch = !bool_watch;
            }
            line if line.starts_with('-') => {
                let id = i32::from_str_radix(line.strip_prefix('-').unwrap(), 16).unwrap();
                if let Some(entry) = id_main.get_mut(&id) {
                    entry.3 = current_time;
                }
            }
            line if line.contains("T=") => {
                if bool_watch && coord_pattern.is_match(line) {
                    if let Some(caps) = coord_pattern.captures(line) {
                        let mut lat = caps[2].to_owned();
                        let mut long = caps[3].to_owned();
                        let mut alt = caps[4].to_owned();
                        let id = i32::from_str_radix(&caps[1], 16).unwrap();

                        if lat.is_empty() {
                            lat = id_coords.get(&id).unwrap().0.to_string();
                        }
                        if long.is_empty() {
                            long = id_coords.get(&id).unwrap().1.to_string();
                        }
                        if alt.is_empty() {
                            alt = id_coords.get(&id).unwrap().2.to_string();
                        }

                        id_coords.insert(
                            id,
                            (
                                lat.parse::<f32>().unwrap(),
                                long.parse::<f32>().unwrap(),
                                alt.parse::<f32>().unwrap(),
                            ),
                        );
                    }
                } else if weapon_creation_pattern.is_match(line) {
                    if let Some(caps) = weapon_creation_pattern.captures(line) {
                        if (id_coords.get(&1027).is_some()) {
                            let lat = caps[2].parse::<f32>().expect("Invalid latitude");
                            let long = caps[3].parse::<f32>().expect("Invalid longitude");

                            for (&id, &(lat2, long2, _)) in &id_coords {
                                let dist = haversine_distance(lat, long, lat2, long2);
                                if dist < 0.1 {
                                    let weapon = caps[6].to_owned();
                                    increment_weapon_counter(&mut id_weapon, id, &weapon);
                                }
                            }
                        }
                    }
                }
            }
            _ => {} // Ignore lines that don't match any condition
        }
    }
    println!("{:#?}", id_weapon);
    println!("{:#?}", id_main);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

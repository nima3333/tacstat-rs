mod models;
mod utils;

use models::{PlayerInfo, Position, GameState};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

use std::path::Path;
use std::time::Instant;

const WEAPON_DISTANCE_THRESHOLD: f32 = 0.1;

fn contains_any(name: &String, list: &Vec<String>) -> bool {
    list.iter().any(|item| name.contains(item))
}

fn create_matcher(ids: &HashMap<i32, PlayerInfo>) -> Regex {
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
    map.entry(id)
        .or_default()
        .entry(weapon_name.to_string())
        .and_modify(|count| *count += 1)
        .or_insert(1);
}

fn main() {
    // Todo: read file from zip
    let start = Instant::now();

    // Create a path to the desired file
    // let path = Path::new("Tacview-20220630-011729-DCS-test_dmt_av8.txt.acmi");
    let path = Path::new("Tacview-20240924-003100-DCS-Client-Operation-Sirab_Part_2_20240923-1.zip\\Tacview-20240924-003100-DCS-Client-Operation-Sirab_Part_2_20240923-1.txt.acmi");
    let _display = path.display();

    // Time
    let mut gamestate = GameState::new();
    // Names to whitelist
    let whitelist: Vec<String> = vec!["nima3333".to_string(), "Nouveau Surnom".to_string()];

    //
    let mut bool_watch = true;

    // Regex patterns
    let pilot_creation_pattern = Regex::new(r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+),Pilot=([\w+\- \|]+)").unwrap();
    let weapon_creation_pattern = Regex::new(r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+)").unwrap();

    let mut coord_pattern = create_matcher(&gamestate.players);
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
                        gamestate.add_player(id, name, vehicle, Position::new(lat, long, alt));
                        coord_pattern = create_matcher(&gamestate.players);
                    }
                }
            }
            line if line.starts_with('#') => {
                gamestate.current_time = line
                    .strip_prefix('#')
                    .unwrap()
                    .parse::<f64>()
                    .expect("Invalid time format");
                bool_watch = !bool_watch;
            }
            line if line.starts_with('-') => {
                let id = i32::from_str_radix(line.strip_prefix('-').unwrap(), 16).unwrap();
                if let Some(entry) = gamestate.players.get_mut(&id) {
                    entry.mark_deleted(gamestate.current_time);
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
                            lat = gamestate.positions.get(&id).unwrap().lat.to_string();
                        }
                        if long.is_empty() {
                            long = gamestate.positions.get(&id).unwrap().long.to_string();
                        }
                        if alt.is_empty() {
                            alt = gamestate.positions.get(&id).unwrap().alt.to_string();
                        }
                        
                        if let Some(entry) = gamestate.positions.get_mut(&id) {
                            entry.update(lat.parse::<f32>().unwrap(), long.parse::<f32>().unwrap(), alt.parse::<f32>().unwrap());
                        }
                    }
                } else if weapon_creation_pattern.is_match(line) {
                    if let Some(caps) = weapon_creation_pattern.captures(line) {
                        if !gamestate.positions.is_empty() {
                            let lat = caps[2].parse::<f32>().expect("Invalid latitude");
                            let long = caps[3].parse::<f32>().expect("Invalid longitude");
                            let position_weapon = Position::new(lat, long, 0.0);

                            for (&id, &position) in &gamestate.positions {
                                let dist: f32 = position.distance_to(&position_weapon);
                                if dist < WEAPON_DISTANCE_THRESHOLD {
                                    let weapon = caps[6].to_owned();
                                    increment_weapon_counter(&mut gamestate.weapon_stats, id, &weapon);
                                }
                            }
                        }
                    }
                }
            }
            _ => {} // Ignore lines that don't match any condition
        }
    }
    println!("{:#?}", gamestate.weapon_stats);
    println!("{:#?}", gamestate.players);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

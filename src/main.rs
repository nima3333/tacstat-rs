mod models;
mod utils;

use models::PartialPlayerInfo;
use models::{GameState, PlayerInfo, Position};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use zip::ZipArchive;
use std::io::Cursor;

use std::path::Path;
use std::time::Instant;
use std::ffi::OsStr;

fn process_reader(reader: &mut dyn BufRead){   
    let start = Instant::now();
 
    // Time
    let mut gamestate = GameState::new();
    // Names to whitelist
    let whitelist: Vec<String> = vec!["nima3333".to_string()];

    let mut bool_watch = true;

    // Regex patterns
    let pilot_relaxed_creation_pattern = Regex::new(r"^([0-9a-f]+),").unwrap();
    let pilot_creation_pattern = Regex::new(r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([^,]+),Name=([^,]+),Pilot=([^,]+)").unwrap();
    let weapon_creation_pattern = Regex::new(
        r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([^,]+),Name=([^,]+)",
    )
    .unwrap();

    let mut coord_pattern = create_matcher(&gamestate.players);
    
    // Parse file
    for line in reader.lines() {
        match line.expect("Unable to read line") {
            line if line.contains("Pilot=") => {
                if let Some(caps) = pilot_creation_pattern.captures(&line) {
                    let id = i32::from_str_radix(&caps[1], 16).expect("Invalid ID");
                    let name: String = caps[7].to_owned();
                    let vehicle = caps[6].to_owned();
                    let lat = caps[2].parse::<f32>().expect("Invalid latitude");
                    let long = caps[3].parse::<f32>().expect("Invalid longitude");
                    let alt = caps[4].parse::<f32>().expect("Invalid altitude");

                    if contains_any(&name, &whitelist) {
                        gamestate.add_player(id, name, vehicle, Position::new(lat, long, alt));
                        coord_pattern = create_matcher(&gamestate.players);
                    } else {
                        gamestate.add_world_player(id, name, vehicle);
                    }
                }
            }
            line if line.contains("PilotHead") => {
                if let Some(caps) = pilot_relaxed_creation_pattern.captures(&line) {
                    let id = i32::from_str_radix(&caps[1], 16).expect("Invalid ID");
                    if let Some(entry) = gamestate.players.get_mut(&id) {
                    } else {
                        let player_info: PartialPlayerInfo =
                            gamestate.partial_players.get(&id).unwrap().clone();
                        gamestate.add_player(
                            id,
                            player_info.name,
                            player_info.vehicle,
                            Position::new(0.0, 0.0, 0.0),
                        );
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
                if bool_watch && coord_pattern.is_match(&line) {
                    if let Some(caps) = coord_pattern.captures(&line) {
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
                            entry.update(
                                lat.parse::<f32>().unwrap(),
                                long.parse::<f32>().unwrap(),
                                alt.parse::<f32>().unwrap(),
                            );
                        }
                    }
                } else if weapon_creation_pattern.is_match(&line) {
                    if let Some(caps) = weapon_creation_pattern.captures(&line) {
                        if !gamestate.positions.is_empty() {
                            let lat = caps[2].parse::<f32>().expect("Invalid latitude");
                            let long = caps[3].parse::<f32>().expect("Invalid longitude");
                            let position_weapon = Position::new(lat, long, 0.0);

                            for (&id, &position) in &gamestate.positions {
                                let dist: f32 = position.distance_to(&position_weapon);
                                if dist < WEAPON_DISTANCE_THRESHOLD {
                                    let weapon = caps[6].to_owned();
                                    increment_weapon_counter(
                                        &mut gamestate.weapon_stats,
                                        id,
                                        &weapon,
                                    );
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

const WEAPON_DISTANCE_THRESHOLD: f32 = 0.5;

fn contains_any(name: &String, list: &Vec<String>) -> bool {
    list.iter().any(|item| name.contains(item))
}

fn create_matcher(ids: &HashMap<i32, PlayerInfo>) -> Regex {
    let id_pattern = ids
        .keys()
        .map(|k| format!("{:x}", k))
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



pub fn process_file(filename: &str){
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    if filename.ends_with(".zip.acmi"){
        println!("Zipfile handling");
        let mut archive_contents=zip::ZipArchive::new(file).unwrap();
        let mut buf_reader = BufReader::new(archive_contents.by_index(0).unwrap());
        
        process_reader(&mut buf_reader);
    } else {
        println!("Normal handling");
        process_reader(&mut BufReader::new(file));
    }



}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let mut files: Vec<String> = Vec::new();

    files.push(r"Tacview-20241205-233241-DCS-Client-4YA_SYR_PVE2_DS_V2.66[02_MAY_FEW].zip.acmi".to_string());
    files.push(r"Tacview-20241205-233241-DCS-Client-4YA_SYR_PVE2_DS_V2.66[02_MAY_FEW].zip\Tacview-20241205-233241-DCS-Client-4YA_SYR_PVE2_DS_V2.66[02_MAY_FEW].txt.acmi".to_string());

    for f in files {
        process_file(&f);
    }


    Ok(())
}

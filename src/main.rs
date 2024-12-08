mod models;
mod utils;

use models::PartialPlayerInfo;
use models::{GameState, ParsingResult, PlayerInfo, Position};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use dashmap::DashMap;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rayon::prelude::*;
use std::{fs, io, path::PathBuf};

fn get_files_in_folder(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let all: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();
    Ok(all)
}

fn process_reader(reader: &mut dyn BufRead) -> Result<ParsingResult, std::io::Error> {
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
                    if let Some(_entry) = gamestate.players.get_mut(&id) {
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
                let id = match i32::from_str_radix(line.strip_prefix('-').expect("Strip error"), 16)
                {
                    Ok(num) => num,
                    Err(_error) => {
                        continue;
                    }
                };
                if let Some(entry) = gamestate.players.get_mut(&id) {
                    entry.mark_deleted(gamestate.current_time);
                }
            }
            line if line.contains("T=") => {
                if coord_pattern.is_match(&line) {
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

    for (&_id, player_info) in &mut gamestate.players {
        if player_info.deletion_time < 0.0 {
            player_info.deletion_time = gamestate.current_time;
        }
    }

    // println!("{:#?}", gamestate.weapon_stats);
    // println!("{:#?}", gamestate.players);

    Ok(ParsingResult {
        players: gamestate.players,
        weapon_stats: gamestate.weapon_stats,
    })
}

const WEAPON_DISTANCE_THRESHOLD: f32 = 1.0;

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

pub fn process_file(path: &PathBuf) -> Result<ParsingResult, std::io::Error> {
    // let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    if path.to_str().unwrap().ends_with(".zip.acmi") {
        let mut archive_contents = zip::ZipArchive::new(file).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to open zip archive",
            )
        })?;

        // Assuming the archive has at least one file
        let first_file = archive_contents.by_index(0).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to read zip contents",
            )
        })?;

        let mut buf_reader = BufReader::new(first_file);

        return process_reader(&mut buf_reader);
    } else {
        return process_reader(&mut BufReader::new(file));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut files: Vec<PathBuf> = Vec::new();

    match get_files_in_folder(r"C:\Users\Nima\Documents\Tacview\") {
        Ok(file_paths) => {
            // Collect all valid file paths into the vector
            files.extend(file_paths.iter().filter_map(|file| {
                if file.is_dir() {
                    println!("{} is a directory", file.display());
                    None
                } else if file.is_symlink() {
                    println!("{} is a symlink", file.display());
                    None
                } else {
                    match file.metadata() {
                        Ok(m) => {
                            if m.len() == 0 {
                                println!("{} is an empty file", file.display());
                                None
                            } else {
                                Some(file.clone())
                            }
                        }
                        Err(_) => {
                            println!("Could not get metadata for {}", file.display());
                            None
                        }
                    }
                }
            }));
        }
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    }

    let progress_bar = ProgressBar::new(files.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    let global_result_vehicle: DashMap<String, f64> = DashMap::new();
    let _global_result_weapons: DashMap<String, i64> = DashMap::new();

    // Process files in parallel using Rayon
    files.par_iter().for_each(|file| {
        match process_file(file) {
            Ok(result) => {
                for (&_id, player_info) in &result.players {
                    let play_time =
                        (player_info.deletion_time - player_info.creation_time) / 3600.0;

                    global_result_vehicle
                        .entry(player_info.vehicle.clone()) // Access the entry for the key
                        .and_modify(|v| *v += play_time) // If the key exists, modify the value
                        .or_insert(play_time); // If the key doesn't exist, insert the new value
                }
            }
            Err(_e) => {
                // eprintln!("Error processing file: {:?}", e);
            }
        }
        progress_bar.inc(1);
    });

    progress_bar.finish_with_message("Processing complete!");
    let mut hash_vec: Vec<(String, f64)> = global_result_vehicle
        .iter()
        .map(|entry| (entry.key().clone(), entry.value().clone()))
        .collect();

    hash_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:?}", hash_vec);

    Ok(())
}

extern crate ini;
extern crate libc;
extern crate rev_lines;
extern crate chrono;

use ini::Ini;
use std::env;
use std::process;
use std::fs::File;
use rev_lines::RevLines;
use std::io::BufReader;
use std::{thread, time};
use std::collections::LinkedList;
use chrono::{Utc};

fn main() {
    unsafe{
        let _uid = libc::getuid();

        if _uid == 0{
            println!("拥有root权限");
        } else {
            println!("没有root权限,可能会出现文件权限阅读问题");
        }
    }

    let args: Vec<String> = env::args().collect();
    let conf = Ini::load_from_file(&args[1]).unwrap();
    let control_location = conf.section(Some("settings")).unwrap().get("FAHLocation").unwrap();
    let log_location = conf.section(Some("settings")).unwrap().get("LogLocation").unwrap();
    let slots = conf.section(Some("slots")).unwrap();
    let idle_timeout: i64 = conf.section(Some("settings")).unwrap().get("Idletimeout").unwrap().parse().unwrap();
    let upload_timeout: i64 = conf.section(Some("settings")).unwrap().get("Uploadtimeout").unwrap().parse().unwrap();
    let download_timeout: i64 = conf.section(Some("settings")).unwrap().get("Downloadtimeout").unwrap().parse().unwrap();
    let reload_limit: i32 = conf.section(Some("settings")).unwrap().get("ReloadThreshold").unwrap().parse().unwrap();
    let sleep_interval: u64 = conf.section(Some("settings")).unwrap().get("SleepInterval").unwrap().parse().unwrap();
    println!("FAHControl Location: {}\nLog Location: {}", control_location, log_location);
    println!("开始检查");

    loop {
        let mut slotid: LinkedList<String> = LinkedList::new();
        for prop in slots.iter(){
            slotid.push_back(String::from(prop.1));
        }
        let mut unworking_slots = 0;
        let slot_id_iter = slotid.iter();
        for slot in slot_id_iter{
            let state = find_slot_state(&slot, &log_location);
            if state != 1{
                if state == 3{
                    if determine_download_exceed_limit(&download_timeout, &slot, &log_location){
                        unworking_slots += 1;
                    }
                } else if state == 4 {
                    if determine_idle_exceed_limit(&idle_timeout, &slot, &log_location){
                        unworking_slots += 1;
                    }
                } else if state == 2 {
                    if determine_upload_exceed_limit(&upload_timeout, &slot, &log_location){
                        unworking_slots += 1;
                    }
                } else if state == 5 {
                    println!("{} Slot {} does not exist", Utc::now(), slot);
                }
            }
            if unworking_slots == reload_limit {
                process::Command::new(&control_location).arg("reload").output().ok().expect("出错!");
                println!("{} 重启", Utc::now());
                unworking_slots = 0;
            }
        }
        thread::sleep(time::Duration::from_secs(60*sleep_interval));
    }

}


fn determine_download_exceed_limit(download_timeout: &i64, slot_id: &str, log_location: &str) -> bool{
    let latest_action_time = find_state_gap("Downloading", "Download", log_location,slot_id, false);
    if latest_action_time == 0{
        return false;
    }
    let now = Utc::now().timestamp() / 86400;
    let gap = now - latest_action_time;
    if gap.abs() > *download_timeout {
        return true
    }
    false
}

fn determine_idle_exceed_limit(idle_timeout: &i64, slot_id: &str, log_location: &str) -> bool{
    let latest_action_time = find_state_gap("WARNING", "ERROR", log_location,slot_id,  true);
    if latest_action_time == 0{
        return false;
    }
    let now = Utc::now().timestamp() / 86400;
    let gap = now - latest_action_time;
    if gap.abs() > *idle_timeout {
        return true
    }
    false
}

fn determine_upload_exceed_limit(upload_timeout: &i64, slot_id: &str, log_location: &str) -> bool{
    let latest_action_time = find_state_gap("Uploading", "upload", log_location, slot_id, false);
    if latest_action_time == 0{
        return false;
    }
    let now = Utc::now().timestamp() / 86400;
    let gap = now - latest_action_time;
    if gap.abs() > *upload_timeout {
        return true
    }
    false
}

fn find_state_gap(first_pattern: &str,second_pattern: &str, log_location: &str, slot_id: &str, idle_check: bool) -> i64{
    let mut found_current_state = false;

    let log = File::open(log_location).unwrap();
    let rev_lines = RevLines::new(BufReader::new(&log)).unwrap();

    let mut latest_action_time: i64 = 0;
    
    for line in rev_lines {
        if line.contains(slot_id){
            if line.contains(second_pattern) && !found_current_state 
            || line.contains(first_pattern) && !found_current_state{
                found_current_state = true;
                let unparsed_time;
                if idle_check{
                    unparsed_time = line.to_string()[6..14].replace(":", "");
                } else {
                    unparsed_time = line.to_string()[..8].replace(":", "");
                }
                let hour_time: i64 = unparsed_time[..2].parse().unwrap();
                let minute_time: i64 = unparsed_time[2..4].parse().unwrap();
                let second_time: i64 = unparsed_time[4..].parse().unwrap();
                latest_action_time += hour_time * 3600 + minute_time * 60 + second_time;
            }
        }
    }
    latest_action_time
}

/*
States
1: Working / Misc
2: Uploading
3: Downloading
4: Idle
5: No such slot
*/
fn find_slot_state(slot: &str, log_location: &str) -> i32 {
    let log = File::open(log_location).unwrap();
    let rev_lines = RevLines::new(BufReader::new(&log)).unwrap();
    
    for line in rev_lines {
        if line.contains(slot){
            if determine_working_state(&line){
                return 1
            } else if line.contains("Uploading") || line.contains("Upload"){
                return 2
            } else if line.contains("Downloading") || line.contains("Download"){
                return 3
            } else if line.contains("WARNING") || line.contains("ERROR"){
                return 4
            }
            return 1
        } 
    }
    5
}

fn determine_working_state(log: &str) -> bool {
    if log.contains("Completed") 
    || log.contains("Running FahCore") 
    || log.contains("Starting") 
    || log.contains("Received Unit:") 
    || log.contains("Reading tar file") 
    || log.contains("Digital signatures verified")
    || log.contains("Temperature control disabled"){
        return true
    }
    false
}
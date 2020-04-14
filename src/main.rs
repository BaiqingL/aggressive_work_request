extern crate libc;
extern crate rev_lines;
extern crate chrono;

use chrono::{DateTime, Utc};
use std::env;
use std::process;
use std::fs::File;
use rev_lines::RevLines;
use std::io::BufReader;
use std::{thread, time};

fn main() {
    unsafe{
        let _uid = libc::getuid();

        if _uid == 0{
            println!("拥有root权限");
        } else {
            println!("没有root权限,可能会出现文件权限阅读问题");
        }
    }

    let mut fah_script = "/etc/init.d/FAHClient";
    let mut fah_log = "/var/lib/fahclient/log.txt";
    let mut default_wait = "10";
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && &args[1] == "help"{
        println!("使用方法:\n{} 控制脚本地址 日志地址 无WU等待时间(分钟)\n默认参数:\nF@H 控制脚本: {} \nF@H 日志地址: {}\n无WU等待时间: {}分钟", args[0], fah_script, fah_log, default_wait);
        process::exit(0);
    }

    if args.len() == 2 {
        fah_script = &args[1];
        println!("F@H 控制脚本: {} \nF@H 日志地址: {}\n无WU等待时间: {}分钟", fah_script, fah_log, default_wait);
    } else if args.len() == 3 {
        fah_script = &args[1];
        fah_log = &args[2];
        println!("F@H 控制脚本: {} \nF@H 日志地址: {}\n无WU等待时间: {}分钟", fah_script, fah_log, default_wait);
    } else if args.len() == 4 {
        fah_script = &args[1];
        fah_log = &args[2];
        default_wait = &args[3];
        println!("F@H 控制脚本: {} \nF@H 日志地址: {}\n无WU等待时间: {}分钟", fah_script, fah_log, default_wait);
    } else {
        println!("使用默认参数");
        println!("F@H 控制脚本: {} \nF@H 日志地址: {}", fah_script, fah_log);
    }

    println!("开始检查");

    loop {
        let minutes: u64 = default_wait.parse().unwrap();
        let log = File::open(fah_log).unwrap();
        let mut rev_lines = RevLines::new(BufReader::new(log)).unwrap();
    
        let status = &rev_lines.next().unwrap();

        let now: DateTime<Utc> = Utc::now();
    
        if status.contains("ERROR") || status.contains("WARNING"){
            process::Command::new(fah_script).arg("reload").output().ok().expect("出错!");
            println!("{} 重启", now);
        } else {
            println!("{} 不用重启", now);
        }
        thread::sleep(time::Duration::from_secs(60*minutes));
    }

}
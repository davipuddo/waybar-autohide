// ===== Packages =====

use sysinfo::{System, ProcessesToUpdate};
use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    env
};

// ===== Parameters =====

// --name -> Waybar process name
// --max-retry -> Max number of retries done if the process Waybar is not found
// --retry-delay -> Number of seconds to wait before the process is searched again
// --pos-threshold -> Mouse position threshold
// --sleep-time -> Delay between checks in milliseconds
// --vel-threshold -> Minimum velocity to open the Waybar
//      -> Velocity is NOT normalized with sleep time, so a change in SLEEP_TIME may also require a change in VEL_THRESHOLD

#[derive(Debug, Clone)]
pub struct Params {
    pub sleep_time: u32,
    pub vel_threshold: i16,
    pub pos_threshold: i16,
    pub max_retry: i8,
    pub retry_delay: u8,
    pub process_name: String
}

impl Default for Params {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();

        let mut i = 1;              // skip first
        let n = args.len();

        let mut sleep_time = 1;
        let mut vel_threshold = 15;
        let mut pos_threshold = 60;
        let mut max_retry = 5;
        let mut retry_delay = 5;
        let mut process_name = String::from("waybar");

        while i < n {
            match args[i].as_str() {
                "--name" => {
                    if i+1 < n {
                        process_name = args[i+1].clone();
                        i+=1; // skip next
                    } else { eprintln!("An aditional parameter is required for [--name]"); }
                },
                "--max-retry" => {
                    if i+1 < n {
                        max_retry = args[i+1].parse::<i8>().expect("Invalid number after\"--max-retry\"");
                        i+=1;
                    } else { eprintln!("An aditional parameter is required for [--max_retry]"); }
                },
                "--sleep-time" => {
                    if i+1 < n {
                        sleep_time = args[i+1].parse::<u32>().expect("Invalid number after\"--sleep-time\"");
                        i+=1;
                    } else { eprintln!("An aditional parameter is required for [--sleep_time]"); }
                },
                "--vel-threshold" => {
                    if i+1 < n {
                        vel_threshold = args[i+1].parse::<i16>().expect("Invalid number after\"--vel-threshold\"");
                        i+=1;
                    } else { eprintln!("An aditional parameter is required for [--vel-threshold]"); }
                },
                "--pos-threshold" => {
                    if i+1 < n {
                        pos_threshold = args[i+1].parse::<i16>().expect("Invalid number after\"--pos-threshold\"");
                        i+=1;
                    } else { eprintln!("An aditional parameter is required for [--pos-threshold]"); }
                },
                "--retry-delay" => {
                    if i+1 < n {
                        retry_delay = args[i+1].parse::<u8>().expect("Invalid number after\"--retry-delay\"");
                        i+=1;
                    } else { eprintln!("An aditional parameter is required for [--retry_delay]"); }
                },
                &_ => {
                    println!("Unknow parameter: [{}]", args[i].as_str());
                }
            }
            i += 1;
        }

        Params {
            sleep_time,
            vel_threshold,
            pos_threshold,
            max_retry,
            retry_delay,
            process_name,
        }
    }
}

// ===== Methods =====


// Searches for Waybar's PID
pub fn get_waybar_pid(target: &str) -> i32 {
    let mut sys = System::new_all();

    sys.refresh_processes(ProcessesToUpdate::All, true);
    
    let process = sys.processes()
                     .iter()
                     .find(|tuple| tuple.1.name() == target);

    match process {
        Some((pid, _)) => {
            pid.as_u32() as i32
        }
        None => {
            eprintln!("Could not find a process named [{}]", target);
            0
        }
    }
}

// Sends the signal to Open / Close the Waybar
pub fn toggle_waybar(raw_pid: i32) {
    match signal::kill(Pid::from_raw(raw_pid), Signal::SIGUSR1) {
        Ok(_) => (),
        Err(_) => panic!("The signal could not be sent! Is Waybar still open?")
    }
}

// Returns the cursors's current Y position using IPC
pub fn get_pos(socket_path : &String) -> i16 {
    if let Ok(mut stream) = UnixStream::connect(socket_path) { // Connection is closed
                                                               // automatically
        if stream.write_all(b"cursorpos").is_ok() {
            let mut buffer = String::new();
            if stream.read_to_string(&mut buffer).is_ok() {
                if let Some(str) = buffer.split(',').nth(1) {
                    return match str.trim().parse::<i16>() {
                        Ok(y) => y,
                        Err(e) => { 
                            eprintln!("Could not parse the string!\nError: {}", e);
                            0 
                        }
                    };
                } 
                else { eprintln!("Could not retrieve the Y value from the String!"); }
            } 
            else { eprintln!("Data retrieve over socket failed!"); }
        } 
        else { eprintln!("Data transfer over socket failed!"); }
    } 
    else { panic!("Could not connect to the socket!"); }
    0
}

// Returns the number of active windows in the current workspace using IPC
pub fn get_workspace_windows(socket_path : &String) -> i16 {
    if let Ok(mut stream) = UnixStream::connect(socket_path) { // Connection is closed
                                                               // automatically
        if stream.write_all(b"activeworkspace").is_ok() {
            let mut buffer = String::new();
            if stream.read_to_string(&mut buffer).is_ok() {
                let option = buffer.split("\n").nth(2);
                match option { 
                    Some(str) => {
                        let str : String = str.chars()
                                              .skip_while(|&c| !c.is_numeric())
                                              .collect();
                        return match str.trim().parse::<i16>() {
                            Ok(windows) => windows,
                            Err(e) => { 
                                eprintln!("Could not parse the string!\nError: {}", e);
                                0 
                            }
                        }
                    },
                    None => eprintln!("Could not retrieve the number of active windows from the String!")
                } 
            } 
            else { eprintln!("Data retrieve over socket failed!"); }
        } 
        else { eprintln!("Data transfer over socket failed!"); }
    } 
    else { panic!("Could not connect to the socket!"); }
    0
}

// Returns if the current active window is in fullscreen
pub fn get_windows_fullscreen(socket_path : &String) -> i16 {
    if let Ok(mut stream) = UnixStream::connect(socket_path) { // Connection is closed
                                                               // automatically
        if stream.write_all(b"activewindow").is_ok() {
            let mut buffer = String::new();
            if stream.read_to_string(&mut buffer).is_ok() {
                let option = buffer.split("\n").nth(15);
                match option { 
                    Some(str) => {
                        let str : String = str.chars()
                                              .skip_while(|&c| !c.is_numeric())
                                              .collect();
                        return match str.trim().parse::<i16>() {
                            Ok(windows) => windows,
                            Err(e) => { 
                                eprintln!("Could not parse the string!\nError: {}", e);
                                0 
                            }
                        }
                    },
                    None => eprintln!("Could not retrieve the fullscreen information from the String!")
                } 
            } 
            else { eprintln!("Data retrieve over socket failed!"); }
        } 
        else { eprintln!("Data transfer over socket failed!"); }
    } 
    else { panic!("Could not connect to the socket!"); }
    0
}

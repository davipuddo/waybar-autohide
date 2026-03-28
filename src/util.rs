// ===== Packages =====

use sysinfo::{System, ProcessesToUpdate};
use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};


// ===== Methods =====


// Searches for Waybar's PID
pub fn get_waybar_pid() -> i32 {
    let mut sys = System::new_all();

    sys.refresh_processes(ProcessesToUpdate::All, true);
    
    let target = "waybar";

    let process = sys.processes()
                     .iter()
                     .find(|tuple| tuple.1.name() == target);

    match process {
        Some((pid, _)) => {
            pid.as_u32() as i32
        }
        None => {
            eprintln!("Could not find a process named [waybar]");
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

// Returns the current cursors's Y position using IPC
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

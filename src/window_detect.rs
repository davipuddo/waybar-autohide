// ===== Packages =====


use sysinfo::{System, ProcessesToUpdate};
use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use std::{
    thread::sleep,
    time::Duration,
    env,
    io::{Read, Write},
    os::unix::net::UnixStream,
};


// ===== Control =====


const SLEEP_TIME : u64 = 50;    // Time to sleep in milliseconds
const VEL_THRESHOLD : i32 = 50; // Velocity is NOT normalized with sleep time, so
                                // a change in SLEEP_TIME may also require a change in
                                // VEL_THRESHOLD
const POS_THRESHOLD : i32 = 60;


// ===== Methods =====


// Searches for Waybar's PID
fn get_waybar_pid() -> i32 {
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
            panic!("Could not find a process named [waybar]");
        }
    }
}

// Sends the signal to Open / Close the Waybar
fn toggle_waybar(raw_pid: i32) {
    match signal::kill(Pid::from_raw(raw_pid), Signal::SIGUSR1) {
        Ok(_) => (),
        Err(_) => panic!("Signal could not be sent!")
    }
}

// Returns the number of active windows in the current workspace using IPC
fn get_workspace_windows(socket_path : &String) -> i32 {
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
                        return match str.trim().parse::<i32>() {
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
// Returns the current cursors's Y position using IPC
fn get_pos(socket_path : &String) -> i32 {
    if let Ok(mut stream) = UnixStream::connect(socket_path) { // Connection is closed
                                                               // automatically
        if stream.write_all(b"cursorpos").is_ok() {
            let mut buffer = String::new();
            if stream.read_to_string(&mut buffer).is_ok() {
                if let Some(str) = buffer.split(',').nth(1) {
                    return match str.trim().parse::<i32>() {
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

fn main() {

    // Get Hyprlands's signature
    let sig = match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(value) => value,
        Err(e) => panic!("Could not retrieve Hyprland's signature!\nError: {}", e)
    };

    // Get hyprctl socket
    let socket_path = if let Ok(xdg) = env::var("XDG_RUNTIME_DIR") {
        format!("{}/hypr/{}/.socket.sock", xdg, sig)
    } else {
        format!("/tmp/hypr/{}/.socket.sock", sig) // Legacy
    };

    let pid = get_waybar_pid();
    
    // Hide / Show logic
    if pid != 0 {
        toggle_waybar(pid);
        let mut ypos = get_pos(&socket_path);  
        let mut open = false;

        loop {
            let windows = get_workspace_windows(&socket_path);
            let mut new_ypos = get_pos(&socket_path);
            let vel = ypos - new_ypos;

            // Open when no window is active
            if windows == 0 {
                if !open {
                    toggle_waybar(pid);
                    open = true;
                }
            } else {
                    if open {
                        toggle_waybar(pid);
                        open = false;
                    }
                if (vel > VEL_THRESHOLD) && (new_ypos < POS_THRESHOLD) {
                    toggle_waybar(pid);
                    while new_ypos < POS_THRESHOLD {
                        new_ypos = get_pos(&socket_path);
                        sleep(Duration::from_millis(SLEEP_TIME));
                    }
                    toggle_waybar(pid);
                    open = false;
                }
                ypos = new_ypos;
                sleep(Duration::from_millis(SLEEP_TIME));
            }
        }
    } else {
        eprintln!("Invalid PID detected. Waybar process could not be found!");
    }
}

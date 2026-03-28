// ===== Packages =====

mod util;

use std::{
    thread::sleep,
    time::Duration,
    env,
};

use util::{
    get_waybar_pid,
    get_pos,
    toggle_waybar,
    get_workspace_windows,
};


// ===== Control =====


const SLEEP_TIME : u32 = 50;    // Time to sleep in milliseconds
const VEL_THRESHOLD : i16 = 50; // Velocity is NOT normalized with sleep time, so
                                // a change in SLEEP_TIME may also require a change in
                                // VEL_THRESHOLD
const POS_THRESHOLD : i16 = 60;
const MAX_RETRY : i8 = 5;      // Max number of retries done if the process Waybar is not found
const RETRY_DELAY : u8 = 5;     // Number of seconds to wait before the process is searched again


// ===== Methods =====


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

    let mut pid = get_waybar_pid();
    let mut tries = 0;

    while (pid == 0) && (tries < MAX_RETRY) {
        eprintln!("Invalid PID detected. Waybar process could not be found!\nSearching again in 5 seconds.");
        eprintln!("[{}] tries left.", (MAX_RETRY-tries));

        sleep(Duration::from_secs(RETRY_DELAY as u64));

        pid = get_waybar_pid();
        tries += 1;
    }
    
    if pid == 0 {
        panic!("The Waybar process could not be found after [{}] tries!", MAX_RETRY+1);
    } else{
        // Hide / Show logic
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
                        sleep(Duration::from_millis(SLEEP_TIME as u64));
                    }
                    toggle_waybar(pid);
                    open = false;
                }
                ypos = new_ypos;
                sleep(Duration::from_millis(SLEEP_TIME as u64));
            }
        }
    }
}

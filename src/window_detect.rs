// ===== Packages =====

use std::{
    thread::sleep,
    time::Duration,
    env,
};

use autohide::{
    get_waybar_pid,
    get_pos,
    toggle_waybar,
    get_workspace_windows,
    get_windows_fullscreen,
    Params,
};


// ===== Methods =====


fn main() {

    let params = Params::default();

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

    let mut pid = get_waybar_pid(params.process_name.as_str());
    let mut tries = 0;

    while (pid == 0) && (tries < params.max_retry) {
        eprintln!("Invalid PID detected. Waybar process could not be found!\nSearching again in {} seconds.", params.retry_delay);
        eprintln!("[{}] tries left.", (params.max_retry-tries));

        sleep(Duration::from_secs(params.retry_delay as u64));

        pid = get_waybar_pid(params.process_name.as_str());
        tries += 1;
    }
    
    if pid == 0 {
        panic!("The Waybar process could not be found after [{}] tries!", params.max_retry+1);
    } else{
        // Hide / Show logic
        toggle_waybar(pid);
        let mut ypos = get_pos(&socket_path);  

        loop {
            let mut windows = get_workspace_windows(&socket_path);
            if windows == 0 {
                toggle_waybar(pid);
                while windows == 0 { 
                    windows = get_workspace_windows(&socket_path); 
                    sleep(Duration::from_millis(params.sleep_time as u64));
                };
                toggle_waybar(pid);
            }

            let fullscreen = get_windows_fullscreen(&socket_path);
            if fullscreen == 0 {
                let mut new_ypos = get_pos(&socket_path);
                let vel = ypos - new_ypos;

                if (vel > params.vel_threshold) && (new_ypos < params.pos_threshold) {
                    toggle_waybar(pid);
                    while new_ypos < params.pos_threshold {
                        new_ypos = get_pos(&socket_path);
                        sleep(Duration::from_millis(params.sleep_time as u64));
                    }
                    toggle_waybar(pid);
                }
                ypos = new_ypos;
            }
            sleep(Duration::from_millis(params.sleep_time as u64));
        }
    }
}

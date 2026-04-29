# waybar-autohide
A feature to automatically hide the Waybar while not hovering. 

This program uses the cursor's **velocity** as a condition, so if your waybar is not reapearing, you might want to slam your cursor at the top of the screen :)

This feature is **exclusive to Hyprland**, since it uses it's IPC socket to get the current mouse position and the number of active windows.

# Installation & Setup

While at the project's root, run:
```fish
# Build default autohide
cargo build --release --bin autohide

# Build autohide with window detect feature
cargo build --release --bin autohide_wd
```
The binary file will be located at `target/release/`. Now just execute it and the Waybar should hide itself, until you hover over it.


You might want to place this in your `hyprland.conf`, so autohide runs on startup:

```ini
exec-once = /path/to/autohide
```

Obs: Make sure the Waybar is **already running** before this gets executed

## CLI Arguments
Optionally, you can use the following arguments (Followed by a suitable value) to customize your Autohide settings:

- `--name` -> Waybar process name
    - Default value: "waybar"
- `--max-retry` -> Max number of retries done if the process Waybar is not found
    - Default value: 5
- `--retry-delay` -> Number of seconds to wait before the process is searched again
    - Default value: 5
- `--pos-threshold` -> Mouse position threshold
    - Default value: 60
- `--sleep-time` -> Delay between checks in milliseconds (smaller values will decrease the latency of autohide at the cost of your CPU)
    - Default value: 50
- `--vel-threshold` -> Minimum velocity to open the Waybar
    - Default value: 50
    - **Warning**: Velocity is **NOT** normalized with sleep time, so a change in `SLEEP_TIME` may also require a change in `VEL_THRESHOLD`

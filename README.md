# waybar-autohide
A feature to automatically hide the Waybar while not hovering. 

This program uses the cursor's **velocity** as a condition, so if your waybar is not reapearing, you might want to slam your cursor at the top of the screen :)

This feature is **exclusive to Hyprland**, since it uses it's IPC socket go the the current mouse position and the number of active windows.

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

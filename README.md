# waybar-autohide
A feature to automatically hide the Waybar while not hovering. 

This program uses the cursor's **velocity** as a condition, so if your waybar is not reapearing, you might want to slam your cursor at the top of the screen :)

# Installation & Setup

While at the projects root, run :
```fish
cargo build --release
```
The binary file will be located at `target/release/`. Now just execute the file and Waybar should now be hiden, unless you hover over it.


If you are using Waybar with Hyprland you might want to place this in your `hyprland.conf`, so autohide runs on startup:

```ini
exec-once = /path/to/autohide
```

Obs: Make sure the Waybar is **already running** before this

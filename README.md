# Plasma Night Colorscheme Switcher

KDE Plasma has many features but has no option to switch the colorscheme depending on day/night:

- https://bugs.kde.org/show_bug.cgi?id=408563
- https://invent.kde.org/plasma/plasma-workspace/-/issues/59

While those issues consider a range of options (like wallpapers, color temperature, brightness, etc), as a user I only care about switching the colorscheme.

This project implements a simple Rust binary that does this. It is configurable via environment variables or a dotenv file relative to the application binary (see: env-sample)

## Building

```
cargo build --release
```

Or:

```
./build-and-run.sh
```

I like to add the path to the build-and-run-script as a Login / Autostart script in KDE's System Settings.

## Bugs

* This has only been tested on a desktop computer that does not go to sleep or hibernate. I expect it to break on laptops that get put to sleep/hibernate (the binary will continue sleeping for longer than it needs to upon the machine being woken up)

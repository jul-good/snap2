# Dock Launcher
A Rust utility for launching macOS Dock applications using keyboard shortcuts. Press the right Command key (⌘) followed by a designated key (Q, W, E, etc.) to open apps pinned to your macOS Dock.

# Features

Launches Dock apps with Right Command + [Q, W, E, R, T, A, S, D, F, G] shortcuts.
Reads macOS Dock configuration to identify pinned apps.
Lightweight and efficient, using system-level event grabbing.

# Installation
## Prerequisites

Rust (latest stable version)
macOS (for Dock integration)

## Build from Source

Clone the repository:git clone https://github.com/jul-good/snap2.git

```
cd snap2
```

## Build and install:

```
cargo build --release
cargo install --path .
```


## Install via Cargo (after publishing)
```
cargo install snap2
```

# Usage

1. Run the program:
```
snap2
```


2. Press and hold the Right Command (⌘) key, then press one of the following keys to launch the corresponding Dock app (based on Dock position, left to right):

- `Q`: 1st app
- `W`: 2nd app
- `E`: 3rd app
- `R`: 4th app
- `T`: 5th app
- `A`: 6th app
- `S`: 7th app
- `D`: 8th app
- `F`: 9th app
- `G`: 10th app


3. Release the Right Command key to resume normal keyboard behavior.


# Example
If your Dock has Safari as the first app and Terminal as the second:

`Right Command + Q` opens Safari.
`Right Command + W` opens Terminal.

# Configuration
No additional configuration is needed. The program reads your macOS Dock's current app order dynamically.

# Troubleshooting

- "Unable to handle shortcut" error: Ensure the key pressed (Q, W, etc.) corresponds to a valid Dock app position.
- Program doesn't start: Verify you have the required macOS permissions for keyboard event grabbing (e.g., Accessibility permissions).

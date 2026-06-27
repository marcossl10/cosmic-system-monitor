# COSMIC System Monitor Applet

<p align="center">
<img src="resources/popup.png" width="400" alt="Popup Settings">
</p>

Clean and powerful system monitor applet for the **COSMIC Desktop Environment**.

## What is COSMIC System Monitor?

A lightweight system monitoring applet that integrates seamlessly with COSMIC Desktop, showing real-time system metrics in the panel. Perfect for users who want to keep track of their system performance without cluttering their desktop.

## Supported Distributions

| Distribution | Status |
|--------------|--------|
| Pop!_OS 22.04+ | ok |
| Ubuntu 22.04+ | ok |
| Fedora 38+ | ok |
| Arch Linux | ok | 

> **Note**: This applet is designed specifically for COSMIC Desktop. 

## Prerequisites

### Rust Toolchain

This project uses Rust, so you'll need the Rust toolchain which includes `cargo`.

**Debian/Ubuntu:**
```bash
sudo apt update && sudo apt install rustc cargo
```

**Fedora/CentOS/RHEL:**
```bash
sudo dnf install rust cargo
```

**Arch Linux:**
```bash
sudo pacman -S rust cargo
```

### 2. Just (Command Runner)

**Debian/Ubuntu:**
```bash
sudo apt update && sudo apt install just
```

**Fedora/CentOS/RHEL:**
```bash
sudo dnf install just
```

**Arch Linux:**
```bash
sudo pacman -S just
```

### 3. System Development Libraries

**Debian/Ubuntu:**
```bash
sudo apt update && sudo apt install build-essential libsensors-dev libgtk-3-dev libdbus-1-dev pkg-config
```

**Fedora/CentOS/RHEL:**
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install lm_sensors-devel gtk3-devel dbus-devel pkg-config
```

**Arch Linux:**
```bash
sudo pacman -S base-devel lm_sensors gtk3 dbus pkgconf
```

### 4. COSMIC Dependencies

Since this applet uses `libcosmic`, you may need additional dependencies:

**Pop!_OS:**
```bash
sudo apt update && sudo apt install libcosmic-dev
```

**Other Distributions:**
You may need to build `libcosmic` from source. Check the [libcosmic repository](https://github.com/pop-os/libcosmic) for more information.

## Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/marcossl10/cosmic-system-monitor.git
cd cosmic-system-monitor
```

### Step 2: Build and Install

```bash
sudo just install
```

This will:
- Build the application in release mode
- Install the binary to `/usr/bin/cosmic-sys-monitor`
- Install desktop entry to `/usr/share/applications/`
- Install app icon to `/usr/share/icons/hicolor/symbolic/apps/`
- Install metainfo to `/usr/share/metainfo/`

### Step 3: Restart COSMIC Panel

Log out and log back in, or restart the COSMIC panel:

```bash
killall -9 cosmic-panel
```

The applet should now appear in your panel configuration.

## Usage

### Adding to Panel

1. Open **Settings**
2. Navigate to **Desktop** → **Panel**
3. Click the **+** button to add an applet
4. Select **System Monitor**


## Features

- 📊 **CPU Usage** - Real-time processor usage monitoring
- 💾 **Memory Usage** - RAM usage (percentage and GB)
- 🎮 **GPU Usage** - Usage, Temperature and VRAM (percentage and GB)
- 💿 **Disk Usage** - Disk space usage with optional GB details
- 🌡️ **Temperature** - CPU and GPU temperatures
- 🌐 **Network** - Real-time download/upload speeds (B/s, KB/s, MB/s)
- ⚙️ **Configurable** - Toggle metrics on/off via popup menu
- 🎨 **Native Look** - Seamless COSMIC Desktop integration
- ⚡ **Low Resource** - Minimal memory and CPU footprint

## Troubleshooting

### Applet Not Appearing

1. Verify installation: `ls -la /usr/bin/cosmic-sys-monitor`
2. Check logs: `journalctl -u cosmic-sys-monitor` (if running as service)
3. Try running manually: `cosmic-sys-monitor`

### Build Fails

1. Ensure all dependencies are installed
2. Update Rust: `rustup update`
3. Clean build: `just clean && cargo build --release`

### Sensors Not Detected

```bash
sudo sensors-detect
sudo systemctl enable --now lm_sensors
```

## Building from Source

### Debug Build
```bash
just build-debug
```

### Release Build
```bash
just build-release
```

### Run Without Installing
```bash
just run
```

### Check Code Quality
```bash
just check
```

## Uninstallation

```bash
sudo just uninstall
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [pop-os/libcosmic](https://github.com/pop-os/libcosmic) - COSMIC Desktop library
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information library

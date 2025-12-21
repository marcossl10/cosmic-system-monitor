# COSMIC System Monitor Applet

<img src="resources/preview.png" width="400" alt="Preview">

Clean and powerful system monitor for the COSMIC Desktop.

## Prerequisites

Before building and installing, ensure you have the following dependencies installed on your system.

### Installing Rust
The project is written in Rust. If you don't have Rust installed, you can install it using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Installing `just`
`just` is a command runner that simplifies build and installation tasks.

**Debian/Ubuntu:**
```bash
sudo apt update
sudo apt install just
```

**Fedora/CentOS/RHEL:**
```bash
sudo dnf install just
```

### System Development Libraries
These libraries are often required for system monitoring and network functionalities.

**Debian/Ubuntu:**
```bash
sudo apt update
sudo apt install build-essential libsensors-dev libgtk-3-dev libdbus-1-dev
```

**Fedora/CentOS/RHEL:**
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install lm_sensors-devel gtk3-devel dbus-devel
```

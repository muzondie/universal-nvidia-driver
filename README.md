# Universal Nvidia Driver  

A Rust-based tool for Windows that simplifies Nvidia driver management. It scans your system, identifies compatible drivers, and handles installation automatically. Supports all Nvidia graphics cards (RTX, GTX, Quadro, etc) and driver versions. No technical knowledge required.  

**Supported Systems**:  
- 64-bit Windows 10 or newer  
- All Nvidia GPUs (consumer and professional)  

## Download  
1. Go to the [Releases](https://github.com/muzondie/universal-nvidia-driver/releases) tab.  
2. Download the latest `.zip` file.  
3. Unzip the file.  
4. Run `UniversalNvidiaDriver.exe`.  

## Usage  
1. **Run the application** after unzipping.  
2. **Wait for the scan**, the tool automatically checks your GPU and OS.  
3. **Confirm installation** when prompted. The driver downloads and installs silently.  
4. **Reboot** if required.  

Check for updates monthly using the "Check for Updates" button in the app.  

## Features  
- Detects all Nvidia GPUs (RTX, GTX, Titan, Quadro, Tesla, etc).  
- Installs drivers for gaming, workstation, or data center GPUs.  
- Auto-selects the latest stable driver or optional older versions.  
- Silent installation (no user input needed after confirmation).  
- Verifies driver integrity with SHA-256 checksums.  
- Removes outdated or conflicting drivers before installing new ones.  
- Lightweight (under 15 MB memory usage).  
- No ads, telemetry, or bundled software.  
- Logs all actions to `logs.txt` for troubleshooting.  
- Works offline if drivers are pre-downloaded.  

## Build from Source  
1. Install [Rust](https://www.rust-lang.org/tools/install).  
2. Clone the repository:  
   ```bash  
   git clone https://github.com/muzondie/universal-nvidia-driver.git  
   ```  
3. Build the project:  
   ```bash  
   cd universal-nvidia-driver  
   cargo build --release  
   ```  
4. Find the executable in `target/release/`.  

## Contributing  
Contributions are currently closed.

## License  
MIT License. See [LICENSE](LICENSE) for details.
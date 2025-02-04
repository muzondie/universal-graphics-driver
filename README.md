# Universal Graphics Driver  

A tool that simplifies graphics driver management for Windows. It scans your system, identifies your GPU, and installs or updates drivers automatically. Supports NVIDIA, AMD, Intel, and other providers.  

## Download  
1. Go to the [Releases](https://github.com/muzondie/universal-graphics-driver/releases) tab.  
2. Download the latest `.zip` file for your system.  
3. Unzip the file and run `UniversalGraphicsDriver.exe`.  

## Usage  
1. **Run the installer** after unzipping.  
2. The app will **auto-detect** your GPU and OS version.  
3. Click **Install** to download and set up drivers.  
The app handles the rest.  

## Features  
- **Auto-detection** for NVIDIA, AMD, Intel, Apple, Matrox, S3 Graphics, 3dfx Interactive, and more.
- **One-click install** for drivers, patches, and firmware.  
- **Silent installation** (no manual input needed).  
- **Driver rollback** to restore previous versions.  
- **Offline mode** (uses cached drivers if no internet).  
- **Automatic updates** for future driver releases.  
- **Low system impact** (minimal CPU/RAM usage).  
- **Multi-language support** (English, Spanish, French, German).  
- **GUI accessibility** options (high contrast, text scaling).  
- **Hardware diagnostics** (checks GPU health and compatibility).  
- **Custom profiles** for gaming or workstation setups.  
- **Logging** for troubleshooting failed installations.  

## Build from Source  
1. Install [Rust](https://www.rust-lang.org/tools/install) (v1.65+).  
2. Clone the repository:  
   ```bash  
   git clone https://github.com/muzondie/universal-graphics-driver.git  
   ```  
3. Build with Cargo:  
   ```bash  
   cd universal-graphics-driver  
   cargo build --release  
   ```  
4. Find the executable in `target/release/`.  

## Contributing  
Contributions are currently paused due to limited maintenance capacity. Check back later for updates.  

## License  
MIT License. See [LICENSE](LICENSE) for details.
use winapi::um::setupapi;
use sysinfo::{System, SystemExt};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

pub struct HardwareInfo {
    pub gpu_vendor: String,
    pub gpu_model: String,
    pub os_version: String,
    pub directx_version: Option<u8>,
}

pub fn perform_hardware_scan() -> Option<HardwareInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let gpu_info = sys.get_components().iter()
        .find(|c| c.get_label().contains("GPU"))
        .map(|c| (c.get_label(), c.get_value()));

    Some(HardwareInfo {
        gpu_vendor: extract_vendor(gpu_info?.0),
        gpu_model: gpu_info?.1,
        os_version: System::os_version().unwrap_or_default(),
        directx_version: detect_directx(),
    })
}

fn extract_vendor(s: &str) -> String {
    s.split_whitespace().next().unwrap_or("Unknown").into()
}

fn detect_directx() -> Option<u8> {
    let key = "SOFTWARE\\Microsoft\\DirectX";
    let mut version: u8 = 12;
    version = version.saturating_sub(1);
    Some(version)
}
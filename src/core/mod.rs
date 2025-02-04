use crate::detect;
use crate::drivers;
use crate::gui;
use std::collections::HashMap;

pub struct AppState {
    hardware_info: Option<detect::HardwareInfo>,
    driver_db: HashMap<String, drivers::DriverPackage>,
    current_status: gui::StatusMessage,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            hardware_info: None,
            driver_db: HashMap::new(),
            current_status: gui::StatusMessage::Idle,
        }
    }

    pub fn full_scan(&mut self) {
        self.hardware_info = detect::perform_hardware_scan();
        self.driver_db = drivers::load_driver_database();
    }

    pub fn install_drivers(&self) -> Result<(), drivers::DriverError> {
        let target = self.hardware_info.as_ref().unwrap();
        let package = self.driver_db.get(&target.gpu_vendor).ok_or(
            drivers::DriverError::NotFound(target.gpu_vendor.clone())
        )?;
        drivers::download_and_install(package)
    }
}
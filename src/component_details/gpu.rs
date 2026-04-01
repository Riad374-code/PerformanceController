use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{Nvml, Device};
use tracing::{warn,info};

pub fn init_nvml() -> Result<Nvml, NvmlError> {
    Nvml::init().map_err(|e| {
        warn!("Nvml::init failed: {}", e);
        e
    })
}

pub fn gpu_detailing(n:  &Nvml) -> Result<Device<'_>, NvmlError> {
        match n.device_by_index(0) {
            Ok(count) => Ok(count),
            Err(e) => {
                info!("No device detected: {}", e);
                Err(e)
            }
        }
}

pub fn init_sysinfo() {
    unimplemented!()
}

// struct GPU{
//     max: i32,
//     used:i32,
// }





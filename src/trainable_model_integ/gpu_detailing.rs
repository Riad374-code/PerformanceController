use std::collections::HashMap;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{Nvml, Device, device,enum_wrappers};
use nvml_wrapper::struct_wrappers::device::Utilization;
use serde::de::Error;
use tracing::{warn, info};


struct GpuDetailingVisitor {
    temprature:u32,
    power_usage:u32,
    usage_percent:Utilization,
}
fn get_perandtemp()-> Result<Vec<GpuDetailingVisitor>,NvmlError>{
    let nvml=Nvml::init()?;
    let device =nvml.device_by_index(0)?;
    let sensors=enum_wrappers::device::TemperatureSensor::Gpu;
    let mut detail:Vec<GpuDetailingVisitor>=Vec::new();
    for _ in 1..1000{
        let temprature=device.temperature(sensors)?;
        let power_usage=device.power_usage()?;
        let usage_perc=device.utilization_rates()?;
        detail.push(GpuDetailingVisitor{
            temprature:temprature,
            power_usage:power_usage,
            usage_percent:usage_perc,
        });
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(detail)
}

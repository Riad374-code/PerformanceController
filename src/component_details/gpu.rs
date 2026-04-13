use crossterm::event::{Event, KeyCode, KeyEventKind};
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{Device, Nvml, enum_wrappers};
use tracing::{info, warn};

use crate::trainable_model_integ::gpu_detailing::{Errors, GpuDetailingVisitor};
use ratatui::widgets::ListState;

pub fn init_nvml() -> Result<Nvml, NvmlError> {
    Nvml::init().map_err(|e| {
        warn!("Nvml::init failed: {}", e);
        e
    })
}

pub fn gpu_detailing(n: &Nvml) -> Result<Device<'_>, NvmlError> {
    match n.device_by_index(0) {
        Ok(count) => Ok(count),
        Err(e) => {
            info!("No device detected: {}", e);
            Err(e)
        }
    }
}

pub fn get_gpu_details(event: Event, state: &mut ListState) -> Result<(), Errors> {
    println!("Initializing NVML...");
    let nvml = Nvml::init().map_err(|e| {
        eprintln!("NVML Init Failed: {:?}", e);
        Errors::NvmlIssue(e)
    })?;

    let device = nvml.device_by_index(0).map_err(|e| {
        eprintln!("Failed to get GPU device 0: {:?}", e);
        Errors::NvmlIssue(e)
    })?;

    println!(
        "GPU detected: {:?}",
        device.name().unwrap_or_else(|_| "Unknown".to_string())
    );

    let sensors = enum_wrappers::device::TemperatureSensor::Gpu;

    println!("Starting data collection loop (1000 iterations)...");
    if let Event::Key(key) = event {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        } else {
            let break_on_esc = key.code == KeyCode::Esc;
            loop {
                let temprature = device.temperature(sensors).unwrap_or(0);
                let power_usage = device.power_usage().unwrap_or(0);
                let usage_percent = device.utilization_rates().map(|u| u.gpu).unwrap_or(0);

                let visitor = GpuDetailingVisitor {
                    temprature: temprature,
                    power_usage: power_usage,
                    usage_percent: usage_percent,
                };

                std::thread::sleep(std::time::Duration::from_millis(10));
                if key.code == KeyCode::Esc {
                    println!("ESC pressed, exiting GPU data collection loop.");
                    state.select(Some(0));
                    break;
                }
            }
        }
    }

    Ok(())
}

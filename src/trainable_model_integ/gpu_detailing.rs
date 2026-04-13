use csv::Writer;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{Nvml, enum_wrappers};
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

#[derive(Serialize)]
pub struct GpuDetailingVisitor {
    pub temprature: u32,
    pub power_usage: u32,
    pub usage_percent: u32,
}

#[derive(Debug)]
pub enum Errors {
    FileIssue(std::io::Error),
    CsvIssue(csv::Error),
    NvmlIssue(NvmlError),
    InvalidData,
}

// We now return the PathBuf so we can print exactly where it's saving
fn create_file() -> Result<(std::fs::File, bool, PathBuf), std::io::Error> {
    let project_csv_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("trainable_model_integ")
        .join("gpu_info.csv");

    // 1. Force create the directory structure
    if let Some(parent) = project_csv_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // 2. Check if file is "Read Only" (Common OneDrive issue)
    if project_csv_path.exists() {
        let mut perms = std::fs::metadata(&project_csv_path)?.permissions();
        if perms.readonly() {
            println!("Detected Read-Only file. Attempting to clear attribute...");
            perms.set_readonly(false);
            std::fs::set_permissions(&project_csv_path, perms)?;
        }
    }

    // 3. Open in append-only mode so writes are never positioned at the start.
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&project_csv_path);

    match file {
        Ok(f) => {
            let should_write_headers = f.metadata()?.len() == 0;
            println!("SUCCESS: Write access granted to primary path.");
            Ok((f, should_write_headers, project_csv_path))
        }
        Err(e) => {
            eprintln!("Primary path still blocked ({}). Trying fallback...", e);
            let fallback_dir = std::env::temp_dir().join("PerformanceController");
            std::fs::create_dir_all(&fallback_dir)?;
            let fallback_path = fallback_dir.join("gpu_info.csv");
            let f = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&fallback_path)?;
            let should_write_headers = f.metadata()?.len() == 0;
            Ok((f, should_write_headers, fallback_path))
        }
    }
}

fn file_saver(writer: &mut Writer<File>, writable: GpuDetailingVisitor) -> Result<(), Errors> {
    if let Err(e) = writer.serialize(writable) {
        eprintln!("Error: Could not serialize data: {}", e);
        return Err(Errors::CsvIssue(e));
    }

    if let Err(e) = writer.flush() {
        eprintln!("Error: Could not flush buffer to file: {}", e);
        return Err(Errors::FileIssue(e));
    }
    Ok(())
}

fn init_file() -> Result<Writer<File>, Errors> {
    let (file, should_write_headers, _path) = match create_file() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("FATAL ERROR: Could not open any gpu_info.csv file: {}", e);
            return Err(Errors::FileIssue(e));
        }
    };

    let write = csv::WriterBuilder::new()
        .has_headers(should_write_headers)
        .from_writer(file);

    Ok(write)
}

pub fn get_perandtemp() -> Result<(), Errors> {
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

    let mut writer = init_file()?;
    let sensors = enum_wrappers::device::TemperatureSensor::Gpu;

    println!("Starting data collection loop (1000 iterations)...");

    for i in 1..=1000 {
        let temprature = device.temperature(sensors).unwrap_or(0);
        let power_usage = device.power_usage().unwrap_or(0);
        let usage_percent = device.utilization_rates().map(|u| u.gpu).unwrap_or(0);

        let visitor = GpuDetailingVisitor {
            temprature,
            power_usage,
            usage_percent,
        };

        if let Err(e) = file_saver(&mut writer, visitor) {
            eprintln!(
                "Loop aborted at iteration {} due to write error: {:?}",
                i, e
            );
            return Err(e); // Actually return the error so main() sees it!
        }

        // Print progress every 100 iterations so you know it's alive
        if i % 100 == 0 {
            println!("Successfully wrote {} rows...", i);
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    println!("Finished collecting data.");
    Ok(())
}

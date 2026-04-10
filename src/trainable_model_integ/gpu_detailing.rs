use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{Nvml, Device, device,enum_wrappers};
use nvml_wrapper::struct_wrappers::device::Utilization;
use serde::de::Error;
use serde::Serialize;
use tracing::{warn, info};



#[derive(Serialize)]
struct GpuDetailingVisitor {
    temprature:u32,
    power_usage:u32,
    usage_percent:u32,
}
#[derive(Debug)]
pub enum Errors {
    FileIssue(std::io::Error),
    CsvIssue(csv::Error),
    InvalidData,
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
            temprature,
            power_usage,
            usage_percent:usage_perc.gpu,
        });
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(detail)
}

fn create_file()->Result<File,std::io::Error>{
    let file=std::fs::File::create(Path::new(r"C:\Users\user\OneDrive - Baku Higher Oil School\Desktop\MiniRustProjects\PerformanceController\PerformanceController\src\trainable_model_integ\gpu_info.csv")).unwrap();
    Ok(file)
}
pub fn file_saver(writable: GpuDetailingVisitor)-> Result<(),Errors>{
    let file=match create_file(){
        Ok(file)=>file,
        _=>{
            warn!("Could not read the file");
            return Err(Errors::InvalidData);
        }
    };
    let mut write=csv::Writer::from_writer(file);

    write.serialize(writable);

    write.flush()?;


    Ok(())

}

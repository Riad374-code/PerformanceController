use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;
use csv::Writer;
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
    NvmlIssue(NvmlError),
    InvalidData,
}

fn create_file()->Result<File,std::io::Error>{
    let file=std::fs::File::create(Path::new(r"C:\Users\user\OneDrive - Baku Higher Oil School\Desktop\MiniRustProjects\PerformanceController\PerformanceController\src\trainable_model_integ\gpu_info.csv")).unwrap();
    Ok(file)
}
pub fn file_saver(writer:&mut Writer<File>,writable: GpuDetailingVisitor)-> Result<(),Errors>{

    if let Err(e) =writer.serialize(writable){
        warn!("Could not write the file");
        return Err(Errors::CsvIssue(e));
    }

    if let Err(e)=writer.flush(){
        warn!("Data is not in buffer");
        return Err(Errors::FileIssue(e));
    }
    Ok(())
}
fn init_file()-> Result<Writer<File>,Errors>{
    let file=match create_file(){
        Ok(file)=>file,
        _=>{
            warn!("Could not read the file");
            return Err(Errors::InvalidData);
        }
    };
    let  write=csv::Writer::from_writer(file);

    Ok(write)
}

pub fn get_perandtemp()-> Result<(),Errors>{
    let nvml=Nvml::init().unwrap();
    let device =nvml.device_by_index(0).unwrap();
    let mut writer = init_file()?;


    let sensors=enum_wrappers::device::TemperatureSensor::Gpu;
    for _ in 1..1000{
        let temprature=device.temperature(sensors).unwrap();
        let power_usage=device.power_usage().unwrap();
        let usage_perc=device.utilization_rates().unwrap();
        match file_saver(&mut writer,GpuDetailingVisitor{
            temprature,
            power_usage,
            usage_percent:usage_perc.gpu,
        }){
            Ok(())=>{},
            Err(e)=>{break}
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    Ok(())
}

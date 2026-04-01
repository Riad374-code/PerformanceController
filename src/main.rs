mod logging_init;
mod component_details;
mod tui;
use tui::design;
use crossterm::event;
use nvml_wrapper::enum_wrappers::device::Brand;
use logging_init::init_logging;
use component_details::gpu;
use crate::tui::design::allign_screen_size;

//-> std::io::Result<()>
fn main(){
    init_logging();

    //let nvml=gpu::init_nvml().unwrap();
    let terminal =design::init();

    let _=allign_screen_size(terminal);

}

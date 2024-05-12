use log::info;


#[allow(dead_code)]
pub fn init() {
    env_logger::init();

    info!("Starting server...");
}
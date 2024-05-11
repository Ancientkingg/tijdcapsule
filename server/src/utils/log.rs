use log::info;



pub fn init() {
    env_logger::init();

    info!("Starting server...");
}
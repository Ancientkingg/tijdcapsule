use log::info;



pub fn init() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    info!("Starting server...");
}
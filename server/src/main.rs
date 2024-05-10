mod routers;
mod log;

#[tokio::main]
async fn main() {
    log::init();

    routers::app::init("0.0.0.0:3000").await;
}
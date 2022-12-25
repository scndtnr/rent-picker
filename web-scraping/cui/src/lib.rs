mod cui;

pub async fn init() {
    infra::env::set_dotenv("web-scraping");
    infra::logging::init_logging("rent-picker-web-scraping");
    let app = cui::Cui::new().await;
    app.process_cmd().await;
    // infra::logging::shutdown_logging().await;
}

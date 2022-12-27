mod cui;

pub async fn init() {
    infra::env::set_dotenv("web-scraping");
    logging::init_logging();
    let app = cui::Cui::new().await;
    app.process_cmd().await;
}

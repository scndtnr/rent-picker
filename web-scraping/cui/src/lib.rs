mod cui;

pub async fn init() {
    infra::env::set_dotenv("web-scraping");
    infra::log::init_cui_log("rent-picker");
    let app = cui::Cui::new().await;
    app.process_cmd().await;
}

//! src/main.rs
use gerenciador_leitos::configuration::get_configuration;
use gerenciador_leitos::startup::Application;
use gerenciador_leitos::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("gerenciador_leitos".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
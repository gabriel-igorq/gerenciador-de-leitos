//! src/main.rs
use gerenciador_leitos::configuration::get_configuration;
use gerenciador_leitos::startup::Application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
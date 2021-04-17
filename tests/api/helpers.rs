//! tests/helpers.rs
use gerenciador_leitos::configuration::{get_configuration, DatabaseSettings};
use gerenciador_leitos::startup::{get_connection_pool, Application};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use gerenciador_leitos::routes::{UnidadeData, LeitoData, PacienteData};
use std::collections::HashMap;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    // cria uma nova unidade usando HTTP POST na rota /unidades
    // note que o username do usuário é uma String fake gerada automaticamente
    pub async fn post_unidade(&self, nome: String, email: String, tipo: String, municipio: String) -> reqwest::Response {
        let unidade = UnidadeData { nome, email, tipo, municipio };
        let mut map = HashMap::new();
        map.insert("nome", unidade.nome.clone());
        map.insert("email", unidade.email.clone());
        map.insert("municipio", unidade.municipio.clone());
        map.insert("tipo", unidade.tipo.clone());

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/unidades", &self.address))
            .header("Content-Type", "application/json")
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.");

        response
    }

    pub async fn post_leito(&self, tipo: String, situacao: String, unidade_id: Uuid) -> reqwest::Response {
        let unidade = LeitoData { tipo, situacao, unidade_id };
        let mut map = HashMap::new();
        map.insert("tipo", unidade.tipo.clone());
        map.insert("situacao", unidade.situacao.clone());
        map.insert("unidade_id", unidade.unidade_id.to_string());

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/leitos", &self.address))
            .header("Content-Type", "application/json")
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.");

        response
    }

    pub async fn post_paciente(&self, nome: String, sexo: String, idade: String, email: String, telefone: String, covid_19: String, leito_id: Uuid) -> reqwest::Response {
        let unidade = PacienteData { nome, sexo, idade, email, telefone, covid_19, leito_id };
        let mut map = HashMap::new();
        map.insert("nome", unidade.nome.clone());
        map.insert("sexo", unidade.sexo.clone());
        map.insert("idade", unidade.idade.to_string());
        map.insert("email", unidade.email.to_string());
        map.insert("telefone", unidade.telefone.to_string());
        map.insert("covid_19", unidade.covid_19.to_string());
        map.insert("leito_id", unidade.leito_id.to_string());

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/pacientes", &self.address))
            .header("Content-Type", "application/json")
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.");

        response
    }
}

// Cria uma nova instância da API
pub async fn create_app() -> TestApp {
    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        c
    };

    // Create and migrate the database
    configure_database(&configuration.database).await;

    // Launch the application as a background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let address = format!("http://localhost:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database)
            .await
            .expect("Failed to connect to the database"),
    }
}

// Configura um novo banco de dados a cada teste executado, 
// promovento isolamento entre os testes
async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Cria base de dados
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Executa migração
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
use crate::helpers::create_app;
use std::collections::HashMap;
use reqwest::Response;
use gerenciador_leitos::routes::{UnidadeSaude, UnidadeId};
use sqlx::{types::Uuid, Row};

#[actix_rt::test]
async fn create_unidade_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("nome", "UBS Teste");
    map.insert("email", "ubsteste@gmail.com");
    map.insert("municipio", "Natal");
    map.insert("tipo", "UBS");
    
    let response = client
        .post(&format!("{}/unidades", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
        
    assert_eq!(200, response.status().as_u16());

    let data = sqlx::query!("SELECT email, nome, tipo, municipio FROM unidadeSaude",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved usuario.");

    assert_eq!(data.nome, "UBS Teste");
    assert_eq!(data.email, "ubsteste@gmail.com");
    assert_eq!(data.tipo, "UBS");
    assert_eq!(data.municipio, "Natal");
}

#[actix_rt::test]
async fn delete_unidade_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria unidade
    let nome = String::from("UBS Delete");
    let email = String::from("ubsdelete@gmail.com");
    let tipo = String::from("UBS");
    let municipio = String::from("Natal");
    
    let response: Response = app.post_unidade(nome, email, tipo, municipio).await;
    let user_id: UnidadeId = response.json().await.unwrap();
    let id: Uuid = user_id.id;

    let response = client
        .delete(&format!("{}/unidades/{}", &app.address, id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o usuário foi removido
    let count: i64 = sqlx::query("SELECT COUNT(nome) as count FROM unidadeSaude")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.")
        .try_get("count")
        .unwrap();

    // verifica se foi retornada alguma coisa, se sim, o usuário não foi removido, levantando falha
    assert_eq!(count, 0);
}
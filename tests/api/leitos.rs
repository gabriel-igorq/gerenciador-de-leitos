use crate::helpers::create_app;
use std::collections::HashMap;
use reqwest::Response;
use gerenciador_leitos::routes::{ UnidadeId, Leito, LeitoId};
use sqlx::{Row};

#[actix_rt::test]
async fn create_leito_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria uma unidade de saude
    let nome = String::from("UBS Aux");
    let email = String::from("ubsaux@gmail.com");
    let tipo = String::from("UBS");
    let municipio = String::from("Natal");
    
    let response: Response = app.post_unidade(nome, email, tipo, municipio).await;
    let hospital_id: UnidadeId = response.json().await.unwrap();
    let id: String = hospital_id.id.to_string();

    let mut map = HashMap::new();
    map.insert("tipo", "UTI");
    map.insert("situacao", "Ocupado");
    map.insert("unidade_id", &id);
    
    let response = client
        .post(&format!("{}/leitos", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
        
    assert_eq!(200, response.status().as_u16());

    let data = sqlx::query!("SELECT tipo, situacao, unidade_id FROM leito",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved usuario.");

    assert_eq!(data.tipo, "UTI");
    assert_eq!(data.situacao, "Ocupado");
    assert_eq!(data.unidade_id, hospital_id.id);
}

#[actix_rt::test]
async fn get_all_leitos_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria uma unidade de saude
    let nome = String::from("UBS Aux");
    let email = String::from("ubsaux@gmail.com");
    let tipo = String::from("UBS");
    let municipio = String::from("Natal");
    
    let response: Response = app.post_unidade(nome, email, tipo, municipio).await;
    assert_eq!(200, response.status().as_u16());
    let hospital_id: UnidadeId = response.json().await.unwrap();
    //let id: String = hospital_id.id.to_string();
    
    //cria primeiro leito
    let tipo_1 = String::from("UTI");
    let situacao_1 = String::from("Ocupado");
    let unidade_id_1 = hospital_id.id;
    
    let response_1: Response = app.post_leito(tipo_1, situacao_1, unidade_id_1).await;
    assert_eq!(200, response_1.status().as_u16());

    //cria segundo leito
    let tipo_2 = String::from("Enfermaria");
    let situacao_2 = String::from("Vazio");
    let unidade_id_2 = hospital_id.id;
    
    let response_2: Response = app.post_leito(tipo_2, situacao_2, unidade_id_2).await;
    assert_eq!(200, response_2.status().as_u16());

    // faz o pedido de todos os leitos
    let response = client
        .get(&format!("{}/leitos", &app.address))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // verifica se foram retornados 2 usuários
    let leitos: Vec<LeitoId> = response.json().await.unwrap();
    assert_eq!(2, leitos.len());
}

#[actix_rt::test]
async fn get_leito_by_id_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria uma unidade de saude
    let nome = String::from("UBS Aux");
    let email = String::from("ubsaux@gmail.com");
    let tipo = String::from("UBS");
    let municipio = String::from("Natal");
    
    let response: Response = app.post_unidade(nome, email, tipo, municipio).await;
    assert_eq!(200, response.status().as_u16());
    let hospital_id: UnidadeId = response.json().await.unwrap();
    
    //cria primeiro leito
    let tipo_1 = String::from("UTI");
    let situacao_1 = String::from("Ocupado");
    let unidade_id_1 = hospital_id.id;
    
    let response_1: Response = app.post_leito(tipo_1, situacao_1, unidade_id_1).await;
    assert_eq!(200, response_1.status().as_u16());
    let leito_id: LeitoId = response_1.json().await.unwrap();
    
    // consulta o leito criado
    let response = client
        .get(&format!("{}/leitos/{}", &app.address, leito_id.id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved_leito: Leito = response.json().await.unwrap();

    // verifica se o username do usuário retornado é igual ao que foi criado
    assert_eq!(saved_leito.tipo, String::from("UTI"));
    assert_eq!(saved_leito.situacao, String::from("Ocupado"));
    assert_eq!(saved_leito.unidade_id, hospital_id.id);
}

#[actix_rt::test]
async fn update_leito_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

     // cria uma unidade de saude
     let nome = String::from("UBS Aux");
     let email = String::from("ubsaux@gmail.com");
     let tipo = String::from("UBS");
     let municipio = String::from("Natal");
     
     let response: Response = app.post_unidade(nome, email, tipo, municipio).await;
     assert_eq!(200, response.status().as_u16());
     let hospital_id: UnidadeId = response.json().await.unwrap();
     
     //cria primeiro leito
     let tipo_1 = String::from("UTI");
     let situacao_1 = String::from("Ocupado");
     let unidade_id_1 = hospital_id.id;
     
     let response_1: Response = app.post_leito(tipo_1, situacao_1, unidade_id_1).await;
     assert_eq!(200, response_1.status().as_u16());
     let leito_id: LeitoId = response_1.json().await.unwrap();

     // instancia um leito
    let leito = Leito {
        id: leito_id.id,
        tipo: String::from("Enfermaria"),
        situacao: String::from("Vazio"),
        unidade_id: unidade_id_1.clone()
    };

    // gera um HashMap que será mapeado pro json a ser enviado na requisição de atualização
    let mut map = HashMap::new();
    map.insert("id", leito.id.to_string());
    map.insert("tipo", leito.tipo.clone());
    map.insert("situacao", leito.situacao.clone());
    map.insert("unidade_id", leito.unidade_id.to_string());

    let response = client
        .put(&format!("{}/leitos", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o usuário foi atualizado
    let saved = sqlx::query!("SELECT tipo, situacao FROM leito WHERE id = $1", leito_id.id)
                    .fetch_one(&app.db_pool)
                    .await
                    .expect("Failed to fetch saved user.");

    assert_eq!(saved.tipo, leito.tipo);
    assert_eq!(saved.situacao, leito.situacao);
}

#[actix_rt::test]
async fn delete_leito_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria uma unidade de saude
    let nome = String::from("UBS Aux");
    let email = String::from("ubsaux@gmail.com");
    let tipo = String::from("UBS");
    let municipio = String::from("Natal");
    
    let response: Response = app.post_unidade(nome, email, tipo, municipio).await;
    assert_eq!(200, response.status().as_u16());
    let hospital_id: UnidadeId = response.json().await.unwrap();
    
    //cria primeiro leito
    let tipo_1 = String::from("UTI");
    let situacao_1 = String::from("Ocupado");
    let unidade_id_1 = hospital_id.id;
    
    let response_1: Response = app.post_leito(tipo_1, situacao_1, unidade_id_1).await;
    assert_eq!(200, response_1.status().as_u16());
    let leito_id: LeitoId = response_1.json().await.unwrap();

    let response = client
        .delete(&format!("{}/leitos/{}", &app.address, leito_id.id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o leito foi removido
    let count: i64 = sqlx::query("SELECT COUNT(id) as count FROM leito")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.")
        .try_get("count")
        .unwrap();

    // verifica se foi retornada alguma coisa, se sim, o leito não foi removido, levantando falha
    assert_eq!(count, 0);
}
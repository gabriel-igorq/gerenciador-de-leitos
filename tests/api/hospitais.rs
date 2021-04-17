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
async fn get_all_unidades_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria primeira unidade
    let nome_1 = String::from("UBS Teste 1");
    let email_1 = String::from("ubs_teste_1@gmail.com");
    let tipo_1 = String::from("UBS");
    let municipio_1 = String::from("Natal");
    
    let response_1: Response = app.post_unidade(nome_1, email_1, tipo_1, municipio_1).await;
    assert_eq!(200, response_1.status().as_u16());

    //cria segunda unidade
    let nome_2 = String::from("UBS Teste 2");
    let email_2 = String::from("ubs_teste_2@gmail.com");
    let tipo_2 = String::from("UBS");
    let municipio_2 = String::from("Parnamirim");
    
    let response_2: Response = app.post_unidade(nome_2, email_2, tipo_2, municipio_2).await;
    assert_eq!(200, response_2.status().as_u16());

    // faz o pedido de todos os usuários na rota /users
    let response = client
        .get(&format!("{}/unidades", &app.address))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // verifica se foram retornados 2 usuários
    let unidades: Vec<UnidadeId> = response.json().await.unwrap();
    assert_eq!(2, unidades.len());
}

#[actix_rt::test]
async fn get_unidade_by_id_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria primeira unidade
    let nome_1 = String::from("UBS Teste 1");
    let email_1 = String::from("ubs_teste_1@gmail.com");
    let tipo_1 = String::from("UBS");
    let municipio_1 = String::from("Natal");
    
    let response_1: Response = app.post_unidade(nome_1, email_1, tipo_1, municipio_1).await;
    assert_eq!(200, response_1.status().as_u16());
    let unidade_id: UnidadeId = response_1.json().await.unwrap();
    
    // consulta o usuário criado usando HTTP GET pela rota /users/{id}
    let response = client
        .get(&format!("{}/unidades/{}", &app.address, unidade_id.id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved_unidade: UnidadeSaude = response.json().await.unwrap();

    // verifica se o username do usuário retornado é igual ao que foi criado
    assert_eq!(saved_unidade.nome, String::from("UBS Teste 1"));
    assert_eq!(saved_unidade.email, String::from("ubs_teste_1@gmail.com"));
    assert_eq!(saved_unidade.tipo, String::from("UBS"));
    assert_eq!(saved_unidade.municipio, String::from("Natal"));
}

#[actix_rt::test]
async fn update_unidade_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria unidade
    let nome = String::from("UBS Update 1");
    let email = String::from("ubsupdate1@gmail.com");
    let tipo = String::from("UBS");
    let municipio = String::from("Natal");
    
    let response: Response = app.post_unidade(nome.clone(), email.clone(), tipo.clone(), municipio.clone()).await;
    let user_id: UnidadeId = response.json().await.unwrap();
    let id: Uuid = user_id.id;

    // instancia um usuário e modifica o username, mantendo o mesmo id
    let unidade = UnidadeSaude {
        id,
        nome: String::from("UBS Update 2"),
        email: String::from("ubsupdate1@gmail.com"),
        tipo,
        municipio
    };

    // gera um HashMap que será mapeado pro json a ser enviado na requisição de atualização
    let mut map = HashMap::new();
    map.insert("id", unidade.id.to_string());
    map.insert("nome", unidade.nome.clone());
    map.insert("email", unidade.email.clone());
    map.insert("tipo", unidade.tipo.clone());
    map.insert("municipio", unidade.municipio.clone());

    let response = client
        .put(&format!("{}/unidades", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o usuário foi atualizado
    let saved = sqlx::query!("SELECT nome, email FROM unidadeSaude WHERE id = $1", id)
                    .fetch_one(&app.db_pool)
                    .await
                    .expect("Failed to fetch saved user.");

    assert_eq!(saved.nome, unidade.nome);
    assert_eq!(saved.email, unidade.email);
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
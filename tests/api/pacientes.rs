use crate::helpers::create_app;
use std::collections::HashMap;
use reqwest::Response;
use gerenciador_leitos::routes::{ UnidadeId, Leito, LeitoId, Paciente, PacienteId};
use sqlx::{Row};

#[actix_rt::test]
async fn create_paciente_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria uma unidade de saude
    let nome = String::from("UBS Aux");
    let email = String::from("ubsaux@gmail.com");
    let tipo = String::from("UBS");
    let municipio = String::from("Natal");
    
    let response: Response = app.post_unidade(nome, email, tipo, municipio).await;
    let hospital_id: UnidadeId = response.json().await.unwrap();

    //cria um leito
    let tipo_1 = String::from("UTI");
    let situacao_1 = String::from("Ocupado");
    let unidade_id_1 = hospital_id.id;
    
    let response_1: Response = app.post_leito(tipo_1, situacao_1, unidade_id_1).await;
    assert_eq!(200, response_1.status().as_u16());
    let leito_id: LeitoId = response_1.json().await.unwrap();
    let id: String = leito_id.id.to_string();

    let mut map = HashMap::new();
    map.insert("nome", "Fulano");
    map.insert("sexo", "Masculino");
    map.insert("idade", "29");
    map.insert("email", "fulano@gmail.com");
    map.insert("telefone", "84998874321");
    map.insert("covid_19", "Sim");
    map.insert("leito_id", &id);
    
    let response = client
        .post(&format!("{}/pacientes", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
        
    assert_eq!(200, response.status().as_u16());


    // nome, sexo, idade, email, telefone, covid_19, leito_id
    let data = sqlx::query!("SELECT nome, sexo, idade, email, telefone, covid_19, leito_id  FROM paciente",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved usuario.");

    assert_eq!(data.nome, "Fulano");
    assert_eq!(data.sexo, "Masculino");
    assert_eq!(data.idade, "29");
    assert_eq!(data.email, "fulano@gmail.com");
    assert_eq!(data.telefone, "84998874321");
    assert_eq!(data.covid_19, "Sim");
    assert_eq!(data.leito_id, leito_id.id);
}

#[actix_rt::test]
async fn get_all_pacientes_returns_200() {
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
    let leito_id_1: LeitoId = response_1.json().await.unwrap();

    //cria segundo leito
    let tipo_2= String::from("UTI");
    let situacao_2 = String::from("Ocupado");
    let unidade_id_2 = hospital_id.id;
    
    let response_2: Response = app.post_leito(tipo_2, situacao_2, unidade_id_2).await;
    assert_eq!(200, response_2.status().as_u16());
    let leito_id_2: LeitoId = response_2.json().await.unwrap();

    //cria primeiro paciente
    let nome = String::from("Fulano");
    let sexo = String::from("Masculino");
    let idade = String::from("54");
    let email = String::from("fulano@gmail.com");
    let telefone = String::from("991223344");
    let covid_19 = String::from("Sim");
    let leito_id = leito_id_1.id;

    let response_1: Response = app.post_paciente(nome, sexo, idade, email, telefone, covid_19, leito_id).await;
    assert_eq!(200, response_1.status().as_u16());

    //cria segundo paciente
    let nome = String::from("Sicrano");
    let sexo = String::from("Masculino");
    let idade = String::from("45");
    let email = String::from("sicrano@gmail.com");
    let telefone = String::from("991225566");
    let covid_19 = String::from("Não");
    let leito_id = leito_id_2.id;

    let response_2: Response = app.post_paciente(nome, sexo, idade, email, telefone, covid_19, leito_id).await;
    assert_eq!(200, response_2.status().as_u16());

    // faz o pedido de todos os leitos
    let response = client
        .get(&format!("{}/pacientes", &app.address))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // verifica se foram retornados 2 usuários
    let pacientes: Vec<PacienteId> = response.json().await.unwrap();
    assert_eq!(2, pacientes.len());
}

#[actix_rt::test]
async fn get_paciente_by_id_returns_200() {
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
    
    //cria um leito
    let tipo_1 = String::from("UTI");
    let situacao_1 = String::from("Ocupado");
    let unidade_id_1 = hospital_id.id;
    
    let response_1: Response = app.post_leito(tipo_1, situacao_1, unidade_id_1).await;
    assert_eq!(200, response_1.status().as_u16());
    let leito_id: LeitoId = response_1.json().await.unwrap();

     //cria paciente
     let nome = String::from("Fulano");
     let sexo = String::from("Masculino");
     let idade = String::from("54");
     let email = String::from("fulano@gmail.com");
     let telefone = String::from("991223344");
     let covid_19 = String::from("Sim");
     let leito_id = leito_id.id;
 
     let response_1: Response = app.post_paciente(nome, sexo, idade, email, telefone, covid_19, leito_id.clone()).await;
     assert_eq!(200, response_1.status().as_u16());
     let paciente_id: PacienteId = response_1.json().await.unwrap();
    
    // consulta o leito criado
    let response = client
        .get(&format!("{}/pacientes/{}", &app.address, paciente_id.id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved: Paciente = response.json().await.unwrap();

    assert_eq!(saved.nome, String::from("Fulano"));
    assert_eq!(saved.sexo, String::from("Masculino"));
    assert_eq!(saved.idade, String::from("54"));
    assert_eq!(saved.email, String::from("fulano@gmail.com"));
    assert_eq!(saved.telefone, String::from("991223344"));
    assert_eq!(saved.covid_19,  String::from("Sim"));
    assert_eq!(saved.leito_id, leito_id);

}

#[actix_rt::test]
async fn update_paciente_returns_200() {
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

    //cria paciente
    let nome = String::from("Fulano");
    let sexo = String::from("Masculino");
    let idade = String::from("54");
    let email = String::from("fulano@gmail.com");
    let telefone = String::from("991223344");
    let covid_19 = String::from("Sim");
    let leito_id = leito_id.id;

    let response_1: Response = app.post_paciente(nome, sexo, idade, email, telefone, covid_19, leito_id.clone()).await;
    assert_eq!(200, response_1.status().as_u16());
    let paciente_id: PacienteId = response_1.json().await.unwrap();

     // instancia um paciente
    let paciente = Paciente {
        id: paciente_id.id,
        nome: String::from("Sicrano"),
        sexo: String::from("Masculino"),
        idade: String::from("54"),
        email: String::from("sicrano@gmail.com"),
        telefone:String::from("991223344"),
        covid_19: String::from("Sim"),
        leito_id: leito_id.clone()
    };

    // gera um HashMap que será mapeado pro json a ser enviado na requisição de atualização
    let mut map = HashMap::new();
    map.insert("id", paciente.id.to_string());
    map.insert("nome", paciente.nome.clone());
    map.insert("sexo", paciente.sexo.clone());
    map.insert("idade", paciente.idade.to_string());
    map.insert("email", paciente.email.clone());
    map.insert("telefone", paciente.telefone.clone());
    map.insert("covid_19", paciente.covid_19.clone());
    map.insert("leito_id", paciente.leito_id.to_string());

    let response = client
        .put(&format!("{}/pacientes", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o usuário foi atualizado
    let saved = sqlx::query!("SELECT nome, email FROM paciente WHERE id = $1", paciente_id.id)
                    .fetch_one(&app.db_pool)
                    .await
                    .expect("Failed to fetch saved user.");

    assert_eq!(saved.nome, paciente.nome);
    assert_eq!(saved.email, paciente.email);
}

#[actix_rt::test]
async fn delete_paciente_returns_200() {
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

     //cria paciente
     let nome = String::from("Fulano");
     let sexo = String::from("Masculino");
     let idade = String::from("54");
     let email = String::from("fulano@gmail.com");
     let telefone = String::from("991223344");
     let covid_19 = String::from("Sim");
     let leito_id = leito_id.id;
 
     let response_1: Response = app.post_paciente(nome, sexo, idade, email, telefone, covid_19, leito_id).await;
     assert_eq!(200, response_1.status().as_u16());
     let paciente_id: PacienteId = response_1.json().await.unwrap();

    let response = client
        .delete(&format!("{}/pacientes/{}", &app.address, paciente_id.id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o leito foi removido
    let count: i64 = sqlx::query("SELECT COUNT(id) as count FROM paciente")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.")
        .try_get("count")
        .unwrap();

    // verifica se foi retornada alguma coisa, se sim, o leito não foi removido, levantando falha
    assert_eq!(count, 0);
}
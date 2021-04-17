use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};
use sqlx::{PgPool, types::Uuid};
use super::serializers::my_uuid;

//#[derive(serde::Deserialize)]
#[derive(Serialize, Deserialize)]
pub struct Paciente {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    pub nome: String,
    pub sexo: String,
    pub idade: String,
    pub email: String,
    pub telefone: String,
    pub covid_19: String,
    #[serde(with = "my_uuid")]
    pub leito_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct PacienteData {
    pub nome: String,
    pub sexo: String,
    pub idade: String,
    pub email: String,
    pub telefone: String,
    pub covid_19: String,
    #[serde(with = "my_uuid")]
    pub leito_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct PacienteId {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct PacienteNome {
    pub nome: String,
}

#[derive(Deserialize)]
pub struct Quantidade {
    pub quantidade: i32,
}

pub async fn create_paciente(
    paciente: web::Json<PacienteData>,
    pool: web::Data<PgPool>, // Renamed!
) -> Result<HttpResponse, HttpResponse> {
    
    let row = sqlx::query!(
        r#"
        INSERT INTO paciente (id, nome, sexo, idade, email, telefone, covid_19, leito_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
        Uuid::new_v4(),
        paciente.nome,
        paciente.sexo,
        paciente.idade,
        paciente.email,
        paciente.telefone,
        paciente.covid_19,
        paciente.leito_id
    )
    // We got rid of the double-wrapping using .app_data()
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let paciente = PacienteId{
        id: row.id
    };

    Ok(HttpResponse::Ok().json(&paciente))
    //Ok(HttpResponse::Ok().finish())
}

pub async fn get_all_pacientes(
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {
    let rows = sqlx::query!(
        r#"
        SELECT id, nome, sexo, idade, email, telefone, covid_19, leito_id
        FROM paciente
        ORDER BY id
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let mut pacientes: Vec<Paciente> = Vec::new();
    for row in rows {
        let paciente = Paciente {
            id: row.id,
            nome: row.nome,
            sexo: row.sexo,
            idade: row.idade,
            email: row.email,
            telefone: row.telefone,
            covid_19: row.covid_19,
            leito_id: row.leito_id
        };
        pacientes.push(paciente);
    }

    Ok(HttpResponse::Ok().json(pacientes))
}

pub async fn get_paciente_by_id(
    req: web::HttpRequest,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    let id:Uuid = req.match_info().get("id").unwrap().parse().unwrap();

    let row = sqlx::query!(
        r#"
        SELECT id, nome, sexo, idade, email, telefone, covid_19, leito_id
        FROM paciente
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let paciente = Paciente {
        id: row.id,
        nome: row.nome,
        sexo: row.sexo,
        idade: row.idade,
        email: row.email,
        telefone: row.telefone,
        covid_19: row.covid_19,
        leito_id: row.leito_id
    };

    Ok(HttpResponse::Ok().json(&paciente))
}

pub async fn get_pacientes_covid(
    req: web::HttpRequest,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    let id:Uuid = req.match_info().get("id").unwrap().parse().unwrap();

    let rows = sqlx::query!(
        r#"
        SELECT P.id, P.nome, P.sexo, P.idade, P.email, P.telefone, P.covid_19, P.leito_id
        FROM (unidadesaude as U JOIN leito as L ON U.id = L.unidade_id) JOIN paciente as P ON L.id = P.leito_id
        WHERE covid_19 = 'Sim' AND U.id = $1
        "#,
        id,
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let mut pacientes: Vec<Paciente> = Vec::new();
    for row in rows {
        let paciente = Paciente {
            id: row.id,
            nome: row.nome,
            sexo: row.sexo,
            idade: row.idade,
            email: row.email,
            telefone: row.telefone,
            covid_19: row.covid_19,
            leito_id: row.leito_id
        };
        pacientes.push(paciente);
    }

    Ok(HttpResponse::Ok().json(pacientes))
}

pub async fn update_paciente(
    paciente: web::Json<Paciente>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    //id, nome, sexo, idade, email, telefone, covid_19, leito_id
    sqlx::query!(
        r#"
        UPDATE paciente
        SET nome = $1, sexo = $2, idade = $3, email = $4, telefone = $5, covid_19 = $6, leito_id = $7
        WHERE id = $8
        "#,
        paciente.nome,
        paciente.sexo,
        paciente.idade,
        paciente.email,
        paciente.telefone,
        paciente.covid_19,
        paciente.leito_id,
        paciente.id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_paciente(
    req: web::HttpRequest,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    let id:Uuid = req.match_info().get("id").unwrap().parse().unwrap();

    sqlx::query!(
        r#"
        DELETE FROM paciente
        WHERE id = $1
        "#,
        id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}
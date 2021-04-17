use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};
use sqlx::{PgPool, types::Uuid};
use super::serializers::my_uuid;

//#[derive(serde::Deserialize)]
#[derive(Serialize, Deserialize)]
pub struct Leito {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    pub tipo: String,
    pub situacao: String,
    #[serde(with = "my_uuid")]
    pub unidade_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct LeitoData {
    pub tipo: String,
    pub situacao: String,
    #[serde(with = "my_uuid")]
    pub unidade_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct LeitoId {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
}

pub async fn create_leito(
    leito: web::Json<LeitoData>,
    pool: web::Data<PgPool>, // Renamed!
) -> Result<HttpResponse, HttpResponse> {
    
    let row = sqlx::query!(
        r#"
        INSERT INTO leito (id, tipo, situacao, unidade_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        Uuid::new_v4(),
        leito.tipo,
        leito.situacao,
        leito.unidade_id,
    )
    // We got rid of the double-wrapping using .app_data()
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    // let unidade = UnidadeSaude {
    //     id: row.id,
    //     email: row.email,
    //     nome: row.nome,
    //     tipo: row.tipo,
    //     municipio: row.municipio
    // };

    let leito = LeitoId{
        id: row.id
    };

    Ok(HttpResponse::Ok().json(&leito))
    //Ok(HttpResponse::Ok().finish())
}

pub async fn get_all_leitos(
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {
    let rows = sqlx::query!(
        r#"
        SELECT id, tipo, situacao, unidade_id
        FROM leito
        ORDER BY id
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let mut leitos: Vec<Leito> = Vec::new();
    for row in rows {
        let leito = Leito {
            id: row.id,
            tipo: row.tipo,
            situacao: row.situacao,
            unidade_id: row.unidade_id
        };
        leitos.push(leito);
    }

    Ok(HttpResponse::Ok().json(leitos))
}

pub async fn get_leito_by_id(
    req: web::HttpRequest,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    let id:Uuid = req.match_info().get("id").unwrap().parse().unwrap();

    let row = sqlx::query!(
        r#"
        SELECT id, tipo, situacao, unidade_id
        FROM leito
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

    let leito = Leito {
        id: row.id,
        tipo: row.tipo,
        situacao: row.situacao,
        unidade_id: row.unidade_id
    };

    Ok(HttpResponse::Ok().json(&leito))
}

pub async fn update_leito(
    leito: web::Json<Leito>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    sqlx::query!(
        r#"
        UPDATE leito
        SET tipo = $1, situacao = $2, unidade_id = $3
        WHERE id = $4
        "#,
        leito.tipo,
        leito.situacao,
        leito.unidade_id,
        leito.id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_leito(
    req: web::HttpRequest,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    let id:Uuid = req.match_info().get("id").unwrap().parse().unwrap();

    sqlx::query!(
        r#"
        DELETE FROM leito
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
use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};
use sqlx::{PgPool, types::Uuid};
use chrono::Utc;
use super::serializers::my_uuid;

//#[derive(serde::Deserialize)]
#[derive(Serialize, Deserialize)]
pub struct UnidadeSaude {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    pub email: String,
    pub nome: String,
    pub tipo: String,
    pub municipio: String
}

#[derive(Deserialize)]
pub struct UnidadeData {
    pub email: String,
    pub nome: String,
    pub tipo: String,
    pub municipio: String
}

#[derive(Serialize, Deserialize)]
pub struct UnidadeId {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
}

pub async fn create_unidade(
    unidade_saude: web::Json<UnidadeData>,
    pool: web::Data<PgPool>, // Renamed!
) -> Result<HttpResponse, HttpResponse> {
    
    let row = sqlx::query!(
        r#"
        INSERT INTO unidadeSaude (id, email, nome, tipo, municipio, subscribed_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        Uuid::new_v4(),
        unidade_saude.email,
        unidade_saude.nome,
        unidade_saude.tipo,
        unidade_saude.municipio,
        Utc::now()
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

    let unidade = UnidadeId{
        id: row.id
    };

    Ok(HttpResponse::Ok().json(&unidade))
    //Ok(HttpResponse::Ok().finish())
}

pub async fn get_all_unidades(
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {
    let rows = sqlx::query!(
        r#"
        SELECT id, nome, email, tipo, municipio
        FROM unidadeSaude
        ORDER BY id
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let mut unidades: Vec<UnidadeSaude> = Vec::new();
    for row in rows {
        let user = UnidadeSaude {
            id: row.id,
            email: row.email,
            nome: row.nome,
            tipo: row.tipo,
            municipio: row.municipio
        };
        unidades.push(user);
    }

    Ok(HttpResponse::Ok().json(unidades))
}
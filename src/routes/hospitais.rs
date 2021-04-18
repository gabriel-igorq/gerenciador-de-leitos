use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};
use sqlx::{PgPool, types::Uuid};
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

#[tracing::instrument(name = "Create unidade", skip(unidade_saude, pool))]
pub async fn create_unidade(
    unidade_saude: web::Json<UnidadeData>,
    pool: web::Data<PgPool>, // Renamed!
) -> Result<HttpResponse, HttpResponse> {
    
    let row = sqlx::query!(
        r#"
        INSERT INTO unidadeSaude (id, email, nome, tipo, municipio)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        Uuid::new_v4(),
        unidade_saude.email,
        unidade_saude.nome,
        unidade_saude.tipo,
        unidade_saude.municipio,
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

#[tracing::instrument(name = "Get all unidades", skip(pool))]
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

#[tracing::instrument(name = "Get unidade by id", skip(req, pool))]
pub async fn get_unidade_by_id(
    req: web::HttpRequest,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    let id:Uuid = req.match_info().get("id").unwrap().parse().unwrap();

    let row = sqlx::query!(
        r#"
        SELECT id, email, nome, tipo, municipio
        FROM unidadeSaude
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

    let unidade = UnidadeSaude{
        id: row.id,
        email: row.email,
        nome: row.nome,
        tipo: row.tipo,
        municipio: row.municipio
    };

    Ok(HttpResponse::Ok().json(&unidade))
}

#[tracing::instrument(name = "Get unidades com leitos", skip(pool))]
pub async fn get_unidades_com_leitos(
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT U.id, U.nome, U.email, U.tipo, U.municipio
        FROM unidadesaude as U JOIN leito as L ON U.id = L.unidade_id
        WHERE situacao != 'Ocupado'
        "#,
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let mut unidades: Vec<UnidadeSaude> = Vec::new();
    for row in rows {
        let unidade = UnidadeSaude {
            id: row.id,
            email: row.email,
            nome: row.nome,
            tipo: row.tipo,
            municipio: row.municipio
        };
        unidades.push(unidade);
    }

    Ok(HttpResponse::Ok().json(unidades))
}

#[tracing::instrument(name = "Update unidade", skip(unidade_saude, pool))]
pub async fn update_unidade(
    unidade_saude: web::Json<UnidadeSaude>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    sqlx::query!(
        r#"
        UPDATE unidadeSaude
        SET nome = $1, email = $2, tipo = $3, municipio = $4
        WHERE id = $5
        "#,
        unidade_saude.nome,
        unidade_saude.email,
        unidade_saude.tipo,
        unidade_saude.municipio,
        unidade_saude.id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Delete unidade", skip(req, pool))]
pub async fn delete_unidade(
    req: web::HttpRequest,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    let id:Uuid = req.match_info().get("id").unwrap().parse().unwrap();

    sqlx::query!(
        r#"
        DELETE FROM unidadeSaude
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
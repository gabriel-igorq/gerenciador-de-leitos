use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    nome: String,
    tipo: String,
    municipio: String
}

pub async fn cadastrar(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>, // Renamed!
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query!(
        r#"
        INSERT INTO hospitais (id, email, nome, tipo, municipio, subscribed_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        form.email,
        form.nome,
        form.tipo,
        form.municipio,
        Utc::now()
    )
    // We got rid of the double-wrapping using .app_data()
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}
use axum::response::Result;
use axum::{extract, http};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::error::AppError;

#[derive(Debug, Serialize, FromRow)]
pub struct Judge {
    pub id: uuid::Uuid,
    pub name: String,
    pub username: String,
    pub password: String,
    pub is_active: bool,
    // Relationships
    pub event_id: uuid::Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreateJudge {
    name: String,
    username: String,
    password: String,
    is_active: bool,
    event_id: uuid::Uuid,
}

pub async fn create_judge(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateJudge>,
) -> Result<(http::StatusCode, axum::Json<Judge>), AppError> {
    let res = sqlx::query_as::<_, Judge>(
        r#"
        INSERT INTO judges (name, username, password, is_active, event_id) 
        VALUES ($1, $2, $3, $4, $5) 
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.username)
    .bind(&payload.password)
    .bind(&payload.is_active)
    .bind(&payload.event_id)
    .fetch_one(&pool)
    .await;

    match res {
        Ok(judge) => Ok((http::StatusCode::CREATED, axum::Json(judge))),
        Err(err) => Err(AppError::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create judge: {}", err),
        )),
    }
}

pub async fn get_judges(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<Judge>>, AppError> {
    let res = sqlx::query_as::<_, Judge>("SELECT * FROM judges")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(judges) => Ok(axum::Json(judges)),
        Err(err) => Err(AppError::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get judges: {}", err),
        )),
    }
}

pub async fn get_judge(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(judge_id): extract::Path<uuid::Uuid>,
) -> Result<axum::Json<Judge>, AppError> {
    let res = sqlx::query_as::<_, Judge>("SELECT * FROM judges WHERE id = ($1)")
        .bind(&judge_id)
        .fetch_one(&pool)
        .await;

    match res {
        Ok(judge) => Ok(axum::Json(judge)),
        Err(err) => Err(AppError::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get judge: {}", err),
        )),
    }
}

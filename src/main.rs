mod errors;
mod models;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use errors::AppError;
use models::AppState;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
pub struct Task {
    id: i32,
    name: String,
    completed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateTask {
    name: String,
}

async fn create_task(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, AppError> {
    let task_name = payload.name;

    let task = sqlx::query_as::<_, Task>("INSERT INTO tasks (name) VALUES ($1) RETURNING *")
        .bind(task_name)
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(AppError::from)?;

    Ok(Json(task))
}

async fn get_tasks(State(app_state): State<AppState>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(AppError::from)?;

    Ok(Json(tasks))
}

async fn toggle_task_status(
    Path(task_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Task>, AppError> {
    let updated_task = sqlx::query_as::<_, Task>(
        "UPDATE tasks SET completed = NOT completed WHERE id = $1 RETURNING *",
    )
    .bind(task_id)
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(AppError::from)?;

    Ok(Json(updated_task))
}

async fn delete_task(
    Path(task_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Task>, AppError> {
    let deleted_task = sqlx::query_as::<_, Task>("DELETE FROM tasks WHERE id = $1 RETURNING *")
        .bind(task_id)
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(AppError::from)?;

    Ok(Json(deleted_task))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_connection_string = std::env::var("DATABASE_URL").unwrap();
    let db_pool = PgPool::connect(&db_connection_string).await?;

    let app_state = AppState { db_pool };

    let task_router = Router::new()
        .route("/", post(create_task).get(get_tasks))
        .route("/:id", put(toggle_task_status).delete(delete_task));

    let app = Router::new()
        .nest("/tasks", task_router)
        .fallback(fallback)
        .with_state(app_state);

    let port = 5500_u16;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Listening at http://{}/ 🦀", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn fallback() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"status": "Not found"})),
    )
}

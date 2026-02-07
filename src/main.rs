use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub budget: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub status: String,
    pub budget: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub completed: bool,
    pub priority: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub email: String,
    pub active: bool,
}

pub struct AppState {
    pub projects: Arc<RwLock<Vec<Project>>>,
    pub tasks: Arc<RwLock<Vec<Task>>>,
    pub customers: Arc<RwLock<Vec<Customer>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            projects: Arc::new(RwLock::new(vec![])),
            tasks: Arc::new(RwLock::new(vec![])),
            customers: Arc::new(RwLock::new(vec![])),
        }
    }
}

async fn list_projects(State(state): State<AppState>) -> impl IntoResponse {
    let items = state.projects.read().await;
    Json(items.clone())
}

async fn create_project(State(state): State<AppState>, Json(payload): Json<CreateProjectRequest>) -> impl IntoResponse {
    let item = Project {
        id: Uuid::new_v4(),
        name: payload.name,
        status: payload.status,
        budget: payload.budget,
        created_at: Utc::now(),
    };
    state.projects.write().await.push(item.clone());
    (StatusCode::CREATED, Json(item))
}

async fn get_project(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let items = state.projects.read().await;
    match items.iter().find(|i| i.id == id) {
        Some(item) => Ok(Json(item.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn list_tasks(State(state): State<AppState>) -> impl IntoResponse {
    let items = state.tasks.read().await;
    Json(items.clone())
}

async fn create_task(State(state): State<AppState>, Json(payload): Json<CreateTaskRequest>) -> impl IntoResponse {
    let item = Task {
        id: Uuid::new_v4(),
        title: payload.title,
        completed: payload.completed,
        priority: payload.priority,
        created_at: Utc::now(),
    };
    state.tasks.write().await.push(item.clone());
    (StatusCode::CREATED, Json(item))
}

async fn get_task(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let items = state.tasks.read().await;
    match items.iter().find(|i| i.id == id) {
        Some(item) => Ok(Json(item.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn list_customers(State(state): State<AppState>) -> impl IntoResponse {
    let items = state.customers.read().await;
    Json(items.clone())
}

async fn create_customer(State(state): State<AppState>, Json(payload): Json<CreateCustomerRequest>) -> impl IntoResponse {
    let item = Customer {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
        active: payload.active,
        created_at: Utc::now(),
    };
    state.customers.write().await.push(item.clone());
    (StatusCode::CREATED, Json(item))
}

async fn get_customer(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let items = state.customers.read().await;
    match items.iter().find(|i| i.id == id) {
        Some(item) => Ok(Json(item.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let state = AppState::new();
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/projects", get(list_projects).post(create_project))
        .route("/projects/:id", get(get_project))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", get(get_task))
        .route("/customers", get(list_customers).post(create_customer))
        .route("/customers/:id", get(get_customer))
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "demo-customer-app",
        "version": "0.1.0",
        "entities": ["Project", "Task", "Customer"]
    }))
}

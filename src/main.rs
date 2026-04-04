mod ga;
mod models;
mod store;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use models::{Course, Day, GenerationRequest, Instructor, Room, Subject};
use serde::Serialize;
use store::AppStore;

#[derive(Clone)]
struct AppState {
    store: Arc<AppStore>,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
    service: &'static str,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        store: Arc::new(AppStore::default()),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/courses", post(add_course))
        .route("/instructors", post(add_instructor))
        .route("/rooms", post(add_room))
        .route("/subjects", post(add_subject))
        .route("/days", post(set_days))
        .route("/schedule/generate", post(generate))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("valid address");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("can bind listener");

    println!("ClassGA API listening on http://{addr}");
    axum::serve(listener, app).await.expect("server should run");
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        service: "classga",
    })
}

async fn add_course(
    State(state): State<AppState>,
    Json(course): Json<Course>,
) -> impl IntoResponse {
    state.store.upsert_course(course);
    StatusCode::CREATED
}

async fn add_instructor(
    State(state): State<AppState>,
    Json(instructor): Json<Instructor>,
) -> impl IntoResponse {
    state.store.upsert_instructor(instructor);
    StatusCode::CREATED
}

async fn add_room(State(state): State<AppState>, Json(room): Json<Room>) -> impl IntoResponse {
    state.store.upsert_room(room);
    StatusCode::CREATED
}

async fn add_subject(
    State(state): State<AppState>,
    Json(subject): Json<Subject>,
) -> impl IntoResponse {
    state.store.upsert_subject(subject);
    StatusCode::CREATED
}

async fn set_days(State(state): State<AppState>, Json(days): Json<Vec<Day>>) -> impl IntoResponse {
    state.store.set_days(days);
    StatusCode::CREATED
}

async fn generate(
    State(state): State<AppState>,
    Json(mut request): Json<GenerationRequest>,
) -> Result<Json<models::Schedule>, (StatusCode, String)> {
    if request.courses.is_empty() {
        request.courses = state.store.courses.read().values().cloned().collect();
    }
    if request.instructors.is_empty() {
        request.instructors = state.store.instructors.read().values().cloned().collect();
    }
    if request.rooms.is_empty() {
        request.rooms = state.store.rooms.read().values().cloned().collect();
    }
    if request.subjects.is_empty() {
        request.subjects = state.store.subjects.read().values().cloned().collect();
    }
    if request.days.is_empty() {
        request.days = state.store.days.read().clone();
    }

    if request.courses.is_empty()
        || request.instructors.is_empty()
        || request.rooms.is_empty()
        || request.subjects.is_empty()
        || request.days.is_empty()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "courses, instructors, rooms, subjects and days are required".to_string(),
        ));
    }

    if request.periods_per_day == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "periods_per_day must be greater than 0".to_string(),
        ));
    }

    Ok(Json(ga::generate_schedule(&request)))
}

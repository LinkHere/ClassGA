mod ga;
mod models;
mod store;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
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
    let store = AppStore::new("classga.db").expect("can initialize sqlite database");
    let state = AppState {
        store: Arc::new(store),
    };

    let app = Router::new()
        .route("/", get(frontend))
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

    println!("ClassGA UI/API listening on http://{addr}");
    axum::serve(listener, app).await.expect("server should run");
}

async fn frontend() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(include_str!("../static/index.html")),
    )
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
) -> Result<StatusCode, (StatusCode, String)> {
    state.store.upsert_course(course).map_err(internal_error)?;
    Ok(StatusCode::CREATED)
}

async fn add_instructor(
    State(state): State<AppState>,
    Json(instructor): Json<Instructor>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .store
        .upsert_instructor(instructor)
        .map_err(internal_error)?;
    Ok(StatusCode::CREATED)
}

async fn add_room(
    State(state): State<AppState>,
    Json(room): Json<Room>,
) -> Result<StatusCode, (StatusCode, String)> {
    state.store.upsert_room(room).map_err(internal_error)?;
    Ok(StatusCode::CREATED)
}

async fn add_subject(
    State(state): State<AppState>,
    Json(subject): Json<Subject>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .store
        .upsert_subject(subject)
        .map_err(internal_error)?;
    Ok(StatusCode::CREATED)
}

async fn set_days(
    State(state): State<AppState>,
    Json(days): Json<Vec<Day>>,
) -> Result<StatusCode, (StatusCode, String)> {
    state.store.set_days(days).map_err(internal_error)?;
    Ok(StatusCode::CREATED)
}

async fn generate(
    State(state): State<AppState>,
    Json(mut request): Json<GenerationRequest>,
) -> Result<Json<models::Schedule>, (StatusCode, String)> {
    if request.courses.is_empty() {
        request.courses = state.store.courses().map_err(internal_error)?;
    }
    if request.instructors.is_empty() {
        request.instructors = state.store.instructors().map_err(internal_error)?;
    }
    if request.rooms.is_empty() {
        request.rooms = state.store.rooms().map_err(internal_error)?;
    }
    if request.subjects.is_empty() {
        request.subjects = state.store.subjects().map_err(internal_error)?;
    }
    if request.days.is_empty() {
        request.days = state.store.days().map_err(internal_error)?;
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

fn internal_error(err: rusqlite::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("database error: {err}"),
    )
}

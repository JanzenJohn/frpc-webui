use std::{collections::HashMap, fmt::Display, sync::Arc, time::Duration};

use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{delete, get},
    Json, Router,
};

use config::{Config, ForwardType, PortForward};
pub use error::Error;
use private::{Privatable, Private};
use serde::Deserialize;
use tokio::{
    net::TcpListener,
    process::{self, Command},
    sync::Mutex,
    time::timeout,
};

mod config;
mod error;
mod private;

#[derive(Debug)]
struct AppState {
    frpc: Option<tokio::process::Child>,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState {
        frpc: None,
        config: config::load_config().await.unwrap(),
    }));

    let router = Router::new()
        .route("/api/ports", get(root))
        .route("/api/ports/:name", delete(delete_port).post(add_port))
        .route("/api/status", get(status))
        .route("/api/restart", get(start_daemon_web))
        .route("/api/stop", get(stop_deamon_web))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

    println!("http://localhost:4000/");
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

async fn root(State(state): State<Arc<Mutex<AppState>>>) -> Json<HashMap<String, PortForward>> {
    let lock = state.lock().await;

    let ports = lock.config.forward_ports.clone();

    Json::from(ports)
}

#[derive(Deserialize)]
struct PortPath {
    name: String,
}

async fn delete_port(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(port): Path<PortPath>,
) -> impl IntoResponse {
    let mut lock = state.lock().await;
    println!("deleting port {}", port.name);
    lock.config.forward_ports.remove(&port.name);

    Html(format!("Port {} deleted", port.name))
}
async fn add_port(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(path_infos): Path<PortPath>,
    Json(port): Json<PortForward>,
) -> Response {
    let mut lock = state.lock().await;
    println!("adding port {}", path_infos.name);
    lock.config
        .forward_ports
        .insert(path_infos.name.clone(), port);

    let mut resp = Response::new("Port added".into());
    *resp.status_mut() = StatusCode::CREATED;
    resp
}

async fn status(State(state): State<Arc<Mutex<AppState>>>) -> Json<bool> {
    let mut lock = state.lock().await;

    let frpc = match lock.frpc.as_mut() {
        Some(child) => child,
        None => return Json(false),
    };

    let status = timeout(Duration::from_micros(10), frpc.wait())
        .await
        .is_err();
    return Json(status);
}

async fn start_daemon_web(State(state): State<Arc<Mutex<AppState>>>) -> Result<Json<bool>, Error> {
    start_deamon(state).await?;
    Ok(Json(true))
}

async fn start_deamon(state: Arc<Mutex<AppState>>) -> Result<(), Error> {
    stop_deamon(state.clone()).await?;
    let mut lock = state.lock().await;
    lock.config.save().await;
    let frpc = Command::new("bin/frpc")
        .arg("-c")
        .arg("frpc.toml")
        .spawn()
        .map_err(|e| Error::ConfigError(format!("failed to start frpc: {}", e)))?;
    lock.frpc = Some(frpc);
    Ok(())
}

async fn stop_deamon_web(State(state): State<Arc<Mutex<AppState>>>) -> Result<Json<bool>, Error> {
    stop_deamon(state).await?;
    Ok(Json(true))
}

async fn stop_deamon(state: Arc<Mutex<AppState>>) -> Result<(), Error> {
    let mut lock = state.lock().await;
    match lock.frpc.as_mut() {
        Some(p) => p.kill().await.map_err(|_| Error::KillingFrpError),
        None => Ok(()),
    }?;
    lock.frpc = None;
    Ok(())
}

use axum::extract::ws::Message;
use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Serialize;
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use tokio::time::interval;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging with more detailed output

    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/usage", get(ws_handler))
        .layer(TraceLayer::new_for_http());

    let app = Router::new().nest("/api", app);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct SystemStats {
    cpu_usage: Vec<f32>,
    memory_usage: f32,
    total_memory: f32,
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut interval = interval(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    // Initialize system
    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    loop {
        interval.tick().await;

        // Refresh system information
        sys.refresh_cpu_usage();

        let stats = SystemStats {
            cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
            memory_usage: convert_to_gb(sys.used_memory()),
            total_memory: convert_to_gb(sys.total_memory()),
        };

        // Serialize and send the stats
        if let Ok(json) = serde_json::to_string(&stats) {
            if socket.send(Message::Text(json)).await.is_err() {
                // Client disconnected
                break;
            }
        }
    }
}

fn convert_to_gb(bytes: u64) -> f32 {
    bytes as f32 / 1024.0 / 1024.0 / 1024.0
}

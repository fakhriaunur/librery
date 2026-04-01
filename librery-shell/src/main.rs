use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Query, State},
    http::{header, HeaderValue, Request, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use biscuit_auth::KeyPair;
use h3_axum::serve_h3_with_axum;
use h3_quinn::Connection as H3QuinnConnection;
use quinn::{Endpoint, ServerConfig, TransportConfig};
use quinn::crypto::rustls::QuicServerConfig;
use rcgen::generate_simple_self_signed;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use librery_shell::api::auth::login::{login, LoginRequest, LoginResponse};
use librery_shell::api::auth::login::__path_login;
use librery_shell::api::auth::logout::{logout, LogoutResponse};
use librery_shell::api::auth::logout::__path_logout;
use librery_shell::patron_credentials_repository::SqlitePatronCredentialsRepository;
use librery_shell::search::SearchIndex;
use librery_shell::AppState;

#[derive(Deserialize, Serialize, ToSchema)]
struct CheckoutRequest {
    item_id: String,
    patron_id: String,
}

#[derive(Deserialize, ToSchema)]
struct SearchParams {
    q: String,
}

#[derive(Serialize, ToSchema)]
struct StatusResponse {
    status: String,
}

#[derive(Serialize, ToSchema)]
struct SearchResultResponse {
    item_id: String,
    patron_id: String,
    event_type: String,
    timestamp: i64,
    indexed_at: i64,
}

#[derive(Serialize, ToSchema)]
struct SearchResponse {
    results: Vec<SearchResultResponse>,
}

#[derive(OpenApi)]
#[openapi(
    paths(checkout, search_items, login, logout),
    components(schemas(
        CheckoutRequest,
        SearchParams,
        StatusResponse,
        SearchResultResponse,
        SearchResponse,
        LoginRequest,
        LoginResponse,
        LogoutResponse
    )),
    tags((name = "api", description = "Library API"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let search_index = Arc::new(SearchIndex::new()?);
    let credentials_repo = Arc::new(SqlitePatronCredentialsRepository::connect("sqlite:librery.db").await?);
    let biscuit_keypair = Arc::new(KeyPair::new());

    let state = AppState {
        search_index,
        credentials_repo,
        biscuit_keypair,
    };

    let app = Router::new()
        .route("/api/v1/checkout", post(checkout))
        .route("/api/v1/search", get(search_items))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/logout", post(logout))
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi()))
        .layer(axum::middleware::from_fn(add_alt_svc))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let (endpoint, tcp_listener) = setup_transports(addr).await?;
    let h3_app = app.clone();
    let h2_app = app.clone();

    let h3_task = tokio::spawn(async move {
        while let Some(incoming) = endpoint.accept().await {
            let conn = match incoming.await {
                Ok(conn) => conn,
                Err(_) => continue,
            };

            let router = h3_app.clone();
            tokio::spawn(async move {
                let _ = run_h3_connection(router, conn).await;
            });
        }
    });

    let h2_task = tokio::spawn(async move {
        let _ = axum::serve(tcp_listener, h2_app).await;
    });

    tokio::select! {
        _ = h3_task => {},
        _ = h2_task => {},
        _ = tokio::signal::ctrl_c() => {},
    }

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/v1/checkout",
    request_body = CheckoutRequest,
    responses((status = OK, body = StatusResponse)),
    tag = "api"
)]
async fn checkout(
    State(_state): State<AppState>,
    Json(_body): Json<CheckoutRequest>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(StatusResponse { status: "ok".into() }))
}

#[utoipa::path(
    get,
    path = "/api/v1/search",
    params(("q" = String, Query, description = "Search query")),
    responses((status = OK, body = SearchResponse)),
    tag = "api"
)]
async fn search_items(
    Query(params): Query<SearchParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let indexed_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    match state.search_index.search(&params.q) {
        Ok(results) => {
            let response = SearchResponse {
                results: results
                    .into_iter()
                    .map(|r| SearchResultResponse {
                        item_id: r.item_id,
                        patron_id: r.patron_id,
                        event_type: r.event_type,
                        timestamp: r.timestamp,
                        indexed_at,
                    })
                    .collect(),
            };
            (StatusCode::OK, Json(response))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SearchResponse { results: vec![] }),
        ),
    }
}

async fn run_h3_connection(
    app: Router,
    conn: quinn::Connection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut h3_conn = h3::server::Connection::new(H3QuinnConnection::new(conn)).await?;

    while let Some(resolver) = h3_conn.accept().await? {
        let app = app.clone();
        tokio::spawn(async move {
            let _ = serve_h3_with_axum(app, resolver).await;
        });
    }

    Ok(())
}

fn alt_svc_header(port: u16) -> HeaderValue {
    HeaderValue::from_str(&format!("h3=\":{}\"; ma=86400", port)).unwrap()
}

async fn setup_transports(
    addr: SocketAddr,
) -> Result<(Endpoint, tokio::net::TcpListener), Box<dyn std::error::Error + Send + Sync>> {
    let rcgen::CertifiedKey { cert, key_pair } = generate_simple_self_signed(vec!["localhost".to_string()])?;
    let cert_der = CertificateDer::from(cert);
    let key_der = PrivatePkcs8KeyDer::from(key_pair.serialize_der());

    let mut tls = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der], PrivateKeyDer::from(key_der))?;
    tls.alpn_protocols = vec![b"h3".to_vec(), b"h2".to_vec()];

    let quic_tls = QuicServerConfig::try_from(tls)?;
    let mut server_config = ServerConfig::with_crypto(Arc::new(quic_tls));
    server_config.transport = Arc::new(TransportConfig::default());

    let endpoint = Endpoint::server(server_config, addr)?;
    let tcp_listener = tokio::net::TcpListener::bind(addr).await?;
    Ok((endpoint, tcp_listener))
}

async fn add_alt_svc(
    req: Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Response<axum::body::Body> {
    let mut response = next.run(req).await;
    response.headers_mut().insert(header::ALT_SVC, alt_svc_header(8080));
    response
}

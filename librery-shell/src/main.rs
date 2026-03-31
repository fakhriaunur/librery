use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::State,
    http::{header, HeaderValue, Request, Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use h3_axum::serve_h3_with_axum;
use h3_quinn::Connection as H3QuinnConnection;
use quinn::{Endpoint, ServerConfig, TransportConfig};
use quinn::crypto::rustls::QuicServerConfig;
use rcgen::generate_simple_self_signed;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
struct AppState;

#[derive(Deserialize, Serialize, ToSchema)]
struct CheckoutRequest {
    item_id: String,
    patron_id: String,
}

#[derive(Serialize, ToSchema)]
struct StatusResponse {
    status: String,
}

#[derive(OpenApi)]
#[openapi(paths(checkout), components(schemas(CheckoutRequest, StatusResponse)), tags((name = "api", description = "Library API")))]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let app = Router::new()
        .route("/api/v1/checkout", post(checkout))
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi()))
        .layer(axum::middleware::from_fn(add_alt_svc))
        .with_state(AppState);

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

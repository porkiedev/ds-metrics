#![allow(unused)]
mod api;

use anyhow::Result;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tracing::{info, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::api::core_service_server::CoreServiceServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a filter that limits the log level to TRACE for this module, and INFO for everything else
    let filter = tracing_subscriber::filter::Targets::new()
        .with_default(Level::INFO)
        .with_target("module", Level::TRACE)
        .with_target(module_path!(), Level::TRACE);
    // Initialize the logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    // Read the TLS certificate and key
    let cert = std::fs::read_to_string(".secrets/cert.pem")
        .expect("Failed to read cert file");
    let key = std::fs::read_to_string(".secrets/key.pem")
        .expect("Failed to read key file");

    // Create the gRPC listener
    let address = "127.0.0.1:443".parse()?;
    info!("gRPC listening at {}", address);
    let server = api::Server {};
    let api_listener = Server::builder()
        .tls_config(ServerTlsConfig::new()
            .identity(Identity::from_pem(&cert, &key)))?
        .add_service(CoreServiceServer::new(server))
        .serve(address);

    // Wait for the gRPC listener to finish
    tokio::join!(api_listener);

    Ok(())
}

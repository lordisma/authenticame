pub mod api;

use tokio::runtime::Builder;
use tokio::time::Duration;
use axum::Server;
use envconfig::Envconfig;

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(feature = "console")] {
            console_subscriber::init();
        }
    }

    let configuration = api::ConfigMap::init_from_env().expect("Unable to parse configuration from env");

    let rt = Builder::new_multi_thread()
        .enable_io() // Enable the io operation into the tokio Runtime
        .max_blocking_threads(8) // Small number of blocking thread since bigger number could affect performance
        .global_queue_interval(61) // TODO: Investigate better relationship between global and local interval
        .thread_keep_alive(Duration::from_secs(2 * 60 * 60)) // Optimistic assumption that once a thread blocked is used we will need it again
        .thread_name("authenticame-worker")
        .build()
        .expect("Unable to create the runtime");

    rt.block_on(async move {
        cfg_if::cfg_if! {
            if #[cfg(not(feature = "console"))] {
                logging::set_up_logging();
            }
        }
        tracing::info!("Logging was successfully started");

        let socket = configuration.get_socket();
        tracing::info!("Starting server on socket: {}", socket);

        let route = routes::create_service();

        let server = Server::bind(&socket)
            .serve(route.into_make_service());

        if let Err(error) = server.await {
            // TODO: Handled the error 
            tracing::error!("Server shutdown unexpectly due to internal error: {}", error);
        }
    });

    // Setting the max time that a task can be running once the runtime is dropped
    rt.shutdown_timeout(Duration::from_secs(10));
}

mod routes {
    use axum::routing::*;
    use super::api::*;

    pub fn create_service() -> Router {
        let route = Router::new()
            .route("/register", post(login::post::register));
        
        route
    }       
}

mod logging {
    use tracing_subscriber::*;

    pub fn set_up_logging() {
        fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    }
}

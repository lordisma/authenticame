pub mod api;

use tokio::runtime::Builder;
use tokio::time::Duration;
use axum::Server;
use envconfig::Envconfig;

fn main() {
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
        let socket = configuration.get_socket();
        let route = routes::create_service();

        let server = Server::bind(&socket)
            .serve(route.into_make_service());

        if let Err(_) = server.await {
            // TODO: Handled the error 
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

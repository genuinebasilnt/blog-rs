use blog::configuration::get_configuration;
use sqlx::PgPool;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let app = blog::startup::run(connection_pool.clone()).await;

    let _ = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

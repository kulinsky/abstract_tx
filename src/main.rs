pub mod app;
pub mod err;
pub mod in_memory;
pub mod postgres;
pub mod repository;
pub mod tx_manager;

#[tokio::main]
async fn main() {
    let repository = in_memory::InMemoryRepository;
    let second_repository = in_memory::InMemorySecondRepository;
    let tx_manager = Box::new(in_memory::InMemoryTxManager);
    let app = app::App::new(repository, second_repository, tx_manager);

    app.save().await.unwrap();

    let pool = sqlx::PgPool::connect("postgres://postgres:postgrespw@localhost:32768/postgres")
        .await
        .unwrap();

    let repository = postgres::SQLXRepository::new(pool.clone());
    let second_repository = postgres::SQLXSecondRepository::new(pool.clone());
    let tx_manager = Box::new(postgres::SQLXTransactionManager::new(pool));
    let app = app::App::new(repository, second_repository, tx_manager);

    app.save().await.unwrap();

    println!("Hello, world!");
}

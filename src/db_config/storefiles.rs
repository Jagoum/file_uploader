use dotenvy::{dotenv, var};
use sqlx::PgPool;
use tokio::fs::File;

pub struct FileData {
    file_name: String,
    compressed_file: String,
}

impl FileData {
    pub fn new(file_name: String, compressed_file: String) -> Self {
        Self {
            file_name,
            compressed_file,
        }
    }
}
pub async fn store_file(content: FileData) {
    let durl = var("DATABASE_URL").expect("DATABASE_URL not set");
    dotenv().ok();
    let pool = PgPool::connect(&durl)
        .await
        .expect("Failed to connect to database");
    sqlx::query(
        r#"
        INSERT INTO files(file_name, compressed_file) VALUES($1, $2)
        "#,
    )
    .bind(content.file_name)
    .bind(content.compressed_file)
    .execute(&pool)
    .await
    .expect("Failed to insert file");
}

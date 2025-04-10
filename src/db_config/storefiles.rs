use chrono::Local;
use dotenvy::{dotenv, var};
use serde::Deserialize;
use sqlx::PgPool;
#[derive(Deserialize, sqlx::FromRow, Debug)]
pub struct FileData {
    pub file_name: String,
    pub  compressed_file: Option<String>,
}

impl FileData {
    pub fn new(file_name: String, compressed_file: Option<String>) -> Self {
        Self {
            file_name,
            compressed_file,
        }
    }
}
pub async fn store_file(content: FileData) {
    let start = Local::now();
    println!("{:?} : Connecting to database ", start);

    let durl = var("DATABASE_URL").expect("DATABASE_URL not set");
    dotenv().ok();
    let pool = PgPool::connect(&durl)
        .await
        .expect("Failed to connect to database");
    println!("{:?} : Connection Successful! ", start);
    println!("{:?} : Saving file to database Initiated", start);
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
    let end = Local::now();
    println!(
        "{:?} : File Store in database successfully , Time Taken {:?}",
        start, end.signed_duration_since(start).num_microseconds()
    );
}

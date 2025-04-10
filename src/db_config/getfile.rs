use axum::response::Html;
use chrono::Local;
use dotenvy::{dotenv, var};
use sqlx::PgPool;

use crate::db_config::storefiles::FileData;

pub async fn get_files() -> Html<String> {
    let start = Local::now();
    println!("{:?} : Connecting to database ", start);

    let durl = var("DATABASE_URL").expect("DATABASE_URL not set");
    dotenv().ok();
    let pool = PgPool::connect(&durl)
        .await
        .expect("Failed To connect to database");
    println!("{} : connection to database successful", start);
    let files: Vec<FileData> = sqlx::query_as::<_, FileData>(
        r##"
        SELECT file_name, compressed_file, created_at FROM files
    "##,
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to extract data from the database");
let mut html = String::from("<h1> Compressed Files</h1><url>");
    for file in files{
        html.push_str(&format!(
            "<li><strong>File Name:</strong> {} <br> <strong>Compressed File:</strong> {} <br> </li>",
            file.file_name,
            file.compressed_file.clone().unwrap_or_else(|| "N/A".to_string()),
        ));
    println!(" {:?} ",file)
}
html.push_str("</url>");
Html(html)
}

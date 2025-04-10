use axum::extract::Multipart;
use chrono::Local;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::{
    db_config::storefiles::{FileData, store_file},
    upload::compress,
};

pub async fn file_uplaod(mut multipart: Multipart)
{
    
    let start = Local::now();

    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get the part")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }
        // getting the file
        let file_name = field.file_name().expect("Failed to get file name");
        println!("{:?} : Got the file {}",start, file_name);

        // creating file dir

        fs::create_dir_all("files").await.unwrap();
        println!("{} : Creating a directory ", start);
        
        // creating a path  for to serve the file to be
        let file_path = format!("files/{}", file_name);
        println!("{} : Creating file_path {}",start, file_name);
        // get the incoming bytes
        let data = field.bytes().await.unwrap();
        // Opening the handle to the file
        let mut file_handle = File::create(file_path.clone())
            .await
            .expect("Failed to create file");

        // writing all the incoming files to handle
        file_handle
            .write_all(&data)
            .await
            .expect("Failed to write all bytes to handler");
        // Here we call the compress fn to compress our collected file.
        let compressed_file = compress(&file_path).await;
        let file = file_path;
        let content = FileData::new(file, Some(compressed_file));
        store_file(content).await;
    }
}

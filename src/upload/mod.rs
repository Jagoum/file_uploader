pub mod upload;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use std::time::Instant;

/// # This function is used to compress a given file 
/// 
/// It takes as parameter the file name you want to compress 
/// It uses GzEncoder to encode the file or compress the file.
pub async fn compress(file: &str) -> String {
    let start = Instant::now();
    println!("Started compressing at {:?} ",start.elapsed());
    let mut input = BufReader::new(File::open(file).unwrap());
    let file = format!("{}_cpr",file);
    let output = File::create(file.clone()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());

    println!("Compression completed , Time Elapsed: {:?}", start.elapsed());
    file
}

use futures::{stream, StreamExt}; 
use reqwest::Client; 
use tokio;

const PARALLEL_REQUESTS: usize = 8;

#[tokio::main]
async fn main() {
    let urls = vec!["http://127.0.0.1:7878/sleep"; 8];

    let client = Client::new();
    let bodies = stream::iter(urls)
        .map(|url| {
            let client = client.clone();
            tokio::spawn(async move {
                let resp = client.get(url).send().await?;
                resp.bytes().await
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    // let  mut bodies = Vec::new();
    // for _ in 0..60 {
    //     let client  = client.clone();
    //     bodies.push(tokio::spawn(async move {
    //         let resp = client.get(url).send().await?;
    //         resp.bytes().await
    //     }));
    //}

    bodies
        .for_each(|b| async {
            match b {
                Ok(Ok(b)) => println!("Got {} bytes", b.len()),
                Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;
}
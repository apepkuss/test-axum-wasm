use bytes::Bytes;
use futures_util::StreamExt;

use axum::{extract::BodyStream, routing::get, routing::post, Router};
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/help", get(help))
        .route("/echo", post(echo));

    // run it
    let addr = "0.0.0.0:12306";
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    if let Err(e) = axum::Server::from_tcp(tcp_listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service())
        .await
    {
        println!("server error: {}", e);
    }
}

async fn help() -> &'static str {
    println!("help handler");

    // sleep 1 second to simulate a slow handler
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    "Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`\n"
}

async fn echo(mut stream: BodyStream) -> Bytes {
    println!("echo handler");

    // sleep 1 second to simulate a slow handler
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    if let Some(Ok(s)) = stream.next().await {
        s
    } else {
        Bytes::new()
    }
}

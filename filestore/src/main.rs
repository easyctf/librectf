use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::Result;
use futures::stream::StreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{
    http::{Method, StatusCode},
    Body, Request, Response, Server,
};
use multipart_async::server::Multipart;
use sha2::{Digest, Sha256};
use tokio::{fs::File, io::AsyncWriteExt};

const DEFAULT_PATH: &str = "/usr/share/nginx/html";

lazy_static::lazy_static! {
    static ref UPLOAD_FOLDER : PathBuf =
        PathBuf::from(env::var("UPLOAD_FOLDER").unwrap_or_else(|_| DEFAULT_PATH.to_owned()));
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match hello_world_inner(req).await {
        Ok(response) => Ok(response),
        Err(err) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(err.to_string().into())
            .unwrap()),
    }
}

async fn hello_world_inner(req: Request<Body>) -> Result<Response<Body>> {
    let uri = req.uri();
    let path = uri.path();

    while req.method() == Method::POST && path == "/save" {
        let mut multipart = match Multipart::try_from_request(req) {
            Ok(v) => v,
            Err(_) => break,
        };

        while let Some(mut field) = multipart.next_field().await? {
            println!("field: {:?}", field.headers);

            if field.headers.name != "file" {
                continue;
            }

            let filename = match field.headers.filename {
                Some(v) if v.len() > 0 => v,
                Some(_) | None => continue,
            };

            let hash = Sha256::digest(filename.as_bytes());

            let file_name = format!("{:x}-{}", hash, filename);
            let file_path = UPLOAD_FOLDER.join(&file_name);

            let mut file = File::open(&file_path).await?;

            while let Some(chunk) = field.data.next().await {
                let chunk = chunk?;
                file.write(&chunk).await?;
            }
        }

        return Ok(Response::new("Hello, World".into()));
    }

    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("not found".into())?)
}

#[tokio::main]
async fn main() {
    println!("Uploading to {:?}", *UPLOAD_FOLDER);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

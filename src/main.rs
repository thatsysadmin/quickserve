mod mime_handler;

use std::path::{Path, PathBuf};
use std::process::exit;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::{arg, Parser};
use colored::Colorize;
use tokio_util::io::ReaderStream;
use crate::mime_handler::get_mime;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Target directory
    #[arg(short, long)]
    directory: String,

    /// Port to listen on
    #[arg(short, long)]
    port: Option<u16>,

    /// IP address to listen on
    #[arg(short, long)]
    ip: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let arguments = Arguments::parse();

    let serve_path = Path::new(&arguments.directory);
    if !serve_path.exists() {
        println!("{}", format!("Path {} does not exist - double check you specified the correct path.", &serve_path.display()).bright_red());
        exit(1);
    }

    let port = match arguments.port {
        None => {
            println!("{}", "No port specified - defaulting port to 8080.".bright_yellow());
            8080
        }
        Some(p) => {p}
    };
    let ip_address = match arguments.ip {
        None => {
            println!("{}", "No IP specified - listening everywhere.".bright_yellow());
            "0.0.0.0".to_string()
        }
        Some(i) => {i}
    };

    // Launch Actix
    let app_state = web::Data::new(ApplicationState {
        serve_path: format!("{}", serve_path.display())
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(webpath)
    })
        .bind((ip_address, port))?
        .run()
        .await
}

struct ApplicationState {
    serve_path: String,
}

const BODY_INTERNALSERVERERROR: &str = "Internal Server Error (Code 500)";
const BODY_NOTFOUND: &str = "Not Found (Code 404)";

#[get("/{tail:.*}")]
async fn webpath(request_parameters: HttpRequest, call_path: web::Path<String>, app_state: web::Data<ApplicationState>) -> impl Responder {
    let path = call_path.into_inner();
    match request_parameters.connection_info().realip_remote_addr() {
        None => {
            if path.is_empty() {
                println!("{}", "Unknown client requested root item".bright_yellow());
            }
            else {
                println!("{}", format!("Unknown client requested item {}", path).bright_yellow());
            }
        }
        Some(client_address) => {
            if path.is_empty() {
                println!("Client {} requested root item", client_address);
            }
            else {
                println!("Client {} requested item {}", client_address, path);
            }
        }
    }

    let mut filesystem_path = PathBuf::new().join(&app_state.serve_path).join(path);
    println!("    Full path {}", filesystem_path.display());
    if filesystem_path.is_dir() {
        filesystem_path.push("index.html");
        println!("    Path is now {:?}", filesystem_path);
    }
    else if !filesystem_path.is_file(){
        println!("{}", "    Failed to find file.".bright_red());
        return HttpResponse::NotFound().body(BODY_NOTFOUND)
    }

    let mime_guess = match get_mime(&filesystem_path) {
        None => {
            println!("{}", "".bright_red());
            return HttpResponse::NotFound().body(BODY_INTERNALSERVERERROR)
        }
        Some(mime) => { mime }
    };

    let file_handle = match tokio::fs::File::open(&filesystem_path).await {
        Ok(x) => { x }
        Err(e) => {
            println!("{}", "    Failed to open file.".bright_red());
            dbg!(e);
            return HttpResponse::InternalServerError().body(BODY_INTERNALSERVERERROR);
        }
    };
    let stream_handle = ReaderStream::new(file_handle);

    println!("{}", "    Success.".bright_green());
    HttpResponse::Ok()
        .content_type(mime_guess)
        .streaming(stream_handle)
}
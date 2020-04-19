pub use geng::prelude::*;

mod client;
mod model;
#[cfg(not(target_arch = "wasm32"))]
mod server;

use client::*;
use model::*;
#[cfg(not(target_arch = "wasm32"))]
use server::*;

#[derive(StructOpt, Debug, Clone)]
pub struct NetOpts {
    #[structopt(default_value, long)]
    server_host: String,
    #[structopt(default_value, long)]
    server_port: u16,
    #[structopt(default_value, long)]
    connect: String,
}

impl NetOpts {
    fn normalize(&mut self) {
        if self.server_host.is_empty() {
            self.server_host = option_env!("LD46_SERVER_HOST")
                .unwrap_or("127.0.0.1")
                .to_owned();
        }
        if self.server_port == 0 {
            self.server_port = option_env!("LD46_SERVER_PORT")
                .unwrap_or("1155")
                .parse()
                .unwrap();
        }
        if self.connect.is_empty() {
            self.connect = option_env!("LD46_CONNECT")
                .unwrap_or("ws://localhost:1155")
                .to_owned();
        }
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct Opts {
    #[structopt(flatten)]
    net_opts: NetOpts,
    #[structopt(long = "name", default_value = "<noname>")]
    name: String,
    #[structopt(long)]
    no_client: bool,
    #[structopt(long)]
    start_server: bool,
}

impl Opts {
    fn normalize(&mut self) {
        self.net_opts.normalize();
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(path) = std::env::var("CARGO_MANIFEST_DIR") {
            std::env::set_current_dir(std::path::Path::new(&path).join("static")).unwrap();
        } else {
            std::env::set_current_dir(std::env::current_exe().unwrap().parent().unwrap()).unwrap();
        }
    }
    logger::init().unwrap();
    let opts = {
        let mut opts: Opts = program_args::parse();
        opts.normalize();
        opts
    };
    info!("Options used:\n{:#?}", opts);

    #[cfg(target_arch = "wasm32")]
    let server = None::<()>;
    #[cfg(not(target_arch = "wasm32"))]
    let (server, server_handle) = if opts.start_server {
        let server = Server::new(&opts.net_opts);
        let server_handle = server.handle();
        ctrlc::set_handler({
            let server_handle = server_handle.clone();
            move || {
                server_handle.shutdown();
            }
        })
        .unwrap();
        (Some(server), Some(server_handle))
    } else {
        (None, None)
    };

    #[cfg(not(target_arch = "wasm32"))]
    let server_thread = if let Some(server) = server {
        if !opts.no_client {
            Some(std::thread::spawn(move || server.run()))
        } else {
            server.run();
            None
        }
    } else {
        None
    };

    if !opts.no_client {
        ClientApp::run(&opts);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(server_thread) = server_thread {
            if !opts.no_client {
                server_handle.unwrap().shutdown();
            }
            server_thread.join().unwrap();
        }
    }
}

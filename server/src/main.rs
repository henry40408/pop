use std::net::SocketAddr;
use std::sync::Arc;

use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::env;
use structopt::StructOpt;
use warp::http::StatusCode;
use warp::Filter;

#[derive(StructOpt)]
#[structopt(about, author)]
struct Opts {
    /// Pushover API token
    #[structopt(short, long, env = "PUSHOVER_TOKEN")]
    token: String,
    /// Pushover user token
    #[structopt(short, long, env = "PUSHOVER_USER")]
    user: String,
    /// Authorization token to protect the proxy
    #[structopt(short, long, env = "AUTHORIZATION")]
    authorization: Option<String>,
    /// host and port to bind
    #[structopt(short, long, env = "BIND", default_value = "127.0.0.1:3000")]
    bind: String,
    /// debug mode
    #[structopt(long)]
    debug: bool,
}

#[derive(Default, Debug, Deserialize, Serialize)]
struct Notification {
    device: Option<String>,
    title: Option<String>,
    message: String,
    html: Option<bool>,
    timestamp: Option<u64>,
    priority: Option<u8>,
    url: Option<String>,
    url_title: Option<String>,
    sound: Option<String>,
    image_url: Option<String>,
}

#[derive(Serialize)]
struct ErrorMessage {
    status: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Arc<Opts> = Arc::new(Opts::from_args());

    if env::var_os("RUST_LOG").is_none() {
        if opts.debug {
            env::set_var("RUST_LOG", "pops=debug");
        } else {
            env::set_var("RUST_LOG", "pops=info");
        }
    }

    pretty_env_logger::init();

    if opts.authorization.is_none() {
        warn!("no authorization set, server is vulnerable");
    }

    let opts2 = opts.clone();
    let messages = warp::path!("v1" / "messages")
        .and(warp::post())
        .and(warp::body::content_length_limit(16 * 1_024))
        .and(warp::body::json())
        .and(warp::header::optional::<String>("authorization"))
        .map(
            move |notification: Notification, authorization: Option<String>| {
                if let Some(ref a) = opts2.authorization {
                    // authorization is expected
                    if let Some(ref b) = authorization {
                        // authorization is given
                        if b != a {
                            // authorization does not match
                            return warp::reply::with_status(
                                warp::reply::json(&ErrorMessage {
                                    status: StatusCode::BAD_REQUEST.as_u16(),
                                }),
                                StatusCode::BAD_REQUEST,
                            );
                        }
                    } else {
                        // authorization is not given
                        return warp::reply::with_status(
                            warp::reply::json(&ErrorMessage {
                                status: StatusCode::BAD_REQUEST.as_u16(),
                            }),
                            StatusCode::BAD_REQUEST,
                        );
                    }
                }

                warp::reply::with_status(
                    warp::reply::json(&Message {
                        message: "Hello, world!".into(),
                    }),
                    StatusCode::OK,
                )
            },
        )
        .with(warp::log::log("pops"));

    let bind: SocketAddr = opts.bind.parse()?;
    info!("server is running on {}", &opts.bind);
    warp::serve(messages).run(bind).await;

    Ok(())
}

#[derive(Serialize)]
struct Message {
    message: String,
}

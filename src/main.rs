use structopt::StructOpt;
use pop::wrapped::Wrapped;
use pop::pushover::Request;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
struct Opts {
    /// Pushover API token, get it on https://pushover.net/apps/build
    #[structopt(short = "t", env = "PUSHOVER_TOKEN")]
    token: Option<String>,
    /// Pushover user key, get it on https://pushover.net/
    #[structopt(short = "u", env = "PUSHOVER_USER")]
    user: Option<String>,
    /// Message to send
    #[structopt(short = "m")]
    message: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::from_args();

    let token = match opts.token {
        Some(t) => t,
        None => bail!("token is required for CLI"),
    };

    let user = match opts.user {
        Some(u) => u,
        None => bail!("user is required for CLI"),
    };

    let message = match opts.message {
        Some(m) => m,
        None => bail!("message is required for CLI"),
    };

    let request = Wrapped {
        request: Request {
            token,
            user,
            message,
            ..Default::default()
        },
    };
    request.send().await?;
    Ok(())
}

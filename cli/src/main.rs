use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

use anyhow::bail;
use atty::Stream;
use structopt::StructOpt;

use pop::pushover::Request;
use pop::wrapped::{Attachment, Wrapped};

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
struct Opts {
    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,
    /// Pushover API token, get it on https://pushover.net/apps/build
    #[structopt(short, long, env = "PUSHOVER_TOKEN")]
    token: String,
    /// Pushover user key, get it on https://pushover.net/
    #[structopt(short, long, env = "PUSHOVER_USER")]
    user: String,
    /// Message to send
    #[structopt(short, long)]
    message: String,
    /// Attachment to send, which is an image usually
    #[structopt(short, long, parse(from_os_str))]
    attachment: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::from_args();

    let attachment = build_attachment(&opts)?;
    let request = Wrapped {
        request: Request {
            token: opts.token.clone(),
            user: opts.user.clone(),
            message: opts.message.clone(),
            ..Default::default()
        },
        attachment,
    };

    let response = request.send().await?;
    if opts.verbose {
        println!("{}", serde_json::to_string(&response)?);
    }

    Ok(())
}

fn read_from_stdin_or_file(opts: &Opts) -> anyhow::Result<Option<Box<dyn BufRead>>> {
    if atty::isnt(Stream::Stdin) {
        // read from STDIN
        Ok(Some(Box::new(BufReader::new(io::stdin()))))
    } else if let Some(ref a) = opts.attachment {
        // read from designated file
        Ok(Some(Box::new(BufReader::new(fs::File::open(a)?))))
    } else {
        // Nothing
        Ok(None)
    }
}

fn build_attachment(opts: &Opts) -> anyhow::Result<Option<Attachment>> {
    if let Some(mut r) = read_from_stdin_or_file(&opts)? {
        let mut content = Vec::new();
        r.read_to_end(&mut content)?;

        let mime_type = match infer::get(&content) {
            Some(m) => m,
            None => bail!("MIME type of attachment is unknown"),
        };

        let filename = match &opts.attachment {
            Some(f) => match f.to_str() {
                Some(s) => s.to_string(),
                None => bail!("failed to extract filename from attachment"),
            },
            None => format!("file.{}", mime_type.extension()),
        };

        Ok(Some(Attachment {
            filename,
            mime_type: mime_type.to_string(),
            content,
        }))
    } else {
        Ok(None)
    }
}

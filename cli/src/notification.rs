use anyhow::bail;
use reqwest::multipart;

use crate::pushover::{Request, Response};

#[cfg(test)]
fn endpoint_url() -> String {
    mockito::server_url()
}

#[cfg(not(test))]
fn endpoint_url() -> String {
    "https://api.pushover.net".to_string()
}

#[derive(Debug)]
pub struct Attachment {
    pub filename: String,
    pub mime_type: String,
    pub content: Vec<u8>,
}

#[derive(Default)]
pub struct Notification {
    pub request: Request,
    pub attachment: Option<Attachment>,
}

impl Notification {
    pub fn new(token: &str, user: &str, message: &str) -> Self {
        Self {
            request: Request {
                token: token.to_string(),
                user: user.to_string(),
                message: message.to_string(),
                ..Default::default()
            },
            attachment: None,
        }
    }

    pub fn attach<S: ToString>(self, filename: S, mime_type: S, content: Vec<u8>) -> Self {
        Self {
            request: self.request,
            attachment: Some(Attachment {
                filename: filename.to_string(),
                mime_type: mime_type.to_string(),
                content,
            }),
        }
    }

    pub async fn attach_url(self, url: &str) -> anyhow::Result<Self> {
        let res = reqwest::get(url).await?;
        let content = res.bytes().await?.to_vec();

        let mime_type = match infer::get(&content) {
            Some(m) => m,
            None => bail!("MIME type of {} is unknown", url),
        };
        let filename = format!("file.{}", mime_type.extension());

        Ok(Self {
            request: self.request,
            attachment: Some(Attachment {
                filename,
                mime_type: mime_type.to_string(),
                content,
            }),
        })
    }

    pub async fn send(&self) -> anyhow::Result<Response> {
        let client = reqwest::Client::new();

        let parts = multipart::Form::new()
            .text("token", self.request.token.clone())
            .text("user", self.request.user.clone())
            .text("message", self.request.message.clone());

        let parts = if let Some(ref a) = self.attachment {
            let part = multipart::Part::bytes(a.content.clone())
                .file_name(a.filename.clone())
                .mime_str(&a.mime_type)?;
            parts.part("attachment", part)
        } else {
            parts
        };

        let url = format!("{0}/1/messages.json", endpoint_url());
        let res = client.post(url).multipart(parts).send().await?;
        let res: Response = res.json::<Response>().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use mockito::mock;

    use crate::notification::Notification;
    use crate::pushover::Request;

    #[tokio::test]
    async fn test_send() -> anyhow::Result<()> {
        let _m = mock("POST", "/1/messages.json")
            .with_status(200)
            .with_body(r#"{"status":1,"request":"647d2300-702c-4b38-8b2f-d56326ae460b"}"#)
            .create();

        let inner = Request {
            token: "token".to_string(),
            user: "user".to_string(),
            message: "message".to_string(),
            ..Default::default()
        };
        let request = Notification {
            request: inner,
            ..Default::default()
        };
        let res = request.send().await?;
        assert_eq!(1, res.status);
        assert_eq!("647d2300-702c-4b38-8b2f-d56326ae460b", res.request);
        Ok(())
    }
}

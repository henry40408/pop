use reqwest::multipart;
use serde::{Deserialize, Serialize};

#[cfg(test)]
fn endpoint_url() -> String {
    return mockito::server_url();
}

#[cfg(not(test))]
fn endpoint_url() -> String {
    return "https://api.pushover.net".to_string();
}

#[derive(Serialize)]
struct Request {
    token: String,
    user: String,
    device: Option<String>,
    title: Option<String>,
    message: String,
    html: Option<bool>,
    timestamp: Option<u64>,
    priority: Option<u8>,
    url: Option<String>,
    url_title: Option<String>,
    sound: Option<String>,
}

struct WithAttachment {
    request: Request,
}

impl WithAttachment {
    async fn send(&self) -> anyhow::Result<Response> {
        let client = reqwest::Client::new();

        let mut parts = multipart::Form::new();
        parts = parts
            .text("token", self.request.token.clone())
            .text("user", self.request.user.clone())
            .text("message", self.request.message.clone());

        let url = format!("{0}/1/messages.json", endpoint_url());
        let res = client.post(url).multipart(parts).send().await?;
        let res: Response = res.json::<Response>().await?;
        Ok(res)
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    status: u64,
    request: String,
    errors: Option<Vec<String>>,
}

#[cfg(test)]
mod test {
    use crate::client::{Request, WithAttachment};
    use mockito::mock;

    #[tokio::test]
    async fn test_send() -> anyhow::Result<()> {
        let _m = mock("POST", "/1/messages.json")
            .with_status(200)
            .with_body(r#"{"status":1,"request":"647d2300-702c-4b38-8b2f-d56326ae460b"}"#)
            .create();

        let with_attachment = WithAttachment {
            request: Request {
                token: "token".to_string(),
                user: "user".to_string(),
                device: None,
                title: None,
                message: "message".to_string(),
                html: None,
                timestamp: None,
                priority: None,
                url: None,
                url_title: None,
                sound: None,
            },
        };
        let res = with_attachment.send().await?;
        assert_eq!(1, res.status);
        assert_eq!("647d2300-702c-4b38-8b2f-d56326ae460b", res.request);
        Ok(())
    }
}

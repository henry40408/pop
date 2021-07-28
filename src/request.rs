use reqwest::multipart;

use crate::client::{InnerRequest, InnerResponse};

#[cfg(test)]
fn endpoint_url() -> String {
    mockito::server_url()
}

#[cfg(not(test))]
fn endpoint_url() -> String {
    "https://api.pushover.net".to_string()
}

pub(crate) struct Request {
    pub(crate) inner: InnerRequest,
}

impl Request {
    async fn send(&self) -> anyhow::Result<InnerResponse> {
        let client = reqwest::Client::new();

        let parts = multipart::Form::new()
            .text("token", self.inner.token.clone())
            .text("user", self.inner.user.clone())
            .text("message", self.inner.message.clone());

        let url = format!("{0}/1/messages.json", endpoint_url());
        let res = client.post(url).multipart(parts).send().await?;
        let res: InnerResponse = res.json::<InnerResponse>().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use mockito::mock;

    use crate::client::InnerRequest;
    use crate::request::Request;

    #[tokio::test]
    async fn test_send() -> anyhow::Result<()> {
        let _m = mock("POST", "/1/messages.json")
            .with_status(200)
            .with_body(r#"{"status":1,"request":"647d2300-702c-4b38-8b2f-d56326ae460b"}"#)
            .create();

        let inner = InnerRequest {
            token: "token".to_string(),
            user: "user".to_string(),
            message: "message".to_string(),
            ..Default::default()
        };
        let request = Request { inner };
        let res = request.send().await?;
        assert_eq!(1, res.status);
        assert_eq!("647d2300-702c-4b38-8b2f-d56326ae460b", res.request);
        Ok(())
    }
}

use base64;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use std::error::Error;

pub struct Credentials {
    pub email: String,
    pub token: String,
}

impl Credentials {
    pub fn new(email: &str, token: &str) -> Self {
        Credentials {
            email: email.to_string(),
            token: token.to_string(),
        }
    }
}

pub struct Jira {
    client: Client,
    host: String,
    credentials: Credentials,
}

impl Jira {
    pub fn new(domain: &str, email: &str, api_token: &str) -> Result<Self, Box<dyn Error>> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_call() {
        let jira_client = JiraClient::new("your-domain", "your-email", "your-api-token").unwrap();
        let response = jira_client.api_call("rest/api/3/myself").await.unwrap();
        println!("{}", response);
    }
}

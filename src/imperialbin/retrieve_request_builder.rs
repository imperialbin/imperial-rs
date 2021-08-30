use serde_json::{Value, Number};
use serde::{Deserialize};
#[allow(non_snake_case)]
pub struct RetrieveRequestBuilder {
    apiToken: String,
    documentId: String,
}

pub struct RetrieveResponse {
    pub success: bool,
    pub content: String,
    pub document: RetrieveResponseDocument
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct RetrieveResponseDocument {
    pub documentId: String,
    pub language: String,
    pub imageEmbed: bool,
    pub instantDelete: bool,
    pub creationDate: Number,
    pub expirationDate: Number,
    pub allowedEditors: Vec<String>,
    pub encrypted: bool,
    pub views: Number

}

pub fn new(document_id: String) -> RetrieveRequestBuilder{
    RetrieveRequestBuilder {
        apiToken: String::new(),
        documentId: document_id,
    }
}

impl RetrieveRequestBuilder {
    pub fn api_token(mut self, api_token: String) -> Self {
        self.apiToken = api_token;
        self
    }

    pub fn send(self) -> anyhow::Result<RetrieveResponse> {
        
        let http_client = reqwest::blocking::Client::new();
        let mut request_builder = http_client.get(format!("https://imperialb.in/api/document/{}", self.documentId));
        
        if self.apiToken != "" {
            request_builder = request_builder.header("authorization", &self.apiToken);
        }

        let response = request_builder.send()?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let response_json: Value = {
                    let response_text = response.text()?;
                    serde_json::from_str(&response_text[..])?
                };
                
                return Ok(RetrieveResponse {
                    success: match response_json["success"].clone() {
                        Value::Bool(e) => e,
                        _ => false,
                    },
                    content: match response_json["content"].clone() {
                        Value::String(s) => s,
                        _ => String::from("No content. Stuff might be bad serverside"),
                    },
                    document: serde_json::from_value(response_json.clone())?,
                })


            }
            _ => {
                return Err(anyhow::format_err!("Non OK status code"))
            }
        }
    }
}
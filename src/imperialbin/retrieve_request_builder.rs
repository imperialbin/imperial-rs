use serde_json::Number;
use serde::{Deserialize};

#[derive(Debug)]
pub enum Errors {
    BadRequest(anyhow::Error),
    Unauthorized(anyhow::Error),
    NotFound(anyhow::Error),
    Forbidden(anyhow::Error),
    TooManyRequests(anyhow::Error),
    InternalServerError(anyhow::Error),
    NonImplementedResponse(anyhow::Error),
    RequestSendError(reqwest::Error),
    ResponseTextError(reqwest::Error),
    ResponseJsonError(serde_json::Error),
}


pub struct RetrieveRequestBuilder {
    api_token: String,
    base_url: String,
    document_id: String,
} // A RetrieveRequestBuilder which contains the specific required parts for a request and apiToken

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveResponse {
    pub success: bool,
    pub content: String,
    pub document: RetrieveResponseDocument
} // This is the "lowest" part of the json. It contains the document which are the part with all the data about the document.

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveResponseDocument {
    pub document_id: String,
    pub language: String,
    pub image_embed: bool,
    pub instant_delete: bool,
    pub creation_date: Number,
    pub expiration_date: Number,
    pub allowed_editors: Vec<String>,
    pub encrypted: bool,
    pub views: Number

} // This contains all data about the document. It is based around the document which exists in the API.

pub fn new(api_token: String, base_url: String, document_id: String) -> RetrieveRequestBuilder{
    RetrieveRequestBuilder {
        api_token: api_token,
        base_url: base_url,
        document_id: document_id,
    }
} // This function creates an empty instance of the RetrieveRequestBuilder

impl RetrieveRequestBuilder {
    pub fn api_token(mut self, api_token: String) -> Self {
        self.api_token = api_token;
        self
    }

    pub fn send(self) -> Result<RetrieveResponse, Errors> {
        
        let http_client = reqwest::blocking::Client::new();

        let url = format!("{}/api/document/{}", self.base_url, self.document_id);

        let mut request_builder = http_client.get(url);
        
        if self.api_token != "" {
            request_builder = request_builder.header("authorization", &self.api_token);
        }

        let response = match request_builder.send() {
            Ok(r) => r,
            Err(e) => return Err(Errors::RequestSendError(e))
        };

        match response.status() {
            reqwest::StatusCode::OK => {

                let response_text = match response.text() {
                    Ok(s) => s,
                    Err(e) => return Err(Errors::ResponseTextError(e))
                };
                
                match serde_json::from_str::<RetrieveResponse>(&response_text) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(Errors::ResponseJsonError(e))
                }
            },
            reqwest::StatusCode::BAD_REQUEST => {
                Err(Errors::BadRequest(anyhow::anyhow!("Non-valid document id")))
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(Errors::Unauthorized(anyhow::anyhow!("You need to provide password")))
            },
            reqwest::StatusCode::NOT_FOUND => {
                Err(Errors::NotFound(anyhow::anyhow!("Document couldn't be found")))
            },
            reqwest::StatusCode::FORBIDDEN => {
                Err(Errors::Forbidden(anyhow::anyhow!("Incorrect password to encrypted document")))
            },
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                Err(Errors::TooManyRequests(anyhow::anyhow!("Too many requests. Implement a rate limit.")))
            },
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                Err(Errors::InternalServerError(anyhow::anyhow!("Internal Server Error.")))
            }
            _ => {
                return Err(Errors::NonImplementedResponse(anyhow::format_err!("Non implemented response.")))
            }
        }
    }
}
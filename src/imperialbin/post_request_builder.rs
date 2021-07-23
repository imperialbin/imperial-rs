use serde::{Deserialize, Serialize};
use serde_json::{Value, Number};
// Object/Struct to contain api_token and have functions to send the request. 
// It stores a PostRequest which has all the data that is needed in the request
pub struct PostRequestBuilder {
    api_token: String,
    post_request: PostRequest
}

// This stores the needed data for the api request
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PostRequest {
    code: String,
    longerUrls: bool,
    imageEmbed: bool,
    instantDelete: bool,
    encrypted: bool,
    password: String,
    expiration: u32,
    editors: Vec<String>
}

// This is a struct for the response
// It also stores a PostResponseDocument which is a specific json object that we get in the response.
#[allow(non_snake_case)]
pub struct PostResponse {
    pub success: bool,
    pub rawLink: String,
    pub formattedLink: String,
    pub document: PostResponseDocument,
    pub message: String
}

// This is the rust implementation of the json object which exists in the response
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct PostResponseDocument {
    pub documentId: String,
    pub language: String,
    pub imageEmbed: bool,
    pub instantDelete: bool,
    pub creationDate: Number,
    pub expirationDate: Number,
    pub allowedEditors: Vec<String>,
    pub encrypted: bool,
    pub password: Option<String>,
    pub views: Number,
}

impl PostRequestBuilder {

    // Functions to change different values. They return itself to allow chaining together
    pub fn api_token(mut self, api_token: String) -> Self {
        self.api_token = api_token;
        self
    }

    pub fn code(mut self, code: String) -> Self {
        self.post_request.code = code;
        self
    }

    pub fn longerUrls(mut self, longerUrls: bool) -> Self {
        self.post_request.longerUrls = longerUrls;
        self
    }

    pub fn imageEmbed(mut self, imageEmbed: bool ) -> Self {
        self.post_request.imageEmbed = imageEmbed;
        self
    }

    pub fn instantDelete(mut self, instantDelete: bool) -> Self {
        self.post_request.instantDelete = instantDelete;
        self
    }

    pub fn encrypted(mut self, encrypted: bool) -> Self {
        self.post_request.encrypted = encrypted;
        self
    }

    pub fn password(mut self, password: String) -> Self {
        self.post_request.password = password;
        self
    }

    pub fn expiration(mut self, expiration: u32) -> Self {
        self.post_request.expiration = expiration;
        self
    }

    pub fn editors(mut self, editors: Vec<String>) -> Self {
        self.post_request.editors = editors;
        self
    }

    // This will create the necessary json and send a request and parse it into PostResponse and PostResponseDocument
    pub fn send( self) -> anyhow::Result<PostResponse> {
        let http_client = reqwest::blocking::Client::new();

        let json_body = serde_json::to_string(&self.post_request)?;
        let mut request_builder = http_client.post("https://imperialb.in/api/document")
        .body(json_body)
        .header("content-type", "application/json");
        // If we have a api_token in configuration make sure to use it in the request.
        if self.api_token != "" {
            request_builder = request_builder.header("authorization", &self.api_token);
        }
        // Send the request and get a response back.
        let response = request_builder.send()?;
        match response.status() {
            reqwest::StatusCode::OK => {
               
                let response_json: Value = {
                    let response_text = response.text()?;
                    serde_json::from_str(&response_text[..])?
                };
                println!("response json done");
                return Ok(PostResponse {
                    success: match response_json["success"].clone() {
                        Value::Bool(b) => b,
                        _ => false
                    },
                    message: match response_json["message"].clone() {
                        Value::String(s) => s,
                        _ => String::from("No message")
                    },
                    rawLink: match response_json["rawLink"].clone() {
                        Value::String(s) => s,
                        _ => String::from("no rawLink"),
                    },
                    formattedLink: match response_json["formattedLink"].clone() {
                        Value::String(s) => s,
                        _ => String::from("no formattedLink")
                    },
                    document: serde_json::from_value(response_json["document"].clone())?
                })
            },
            s => Err(anyhow::format_err!(format!("tf. not OK 200 response. this is sussy. its {}. response text: {}", s, response.text()?)))
        }
    }
    
}

pub fn new() -> PostRequestBuilder {
    PostRequestBuilder {
        api_token: String::new(),
        post_request: PostRequest {
            code: String::new(),
            longerUrls: false,
            imageEmbed: false,
            instantDelete: false,
            encrypted: false,
            password: String::new(),
            expiration: 5,
            editors: Vec::new()
        }
    }
}
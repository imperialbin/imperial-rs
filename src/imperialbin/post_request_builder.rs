use serde::{Deserialize, Serialize, };
use serde_json::{Number};

fn default_string() -> String {
    String::new()
}


// Object/Struct to contain api_token and have functions to send the request. 
// It stores a PostRequest which has all the data that is needed in the request
pub struct PostRequestBuilder {
    api_token: String,
    post_request: PostRequest
}

// This stores the needed data for the api request
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRequest {
    code: String,
    longer_urls: bool,
    image_embed: bool,
    instant_delete: bool,
    encrypted: bool,
    password: String,
    expiration: u32,
    editors: Vec<String>
}

// This is a struct for the response
// It also stores a PostResponseDocument which is a specific json object that we get in the response.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostResponse {
    pub success: bool,
    pub raw_link: String,
    pub formatted_link: String,
    pub document: PostResponseDocument,
    #[serde(default = "default_string")]
    pub message: String
}

// This is the rust implementation of the json object which exists in the response
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostResponseDocument {
    pub document_id: String,
    pub language: String,
    pub image_embed: bool,
    pub instant_delete: bool,
    pub creation_date: Number,
    pub expiration_date: Number,
    pub allowed_editors: Vec<String>,
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

    pub fn longer_urls(mut self, longer_urls: bool) -> Self {
        self.post_request.longer_urls = longer_urls;
        self
    }

    pub fn image_embed(mut self, image_embed: bool ) -> Self {
        self.post_request.image_embed = image_embed;
        self
    }

    pub fn instant_delete(mut self, instant_delete: bool) -> Self {
        self.post_request.instant_delete = instant_delete;
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
            // TODO: Implement all the responses
            reqwest::StatusCode::OK => {
                let response_text = response.text()?;
                return Ok(serde_json::from_str::<PostResponse>(&response_text)?)
            },
            s => Err(anyhow::format_err!(format!("Got response {}. response text: {}", s, response.text()?)))
        }
    }
    
}

pub fn new() -> PostRequestBuilder {
    PostRequestBuilder {
        api_token: String::new(),
        post_request: PostRequest {
            code: String::new(),
            longer_urls: false,
            image_embed: false,
            instant_delete: false,
            encrypted: false,
            password: String::new(),
            expiration: 5,
            editors: Vec::new()
        }
    }
}
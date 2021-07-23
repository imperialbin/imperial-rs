use serde::{Deserialize, Serialize};

// Object/Struct to contain api_token and have functions to send the request. 
// It stores a PostRequest which has all the data that is needed in the request
pub struct PostRequestBuilder {
    api_token: String,
    post_request: PostRequest
}

// This stores the needed data for the api request
#[allow(non_snake_case)]
pub struct PostRequest {
    code: String,
    longerUrls: bool,
    imageEmbed: bool,
    instantDelete: bool,
    encrypted: bool,
    password: String,
    expiration: i32,
    editors: Vec<String>
}

// This is a struct for the response
// It also stores a PostResponseDocument which is a specific json object that we get in the response.
#[allow(non_snake_case)]
pub struct PostResponse {
    success: bool,
    rawLink: Option<String>,
    formattedLink: Option<String>,
    document: Option<PostResponseDocument>,
    message: Option<String>
}

// This is the rust implementation of the json object which exists in the response
#[allow(non_snake_case)]
pub struct PostResponseDocument {
    documentId: String,
    language: String,
    imageEmbed: bool,
    instantDelete: bool,
    creationDate: i32,
}

/*
success: true,
rawLink: `https://imperialb.in/r/${URL}`,
formattedLink: `https://imperialb.in/p/${URL}`,
document: {
  documentId: "xxxxxxxx", // string
  language: "xxxxxxxxxx", // string
  imageEmbed: xxxxx, // boolean
  instantDelete: xxxxx, // boolean
  creationDate: xxxxx, // number
  expirationDate: xxxxx, // number
  allowedEditors: ["xxxxx","xxxxx"], // Array of strings
  encrypted: xxxxx, // boolean
  password: xxxx, // null or a string
  views: xx, // number
},
*/
impl PostRequestBuilder {


    pub fn api_token(mut self, api_token: String) -> Self {
        self.api_token = api_token;
        self
    }

    pub fn code(mut self, code: String) -> Self {
        self.post_request.code = code;
        self
    }


    pub fn send(&self) -> anyhow::Result<String> {
        let http_client = reqwest::blocking::Client::new();
        let mut request_builder = http_client.post("https://imperialb.in/api/document")
        .body(format!("{{\"code\": \"{}\"}}", self.post_request.code)) // Shitty construction of JSON
        .header("content-type", "application/json"); // Specified in the API
        // If we have a api_token in configuration make sure to use it in the request.
        if self.api_token != "" {
            request_builder = request_builder.header("authorization", &self.api_token);
        }
        // Send the request and get a response back.
        let response = request_builder.send()?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let response_text = response.text()?;
                return Ok(response_text);
            },
            s => println!("Received response status: {:?}", s)
        }
        Ok(String::new())
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
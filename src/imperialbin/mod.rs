
pub mod post_request_builder;
pub mod retrieve_request_builder;

// TODO: Allow other imperialb.in instances

// Function to create an empty Imperialbin Client
pub fn init() -> ImperialbinClient {
    return ImperialbinClient {
        api_token: String::new(),
        base_url: String::from("https://imperialb.in")
    }
}
pub struct ImperialbinClient {
    api_token: String,
    base_url: String,
}

impl ImperialbinClient {
    //Setting api token in the struct
    pub fn api_token(&mut self, api_token: String) {
        self.api_token = api_token;
    }

    pub fn base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

    pub fn post(&self, code: String) -> post_request_builder::PostRequestBuilder {
        // Creating a PostRequestBuilder which you can change different arguments with.
        post_request_builder::new(self.api_token.clone(), self.base_url.clone()).code(code)
    }

    pub fn retrieve(&self, document_id: String) -> retrieve_request_builder::RetrieveRequestBuilder {
        retrieve_request_builder::new(self.api_token.clone(), self.base_url.clone(), document_id).api_token(self.api_token.clone())
    }
}
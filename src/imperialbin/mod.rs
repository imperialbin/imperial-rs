
mod post_request_builder;

// Function to create an empty Imperialbin Client
pub fn init() -> ImperialbinClient {
    return ImperialbinClient {
        api_token: String::new(),
    }
}
pub struct ImperialbinClient {
    api_token: String,
}

impl ImperialbinClient {
    //Setting api token in the struct
    pub fn api_token(&mut self, new_token: String) {
        self.api_token = new_token;
    }

    pub fn post(&self, code: String) -> post_request_builder::PostRequestBuilder {
        // Creating a PostRequestBuilder which you can change different arguments with.
        post_request_builder::new().api_token(self.api_token.clone()).code(code)

    }
}
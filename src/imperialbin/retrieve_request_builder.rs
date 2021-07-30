#[allow(non_snake_case)]
pub struct RetrieveRequestBuilder {
    apiToken: String,
    documentId: String,
}

pub fn new() -> RetrieveRequestBuilder{
    RetrieveRequestBuilder {
        apiToken: String::new(),
        documentId: String::new(),
    }
}

impl RetrieveRequestBuilder {
    pub fn api_token(mut self, api_token: String) -> Self {
        self.apiToken = api_token;
        self
    }

    pub fn document_id(mut self, document_id: String) -> Self {
        self.documentId = document_id;
        self
    }
}
use serde_json::Value;

#[allow(non_snake_case)]
pub struct RetrieveRequestBuilder {
    apiToken: String,
    documentId: String,
}

pub struct RetrieveResponse {
    success: bool,
    content: String,
    document: RetrieveResponseDocument
}

pub struct RetrieveResponseDocument {

}

pub fn new(documentId: String) -> RetrieveRequestBuilder{
    RetrieveRequestBuilder {
        apiToken: String::new(),
        documentId: documentId,
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

                


            }
            _ => {}
        }


        Ok(RetrieveResponse {
            success: true,
            content: String::new(),
            document: RetrieveResponseDocument {

            }
        })
    }
}

mod imperialbin;
pub use imperialbin::*;


#[cfg(test)]
mod tests {
    #[test]
    fn post_and_retrieve() -> anyhow::Result<()>{
        use crate::imperialbin;
        let imperialbin_client = imperialbin::init();
        let response = imperialbin_client.post(String::from("Hello")).send()?;
        
        
        println!("{}", response.document.documentId.clone());
        let retrieve_response = imperialbin_client.retrieve(response.document.documentId).send()?;
        println!("{}", retrieve_response.content);
        Ok(())
    }
}


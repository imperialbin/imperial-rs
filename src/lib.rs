
mod imperialbin;
pub use imperialbin::*;


#[cfg(test)]
mod tests {
    #[test]
    fn post_and_retrieve() -> anyhow::Result<()>{
        use crate::imperialbin;
        let imperialbin_client = imperialbin::init();
        let response = imperialbin_client.post(String::from("Hello")).send()?;
        
        
        println!("{}", response.rawLink);
        Ok(())
    }
}


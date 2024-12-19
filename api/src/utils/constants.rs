use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::error::Error;

lazy_static! {
    pub static ref ADDRESS: String = set_address().expect("Failed to get ADDRESS");
    pub static ref PORT: u16 = set_port().expect("Failed to get PORT");
}

fn set_address() -> Result<String, env::VarError> {
    dotenv().ok();
    match env::var("ADDRESS") {
        Ok(address) => {
            println!("ADDRESS: {}", address); // Log the ADDRESS
            Ok(address)
        }
        Err(e) => {
            println!("Failed to get ADDRESS: {}", e); // Log the error
            Err(e)
        }
    }
}

fn set_port() -> Result<u16, Box<dyn Error>> {
    dotenv().ok();
    match env::var("PORT") {
        Ok(port_str) => {
            println!("PORT: {}", port_str); // Log the PORT
            match port_str.parse::<u16>() {
                Ok(port) => Ok(port),
                Err(e) => {
                    println!("Failed to parse PORT: {}", e); // Log the parsing error
                    Err(Box::new(e))
                }
            }
        }
        Err(e) => {
            println!("Failed to get PORT: {}", e); // Log the error
            Err(Box::new(e))
        }
    }
}

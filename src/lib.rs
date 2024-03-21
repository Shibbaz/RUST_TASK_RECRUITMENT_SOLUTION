use serde_json;
use crate::api::{
    credentials::Credentials,
    arguments::{
ExchangeCurrencyArguments,
        RatiosListArguments,
    }
};

pub mod api {
    use log::{info, error};
    use reqwest;
    use crate::serde_json::Map;
    use crate::Credentials;
    use crate::ExchangeCurrencyArguments;
    use crate::RatiosListArguments;
    use core::any::Any;
    use std::collections::HashMap;

    pub type Result<T> = std::result::Result<T, Box<(dyn std::error::Error + 'static)>>;

    pub mod arguments {
        pub struct RatiosListArguments{
            pub base: String,
        }
        
        pub struct ExchangeCurrencyArguments{
            pub from: String,
            pub to: String,
            pub amount: f64,
        }
    }
    pub mod credentials {
        use std::env;

        pub struct Credentials <'a> {
            pub url: &'a str,
            pub api_key: &'a str,
        }
        
        impl Credentials<'_> {
            /// The `new` function in Rust initializes a new Credentials struct.
            /// 
            /// Returns:
            /// 
            /// An instance of the struct that the `new` function is defined in is being returned.
            pub fn new<'a>() -> Self {
                let url: String = match env::var("API_URL") {
                    Ok(val) => val,
                    Err(_) => panic!("API_URL is not defined in the environment"),
                };
                let api_key: String = match env::var("API_KEY") {
                    Ok(val) => val,
                    Err(_) => panic!("API_KEY is not defined in the environment"),
                };
        
                Credentials { url: Box::leak(url.into_boxed_str()), api_key: Box::leak(api_key.into_boxed_str()) }     
            }
        }
    }
    pub trait Requests {
        fn call(&mut self) -> impl std::future::Future<Output = Result<Box<dyn Any>>> + Send;
    }

    pub struct Request<'a>{
        pub credentials: &'a Credentials<'a>
    }
    
    pub struct CurrencyListRequest<'a>{
        pub credentials: &'a Credentials<'a>
    }

    pub struct RatiosListRequest<'a>{
        pub args: RatiosListArguments,
        pub credentials: &'a Credentials<'a>
    }
    
    pub struct ExchangeCurrencyRequest<'a>{
        pub args: ExchangeCurrencyArguments,
        pub credentials: &'a Credentials<'a>
    }
/// The `impl Requests for ExchangeCurrencyRequest<'_>` block in the Rust code is implementing the `Requests` trait for
/// the `ExchangeCurrencyRequest` struct. This means that the `ExchangeCurrencyRequest` struct will now have the behavior defined in the
/// `Requests` trait.
    impl Requests for ExchangeCurrencyRequest<'_>{
        /// This async function sends a request to a specified URL with parameters, retrieves JSON data, and
        /// prints converted value and its currency based on the provided params.
        /// 
        /// Returns:
        /// 
        /// The `call` function returns a `Result` with the success type `()` (unit type) and the error type
        /// `Box<dyn std::error::Error>`.
        async fn call(&mut self) -> Result<Box<dyn Any>> {
            let url: &str = &(self.credentials.url.to_string() + "/v1/convert");
            let request = reqwest::Url::parse_with_params(url, &[
                (
                    "api_key",  self.credentials.api_key
                ),
                (
                    "from", &self.args.from.to_ascii_uppercase()
                ),
                (
                    "to", &self.args.to.to_ascii_uppercase()
                ),
                (
                    "amount", &self.args.amount.to_string()
                ),
            ]
            )?;
            match reqwest::get(request).await {
                Ok(resp) => {
                    let json: serde_json::Value = resp.json().await?;
                    let data: &Map<String, serde_json::Value> = &json.as_object().unwrap();
                    if data["response"]["value"].to_string() != "[]" {
                        let f: f32 = data["response"]["value"].to_string().parse().unwrap();
                        return Ok(Box::new(f));
                    }else{
                        return Ok(Box::new(0));
                    }
                }
                Err(_) => {
                    return Err("Request got bad".into())
                }
            }
        }
    }
/// The `impl Requests for RatiosListRequest<'_>` block in the Rust code is implementing the `Requests` trait for
/// the `RatiosListRequest` struct. This means that the `RatiosListRequest` struct will now have the behavior defined in the
/// `Requests` trait.
    impl Requests for RatiosListRequest<'_>{
        /// This async function sends a request to a specified URL with parameters, retrieves JSON data, and
        /// prints exchange rates based on the provided base currency.
        /// 
        /// Returns:
        /// 
        /// The `call` function returns a `Result` with the success type `()` (unit type) and the error type
        /// `Box<dyn std::error::Error>`.
        async fn call(&mut self) -> Result<Box<dyn Any>> {
            let url: &str = &(self.credentials.url.to_string() + "/v1/latest");
            let request = reqwest::Url::parse_with_params(url, &[
                (
                    "api_key",  self.credentials.api_key
                ),
                (
                    "base", &self.args.base.to_ascii_uppercase()
                )
            ])?;

            match reqwest::get(request).await {
                Ok(resp) => {
                    let json: serde_json::Value = resp.json().await?;
                    let data = json.as_object().unwrap().iter();
                    let mut ratios: String = "[]".to_string();
                    for (_,value) in data {
                        if value["rates"].to_string() != "null" && value["rates"].to_string() != "[]" {
                            ratios = value["rates".to_string()].to_string();
                            break;
                        }
                    }
                    return Ok(Box::new(ratios));
                }
                Err(_err) => {
                    return Err("Request went bad".into());
                }
            }
        }
    }
    
/// The `impl Requests for CurrencyListRequest<'_>` block in the Rust code is implementing the `Requests` trait for
/// the `CurrencyListRequest` struct. This means that the `CurrencyListRequest` struct will now have the behavior defined in the
/// `Requests` trait.
    impl Requests for CurrencyListRequest<'_>{
        /// The function `call` sends a request to a specified URL, retrieves JSON data, and prints out the
        /// short code and name of each currency in the response.
        // Add API_KEY and API_URL=https://api.currencybeacon.com values in ~/.bashrc
        // You can generate API KEY on https://api.currencybeacon.com
        /// 
        /// Returns:
        /// 
        /// The `call` function is returning a `Result` enum with the success variant containing `()` (unit
        /// type) and the error variant containing a boxed trait object that implements the `std::error::Error`
        /// trait.
        async fn call(&mut self) -> Result<Box<dyn Any>> {
            let url: &str = &(self.credentials.url.to_string() + "/v1/currencies?api_key=" + self.credentials.api_key);
            match reqwest::get(url).await{
                Ok(resp) => {
                    let json: serde_json::Value = resp.json().await?;
                    let data: Map<String, serde_json::Value> = json.as_object().unwrap().clone();
                    let mut result: HashMap<std::string::String, std::string::String> = HashMap::new();
                    for (_, value) in data.into_iter() {
                        match value["short_code"].to_string().as_ref(){
                            "null" => {},
                            _ => {
                                result.insert(value["short_code"].to_string(), value["name"].to_string());
                            }
                        }
                    }
                    return Ok(Box::new(result));
                }
                Err(_) => {
                    return Err("Request went bad".into());
                }
            };    
        }
    }
    
/// The `impl Requests for Request<'_>` block in the Rust code is implementing the `Requests` trait for
/// the `Request` struct. This means that the `Request` struct will now have the behavior defined in the
/// `Requests` trait.
    impl Requests for Request<'_>{
        /// The function `call` sends a request to an external API to check out aliveness of the service
        /// 
        /// Returns:
        /// 
        /// The `call` function returns a `Result` enum with the success variant containing `()`
        /// (unit type) and the error variant containing a boxed `dyn std::error::Error` trait object.
        async fn call(&mut self) -> Result<Box<dyn Any>> {
            let url: &str = &(self.credentials.url.to_string() + "?api_key=" + self.credentials.api_key);
            match reqwest::get(url).await {
                Ok(_) => {
                    info!("Request was sent to {}", self.credentials.url)
                }
                Err(err) => {
                    error!("Request got an error: {}", err)
                }
            }
            Ok(Box::new(()))
        }
    }
}

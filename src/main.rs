use serde_json;
use crate::api::ExchangeCurrencyRequest;
use crate::api::Requests;
use crate::api::RatiosListRequest;
use crate::api::CurrencyListRequest;
use crate::api::Request;
use crate::api::credentials::Credentials;
use crate::api::arguments::ExchangeCurrencyArguments;
use crate::api::arguments::RatiosListArguments;
use crate::input::read_input;

pub mod api {
    use reqwest;
    use serde_json;
    use crate::serde_json::Map;
    use crate::Credentials;
    use crate::ExchangeCurrencyArguments;
    use crate::RatiosListArguments;

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
                let params: Credentials = Self{
                        url: env!("API_URL"),
                        api_key: env!("API_KEY"),
                };
                return params;
        
            }
        }
    }
    pub trait Requests {
        fn call(&mut self) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
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
        async fn call(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
                    println!("Request was sent to {}/v1/convert", self.credentials.url);
                    if data["response"]["value"].to_string() != "[]" {
                        let f: f32 = data["response"]["value"].to_string().parse().unwrap();
                        println!("{}{}'ve been converted to {}{}", &self.args.amount, &self.args.from, (f* 100.0).round() / 100.0, &self.args.to);
                    }
                }
                Err(err) => {
                    println!("Request got an error: {}", err)
                }
            }
            Ok(())
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
        async fn call(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
                    let data: &Map<String, serde_json::Value> = &json.as_object().unwrap();
                    println!("Request was sent to {}/v1/latest", self.credentials.url);
                    for (_,value) in data.iter() {
                        if value["rates"].to_string() != "null" && value["rates"].to_string() != "[]" {
                            println!("{}", value["rates"])
                        }
                        if value["rates"].to_string() == "[]"{
                            println!("You picked up wrong base currency, such does not exist")
                        }
                    }
                }
                Err(err) => {
                    println!("Request got an error: {}", err)
                }
            }
            Ok(())
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
        async fn call(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            let url: &str = &(self.credentials.url.to_string() + "/v1/currencies?api_key=" + self.credentials.api_key);
            match reqwest::get(url).await {
                Ok(resp) => {
                    let json: serde_json::Value = resp.json().await?;
                    let data: &Map<String, serde_json::Value> = &json.as_object().unwrap();
                    println!("Request was sent to {}/v1/currencies", self.credentials.url);
                    for (_, value) in data {
                        match value["short_code"].to_string().as_ref(){
                            "null" => {},
                            _ => {
                                println!("{}:{}", value["short_code"], value["name"])
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Request got an error: {}", err)
                }
            }
    
            Ok(())
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
        async fn call(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            let url: &str = &(self.credentials.url.to_string() + "?api_key=" + self.credentials.api_key);
            match reqwest::get(url).await {
                Ok(_) => {
                    println!("Request was sent to {}", self.credentials.url)
                }
                Err(err) => {
                    println!("Request got an error: {}", err)
                }
            }
            Ok(())
        }
    }
}

mod input {
    use std::io;
    pub fn read_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}


/// The above Rust code defines a main function that interacts with different API requests based on user
/// input in a loop until the user chooses to exit the program.
/// 
/// Returns:
/// 
/// The `main` function is returning a `Result<(), Box<dyn std::error::Error>>`. This means it will
/// return `Ok(())` if the program runs successfully, and it will return an error if any errors occur
/// during execution.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials: &mut Credentials<'_> = &mut Credentials::new();
    loop {
        let action: &str = &read_input("Type '1' => to check if an API is alive\nType '2' => to print currency list\nType '3' => to print ratios\nType '4' => to exchange currencies\nType 'exit' to exit program");
        match action{ 
            "1" => {
                Request{
                    credentials: credentials,
                }.call().await.expect("Error!");
            },
            "2" => {
                CurrencyListRequest{
                    credentials: credentials,
                }.call().await.expect("Error!")
            },
            "3" => {
                let base: String = read_input("Choose param 'base'");

                match (RatiosListRequest{
                    credentials: credentials,
                    args: RatiosListArguments{
                        base: base,
                    }
                }).call().await{
                    Ok(_) => {
                        println!("Success!")
                    },
                    Err(_err) => {
                        println!("Failure!")
                    }

                };
            },
            "4" => {
                let amount: String = read_input("Choose param 'amount'");
                if amount.parse::<f64>().is_ok() == false {
                    println!("Amount must be number");
                    continue;
                }
                let from: String = read_input("Choose param 'from'");
                let to: String = read_input("Choose param 'to'");
                let parsed_amount: f64 = amount.parse().unwrap();
                match (ExchangeCurrencyRequest{
                    credentials: credentials,
                    args: ExchangeCurrencyArguments{
                        from: from.to_string(),
                        to: to.to_string(),
                        amount: parsed_amount,
                    }
                }).call().await{
                    Ok(_) => {
                        println!("Success!")
                    },
                    Err(_err) => {
                        println!("Failure!")
                    }

                };
            },
            "exit"=> {
                break
            }, 
            _=>println!("If you wished to exit, type 'exit'"),
        }; 
    }
    Ok(())
}

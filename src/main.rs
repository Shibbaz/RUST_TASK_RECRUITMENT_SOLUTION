
use crate::input::read_input;
use std::collections::HashMap;

use task::api::{
    ExchangeCurrencyRequest,
    Requests,
    RatiosListRequest,
    CurrencyListRequest,
    Request,
    credentials::{
        Credentials,
    },
    arguments::{
ExchangeCurrencyArguments,
        RatiosListArguments,
    }
};
use log::{LevelFilter, log_enabled, Level, info, error};

mod input {
    use std::io;
    use crate::info;
    pub fn read_input(prompt: &str) -> String{
        info!("{}",prompt);
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
    env_logger::init();
    log::set_max_level(LevelFilter::Debug);
    if log_enabled!(Level::Debug) {
        println!("LOG LEVEL DEBUG");
    }
    let credentials: &mut Credentials<'_> = &mut Credentials::new();
    loop {
        info!("Type '1' => to check if an API is alive");
        info!("Type '2' => to print currency list");
        info!("Type '3' => to print ratios");
        info!("Type '4' => to exchange currencies");
        info!("Type 'exit' to exit program\n");
        let action: &str = &read_input("Choose action");
        match action{ 
            "1" => {
                match (Request{
                    credentials: credentials,
                }).call().await{
                    Ok(_) => {
                        info!("Success!")
                    },
                    Err(_err) => {
                        error!("Failure!")
                    }

                };
            },
            "2" => {
                match (CurrencyListRequest{
                    credentials: credentials,
                }).call().await{
                    Ok(resp) => {
                        info!("Request was sent to {}/v1/currencies", credentials.url);
                        let data = resp.downcast::<HashMap<String, String>>().unwrap();

                        for (key, value) in data.into_iter() {
                            println!("{}:{}", key, value)
                        }
                        info!("Success!")
                    },
                    Err(_err) => {
                        error!("Failure!")
                    }

                };
            },
            "3" => {
                let base: String = read_input("Choose param 'base'");

                let data = RatiosListRequest{
                    credentials: credentials,
                    args: RatiosListArguments{
                        base: base,
                    }
                }.call().await?;
                info!("Request was sent to {}/v1/latest", credentials.url);
                let rates = &data.downcast::<String>().unwrap();
                if rates.to_string() == "[]" {
                    error!("You picked up wrong base currency, such does not exist")
                }
                else{
                    info!("{}", rates.to_string());
                    info!("Success!");
                }
            },
            "4" => {
                let amount: String = read_input("Choose param 'amount'");
                if amount.parse::<f64>().is_ok() == false {
                    error!("Amount must be number");
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
                    Ok(resp) => {
                        let value = resp.downcast::<f32>().unwrap();
                        info!("{}{}'ve been converted to {}{}", &amount, &from, value, to);
                        println!("{:?}", value);
                        info!("Success!")
                    },
                    Err(_err) => {
                        error!("Request got an error!")
                    }

                };
            },
            "exit"=> {
                break
            }, 
            _=>info!("If you wished to exit, type 'exit'"),
        };
    println!("") 
    }
    Ok(())
}

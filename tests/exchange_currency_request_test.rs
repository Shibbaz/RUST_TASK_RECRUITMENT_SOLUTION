pub(crate) use core::any::Any;
use serde_json::json;
use task::api::arguments::ExchangeCurrencyArguments;
use task::api::Requests;
use task::api::Result;
pub struct ExchangeCurrencyRequestsMock {
  pub args: ExchangeCurrencyArguments,
}
/// Mocking ExchangeCurrencyRequest
/// testing functionality by faking data instead of taking it from external API call
impl Requests for ExchangeCurrencyRequestsMock {
    async fn call(&mut self) -> Result<Box<dyn Any>> {
        let json =  json!({
              "USD": {
                "USD": 1.0,
                "PLN": 3.96,
                "AUD": 1.52,
                "EUR": 0.92,
              },
              "PLN": {
                "PLN": 1.0,
                "USD": 0.25,
                "AUD": 0.38,
                "EUR": 0.23,
              }
            });
        if json[&self.args.from.to_ascii_uppercase()].is_null() || json[&self.args.from.to_ascii_uppercase()][&self.args.to.to_ascii_uppercase()].is_null(){
          return Err("Request got bad".into())
        }
        let rate: f64 = json[&self.args.from.to_ascii_uppercase()][&self.args.to.to_ascii_uppercase()].to_string().parse::<f64>().unwrap();
        let value: f64 = self.args.amount* (rate * 100.00).round() / 100.00;
        Ok(Box::new(value))
    }
}


#[tokio::test]
async fn test_exchange_currency_request() {
      let mut client = ExchangeCurrencyRequestsMock{
        args: ExchangeCurrencyArguments{
          from: "USD".to_string(),
          to: "AUD".to_string(),
          amount: 121.6,
        }
      };
      let result: Result<Box<dyn Any>> = client.call().await;
      let value: f64 = *result.unwrap().downcast::<f64>().unwrap();
      assert_eq!(value, 184.832);
}

#[tokio::test]
async fn test_exchange_currency_request_non_existant_currency_to_returns_err() {
  let mut client = ExchangeCurrencyRequestsMock{
    args: ExchangeCurrencyArguments{
      from: "USD".to_string(),
      to: "xd".to_string(),
      amount: 123.6,
    }
  };
  assert_eq!(*client.call().await.unwrap_err().to_string(), "Request got bad".to_string());
}

#[tokio::test]
async fn test_exchange_currency_request_non_existant_currency_from_returns_err() {
  let mut client = ExchangeCurrencyRequestsMock{
    args: ExchangeCurrencyArguments{
      from: "xd".to_string(),
      to: "AUD".to_string(),
      amount: 123.6,
    }
  };
  assert_eq!(*client.call().await.unwrap_err().to_string(), "Request got bad".to_string());
}

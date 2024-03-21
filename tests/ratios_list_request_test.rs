pub(crate) use core::any::Any;
use serde_json::{json, Value};
use task::api::Requests;
use task::api::arguments::RatiosListArguments;
use task::api::Result;
pub struct RatiosListRequestRequestsMock {
  pub args: RatiosListArguments,
}
/// Mocking RatiosListRequest
///testing functionality by faking data instead of taking it from external API call
impl Requests for RatiosListRequestRequestsMock {
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
        if json[&self.args.base.to_ascii_uppercase()].is_null(){
          return Err("Request got bad".into())
        }
        let value: Value = json[&self.args.base.to_ascii_uppercase()].clone();
        Ok(Box::new(value))
    }
}


#[tokio::test]
async fn test_ratios_list_request_succesful() {
      let mut client = RatiosListRequestRequestsMock{
        args: RatiosListArguments{
          base: "USD".to_string(),
        }
      };
      let result: Result<Box<dyn Any>> = client.call().await;
      let value: Value = *result.unwrap().downcast::<Value>().unwrap();
      assert_eq!(value["USD"], 1.0);
      assert_eq!(value["PLN"], 3.96);
      assert_eq!(value["AUD"], 1.52);
      assert_eq!(value["EUR"], 0.92);
      assert_eq!(value.as_object().unwrap().len(), 4);

      let mut client = RatiosListRequestRequestsMock{
        args: RatiosListArguments{
          base: "PLN".to_string(),
        }
      };
      let result: Result<Box<dyn Any>> = client.call().await;
      let value: Value = *result.unwrap().downcast::<Value>().unwrap();
      assert_eq!(value["USD"], 0.25);
      assert_eq!(value["PLN"], 1.0);
      assert_eq!(value["AUD"], 0.38);
      assert_eq!(value["EUR"], 0.23);
      assert_eq!(value.as_object().unwrap().len(), 4);
}

#[tokio::test]
async fn test_ratios_list_request_non_existant_currency_base_returns_err() {
  let mut client = RatiosListRequestRequestsMock{
    args: RatiosListArguments{
      base: "XD".to_string(),
    }
  };
  assert_eq!(*client.call().await.unwrap_err().to_string(), "Request got bad".to_string());
}

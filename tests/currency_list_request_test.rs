pub(crate) use core::any::Any;
use serde_json::{json, Value};
use task::api::Requests;
use task::api::Result;
pub struct CurrencyListRequestRequestsMock {}

impl Requests for CurrencyListRequestRequestsMock {
    async fn call(&mut self) -> Result<Box<dyn Any>> {
        let json =  json!({
              "USD": "United States dollar",
              "PLN": "Polish Zloty",
              "AUD": "Australian dollar",
              "EUR": "Euro",
            });
        Ok(Box::new(json))
    }
}


#[tokio::test]
async fn test_currency_list_request_succesful() {
      let mut client = CurrencyListRequestRequestsMock{};
      let result: Result<Box<dyn Any>> = client.call().await;
      let value: Value = *result.unwrap().downcast::<Value>().unwrap();
      assert_eq!(value.as_object().unwrap().len(), 4);
      assert_eq!(value["USD"].to_string(), "\"United States dollar\"");
      assert_eq!(value["PLN"].to_string(), "\"Polish Zloty\"");
      assert_eq!(value["AUD"].to_string(), "\"Australian dollar\"");
      assert_eq!(value["EUR"].to_string(), "\"Euro\"");
}
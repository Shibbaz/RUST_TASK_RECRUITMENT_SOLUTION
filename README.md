# Instructions
- Setting up

  - To build docs ```cargo doc```
  - to build project ```cargo build```
  - to run ```RUST_LOG=task cargo run```
  - To set environment variables in ~/.bashrc
  ```
  - export API_URL=https://api.currencybeacon.com 
  - export API_KEY= /* Token you found in the 'API Token Information' section */
  ```
  - Log in through github on https://currencybeacon.com/login, then in the 'API Token Information' section on https://currencybeacon.com/account/dashboard You can find token.
- Docker
	- Building image
	  ```
	  sudo docker build -t app --build-arg API_KEY=$API_KEY --build-arg API_URL=$API_URL .
	  ```
	- Running docker container
	  ```
	  sudo docker run -it app
	  ```

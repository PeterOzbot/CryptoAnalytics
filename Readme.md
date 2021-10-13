# Crypto Analytics

Web app that displays analytics about some crypto coins.

Written in Rust with Yew.

### Resources:

Crypto icons:
http://cryptoicons.co/

Crypto data:
https://www.coingecko.com/api/documentations/v3

# Docker
docker build -t crypto-analytics .
docker run -dp 8080:80 --rm --name crypto-analytics crypto-analytics
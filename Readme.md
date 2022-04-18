# Crypto Analytics

Web app that displays analytics about some crypto coins.

Written in Rust with Yew.

### Resources:

Crypto icons:
http://cryptoicons.co/

Crypto data:
https://www.coingecko.com/api/documentations/v3

# Docker
sudo docker build -t crypto-analytics .

sudo docker run -dp 1080:80 --restart always --name crypto-analytics crypto-analytics 

### Docker Hub
docker tag crypto-analytics:latest peterozbot/crypto-analytics:latest

docker push peterozbot/crypto-analytics:latest
# Crypto Analytics

Web app that displays analytics about some crypto coins.

Written in Rust with Yew.

# Resources:

Crypto data:
https://www.coingecko.com/api/documentations/v3

# Run app
## Frontend
Run frontend:
```
trunk serve
```
Configuration in Trunk.toml.
## Backend
Run and watch for changes:
```
cargo watch -c -w src -x run
```
Host name and port defined in .env.

# State management
TODO: https://github.com/intendednull/yewdux

# Docker
```
sudo docker build -t crypto-analytics .

sudo docker run -dp 1080:80 --restart always --name crypto-analytics
```

### Docker Hub
```
docker tag crypto-analytics:latest peterozbot/crypto-analytics:latest

docker push peterozbot/crypto-analytics:latest
```

# PostgreSQL

```
sudo docker run --name crypto-analytics-postgresql -p 5432:5432 -e POSTGRES_PASSWORD=postgres -v ~/crypto-analytics:/var/lib/postgresql/data -d postgres
```
**Set up database && schema**

Navigate to scripts folder and execute:
```
psql -h localhost -p 5432 -U postgres -a -f schema.sql -f data.sql;
```

*Log into postgress contaier with bash*
```
docker exec -it crypto-analytics-postgresql bash
```
*Connect with psql*
```
psql -h localhost -p 5432 -U postgres
```
*Connect to database && Execute script from file*
```
\c crypto_analytics
\i /home/.../schema.sql;
```



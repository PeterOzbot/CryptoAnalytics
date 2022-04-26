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

### Frontend:
```
sudo docker build --file=frontend.dockerfile -t crypto-analytics-frontend .
```

### Backend
```
sudo docker build --file=backend.dockerfile -t crypto-analytics-backend .
```

sudo docker run -dp 1080:80 --restart always --name crypto-analytics crypto-analytics:latest

sudo docker run -dp 1080:80 --restart always --name crypto-analytics-frontend crypto-analytics-frontend:latest
sudo docker run -dp 5010:5010 --restart always --name crypto-analytics-backend crypto-analytics-backend:latest


sudo docker run -p 5010:5010 -e DATABASE_URL='postgres://postgres:postgres@172.19.0.2:5432/crypto_analytics' -e SERVER_URL='172.19.0.3:5010' --network crypto-analytics-network --name crypto-analytics-backend crypto-analytics-backend:latest

### Docker Hub
```
docker tag crypto-analytics:latest peterozbot/crypto-analytics:latest

docker push peterozbot/crypto-analytics:latest
```

### Docker network

```
sudo docker network create crypto-analytics-network
sudo docker network inspect  crypto-analytics-network
sudo docker network connect crypto-analytics-network crypto-analytics-backend
sudo docker network connect crypto-analytics-network crypto-analytics-postgresql
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
sudo docker exec -it crypto-analytics-postgresql bash
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

# SQLx

https://github.com/launchbadge/sqlx/tree/master/sqlx-cli

Install CLI:
```
cargo install sqlx-cli
```

Create/Drop database from .env file:
```
sqlx database create
sqlx database drop
```

New migration:
```
sqlx migrate add <name>
```

Run migration:
```
sqlx migrate run
```

Offline build:
```
cargo sqlx prepare
```
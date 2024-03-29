# Crypto Analytics

Web app that displays analytics about some crypto coins.

Written in Rust with Yew.

### Environment variables

**API_URL**: URL to the backend API<br>
**RELOAD_INTERVAL**: In ms data refresh interval, if 0 no automatic data refresh<br>
**COINGECKO_API_KEY**: API key for coingecko v3 **!!! set the variable before deploy !!!**<br>If value is ```****UPDATE****```then fake data is loaded.<br>
**ETHERSCAN_API_KEY**: API kej for ether scan **!!! set the variable before deploy !!!**<br>
If value is ```****UPDATE****```then fake data is loaded.<br>

# Resources:

Crypto data:
https://www.coingecko.com/api/documentations/v3

# Development
## Frontend
Run frontend:
```
trunk serve
```
Configuration in Trunk.toml.

### State management
https://github.com/intendednull/yewdux

### Example Apps
https://github.com/jetli/rust-yew-realworld-example-app


## Backend
Run and watch for changes:
```
cargo watch -c -w src -x run
```
Host name and port defined in .env.

# Docker

*Log into container with sh*
```
sudo docker exec -it crypto-analytics-frontend /bin/sh
```

*Docker compose*
```
sudo docker-compose up -d
sudo docker-compose stop
```


### Frontend:
```
sudo docker build --build-arg API_URL=http://localhost:1020 --file=frontend.Dockerfile -t crypto-analytics-frontend ./

sudo docker run -d -p 1030:80 -e API_URL='http://localhost:1020' --name crypto-analytics-frontend crypto-analytics-frontend:latest
```

### Backend
```
sudo docker build --file=backend.Dockerfile -t crypto-analytics-backend .

sudo docker run -p 1020:8000 -e DATABASE_URL='postgres://postgres:postgres@db:5432/crypto_analytics' -e SERVER_URL='localhost:8000' --name crypto-analytics-backend crypto-analytics-backend:latest
```

### Docker Hub
Example:
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

*Connect with psql*
```
psql -h localhost -p 5432 -U postgres
psql -h localhost -p 1010 -U postgres
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
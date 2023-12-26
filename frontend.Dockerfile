# This tells docker to use the Rust official image
FROM rust:1.74 as build-step

# update RUST and use nightly version
RUN rustup default nightly
RUN rustup update
RUN rustup toolchain install nightly

# for cross-compiling our code to wasm32 
RUN rustup target add wasm32-unknown-unknown

# Install trunk
RUN cargo install trunk

# Copy the files in your machine to the Docker image
COPY ./frontend ./

# Set enviroment variables
ARG API_URL
ENV API_URL $API_URL

# build app
RUN trunk build

# Instal NGINX
FROM nginx:1.17.1-alpine

# Remove and copy nginx configuration file
RUN rm /etc/nginx/nginx.conf
COPY ./nginx/nginx.conf /etc/nginx/nginx.conf

# deploy app
COPY --from=build-step /dist /usr/share/nginx/html
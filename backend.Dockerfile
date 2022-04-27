
# Tell docker to use the RUST MUSL image
FROM ekidd/rust-musl-builder:stable as build-step

# create empty project and copy toml for optimization purposes
RUN cargo new --bin backend
WORKDIR /home/rust/src/backend
COPY ./backend/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# remove dummy
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/backend*

# Copy the files in your machine to the Docker image
COPY ./backend ./

# build actual app
RUN cargo build --release

# Install alpine image
FROM alpine:latest

# expose the server port
EXPOSE 8000

# copy app
COPY --from=build-step /home/rust/src/backend/target/x86_64-unknown-linux-musl/release/backend  /backend

# run
CMD ["./backend"]
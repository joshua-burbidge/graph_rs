FROM rust:1.82 as builder

WORKDIR /app

COPY ["Cargo.toml", "Cargo.lock", "./"]
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli

COPY src ./src
COPY wasm ./wasm

RUN cargo build --target=wasm32-unknown-unknown --release
RUN wasm-bindgen ./target/wasm32-unknown-unknown/release/graph.wasm --out-dir wasm/generated --target web

FROM nginx:latest
COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /app/wasm /usr/share/nginx/html
EXPOSE 80
CMD [ "nginx", "-g", "daemon off;" ]

# docker build -t my-rust-app .
# docker run -p 8080:80 my-rust-app

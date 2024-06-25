# Needs optimization with multi-stage builds
FROM rust:1.78

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN apt-get update && apt-get install -y libclang-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

CMD ["./target/release/indexers"]
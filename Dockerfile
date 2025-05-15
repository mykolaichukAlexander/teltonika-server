FROM rust

RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo install cargo-deb
RUN cargo build --release
RUN cargo deb

CMD ["cp", "target/debian/*.deb", "/output/"]

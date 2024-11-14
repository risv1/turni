FROM rust:1.82 AS build

COPY . .

RUN cargo build --release

CMD ["./target/release/turni"]
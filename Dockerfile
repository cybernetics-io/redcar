FROM rust:latest

WORKDIR /usr/src/redcar
COPY . .

RUN rustup component add rustfmt
RUN rustup component add clippy

RUN cargo build --release
RUN cp target/release/redcar-server /usr/bin/

EXPOSE 1985 1986

ENTRYPOINT ["redcar-server", "-o  stdout", "-d"]

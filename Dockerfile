FROM rust:1.90

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

CMD ["azarole"]

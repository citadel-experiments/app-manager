# TODO: Use a better image for doing this, but this seems to work
FROM ubuntu:23.04 as build-env
RUN apt update && apt install -y libssl-dev pkg-config build-essential cmake golang-go curl llvm-14 clang-14

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /app
COPY . /app
RUN cargo build --bin app-cli --release --features=cli,umbrel,git

FROM ubuntu:23.04

RUN apt update && apt install -y libssl3 ca-certificates && apt clean && rm -rf /var/lib/apt/lists/*

COPY --from=build-env /app/target/release/app-cli /

CMD ["/app-cli"]

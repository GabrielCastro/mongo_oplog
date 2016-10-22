FROM ubuntu:14.04

## Just rust and what it needs
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y curl sudo build-essential \
 && curl -sSf https://static.rust-lang.org/rustup.sh | sh \
 && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

## Library dependencies
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y libssl-dev \
 && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN cargo
ENV RUST_LOG warn
ENV RUST_TEST_NOCAPTURE 1
ENV RUST_BACKTRACE 1
ENV PATH /root/.cargo/bin:$PATH

WORKDIR /src/

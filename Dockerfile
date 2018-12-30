FROM rust:1.31.1-slim

# Disable prompts from apt.
ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update -y &&\
 apt-get install -y --no-install-recommends curl jq bash && \
 apt-get autoremove -y && \
 apt-get clean -y && \
 rm -rf /tmp/* /var/tmp/* /var/cache/apt/archives/* /var/lib/apt/lists/*

COPY ./Cargo.toml /src/Cargo.toml
COPY ./src /src/src
COPY ./Cargo.lock /src/Cargo.lock
WORKDIR /src

RUN cargo build --release
RUN mkdir bin && mv ./target/release/json_to_yaml ./bin/json_to_yaml
ENV PATH="/src/bin:${PATH}"

CMD ["tail", "-F", "-n0", "/etc/hosts" ]


FROM docker.io/library/rust:1.60 AS BUILD

RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev musl && rm -rf /var/lib/apt/lists/*

ARG TARGETPLATFORM
WORKDIR /build
RUN rustup target install aarch64-unknown-linux-musl x86_64-unknown-linux-musl
RUN mkdir src/ && touch src/lib.rs
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./compile-deps.bash ./compile-deps.bash
RUN ./compile-deps.bash $TARGETPLATFORM
RUN rm src/*.rs
COPY ./build.bash ./build.bash

# copy your source tree
COPY ./src ./src
RUN ./build.bash $TARGETPLATFORM book-list

FROM scratch
WORKDIR /app
COPY --from=BUILD /build/book-list /app/book-list
ENTRYPOINT ["/app/book-list"]

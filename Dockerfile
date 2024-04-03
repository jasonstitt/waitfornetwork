FROM rust:1.77 as builder
WORKDIR /usr/src/waitfornetwork
ARG TARGETPLATFORM
COPY Cargo.lock Cargo.toml build.sh ./
COPY src src
RUN ./build.sh

FROM ubuntu
COPY --from=builder /usr/local/cargo/bin/waitfornetwork /usr/bin/waitfornetwork
ENTRYPOINT ["/usr/bin/waitfornetwork"]
CMD []

FROM rust:alpine as build-image
WORKDIR /build
COPY Cargo.* ./
COPY ./src ./src
RUN cargo build --release

FROM alpine:latest
WORKDIR /project
COPY --from=build-image ./build/target/release/protohacker ./
EXPOSE 8080 5000/udp
CMD [ "./protohacker" ]

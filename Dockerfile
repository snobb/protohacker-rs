FROM rust:slim-bookworm as build-image

ARG TASKDIR
ARG PKG

COPY ./protohacker-lib ./protohacker-lib
WORKDIR ./build
COPY ./${TASKDIR}/Cargo.* ./
COPY ./${TASKDIR}/src ./src
# Change the --bin here to the project in question
RUN cargo build --release --bin ${PKG}

FROM debian:bookworm-slim
ARG PKG
WORKDIR /project
# Change the binary name project in question
COPY --from=build-image ./build/target/release/${PKG} ./prog
EXPOSE 8080 5000/udp
CMD [ "./prog" ]

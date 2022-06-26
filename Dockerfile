ARG BASE_IMAGE=docker.io/library/debian:bullseye-slim

FROM ${BASE_IMAGE} as BASE

ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini

ENV DEBIAN_FRONTEND=noninteractive

USER root

RUN groupadd -g 999 dvop_user && \
    useradd -r -u 999 -g dvop_user dvop_user && \
    mkdir -p /home/dvop_user && \
    chown dvop_user:0 /home/dvop_user &&\
    chmod g=u /home/dvop_user &&\ 
    apt-get update && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

ENV USER=dvop_user
USER 999
WORKDIR /home/dvop_user

FROM rust:1.61-slim-bullseye as BUILDER

RUN USER=root cargo new --bin sr_repository
WORKDIR /sr_repository

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock


# Caching dependencies
RUN cargo build --release &&\
    rm src/*.rs &&\
    rm ./target/release/deps/sr_repository*

# Copyng source code
COPY ./src ./src

RUN cargo build --release

FROM BASE

COPY --from=BUILDER /sr_repository/target/release/sr_repository /home/dvop_user/main
EXPOSE 8080
ENTRYPOINT [ "/tini", "--" ]
CMD [ "./main" ]
USER 999
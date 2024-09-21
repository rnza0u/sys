# syntax=docker/dockerfile:1.4

FROM alpine:3.20

ARG NODE_VERSION="20"
ARG NPM_VERSION="10"
ARG RUSTUP_VERSION="1.25"
ARG RUST_VERSION="nightly"
ARG DOCKER_VERSION="27.2.1"
ARG BUILDX_VERSION="0.17.1"
ARG COMPOSE_VERSION="2.29.4"
ARG CARGO_HOME="/root/.cargo"
ARG BLAZE_VERSION="0.2.16"
ARG BINARIES="target/x86_64-unknown-linux-musl/release"

RUN apk update && \
    apk add alpine-sdk \
    pkgconfig \
    git \
    curl \
    openssh \
    openssl \
    openssl-dev \
    openssl-libs-static \
    perl-utils \
    rustup=~${RUSTUP_VERSION} \
    nodejs=~${NODE_VERSION} \
    npm=~${NPM_VERSION} && \
    rustup-init -y && \
    "$CARGO_HOME/bin/rustup" toolchain install ${RUST_VERSION} \
    --target x86_64-unknown-linux-gnu \
    --target x86_64-unknown-linux-musl \
    --target x86_64-pc-windows-gnu \
    --target x86_64-apple-darwin \
    --target aarch64-apple-darwin && \
    source "$HOME/.cargo/env" && \
    cargo +${RUST_VERSION} install cross --git https://github.com/cross-rs/cross.git && \
    cargo +${RUST_VERSION} install cargo-bump && \
    curl --output /tmp/docker.tgz https://download.docker.com/linux/static/stable/x86_64/docker-${DOCKER_VERSION}.tgz && \
    tar -C /tmp -xzf /tmp/docker.tgz docker/docker && \
    rm /tmp/docker.tgz && \
    mv /tmp/docker/docker /usr/bin/docker && \
    mkdir -p "$HOME/.docker/cli-plugins" && \
    curl --output "$HOME/.docker/cli-plugins/docker-buildx" --location https://github.com/docker/buildx/releases/download/v${BUILDX_VERSION}/buildx-v${BUILDX_VERSION}.linux-amd64 && \
    chmod +x "$HOME/.docker/cli-plugins/docker-buildx" && \
    curl -SL https://github.com/docker/compose/releases/download/v${COMPOSE_VERSION}/docker-compose-linux-x86_64 -o "$HOME/.docker/cli-plugins/docker-compose" && \
    chmod +x "$HOME/.docker/cli-plugins/docker-compose" && \
    cargo +${RUST_VERSION} install -Z bindeps "blaze-cli@${BLAZE_VERSION}"

ENV PATH="$PATH:$CARGO_HOME/bin/"
ENV CROSS_CONTAINER_IN_CONTAINER="true"

COPY --chmod=500 --chown=0:0 ${BINARIES}/create-cache /usr/bin/create-cache
COPY --chmod=500 --chown=0:0 ${BINARIES}/restore-cache /usr/bin/restore-cache
COPY --chmod=400 --chown=0:0 "conf/.npmrc" "/root/.npmrc"

ENTRYPOINT ["/bin/sh"]
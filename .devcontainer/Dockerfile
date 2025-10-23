# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.80
FROM rust:${RUST_VERSION}-bookworm

# Create non-root user similar to VS Code defaults
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      git curl ca-certificates pkg-config build-essential clang lld \
    && rm -rf /var/lib/apt/lists/* && \
    groupadd --gid ${USER_GID} ${USERNAME} && \
    useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME}

USER ${USERNAME}
WORKDIR /workspaces/osc-data-model
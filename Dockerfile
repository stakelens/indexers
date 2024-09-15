# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.81.0
ARG APP_HOME=/app

FROM rust:${RUST_VERSION}-bookworm AS build

ARG APP_HOME
WORKDIR ${APP_HOME}

# Install host build dependencies.
RUN apt-get -qq update && \
    apt-get -qq install -y --no-install-recommends \
    libclang-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* /tmp/*

# Build arguments and variables set for tracelog levels and debug information
#
# We set defaults to all variables.
ARG RUST_LOG
ENV RUST_LOG=${RUST_LOG:-info}

ARG RUST_BACKTRACE
ENV RUST_BACKTRACE=${RUST_BACKTRACE:-1}

ARG RUST_LIB_BACKTRACE
ENV RUST_LIB_BACKTRACE=${RUST_LIB_BACKTRACE:-1}

ENV CARGO_HOME="${APP_HOME}/.cargo/"

# Application specific environment variables.
ARG SQLX_OFFLINE=true
ENV SQLX_OFFLINE=${SQLX_OFFLINE}
ARG ETH_RPC_URL
ENV ETH_RPC_URL=${ETH_RPC_URL}
ARG OPT_RPC_URL
ENV OPT_RPC_URL=${OPT_RPC_URL}

# Build the application leveraging cache mounts to speed up the build process.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=abis,target=abis \
    --mount=type=bind,source=.sqlx,target=.sqlx \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=config.json,target=config.json \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release --bin indexers  && \
cp ./target/release/indexers /bin/indexers

########################################
# Create a new image that only contains the runtime artifacts.
########################################
FROM debian:bookworm AS runtime

ARG APP_HOME
WORKDIR ${APP_HOME}

RUN apt-get -qq update && \
    apt-get -qq install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* /tmp/*

# Create a non-privileged user that the app will run under.
ARG USER=appuser
ARG UID=10001
ARG GID=10001
RUN addgroup --system --gid ${GID} ${USER} \
    && adduser \
    --system \
    --disabled-login \
    --shell /bin/bash \
    --home ${APP_HOME} \
    --uid "${UID}" \
    --gid "${GID}" \
    ${USER}

RUN chown ${UID}:${UID} ${APP_HOME}

USER ${USER}

# Copy the executable from the "build" stage.
COPY --from=build /bin/indexers /usr/local/bin
COPY config.json ./config.json

# Expose the port that the application listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["indexers"]

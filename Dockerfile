# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.81.0
ARG APP_NAME=indexers
ARG APP_HOME=/app

FROM rust:${RUST_VERSION}-bookworm AS build
ARG APP_NAME
ARG APP_HOME
WORKDIR ${APP_HOME}

# Install host build dependencies.
RUN apt-get -qq update && \
    apt-get -qq install -y --no-install-recommends \
    libclang-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* /tmp/*

# Build time environment variables.
ENV CARGO_HOME="${APP_HOME}/.cargo/"
ARG SQLX_OFFLINE=true
ENV SQLX_OFFLINE=${SQLX_OFFLINE}

# Build the application leveraging cache mounts to speed up the build process.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=config.json,target=config.json \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release --bin ${APP_NAME}  && \
cp ./target/release/${APP_NAME} /bin/indexers

########################################
# Create a new image that only contains the runtime artifacts.
########################################
FROM debian:bookworm AS runtime

ARG APP_HOME

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

USER ${USER}

# Copy the executable from the "build" stage.
COPY --from=build /bin/indexers /usr/local/bin

# Expose the port that the application listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["indexers"]

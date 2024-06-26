# syntax=docker/dockerfile:1

# This dockerfile build executable from source code before create the final image
# - use cargo-chef to cache build of dependencies into docker's layer
# - using this dockerfile into ci, do NOT require to provide the executable (steps could be remove, but more time will be needed)

ARG DEVCONTAINER_IMAGE=public.ecr.aws/docker/library/rust:1.78.0
ARG APP_NAME="appname_notset"

# checkov:skip=CKV_DOCKER_7:Ensure the base image uses a non latest version tag
# hadolint ignore=DL3006
FROM ${DEVCONTAINER_IMAGE} AS builder
WORKDIR /workspace

ARG AWS_SESSION_TOKEN \
  AWS_SECRET_ACCESS_KEY \
  AWS_ACCESS_KEY_ID \
  AWS_REGION="eu-central-1" \
  APP_NAME

ENV RUSTC_WRAPPER="sccache" \
  CARGO_INCREMENTAL=false \
  SCCACHE_BUCKET="wefox-ai-wai-sccache-dev" \
  SCCACHE_REGION="eu-central-1" \
  SCCACHE_S3_USE_SSL=true \
  SCCACHE_S3_KEY_PREFIX="$APP_NAME"

# Build application
COPY . .
RUN --mount=type=secret,id=CARGO_REGISTRIES_WEFOX_TOKEN \
  CARGO_REGISTRIES_WEFOX_TOKEN="$(cat /run/secrets/CARGO_REGISTRIES_WEFOX_TOKEN)" && \
  export CARGO_REGISTRIES_WEFOX_TOKEN && \
  sccache --start-server && \
  cargo build --release && \
  sccache --stop-server

# We do not need the Rust toolchain to run the binary!

# To have the digest (instead of tag):
# - docker pull gcr.io/distroless/cc:nonroot | grep digest
# - or go to https://console.cloud.google.com/gcr/images/distroless/global/cc and filter by tags
FROM gcr.io/distroless/cc:nonroot AS ms

USER nonroot

ARG APP_NAME
ARG APP_VERSION="0.0.0"

# An environment variable could also be seen from inside the container as well during run.
ENV APP_NAME="$APP_NAME"
ENV APP_VERSION="$APP_VERSION"
ENV OTEL_SERVICE_NAME="$APP_NAME"

COPY --from=builder /workspace/target/release/my_rusty_sandbox /app

ENV APP_PORT=8080
EXPOSE $APP_PORT
HEALTHCHECK NONE

ENTRYPOINT ["/app"]

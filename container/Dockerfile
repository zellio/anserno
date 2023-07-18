# syntax=docker/dockerfile:1.6.0-labs -*- dockerfile-image-name: "anserno" -*-

### Dockerfile --

## Copyright (C) 2023 Zachary Elliott <contact(at)zell.io>

### Commentary:

##

### Code:

ARG debian_version="slim-bookworm"
ARG rust_version="1.72.1"


FROM rust:${rust_version}-${debian_version} AS chef
RUN cargo install cargo-chef
WORKDIR /chef


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder

ENV \
    DEBIAN_FRONTEND='noninteractive' \
    TERM='xterm'

COPY --from=planner /chef/recipe.json recipe.json

RUN apt-get update && apt-get install --yes libssl-dev pkg-config ca-certificates libssl3

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=/chef/target/build \
    --mount=type=cache,target=/chef/target/deps \
    --mount=type=cache,target=/chef/target/examples \
    --mount=type=cache,target=/chef/target/incremental \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --bin anserno-server --features search

RUN \
    install -D --mode=0555 /chef/target/release/anserno-server /usr/sbin/anserno-server


RUN /bin/bash <<'EOF'
set -eu -o pipefail

declare -a bins=(
    'anserno-server'
)

declare -a files=(
    '/etc/nsswitch.conf'
    '/etc/ssl/certs/ca-certificates.crt'
    '/etc/timezone'
    '/usr/share/zoneinfo/Etc/UTC'
)

declare bin
declare -a bin_libs=()
for bin in "${bins[@]}"; do
    bin="$(command -v "$bin")"
    readarray -t -d$'\n' bin_libs < <(ldd "$bin" | grep -o '/[^ ]\+')
    files+=("$bin" "${bin_libs[@]}")
done

declare file real_path mode
for file in "${files[@]}"; do
    real_path="$(readlink --canonicalize "$file")"
    mode="$(stat --format='%a' "$real_path")"
    test "${#mode}" -lt 4 && mode="0${mode}"
    install --verbose --mode="$mode" -D "$file" /image-root"$file"
done

EOF


FROM scratch AS image

ADD container/root /

COPY --from=builder /image-root/ /
COPY ./templates/ /templates/
COPY ./static/ /static/

ENV \
    ANSERNO_LOG_LEVEL='info' \
    ANSERNO_LOG_FORMAT='json' \
    ANSERNO_HOST='0.0.0.0' \
    ANSERNO_PORT='8080' \
    ANSERNO_TEMPLATES_DIRECTORY='/templates/**/*.html' \
    ANSERNO_STATIC_FILES_DIRECTORY='/static' \
    TMPDIR='/'

ENTRYPOINT ["/usr/sbin/anserno-server"]

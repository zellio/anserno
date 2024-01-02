# syntax=docker/dockerfile:1.6.0-labs -*- dockerfile-image-name: "anserno" -*-

### Dockerfile --

## Copyright (C) 2023-2024 Zachary Elliott <contact(at)zell.io>

### Commentary:

##

### Code:

ARG debian_version=bookworm
ARG rust_version=1.75.0
ARG gosu_version=1.17


FROM rust:${rust_version}-${debian_version} AS chef
RUN cargo install cargo-chef
WORKDIR /chef


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS anserno-source

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
    cargo build --release --bin anserno-server

RUN install -D --mode=0555 /chef/target/release/anserno-server /usr/sbin/anserno-server

RUN /usr/bin/bash <<'EOF'
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

readarray -t -d$'\n' files < <(printf '%s\n' "${files[@]}" | sort | uniq)

declare file real_path mode
for file in "${files[@]}"; do
    real_path="$(readlink --canonicalize "$file")"
    mode="$(stat --format='%a' "$real_path")"
    test "${#mode}" -lt 4 && mode="0${mode}"
    install --verbose --mode="$mode" -D "$file" /image-root"$file"
done

mkdir -p /image-root/var/lib/anserno /image-root/usr/lib/anserno
chown 65533:65533 /image-root/var/lib/anserno
EOF


## Gosu Source

FROM debian:${debian_version}-slim AS gosu-source

RUN apt-get update

RUN apt-get install --yes curl gnupg

ARG TARGETARCH
ARG gosu_version

ENV \
    TARGETARCH="$TARGETARCH" \
    gosu_version="$gosu_version"

RUN /usr/bin/bash <<'EOF'
set -eu -o pipefail -x

declare gosu_arch=unknown

case "$TARGETARCH" in
    'arm64'|'aarch64')
        gosu_arch='arm64'
        ;;

    'amd64'|'x86_64')
        gosu_arch='amd64'
        ;;
esac

curl \
    --proto '=https' --tlsv1.2 --silent --show-error --location --fail \
    --header 'accept: application/octet-stream' \
    --url "https://github.com/tianon/gosu/releases/download/${gosu_version}/gosu-${gosu_arch}" \
    --output /gosu

curl \
    --proto '=https' --tlsv1.2 --silent --show-error --location --fail \
    --header 'accept: application/octet-stream' \
    --url "https://github.com/tianon/gosu/releases/download/${gosu_version}/gosu-${gosu_arch}.asc" \
    --output /gosu.asc

gpg --recv-keys B42F6819007F00F88E364FD4036A9C25BF357DD4
gpg --verify gosu.asc

install --verbose --mode=0555 -D /gosu /image-root/usr/sbin/gosu
EOF


FROM scratch AS image

ADD ./container/root /

COPY ./templates/ /usr/lib/anserno/templates
COPY ./static/ /usr/lib/anserno/static

COPY --from=gosu-source /image-root /
COPY --from=anserno-source /image-root /

ENV \
    ANSERNO_LOG_LEVEL='info' \
    ANSERNO_LOG_FORMAT='json' \
    ANSERNO_HOST='0.0.0.0' \
    ANSERNO_PORT='8080' \
    ANSERNO_TEMPLATES_GLOB='/usr/lib/anserno/templates/**/*.html' \
    ANSERNO_STATIC_FILES_PATH='/usr/lib/anserno/static' \
    TMPDIR='/var/lib/anserno'

ENTRYPOINT ["/usr/sbin/gosu", "anserno:anserno", "/usr/sbin/anserno-server"]

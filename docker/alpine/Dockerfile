FROM rust:1.91-alpine AS builder

RUN apk --no-cache add alpine-sdk build-base openssl-dev

ARG BUILDDIR=/app
WORKDIR ${BUILDDIR}
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=${BUILDDIR}/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
<<EOF
set -e
RUSTFLAGS="-C target-feature=-crt-static" cargo build --locked --release
cp ./target/release/azarole /usr/local/bin/
EOF

FROM rust:1.91-alpine

RUN apk --no-cache add ca-certificates

COPY --from=builder /usr/local/bin/azarole /usr/local/bin/azarole
CMD ["azarole"]

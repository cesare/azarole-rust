FROM amazonlinux:2023

RUN dnf install -y rust cargo gcc openssl-devel \
    && dnf clean all

ARG BUILDDIR=/app
WORKDIR ${BUILDDIR}
WORKDIR ${BUILDDIR}
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=${BUILDDIR}/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
<<EOF
set -e
cargo build --locked --release
cp ./target/release/azarole /usr/local/bin/
EOF

CMD ["azarole"]

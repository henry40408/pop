.PHONY: all amd64 arm64 clean

DIGEST=$(shell git rev-parse --short HEAD)
REGISTRY?=registry-1.docker.io

all: amd64 arm64 manifest

amd64: target/x86_64-unknown-linux-musl/release/pops
	DOCKER_BUILDKIT=1 docker build -t ${REGISTRY}/henry40408/pops:${DIGEST}-amd64 .
	docker push ${REGISTRY}/henry40408/pops:${DIGEST}-amd64

arm64: target/armv7-unknown-linux-musleabihf/release/pops
	DOCKER_BUILDKIT=1 docker build -t ${REGISTRY}/henry40408/pops:${DIGEST}-arm64 -f Dockerfile.arm64 .
	docker push ${REGISTRY}/henry40408/pops:${DIGEST}-arm64

manifest: amd64 arm64
	docker manifest create --amend ${REGISTRY}/henry40408/pops:${DIGEST} ${REGISTRY}/henry40408/pops:${DIGEST}-amd64 ${REGISTRY}/henry40408/pops:${DIGEST}-arm64
	docker manifest annotate ${REGISTRY}/henry40408/pops:${DIGEST} ${REGISTRY}/henry40408/pops:${DIGEST}-arm64 --os linux --arch arm64
	docker manifest annotate ${REGISTRY}/henry40408/pops:${DIGEST} ${REGISTRY}/henry40408/pops:${DIGEST}-amd64 --os linux --arch amd64
	docker manifest push ${REGISTRY}/henry40408/pops:${DIGEST}

clean:
	cargo clean

target/x86_64-unknown-linux-musl/release/pops: cli server Cargo.toml Cargo.lock Cross.toml
	RUSTFLAGS="-C link-arg=-s" cross build --release --target x86_64-unknown-linux-musl

target/armv7-unknown-linux-musleabihf/release/pops: cli server Cargo.toml Cargo.lock Cross.toml
	RUSTFLAGS="-C link-arg=-s" cross build --release --target armv7-unknown-linux-musleabihf

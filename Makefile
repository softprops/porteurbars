default: build

build-static:
	docker run --rm -it -v "$(shell pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release

build:
	@cargo build
.PHONY: build

clean:
	@cargo clean
.PHONY: clean

build:
	docker build -t aes_rust .

run: build
	docker run aes_rust

_dev_build:
	docker build -t aes_rust_dev -f Dockerfile-dev .

test: _dev_build
	docker run aes_rust_dev
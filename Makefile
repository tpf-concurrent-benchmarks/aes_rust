N_THREADS = 4

.PHONY: init build remove deploy run _dev_build test

init:
	docker swarm init || true

build:
	docker rmi aes_rust -f || true
	docker build -t aes_rust .

remove:
	if docker stack ls | grep -q aes_rust; then \
            docker stack rm aes_rust; \
	fi

deploy: remove build
	mkdir -p graphite
	until N_THREADS=$(N_THREADS) \
	docker stack deploy \
	-c docker/docker-compose.yaml \
	aes_rust; \
	do sleep 1; \
	done

run: build
	docker run aes_rust

_dev_build:
	docker build -t aes_rust_dev -f Dockerfile-dev .

test: _dev_build
	docker run aes_rust_dev
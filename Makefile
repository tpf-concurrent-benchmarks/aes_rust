N_THREADS = 4

.PHONY: init build remove deploy run _dev_build test

init:
	docker swarm init || true

_common_folders:
	mkdir -p configs/graphite
	mkdir -p configs/grafana_config
.PHONY: _common_folders

setup: _common_folders

build:
	docker rmi aes_rust -f || true
	docker build -t aes_rust .

_dev_build:
	docker rmi aes_rust_dev -f || true
	docker build -t aes_rust_dev -f Dockerfile-dev .

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

dev_deploy: remove _dev_build
	mkdir -p graphite
	until N_THREADS=$(N_THREADS) \
	docker stack deploy \
	-c docker/docker-compose-dev.yaml \
	aes_rust; \
	do sleep 1; \
	done

run: build
	docker run aes_rust

_tests_build:
	docker build -t aes_rust_dev -f Dockerfile-tests .

test: _tests_build
	docker run aes_rust_dev
.PHONY: init _common_folders _create_env setup build remove deploy dev_deploy run _tests_build test

init:
	docker swarm init || true

_common_folders:
	mkdir -p graphite
.PHONY: _common_folders

_create_env:
	if [ ! -f .env ]; then \
		cp .env.example .env; \
	fi

dummy_file:
	mkdir -p data
	echo "Hello World!" > data/input.txt

setup: init _create_env _common_folders

build:
	docker rmi aes_rust -f || true
	docker build -t aes_rust -f docker/Dockerfile .

_dev_build:
	docker rmi aes_rust_dev -f || true
	docker build -t aes_rust_dev -f docker/Dockerfile-dev .

remove:
	if docker stack ls | grep -q aes_rust; then \
            docker stack rm aes_rust; \
	fi

deploy: remove build
	until \
	docker stack deploy \
	-c docker/docker-compose.yaml \
	aes_rust; \
	do sleep 1; \
	done

dev_deploy: remove _dev_build
	until \
	docker stack deploy \
	-c docker/docker-compose-dev.yaml \
	aes_rust; \
	do sleep 1; \
	done

run: build
	docker run -v "$(PWD)/.env:/opt/app/.env:ro" -v "$(PWD)/data:/opt/app/data" aes_rust

_tests_build:
	docker build -t aes_rust_dev -f docker/Dockerfile-tests .

test: _tests_build
	docker run aes_rust_dev
all: build run


build:
	./scripts/build.sh

check:
	./scripts/check.sh

run:
	./scripts/run.sh

deploy:
	docker build -t afsec/pod-inspector .
	docker run --name pod-inspector -d -p 8081:8081 afsec/pod-inspector
	docker logs -f pod-inspector

clean:
	rm -rf ./dist
	cargo clean

undeploy:
	docker rm -f pod-inspector
	docker rmi afsec/pod-inspector:latest

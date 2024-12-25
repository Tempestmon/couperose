messenger_build:
	docker build -t tempestmon/messenger-0.0.1 -f dockerfile.messenger .

messenger_run:
	docker run --rm -it -p 50051:50051 tempestmon/messenger-0.0.1

api_build:
	docker build -t tempestmon/api-0.0.1 -f dockerfile.api .

api_run:
	docker run --rm -it -p 8080:8080 tempestmon/api-0.0.1

svelte_build:
	docker build -t tempestmon/svelte-0.0.1 -f dockerfile.frontend .

svelte_run:
	docker run --rm -p 80:80 tempestmon/svelte-0.0.1

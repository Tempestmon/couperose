messenger_build:
	docker build --platform=linux/amd64 -t messenger -f Dockerfile.messenger .

messenger_start:
	docker run --rm --platform=linux/amd64 -p 50051:50051 messenger

api_build:
	docker build -t api -f Dockerfile.api .

api_start:
	docker run --rm -p 8080:8080 api

svelte_build:
	docker build -t svelte -f Dockerfile.frontend .

svelte_start:
	docker run --rm -p 80:80 svelte

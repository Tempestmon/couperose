build_messenger:
	docker build --platform=linux/amd64 -t messenger -f Dockerfile.messenger .

messenger_start:
	docker run --rm --platform=linux/amd64 -p 50051:50051 messenger

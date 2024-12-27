COMPOSE_FILE = docker/docker-compose.yml
TENANT = tempestmon
API_VERSION = 0.0.1
MESSENGER_VERSION = 0.0.1
FRONTEND_VERSION = 0.0.2


messenger_build:
	docker build -t $(TENANT)/messenger:$(MESSENGER_VERSION) -f docker/Dockerfile.messenger .

messenger_run:
	docker run --rm -it -p 50051:50051 $(TENANT)/messenger:$(MESSENGER_VERSION)

api_build:
	docker build -t $(TENANT)/api:$(API_VERSION) -f docker/Dockerfile.api .

api_run:
	docker run --rm -it -p 8080:8080 $(TENANT)/api:$(API_VERSION)

svelte_build:
	docker build -t $(TENANT)/svelte:$(FRONTEND_VERSION) -f docker/Dockerfile.frontend .

svelte_run:
	docker run --rm -p 80:80 $(TENANT)/svelte:$(FRONTEND_VERSION)

up:
	docker-compose -f $(COMPOSE_FILE) up -d

build:
	docker-compose -f $(COMPOSE_FILE) up --build -d

down:
	docker-compose -f $(COMPOSE_FILE) down

logs:
	docker-compose -f $(COMPOSE_FILE) logs -f

ps:
	docker-compose -f $(COMPOSE_FILE) ps

COMPOSE_FILE = docker/docker-compose.yml
TENANT = tempestmon
API_VERSION = 0.0.2
MESSENGER_VERSION = 0.0.1
FRONTEND_VERSION = 0.0.2

# MESSENGER SECTION
messenger_build:
	docker build -t $(TENANT)/messenger:$(MESSENGER_VERSION) -f docker/Dockerfile.messenger .

messenger_tag:
	docker tag $(TENANT)/messenger:$(MESSENGER_VERSION) $(TENANT)/messenger:latest

messenger_push: 
	docker push $(TENANT)/messenger:$(MESSENGER_VERSION) && docker push $(TENANT)/messenger:latest

messenger_run:
	docker run --rm -it -p 50051:50051 $(TENANT)/messenger:$(MESSENGER_VERSION)

# API SECTION
api_build:
	docker build -t $(TENANT)/api:$(API_VERSION) -f docker/Dockerfile.api .

api_tag:
	docker tag $(TENANT)/api:$(API_VERSION) $(TENANT)/api:latest

api_push: 
	docker push $(TENANT)/api:$(API_VERSION) && docker push $(TENANT)/api:latest

api_run:
	docker run --rm -it -p 8080:8080 $(TENANT)/api:$(API_VERSION)

# FRONTEND SECTION
svelte_build:
	docker build -t $(TENANT)/svelte:$(FRONTEND_VERSION) -f docker/Dockerfile.frontend .

svelte_tag:
	docker tag $(TENANT)/svelte:$(FRONTEND_VERSION) $(TENANT)/svelte:latest

svelte_push: 
	docker push $(TENANT)/svelte:$(FRONTEND_VERSION) && docker push $(TENANT)/svelte:latest

svelte_run:
	docker run --rm -p 80:80 $(TENANT)/svelte:$(FRONTEND_VERSION)

# COMPOSE SECTION
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

COMPOSE_FILE = docker/docker-compose.yml
TENANT = tempestmon
API_VERSION = 0.1.2
MESSENGER_VERSION = 0.1.1
FRONTEND_VERSION = 0.1.5
PROMETHEUS_VERSION = "v3.1.0"

# MESSENGER SECTION
akagi_build:
	docker build -t $(TENANT)/akagi:$(MESSENGER_VERSION) -f docker/Dockerfile.akagi .

akagi_tag:
	docker tag $(TENANT)/akagi:$(MESSENGER_VERSION) $(TENANT)/akagi:latest

akagi_push: 
	docker push $(TENANT)/akagi:$(MESSENGER_VERSION) && docker push $(TENANT)/akagi:latest

akagi_run:
	docker run --rm -it -p 50051:50051 $(TENANT)/akagi:$(MESSENGER_VERSION)

# API SECTION
kaga_build:
	docker build -t $(TENANT)/kaga:$(API_VERSION) -f docker/Dockerfile.kaga .

kaga_tag:
	docker tag $(TENANT)/kaga:$(API_VERSION) $(TENANT)/kaga:latest

kaga_push: 
	docker push $(TENANT)/kaga:$(API_VERSION) && docker push $(TENANT)/kaga:latest

kaga_run:
	docker run --rm -it -p 8080:8080 $(TENANT)/kaga:$(API_VERSION)

# FRONTEND SECTION
hiryu_build:
	docker build -t $(TENANT)/hiryu:$(FRONTEND_VERSION) -f docker/Dockerfile.hiryu .

hiryu_tag:
	docker tag $(TENANT)/hiryu:$(FRONTEND_VERSION) $(TENANT)/hiryu:latest

hiryu_push: 
	docker push $(TENANT)/hiryu:$(FRONTEND_VERSION) && docker push $(TENANT)/hiryu:latest

hiryu_run:
	docker run --rm -p 80:80 $(TENANT)/hiryu:$(FRONTEND_VERSION)

# PROMETHEUS SECTION
prometheus_run:
	docker run --rm -it -p 9090:9090 \
		-v $(PWD)/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml \
		--name prometheus prom/prometheus:$(PROMETHEUS_VERSION)

prometheus_logs:
	docker logs prometheus

prometheus_stop:
	docker stop prometheus

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

clean:
	docker-compose -f $(COMPOSE_FILE) down --rmi all --volumes --remove-orphans

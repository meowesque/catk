# Docker image name and tag
IMAGE_NAME = catk
TAG = latest

# Docker build command
.PHONY: build
build:
	docker build -t $(IMAGE_NAME):$(TAG) .

# Docker run command
.PHONY: run
run:
	docker run -it --rm $(IMAGE_NAME):$(TAG)

# Clean up docker images
.PHONY: clean
clean:
	docker rmi $(IMAGE_NAME):$(TAG)

# Build and run in one command
.PHONY: all
all: build run
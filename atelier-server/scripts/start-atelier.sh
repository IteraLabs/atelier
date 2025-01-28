#!/bin/bash

# -- Start Kafka, ZooKeeper and PostgreSQL Instance using Docker Compose

# Simple version
# docker-compose up -d

# Folder-file search version
# Get the directory of the current script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Navigate to the root directory
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Specify the path to the docker-compose.yml file
DOCKER_COMPOSE_FILE="$ROOT_DIR/docker/docker-compose.yml"

# Run docker-compose up -d with the specified file
docker-compose -f "$DOCKER_COMPOSE_FILE" up -d

# -- Wait for resource availability -- #

# Wait for Kafka to be ready
echo "Waiting for Zookeper to be ready..."
sleep 5

# Wait for Kafka to be ready
echo "Waiting for Kafka to be ready..."
sleep 20

echo "Stack is now active and running"


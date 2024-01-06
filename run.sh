#!/bin/bash

# Stop and remove existing containers (if any)
docker stop zmq-controller zmq-node
docker rm zmq-controller zmq-node

# Build the Docker image (if not already built or if there are changes)
docker build . -t pubsub

# Run the controller container in the background
docker run -d --name zmq-controller -e MODE=controller pubsub

# Run the node container in the foreground to see the message passing
# Assuming the node container needs to connect to the controller at port 5555
docker run --name zmq-node -e MODE=node -p 5555:5555 pubsub

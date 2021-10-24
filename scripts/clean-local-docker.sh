#!/usr/bin/env bash

# no `set -e` here as we allow commands to fail in this script

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/docker/generated-dev"

docker images

echo "stop and remove docker containers..."
docker-compose rm -f -s -v

echo "remove docker volumes..."
docker volume ls | grep generated-dev | sed 's/local *//' | xargs docker volume rm

echo "remove dangling docker images if any..."
IMG=$(docker images --filter=dangling=true -q)
[ -z "$IMG" ] || docker rmi -f $IMG

echo "keep litentry/litentry-parachain:latest while removing other tags..."
IMG=$(docker images litentry/litentry-parachain --format "{{.Repository}}:{{.Tag}}" | grep -v latest)
[ -z "$IMG" ] || docker rmi -f $IMG

echo "remove generated images..."
IMG=$(docker images --filter=reference='generated-dev*' --format "{{.Repository}}:{{.Tag}}")
[ -z "$IMG" ] || docker rmi -f $IMG

echo "cleaned up."
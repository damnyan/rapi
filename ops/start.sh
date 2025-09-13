#!/bin/bash
set -e

cd "$(dirname "$0")"

ENV_FILE="../project/.env"
if [ ! -f "$ENV_FILE" ]; then
  echo ".env file not found at $ENV_FILE"
  exit 1
fi
set -a
source "$ENV_FILE"
set +a

COMPOSE_PROJECT_NAME=${COMPOSE_PROJECT_NAME:-dmn}
PROFILE=${APP_ENV:-development}
COMPOSE_FILE="docker-compose.yml"

function usage() {
  echo "Usage: $0 [--build] [--detach] [--recreate|--no-recreate|--force-recreate|--restart] [--clean]"
  exit 1
}


BUILD=""
DETACH="-d" # Default to detached mode
RECREATE=""
CLEAN=""

while [[ $# -gt 0 ]]; do
  case $1 in
    --build)
      BUILD="--build"
      ;;
    --detach)
        DETACH="-d" # Explicitly set, but default is already detached
      ;;
    --no-detach)
        DETACH="" # Allow user to override and run in foreground
      ;;
    --recreate)
      RECREATE="--renew-anon-volumes --force-recreate"
      ;;
    --no-recreate)
      RECREATE="--no-recreate"
      ;;
    --force-recreate)
      RECREATE="--force-recreate"
      ;;
    --restart)
      RECREATE="--force-recreate"
      ;;
    --clean)
      CLEAN=1
      ;;
    *)
      usage
      ;;
  esac
  shift
done

if [ "$CLEAN" = "1" ]; then
  echo "Stopping and removing containers, images, volumes, and networks..."
  docker compose -p "$COMPOSE_PROJECT_NAME" -f "$COMPOSE_FILE" down -v --rmi all --remove-orphans
  docker volume prune -f
  docker network prune -f
  echo "Cleaned up. Rebuilding..."
  docker compose -p "$COMPOSE_PROJECT_NAME" -f "$COMPOSE_FILE" --profile "$PROFILE" up --build $DETACH $RECREATE
  exit $?
fi

docker compose -p "$COMPOSE_PROJECT_NAME" -f "$COMPOSE_FILE" --profile "$PROFILE" up $BUILD $DETACH $RECREATE

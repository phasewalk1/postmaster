#!/bin/bash

if [[ " $@ " =~ " --prod " ]]; then
  echo "Running in production mode --> (ensure the DATABASE_URL env var is pointed at the production data store)"
  source scripts/environ.sh --prod
else
  echo "Running in development mode"
  source scripts/environ.sh --dev
fi

trap "kill 0" SIGINT

cd $APP_DIR
cargo run --bin http-server &
cargo run --bin grpc-service &

wait

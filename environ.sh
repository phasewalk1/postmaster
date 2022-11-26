# !/bin/bash

export DATABASE_URL="postgres://localhost:5432/your_database_name"
export RUST_LOG="debug"
export RUST_BACKTRACE=1

export TONIC_PORT="50051"
export TONIC_HOST="localhost"

export ROCKET_PORT="8000"
export ROCKET_HOST="localhost"

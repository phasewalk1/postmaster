# !/bin/bash

if [[ " $@ " =~ " --prod " ]]; then
  :
else
  export DATABASE_URL="postgresql://postgres:example@localhost:5432/postgres"
fi

export RUST_LOG="debug"
export RUST_BACKTRACE=1

export TONIC_PORT="50051"
export TONIC_HOST="localhost"

export ROCKET_PORT="8000"
export ROCKET_HOST="localhost"

export APP_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

source scripts/environ.sh

cd $APP_DIR
cargo run --bin http-server &
cargo run --bin grpc-service

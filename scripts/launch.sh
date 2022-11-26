source scripts/environ.sh

cd $APP_DIR
cargo run --release --bin http-server -- --rocket-host $ROCKET_HOST --rocket-port $ROCKET_PORT --tonic-host $TONIC_HOST --tonic-port $TONIC_PORT
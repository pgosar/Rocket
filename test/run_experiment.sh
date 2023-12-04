cd ../socket-server
cargo build
cd ../socket-client
cargo build
cd ../test

./run_experiment.py 8 5 5 1 50
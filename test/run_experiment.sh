cd ../socket-server
cargo build --quiet 
cd ../socket-client
cargo build --quiet 
cd ../test

rm commands.txt; touch commands.txt

./generate-client-commands.py

./run_experiment.py 8
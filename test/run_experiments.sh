cd ../socket-server
cargo build --quiet 
cd ../socket-client
cargo build --quiet 
cd ../test

rm logs/*.tmp
rm commands.txt; touch commands.txt

./generate_client_commands.py

LOG_FILE=./logs/experiment-$1-threads.txt
rm $LOG_FILE
echo "threads,num_clients,repeats,out_degree,sleep_mean,total_time" >> $LOG_FILE

./run_experiments.py $1
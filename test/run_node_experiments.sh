rm logs/*.tmp
rm node_commands.txt; touch node_commands.txt
killall -9 node
./generate_node_client_commands.py

LOG_FILE=./logs/node-experiment-$1.txt
rm $LOG_FILE
echo "num_clients,repeats,out_degree,sleep_mean,total_time" >> $LOG_FILE

./run_node_experiments.py $1
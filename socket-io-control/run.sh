#!/bin/bash
cd server
node index.js &
SERVER_PID=$!

cd ../client
CLIENT_PIDS=()
for i in {1..5} 
do
    node index.js $i 5 2 5 50  &
    CLIENT_PIDS+=($!)
done


for pid in ${allThreads[@]};
do
    wait $pid
done
echo "All done"
kill -9 $SERVER_PID

### WebSocket server

Custom WebSocket server that implements the protocol from scratch. To run the server use the binary or
cd into `socket-server` and use ```cargo run -- -t <threads>```

Optionally use the `-d` flag to turn on debug mode.

To run the test client, cd into `socket-client` and 
use ```cargo run -- -i <specified ID> -r <number of messages> -n <number of other clients> -o <number of recipients> -s <sleep time between messages> -f <output file for timing> -m <message length in characters>```.

To get more generic client socket functionality, add `clientsocket.rs` and `utils.rs` to your client of choice. 

To run our experiments, go into `test`, modify `generate-client-commands.py` as you please, and run `./run_experiments.sh` 

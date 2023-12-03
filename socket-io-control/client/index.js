
const io = require("socket.io-client");


const id = parseInt(process.argv[2])
const repeats = parseInt(process.argv[3])
const outDegree = parseInt(process.argv[4]) 
const numClients = parseInt(process.argv[5])
const messageDelay = parseInt(process.argv[6])


const socket = io("http://localhost:5002", {
  transports: ["websocket"],
  query: {id}
});

async function sleep(millis) {
  return new Promise(resolve => setTimeout(resolve, millis));
}

socket.on("connect", async () => {
  console.log(`client ${id} knows it is connected`)
  let $ = await sleep(2000);
  for (let i = 0; i < repeats; i++) {
    const recipients = [(id + 1) % numClients, (id + numClients - 1) % numClients]
    console.log(recipients)
    socket.emit("message", {
      recipients,
      message: "Hi guys"
    })
    $ = await sleep(messageDelay);
  }
});

socket.on("message", (data) => {
  console.log(`client ${id} received message ${data.message}`)
})




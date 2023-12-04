
const io = require("socket.io-client");



const id = parseInt(process.argv[7])
const repeats = parseInt(process.argv[2])
const outDegree = parseInt(process.argv[3]) 
const numClients = parseInt(process.argv[4])
const messageDelay = parseInt(process.argv[5])
const mesageLength = parseInt(process.argv[6])

async function sleep(millis) {
  return new Promise(resolve => setTimeout(resolve, millis));
}

let mymessage = '';
const alphabet = 'abcdefghijklmnopqrstuvwxyz';
for (let counter = 0; counter < mesageLength; counter++) {
  mymessage += alphabet.charAt(Math.floor(Math.random() * alphabet.length));
}

function pickRecipients() {
  let recipients = [];
  while (recipients.length < outDegree) {
      let r = Math.floor(Math.random() * numClients);
      if (recipients.indexOf(r) === -1) { // could also exclude yourself
        recipients.push(r);
      }
  }
  return recipients
}

let mcounter = 0;

const socket = io("http://localhost:5002", {
  transports: ["websocket"],
  query: {id}
});

socket.on("message", (data) => {
  mcounter++
})

socket.on("connect", async () => {
  let start = process.hrtime();
  let $ = await sleep(2000);
  for (let i = 0; i < repeats; i++) {
    socket.emit("message", {
      recipients: pickRecipients(),
      message: mymessage
    })
    $ = await sleep(messageDelay);
  }
  socket.disconnect()
  let elapsed = process.hrtime(start)[1] / 1000000;
  console.log(`${numClients},${repeats},${outDegree},${messageDelay},${elapsed}`)

});






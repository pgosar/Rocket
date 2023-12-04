//const { Server } = require("socket.io");
const express = require("express");
const http = require("http");

const app = express();
const server = http.createServer(app);

app.get("/", (req, res) => {
  res.status(200).send(`Status OK: ${new Date()}`);
});

const io = require("socket.io")(server, {
  /*cors: {
    origin: [process.env.CORS_ORIGIN_URL],    
    credentials: true,
  },*/
});

/*io.use((socket, next) => {
  if (socket.handshake.query?.id) {
    console.log(socket.handshake.query.id)

    socket.data.customId = socket.handshake.query.id
  }
  next()
})*/

io.on("connection", (socket) => {
  if (socket.handshake.query?.id) {
    console.log(socket.handshake.query.id)

    socket.data.customId = socket.handshake.query.id
    socket.join(socket.handshake.query.id)
    console.log(`server knows client ${socket.handshake.query.id} is connected`)
  
  }

  socket.on("message", (data) => {
    console.log("message")
    console.log(data.recipients)
    for (const id of data.recipients) {
      socket.to(String(id)).emit("message", {message: data.message})
    }
  })
})

const port = 5002;

server.listen(port, () =>
  console.log(`CHAT server up and running on port ${port} !`)
);

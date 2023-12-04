//const { Server } = require("socket.io");
const express = require("express");
const http = require("http");

const app = express();
const server = http.createServer(app);

app.get("/", (req, res) => {
  res.status(200).send(`Status OK: ${new Date()}`);
});

const io = require("socket.io")(server, {
});


io.on("connection", (socket) => {
  if (socket.handshake.query?.id) {
    socket.data.customId = socket.handshake.query.id
    socket.join(socket.handshake.query.id)  
  }

  socket.on("message", (data) => {
    for (const id of data.recipients) {
      socket.to(String(id)).emit("message", {message: data.message})
    }
  })
})

const port = 5002;

server.listen(port, () =>
  console.log(`CHAT server up and running on port ${port} !`)
);

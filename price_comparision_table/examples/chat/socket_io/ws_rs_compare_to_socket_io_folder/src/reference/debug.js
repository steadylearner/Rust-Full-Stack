const socket = new WebSocket("ws://" + window.location.host + "/ws");

const form = document.getElementById("form");
form.addEventListener('submit', function (event) {
  event.preventDefault();
  const input = document.getElementById("msg");
  if (input.value === "!stop") {
    alert("You stopped entire chat server.")
  }
  socket.send(input.value);
  input.value = "";
});

socket.addEventListener('open', function (event) {
  socket.send('Someone entered the chat.');
});

socket.onmessage = function (event) {
  console.log(`${event.data} from ${event.origin}`);
  const messages = document.getElementById("messages");
  const li = document.createElement("li");
  li.append(event.data)
  messages.append(li);
};

socket.onclose = function(event) {
  const messages = document.getElementById("messages");
  const li = document.createElement("li");
  li.append("The chat server stopped. It won't work anymore");
  messages.append(li);
  console.log("Chat stopped");
};
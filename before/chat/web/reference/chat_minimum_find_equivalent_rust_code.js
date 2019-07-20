// 1. Socket connection with Yew
// 2. Open
// 3. Send message and received message
// 4. Close
// 5. Error

const socket = new WebSocket("ws://127.0.0.1:7777/ws");

// 1. custom function to log time and remove messages(remove list under ul)

// Use Rust crate and function intead

function removeMessages() {
  const messages = document.getElementById("messages");
  while (messages.firstChild) {
    messages.removeChild(messages.firstChild);
  }
}

// 2. state for each client(user)

let open = false;

let userId = "";

// 3. Log conosle message when socket is open

socket.addEventListener('open', function (event) {
  console.log("Start to chat");
});

// 4. Clear the message, eqaul to !clear later

const clear = document.getElementById("clear");
clear.onclick = removeMessages;

// 5. Exit the chat, eqaul to !exit later

const exit = document.getElementById("exit");
exit.onclick = function () {
  socket.close();
}

// 6. id - form, directly relevant to who type message

const form = document.getElementById("form");

form.onsubmit = function (event) {
  event.preventDefault();
  const input = document.getElementById("msg");

  if (input.value === "") {
    return;
  }

  if (input.value === "!clear") {
    removeMessages()
    input.value = "";
    return;
  }

  if (input.value === "!exit") {
    socket.close();
    return;
  }

  //

  socket.send(`${userId}: ${input.value}`);
  input.value = "";

  setTimeout(() => window.scrollTo({ top: window.innerHeight, behavior: "auto" }), 10);
};

// 7. From Server(User, Other user or Server)

socket.onmessage = function (event) {
  if (!open) {
    // to give id to user and verify the maximum number, only work once
    let separate = event.data.split(" ");
    userId = separate[0];

    const messages = document.getElementById("messages");
    const li = document.createElement("li");
    const p = document.createElement("p");

    let totalNumber = separate[separate.length - 1];
    if (totalNumber > 5) {
      p.textContent = `${totalNumber - 1} is maximum user allowed. Wait for others exit the chat.`;
      p.className = "red-white";
      li.append(p)
      messages.append(li);
      socket.close();
      return;
    }

    open = true;

    p.textContent = `Your id is ${userId} and "You" will be used in this page instead`;
    p.className = "blue";
    li.append(p)
    messages.append(li);
    return;
  } else {
    let fromServer = event.data;
    const beforePayload = fromServer.split(" ")[0];
    const authorOfMessage = beforePayload.slice(0, beforePayload.length - 1); // to get the id part of the message

    if (fromServer.includes(`!exclude ${userId}`)) {
      socket.close();
      return;
    }

    const messages = document.getElementById("messages");
    const li = document.createElement("li");

    if (authorOfMessage === userId) {
      li.className = "red-white";
      fromServer = fromServer.replace(userId, "You");
    }

    const p = document.createElement("p");
    p.append(fromServer)
    li.append(p);
    messages.append(li);
    return;
  }
};

socket.onclose = function (event) {
  const closeMessage = event.data === undefined ? "Server, You or another user closed the connection." : "WebSocket is closed now."

  const li = document.createElement("li");
  li.append(closeMessage)
  li.className = "blue";

  const messages = document.getElementById("messages");

  messages.append(li);
};

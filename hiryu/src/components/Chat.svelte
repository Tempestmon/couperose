<script>
  import { onMount } from "svelte";
  import Message from "./Message.svelte";
  import { fetchMessages, sendMessage } from "../api.js";

  export let nickname;

  let messages = [];
  let newMessage = "";

  // Загружаем сообщения при монтировании компонента
  onMount(async () => {
    await loadMessages();
    setInterval(loadMessages, 3000); // Обновляем каждые 3 секунды
  });

  async function loadMessages() {
    try {
      messages = await fetchMessages();
    } catch (err) {
      console.error("Failed to load messages:", err);
    }
  }

  async function handleSendMessage() {
    console.log("Sending message:", newMessage);
    if (!newMessage.trim()) {
      alert("Message cannot be empty");
      return;
    }

    try {
      await sendMessage(nickname, newMessage);
      console.log("Message sent successfully");
      messages = [...messages, { sender: nickname, content: newMessage }];
      newMessage = "";
    } catch (err) {
      console.error("Error while sending message:", err);
      alert(`Failed to send message: ${err.message}`);
    }
  }
</script>

<div class="chat">
  <h2>Chat</h2>
  <div class="messages">
    {#each messages as { sender, content }}
      <Message {sender} {content} />
    {/each}
  </div>
  <div class="input-area">
    <input
      type="text"
      placeholder="Write a message..."
      bind:value={newMessage}
      on:keydown={(e) => e.key === "Enter" && handleSendMessage()}
    />
    <button on:click={handleSendMessage}>Send</button>
  </div>
</div>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Orbitron:wght@400;700&family=Share+Tech+Mono&display=swap");

  .chat {
    margin: 20px;
    font-family: "Share Tech Mono", monospace;
  }

  .messages {
    border: 2px solid #8aebf1;
    padding: 15px;
    height: 300px;
    overflow-y: scroll;
    margin-bottom: 15px;
    background: rgba(0, 0, 0, 0.7);
    border-radius: 8px;
    box-shadow: 0 0 15px rgba(94, 129, 245, 0.3);
  }

  .input-area {
    display: flex;
    gap: 10px;
  }

  input {
    flex: 1;
    padding: 12px;
    background-color: rgba(0, 0, 0, 0.7);
    border: 2px solid #8aebf1;
    border-radius: 8px;
    color: #8aebf1;
    font-family: "Share Tech Mono", monospace;
    font-size: 1.1em;
    outline: none;
    transition: all 0.3s ease;
  }

  input:focus {
    box-shadow: 0 0 15px #edf37e;
    border-color: #edf37e;
  }

  button {
    padding: 12px 24px;
    background-color: rgba(0, 0, 0, 0.7);
    color: #8aebf1;
    border: 2px solid #edf37e;
    border-radius: 8px;
    font-family: "Orbitron", sans-serif;
    font-size: 1.1em;
    cursor: pointer;
    text-shadow: 0 0 10px #5e81f5;
    transition: all 0.3s ease;
  }

  button:hover {
    background-color: #5e81f5;
    color: #0d0d0d;
    transform: scale(1.05);
    box-shadow: 0 0 15px #edf37e;
  }
</style>

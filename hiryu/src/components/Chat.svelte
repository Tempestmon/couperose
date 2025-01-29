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
  .chat {
    margin: 20px;
  }

  .messages {
    border: 1px solid #ddd;
    padding: 10px;
    height: 300px;
    overflow-y: scroll;
    margin-bottom: 10px;
  }

  .input-area {
    display: flex;
  }

  input {
    flex: 1;
    padding: 10px;
    margin-right: 10px;
  }

  button {
    padding: 10px;
    background-color: #007bff;
    color: white;
    border: none;
    cursor: pointer;
  }

  button:hover {
    background-color: #0056b3;
  }
</style>

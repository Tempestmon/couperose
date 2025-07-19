<script>
  import Chat from "./components/Chat.svelte";

  let nickname = ""; // Никнейм пользователя
  let isNicknameSet = false;

  function setNickname() {
    if (nickname.trim()) {
      isNicknameSet = true;
    }
  }
</script>

{#if !isNicknameSet}
  <div class="welcome">
    <h1 class="neon-glow">Welcome to Messenger</h1>
    <input
      type="text"
      placeholder="Enter your nickname"
      bind:value={nickname}
      on:keydown={(e) => e.key === "Enter" && setNickname()}
    />
    <button on:click={setNickname}>Enter Chat</button>
  </div>
{:else}
  <Chat {nickname} />
{/if}

<style>
  @import url("https://fonts.googleapis.com/css2?family=Orbitron:wght@400;700&family=Share+Tech+Mono&display=swap");

  .welcome {
    text-align: center;
    margin-top: 100px;
    font-family: "Share Tech Mono", monospace;
  }

  h1 {
    font-family: "Orbitron", sans-serif;
    color: #5e81f5;
    font-size: 2.5em;
    text-shadow:
      0 0 5px #8aebf1,
      0 0 10px #5e81f5,
      0 0 20px #edf37e;
    animation: glitch 2s linear infinite;
  }

  @keyframes glitch {
    2%,
    64% {
      transform: translate(2px, 0);
    }
    4%,
    60% {
      transform: translate(-2px, 0);
    }
    62% {
      transform: translate(0, 0);
    }
  }

  input {
    padding: 12px;
    margin: 10px;
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

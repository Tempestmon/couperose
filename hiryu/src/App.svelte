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
    <h1>Welcome to the Messenger</h1>
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
  .welcome {
    text-align: center;
    margin-top: 100px;
  }

  input {
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

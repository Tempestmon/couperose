<script>
  export let sender;
  export let content;
</script>

<div class="message">
  <strong class="neon-glow">{sender}:</strong>
  {content}
</div>

<style>
  .message {
    margin: 8px 0;
    padding: 10px;
    background: rgba(0, 0, 0, 0.6);
    border-left: 3px solid #8aebf1;
    color: #8aebf1;
    font-family: "Share Tech Mono", monospace;
    font-size: 1em;
    transition: all 0.3s ease;
  }

  .message:hover {
    box-shadow: 0 0 10px rgba(94, 129, 245, 0.5);
  }

  strong {
    color: #edf37e;
    text-shadow: 0 0 5px #5e81f5;
  }
</style>

<script>
  export let sender;
  export let content;
</script>

<div>
  <strong>{sender}:</strong>
  {content}
</div>

<style>
  div {
    margin: 5px 0;
  }

  strong {
    color: #007bff;
  }
</style>

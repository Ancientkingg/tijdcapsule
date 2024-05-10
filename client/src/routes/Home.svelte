<script lang="ts">
  import Editor from "../lib/Editor.svelte";

  let value: string = "Poephoofd";

  const onSave = () => {
    alert(value);
  }
</script>

<section class="home">
  <Editor bind:value mode="tabs" />
  <div class="button-wrapper">
    <button on:click={onSave}>Save</button>
  </div>
</section>

<style lang="scss">
    .home {
        display: flex;
        flex-direction: column;
        align-items: center;
        height: 100vh;
        width: 100vw;
        padding: 2rem;
    }

    .button-wrapper {
        display: flex;
        justify-content: flex-end;
        width: 100% ;
    }
</style>

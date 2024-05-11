<script lang="ts">
  import { createCapsule } from "../client";

  import { navigate } from "svelte-routing";

  import CapsuleSettings from "../lib/CapsuleSettings.svelte";
  import Editor from "../lib/Editor.svelte";

  let name: string = "";
  let content: string = "Type here...";
  let author: string = "";
  let deadline: Date = new Date();

  const onSave = () => {
    createCapsule({
      name,
      content,
      author,
      deadline: deadline.toISOString(),
    }).then((capsule) => {
      navigate(`/capsule/${capsule.capsule.id}#${capsule.key}`)
      console.log("Capsule created", capsule);
    });
  };
</script>

<section class="home">
  <Editor bind:value={content} mode="tabs" />
  <CapsuleSettings bind:deadline bind:name bind:author />
  <div class="button-wrapper">
    <button class="button" on:click={onSave}>Save</button>
  </div>
</section>

<style lang="scss">
  .home {
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100vh;
    width: 100%;
    max-width: 1500px;
    padding: 2rem;
    gap: 1rem;
  }

  .button-wrapper {
    display: flex;
    justify-content: flex-end;
    width: 100%;
  }
</style>

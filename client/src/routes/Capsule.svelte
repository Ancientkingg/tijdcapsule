<script lang="ts">
  import CapsuleSettings from "../lib/CapsuleSettings.svelte";
  import Markdown from "../lib/Markdown.svelte";

  import { fetchCapsule, type Capsule } from "../client";
  import Waiting from "../lib/waiting/Waiting.svelte";
  import { useHistory } from "svelte-routing";

  export let id: string;
  let key: string = window.location.hash.substring(1);

  let capsule: Capsule;
  let status: "LOADING" | "ERROR" | "WAITING" | "SUCCESS" = "LOADING";
  let errorMessage: string = "";

  const refreshCapsule = () => {
    fetchCapsule(id, key)
      .then((fetchedCapsule) => {
        status = "SUCCESS";
        capsule = fetchedCapsule;
      })
      .catch((err) => {
        switch (err.errorCode) {
          case "CAPSULE_NOT_FOUND":
            status = "ERROR";
            errorMessage = err.error;
            break;
          case "CAPSULE_NOT_READY":
            status = "WAITING";
            capsule = err.data;
            errorMessage = err.error;
            break;
          default:
            status = "ERROR";
            errorMessage = "Unknown error, check console";
            console.error(err);
        }
      });
  };

  refreshCapsule();
</script>

<section class={status === "SUCCESS" ? "capsule" : "capsule centered"}>
  {#if status === "LOADING"}
    <h1>Loading...</h1>
  {:else if status === "ERROR"}
    <h1>{errorMessage}</h1>
  {:else if status === "WAITING"}
    <Waiting
      onReady={refreshCapsule}
      mode="random"
      deadline={capsule.deadline}
      createdAt={capsule.createdAt}
    />
  {:else}
    <Markdown value={capsule.content} />
    <CapsuleSettings deadline={capsule.deadline} name={capsule.name} author={capsule.author} disabled={true} />
  {/if}

  <!-- <Editor value={"L"} mode="tabs" />
  <CapsuleSettings disabled={true} /> -->
</section>

<style lang="scss">
  .capsule {
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100vh;
    width: 100%;
    max-width: 1500px;
    padding: 2rem;
    gap: 1rem;

    h1 {
      color: white;
    }
  }

  .centered {
    justify-content: center;
    align-items: center;
  }
</style>

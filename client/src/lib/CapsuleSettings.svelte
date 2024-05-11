<script lang="ts">
  import { DateInput } from "date-picker-svelte";

  const MIN_DATE = new Date();
  const MAX_DATE = new Date(MIN_DATE.getFullYear() + 10, 11, 31);

  export let disabled: boolean = false;
  export let name: string = "";
  export let author: string = "";
  export let deadline = MIN_DATE;
</script>

<section class="settings">
  <div class="setting">
    <label for="deadline">Deadline</label>
    <DateInput
      bind:value={deadline}
      id="deadline"
      timePrecision={"second"}
      min={MIN_DATE}
      max={MAX_DATE}
      {disabled}
    />
  </div>
  <div class="setting">
    <label for="name">Name</label>
    <input id="name" placeholder="Capsule Name" bind:value={name} {disabled} />
  </div>
  <div class="setting">
    <label for="author">Author</label>
    <input id="author" placeholder="Author" bind:value={author} {disabled} />
  </div>
</section>

<style lang="scss">
  :root {
    --date-picker-background: var(--input-background-color);
    --date-picker-foreground: var(--input-foreground-color);
    --date-picker-highlight-border: var(--accent-color)
  }

  .settings {
    display: flex;
    width: 100%;
    padding: 1rem;
    background-color: var(--primary-background-color);
    border-radius: 8px;
    border: 1px solid rgb(43, 49, 56);
    gap: 1rem;
    flex-wrap: wrap;
  }

  .setting {
    display: flex;
    flex-direction: column;
  }
</style>

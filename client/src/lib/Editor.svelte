<script lang="ts">
  import { Carta, MarkdownEditor } from "carta-md";

  import { emoji } from "@cartamd/plugin-emoji";
  import { slash } from "@cartamd/plugin-slash";
  import { code } from "@cartamd/plugin-code";

  import "./styles/editor.scss";

  export let value = "";
  export let mode: "auto" | "split" | "tabs" | undefined = "tabs";

  const carta = new Carta({
    sanitizer: false,
    extensions: [
			emoji(),
			slash(),
			code()
		]
  });
</script>

<MarkdownEditor bind:value {mode} {carta} />

<style>
  /* Or in global stylesheet */
  /* Set your custom monospace font */
  :global(.carta-font-code) {
    font-family: monospace;
    font-size: 1.1rem;
  }
</style>

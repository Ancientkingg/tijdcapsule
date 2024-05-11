<script lang="ts">
  import Countdown from "./modes/Countdown.svelte";
  import Fakebar from "./modes/Fakebar.svelte";
  import Hourglass from "./modes/Hourglass.svelte";

  const ALL_MODES = ["countdown", "hourglass", "fakebar"] as const;
  type Mode = (typeof ALL_MODES)[number];

  export let mode: "random" | Mode;
  export let deadline: Date = new Date();
  export let createdAt: Date = new Date();

  export let onReady: () => void;

  const realMode =
    mode === "random"
      ? ALL_MODES[Math.floor(Math.random() * ALL_MODES.length)]
      : mode;
  const modes = {
    countdown: Countdown,
    hourglass: Hourglass,
    fakebar: Fakebar,
  };

  setTimeout(onReady, deadline.getTime() - new Date().getTime());
</script>

<svelte:component this={modes[realMode]} {deadline} {createdAt}></svelte:component>

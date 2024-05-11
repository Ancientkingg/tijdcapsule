<script lang="ts">
  import { onMount } from "svelte";
  import winton from '../../../assets/winton.jpg';

  export let deadline: Date = new Date();
  export let createdAt: Date = new Date();

  let progress: HTMLSpanElement;

  let fakeout = true;
  let percentage: number = 0;
  let barDeadline = new Date((new Date().getTime() + deadline.getTime()) / 2);

  const setProgress = (percentage: number) => {
    progress.style.width = `${percentage}%`;
  };

  const updateHourglass = () => {
    const now = new Date();
    const timeLeft = barDeadline.getTime() - now.getTime();
    const timeSinceCreation = barDeadline.getTime() - createdAt.getTime();
    percentage = ((timeSinceCreation - timeLeft) / timeSinceCreation) * 100;

    setProgress(percentage);

    if (percentage >= 100 && fakeout) {
      fakeout = false;
      barDeadline = deadline;
    }
  };

  onMount(() => {
    updateHourglass();
    setInterval(updateHourglass, 1000);
  });
</script>

{#if !fakeout}
<img src={winton} alt="Just kidding.. the wait continues!" />
  <p>Just kidding.. the wait continues!</p>
{/if}

<div class="progressbar">
  <span bind:this={progress} style="width:60%"></span>
</div>
{percentage.toFixed(2)}%

<style lang="scss">
  .progressbar {
    width: 100%;
    height: 2rem;
    background-color: var(--primary-background-color);
    border-radius: 1rem;
    overflow: hidden;

    span {
      display: block;
      height: 100%;
      background-color: var(--accent-color);
      transition: width 1s;
    }
  }

  img {
    width: 100%;
    max-width: 800px;
  }
</style>

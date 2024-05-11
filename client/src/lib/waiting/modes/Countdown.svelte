<script lang="ts">
  export let deadline: Date = new Date();

  let seconds: number = 0;
  let minutes: number = 0;
  let hours: number = 0;
  let days: number = 0;

  const updateCountdown = () => {
    const now = new Date();
    const timeLeft = deadline.getTime() - now.getTime();
    seconds = Math.floor(timeLeft / 1000) % 60;
    minutes = Math.floor(timeLeft / (1000 * 60)) % 60;
    hours = Math.floor(timeLeft / (1000 * 60 * 60)) % 24;
    days = Math.floor(timeLeft / (1000 * 60 * 60 * 24));
  };

  updateCountdown();
  setInterval(updateCountdown, 1000);
</script>

<div class="countdown-wrapper">
  <div class="countdown-number">
    {days}
    <span>DAYS</span>
  </div>
  <div class="countdown-number">
    {hours}
    <span>HOURS</span>
  </div>
  <div class="countdown-number">
    {minutes}
    <span>MINUTES</span>
  </div>
  <div class="countdown-number">
    {seconds}
    <span>SECONDS</span>
  </div>
</div>

<style lang="scss">
  .countdown-wrapper {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .countdown-number {
    display: flex;
    flex-direction: column;
    align-items: center;
    border-radius: 2rem;
    width: 3ch;
    font-size: 8rem;
    padding: 1rem;
    color: white;
    background-color: var(--primary-background-color);

    span {
        font-size: 1rem;
    }
  }
</style>

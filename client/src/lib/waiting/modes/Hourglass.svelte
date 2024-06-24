<script lang="ts">
    import { onMount } from 'svelte';

    export let deadline: Date = new Date();
    export let createdAt: Date = new Date();

    let sandTop: HTMLDivElement;
    let sandBottom: HTMLDivElement;

    let percentage: number = 0;

    const setSandFullness = (percentage: number) => {
        sandTop.style.height = `${percentage}%`;
        sandBottom.style.height = `${100 - percentage}%`;
    };

    const updateHourglass = () => {
        const now = new Date();
        const timeLeft = deadline.getTime() - now.getTime();
        const timeSinceCreation = deadline.getTime() - createdAt.getTime();
        percentage = ((timeSinceCreation - timeLeft) / timeSinceCreation) * 100;

        setSandFullness(100 - percentage);
    };

    onMount(() => {
        updateHourglass();
        setInterval(updateHourglass, 1000);
    });
</script>

<div id="hourglass" class="ready">
    <div class="glass">
        <div class="sand top" bind:this={sandTop}></div>
    </div>
    <div class="glass bottom">
        <div class="sand bottom" bind:this={sandBottom}></div>
    </div>
</div>
{percentage.toFixed(2)}% of the way there...

<style lang="scss">
    :root {
        --sandColor: rgba(222, 192, 135, 1);
        --glassColor: rgba(0, 123, 138, 0.2);
        --hourglassSize: 12rem;
    }

    #hourglass {
        display: inline-block;
        transition-property: transform;
        transition-duration: 700ms;
        transition-timing-function: ease;
    }

    .glass {
        z-index: 1;
        position: relative;
        background-color: var(--glassColor);
        clip-path: polygon(0 0, 100% 0, 50% 100%);
        height: var(--hourglassSize);
        width: calc(var(--hourglassSize) * 1.5);
        bottom: -0.2em;
        overflow: hidden;
        border-top-left-radius: 20% 100%;
        border-top-right-radius: 20% 100%;
    }

    .top {
        z-index: 2;
    }

    .glass.bottom {
        z-index: 0;
        top: -0.2em;
        clip-path: polygon(50% 0, 100% 100%, 0 100%);
        border-top-left-radius: 0;
        border-top-right-radius: 0;
        border-bottom-left-radius: 20% 100%;
        border-bottom-right-radius: 20% 100%;
    }

    .sand.top:after {
        position: absolute;
        content: '';
        width: 100%;
        top: -20px;
        height: 20px;
        border-bottom-left-radius: 50% 70%;
        border-bottom-right-radius: 50% 70%;
        background-color: transparent;
        box-shadow: 0 10px 0 var(--sandColor);
        animation: kfSandTop 2000ms ease-in-out infinite;
    }

    .sand {
        position: absolute;
        background-color: var(--sandColor);
        min-width: 100%;
        height: 100%;
        bottom: 0;
    }

    #hourglass.ready .sand {
        transition-property: height;
        transition-timing-function: ease-out;
        transition-duration: 200ms;
    }

    .glass.bottom .sand {
        height: 0;
        border-top-left-radius: 50% 100%;
        border-top-right-radius: 50% 100%;
        border-bottom-left-radius: 0;
        border-bottom-right-radius: 0;
        transition-timing-function: ease-in;
        animation: kfSandBottom 3000ms ease infinite;
    }

    .glass.bottom:before {
        position: absolute;
        content: '';
        background-color: var(--sandColor);
        height: 98%;
        width: 3px;
        bottom: 0;
        left: 50%;
        transform: translate(-50%);
    }

    @keyframes kfSandTop {
        0% {
            border-bottom-left-radius: 50% 50%;
            border-bottom-right-radius: 50% 50%;
        }
        50% {
            border-bottom-left-radius: 50% 80%;
            border-bottom-right-radius: 50% 80%;
        }
        100% {
            border-bottom-left-radius: 50% 50%;
            border-bottom-right-radius: 50% 50%;
        }
    }

    @keyframes kfSandBottom {
        0% {
            border-top-left-radius: 50% 100%;
            border-top-right-radius: 50% 100%;
        }
        50% {
            border-top-left-radius: 100% 150%;
            border-top-right-radius: 100% 150%;
        }
        100% {
            border-top-left-radius: 50% 100%;
            border-top-right-radius: 50% 100%;
        }
    }
</style>

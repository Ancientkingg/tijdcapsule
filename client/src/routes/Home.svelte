<script lang="ts">
    import { createCapsule } from '../client';

    import { navigate } from 'svelte-routing';

    import CapsuleSettings from '../lib/CapsuleSettings.svelte';
    import Editor from '../lib/Editor.svelte';

    import Nav from '../lib/Nav.svelte';
    import Footer from '../lib/Footer.svelte';

    const convertToDateInput = (now: Date) => {
        now.setMinutes(now.getMinutes() - now.getTimezoneOffset());
        return now.toISOString().slice(0,16);
    }

    const MIN_DATE = convertToDateInput(new Date());

    let name: string = '';
    let content: string = '';
    let author: string = '';
    let deadline: string = MIN_DATE;

    const onSave = () => {
        createCapsule({
            name,
            content,
            author,
            deadline: (new Date(deadline)).toISOString(),
        }).then((capsule) => {
            navigate(`/capsule/${capsule.capsule.id}?sleutel=${capsule.key}`);
            console.log('Capsule created', capsule);
        });
    };

    import ShovelIcon from '../lib/ShovelIcon.svelte';
    import { Tooltip } from 'flowbite-svelte';
</script>

<Nav></Nav>

<section class="home">
    <div class="m-0 p-0 w-full h-full gap-5 flex flex-col">
        <Editor bind:value={content} mode="tabs" />
        <CapsuleSettings bind:deadline bind:name bind:author />
        <div class="button-wrapper">
            <button
                class="flex justify-center items-center button"
                on:click={onSave}>
                <p>Bury</p>
                <ShovelIcon class="shovel-icon ml-3" />
            </button>
            <Tooltip
                defaultClass="py-2 px-3 text-sm font-medium tooltip-wrapper"
                arrow={false}>Save your capsule</Tooltip>
        </div>
    </div>
    <Footer></Footer>
</section>

<style lang="scss">
    @keyframes show {
        100% {
            opacity: 1;
            transform: none;
        }
    }
    :global(.tooltip-wrapper) {
        background-color: var(--input-background-color);
        opacity: 0;
        transform: translateY(10px);
        transform-origin: bottom center;
        animation: show 600ms 1000ms cubic-bezier(0.38, 0.97, 0.56, 0.76)
            forwards;
    }

    button:hover :global(.shovel-icon) {
        fill: var(--input-background-color);
        transition: fill ease-in-out 0.2s;
    }

    .home {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        min-height: 85vh;
        max-width: 1500px;
        padding: 2rem;
        padding-bottom: 0;
        gap: 1rem;
    }

    @media (max-width: 768px) {
        .home {
            min-height: 88.5vh;
        }
    }

    .button-wrapper {
        display: flex;
        justify-content: flex-end;
        width: 100%;
    }

    button:hover p {
        color: var(--input-background-color);
        font-weight: bolder;
        transform: scaleY(1.1);
        transition: all ease-in-out 0.2s;
    }
</style>

<script lang="ts">
    import CapsuleSettings from '../lib/CapsuleSettings.svelte';
    import Markdown from '../lib/Markdown.svelte';

    import { fetchCapsule, type Capsule } from '../client';
    import Waiting from '../lib/waiting/Waiting.svelte';
    import Nav from '../lib/Nav.svelte';
    import Footer from '../lib/Footer.svelte';
    import { Spinner } from 'flowbite-svelte';

    const convertToDateInput = (now: Date) => {
        now.setMinutes(now.getMinutes() - now.getTimezoneOffset());
        return now.toISOString().slice(0, 16);
    };

    export let id: string;

    let invalidKey: boolean = false;

    const key: string | null = (() => {
        const key = new URLSearchParams(window.location.search).get('sleutel');

        if (!key) {
            invalidKey = true;
        }

        return key;
    })();

    let capsule: Capsule;

    type CapsuleStatus = 'LOADING' | 'ERROR' | 'WAITING' | 'SUCCESS';
    let status: CapsuleStatus = 'LOADING';

    let errorMessage: string = 'An unexpected error occurred.';

    const refreshCapsule = () => {
        if (invalidKey) {
            status = 'ERROR';
            errorMessage = 'No key was provided.';
            return;
        }

        fetchCapsule(id, key!)
            .then((fetchedCapsule) => {
                status = 'SUCCESS';
                capsule = fetchedCapsule;
            })
            .catch((err) => {
                switch (err.errorCode) {
                    case 'CAPSULE_NOT_FOUND':
                        status = 'ERROR';
                        errorMessage = err.error;
                        break;
                    case 'CAPSULE_NOT_READY':
                        status = 'WAITING';
                        capsule = err.data;
                        errorMessage = err.error;
                        break;
                    case 'INTERNAL_SERVER':
                        status = 'ERROR';
                        errorMessage = err.error;
                        break;
                    case 'INVALID_KEY':
                        status = 'ERROR';
                        errorMessage = err.error;
                        break;
                    default:
                        status = 'ERROR';
                        console.error(err);
                }
            });
    };

    refreshCapsule();

    const statusClass = (() => {
        switch (status as CapsuleStatus) {
            case 'LOADING':
                return 'capsule loading';
            case 'ERROR':
                return 'capsule error';
            case 'WAITING':
                return 'capsule waiting';
            case 'SUCCESS':
                return 'capsule';
            default:
                return '';
        }
    })();
</script>

<Nav></Nav>

<section class={statusClass}>
    {#if status === 'LOADING'}
        <div
            class="flex justify-center items-center m-auto min-h-full basis-full">
            <Spinner size="14" color="green" />
        </div>
    {:else if status === 'ERROR'}
        <h1 class="flex justify-center items-center m-auto">{errorMessage}</h1>
    {:else if status === 'WAITING'}
        <div
            class="flex flex-col gap-2 justify-center items-center text-center m-auto">
            <Waiting
                onReady={refreshCapsule}
                mode="random"
                deadline={capsule.deadline}
                createdAt={capsule.created_at} />
        </div>
    {:else}
        <div class="m-0 p-0 w-full h-full gap-5 flex flex-col justify-between">
            <Markdown value={capsule.content} />
            <CapsuleSettings
                deadline={convertToDateInput(capsule.deadline)}
                name={capsule.name}
                author={capsule.author}
                disabled={true} />
        </div>
    {/if}

    <Footer></Footer>
</section>

<style lang="scss">
    .capsule {
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

        h1 {
            color: white;
        }
    }

    @media (max-width: 768px) {
        .capsule {
            min-height: 88.5vh;
        }
    }

    .centered {
        justify-content: center;
        align-items: center;
    }
</style>

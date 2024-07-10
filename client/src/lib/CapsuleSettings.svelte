<script lang="ts">
    import './styles/slider.scss';

    const convertToDateInput = (now: Date) => {
        now.setMinutes(now.getMinutes() - now.getTimezoneOffset());
        return now.toISOString().slice(0, 16);
    };

    const MIN_DATE = convertToDateInput(new Date());

    export let disabled: boolean = false;
    export let name: string = '';
    export let author: string = '';
    export let deadline = MIN_DATE;
    // export let isPrivate: boolean = false;

    import { AccordionItem, Accordion } from 'flowbite-svelte';
</script>

<section class="settings">
    <Accordion flush>
        <AccordionItem borderBottomClass="settings-wrapper" open>
            <div class="setting">
                <label for="deadline">Deadline</label>
                <input
                    class="rounded-md"
                    id="deadline"
                    bind:value={deadline}
                    type="datetime-local"
                    {disabled} />
            </div>
            <div class="setting">
                <label for="name">Name</label>
                <input
                    class="rounded-md input-wrapper"
                    id="name"
                    placeholder="Capsule Name"
                    bind:value={name}
                    {disabled} />
            </div>
            <div class="setting">
                <label for="author">Author</label>
                <input
                    class="rounded-md input-wrapper"
                    id="author"
                    placeholder="Author"
                    bind:value={author}
                    {disabled} />
            </div>

            <!-- <div class="setting items-center">
                <label class="mb-1" for="private">Private</label>
                <label class="switch">
                    <input {disabled} bind:value={isPrivate} type="checkbox" />
                    <span class="slider round {disabled ? 'disabled' : ''}"
                    ></span>
                </label>
            </div> -->
        </AccordionItem>
    </Accordion>
</section>

<style lang="scss">
    :root {
        --date-picker-background: var(--input-background-color);
        --date-picker-foreground: var(--input-foreground-color);
        --date-picker-highlight-border: var(--accent-color);
    }

    .settings {
        display: flex;
        align-self: start;
        padding: 0.5rem;
        width: 100%;
        max-width: 41rem;
        background-color: var(--primary-background-color);
        border-radius: 8px;
        border: 1px solid rgb(43, 49, 56);
        gap: 1rem;
        flex-wrap: wrap;
    }

    @media (max-width: 768px) {
        :global(h2.group) {
            width: 70vw;
        }
    }
    :global(button.settings-wrapper) {
        width: 100%;
    }

    :global(.settings-wrapper) {
        display: flex;
        padding: 0.5rem;
        gap: 1rem;
        flex-wrap: wrap;
    }

    .setting {
        display: flex;
        flex-direction: column;
    }
</style>

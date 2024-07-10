<script lang="ts">
    import { Markdown } from 'carta-md';

    import './styles/markdown.scss';

    import 'katex/dist/katex.css';

    import { carta } from './carta';

    export let value = '';
</script>

<div class="markdown-wrapper">
    <Markdown bind:value {carta} />
</div>

<style lang="scss">
    .markdown-wrapper {
        display: flex;
        width: 100%;
        padding: 1rem;
        background-color: var(--primary-background-color);
        border-radius: 8px;
        border: 1px solid rgb(43, 49, 56);
        gap: 1rem;
        flex-wrap: wrap;
    }
</style>

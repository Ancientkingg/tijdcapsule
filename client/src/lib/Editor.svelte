<script lang="ts">
    import { MarkdownEditor } from 'carta-md';

    import './styles/editor.scss';
    import './styles/markdown.scss';

    import 'katex/dist/katex.css';

    import { carta } from './carta'

    export let value = '';
    export let mode: 'auto' | 'split' | 'tabs' | undefined = 'tabs';
</script>

<MarkdownEditor placeholder="Type here..." bind:value {mode} {carta} />

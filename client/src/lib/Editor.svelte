<script lang="ts">
    import { Carta, MarkdownEditor } from 'carta-md';

    import { emoji } from '@cartamd/plugin-emoji';
    import { slash } from '@cartamd/plugin-slash';
    import { code } from '@cartamd/plugin-code';
    import { math } from '@cartamd/plugin-math';

    import './styles/editor.scss';
    import './styles/markdown.scss';

    import 'katex/dist/katex.css';

    export let value = '';
    export let mode: 'auto' | 'split' | 'tabs' | undefined = 'tabs';

    const carta = new Carta({
        sanitizer: false,
        extensions: [emoji(), slash(), code(), math()],
    });
</script>

<MarkdownEditor bind:value {mode} {carta} />

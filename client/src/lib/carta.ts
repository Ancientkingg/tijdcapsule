import { Carta, MarkdownEditor } from 'carta-md';
import DOMPurify from 'isomorphic-dompurify';

import { emoji } from '@cartamd/plugin-emoji';
import { slash } from '@cartamd/plugin-slash';
import { code } from '@cartamd/plugin-code';
import { math } from '@cartamd/plugin-math';

export const carta = new Carta({
    sanitizer: DOMPurify.sanitize,
    extensions: [emoji(), slash(), code(), math()],
});

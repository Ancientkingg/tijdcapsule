<script>
    import {
        Footer,
        FooterCopyright,
        FooterLinkGroup,
        FooterLink,
    } from 'flowbite-svelte';
    import { GithubSolid, LinkedinSolid } from 'flowbite-svelte-icons';
</script>

<Footer class="relative bottom-0">
    <div class="sm:flex sm:items-center sm:justify-between mb-5">
        <FooterCopyright
            spanClass="text-center block text-sm text-gray-500 sm:text-center dark:text-gray-400"
            href="https://github.com/ancientkingg"
            by="Samuel Bruin"
            year={2024} />
        <FooterLinkGroup
            ulClass="flex md:flex-row flex-col justify-center flex-wrap mt-3 text-sm text-gray-500 dark:text-gray-400 sm:mt-0 md:ml-10 text-center">
            <a href="https://github.com/ancientkingg/tijdcapsule"
                class="flex flex-row last:me-0 md:me-6 w-full md:w-fit justify-center me-0">
                <GithubSolid class="mr-1 relative -top-[1px]" size="md"
                ></GithubSolid
                ><FooterLink href="https://github.com/ancientkingg/tijdcapsule"
                    >Github</FooterLink>
            </a>
            <FooterLink liClass="me-0 last:me-0 md:me-6" href="https://github.com/Ancientkingg/tijdcapsule/blob/main/PRIVACY.md"
                >Privacy Policy</FooterLink>
            <FooterLink liClass="me-0 last:me-0 md:me-6" href="https://github.com/Ancientkingg/tijdcapsule/blob/main/LICENSE"
                >Licensing</FooterLink>
            <a href="https://www.linkedin.com/in/samuelbruin/"
                class="flex flex-row last:me-0 md:me-6 w-full md:w-fit justify-center me-0">
                <LinkedinSolid class="mr-1 relative -top-[2px]" size="md"
                ></LinkedinSolid
                ><FooterLink href="https://www.linkedin.com/in/samuelbruin/"
                    >Contact</FooterLink>
            </a>
        </FooterLinkGroup>
    </div>
</Footer>

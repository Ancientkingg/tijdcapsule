<script>
    import {
        Navbar,
        NavBrand,
        NavLi,
        NavUl,
        NavHamburger,
        Input,
        Search,
        Button,
    } from 'flowbite-svelte';
    import { SearchOutline } from 'flowbite-svelte-icons';
</script>

<Navbar let:NavContainer class="mt-8" color="none">
    <NavContainer
        class="max-w-[1425px] justify-between flex-row items-center">
        <NavBrand class="self-start ml-3 md:ml-0 mt-1.5" href="/">
            <img
                src="/logo.png"
                class="me-1.5 md:me-3 h-6 sm:h-9"
                alt="TijdCapsule Logo" />
            <span
                class="transition ease-in-out duration-300 hover:brightness-75 self-center whitespace-nowrap text-xl font-semibold dark:text-white"
                >TijdCapsule</span>
        </NavBrand>

        <!-- Search bar that was scrapped since it is not relevant to the context of the usecase -->
<!-- 
        <div class="flex md:order-1 md:w-[40%] w-[90%] md:mt-0 mt-5">
            <form class="flex gap-2 w-screen">
                <Search
                    class="search-bar"
                    placeholder="Search capsules..."
                    size="md" />
                <Button color="green" class="button-color" size="xs">
                    <SearchOutline class="w-6 h-6 -m-1" />
                </Button>
            </form>
        </div> -->

        <Button
            href="/"
            color="green"
            class="button-color self-center hidden md:flex md:order-2 md:p-2.5 mt-0.5 px-2 mr-2 md:mr-0"
            size="sm">Create Capsule</Button>
    </NavContainer>
</Navbar>

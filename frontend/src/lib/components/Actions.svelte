<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import { onDestroy, onMount } from "svelte";
    import { Settings, Network, ChevronsLeftRightEllipsis } from "lucide-svelte";


    let nav: HTMLElement;
    let timeoutID: number;

    onMount(() => {
        // Add the event listener
        window.addEventListener("mousemove", handleMouseMove);

        // Cleanup event listener on component destroy
        onDestroy(() => {
            window.removeEventListener("mousemove", handleMouseMove);
        });
    });

    function handleMouseMove(event: MouseEvent) {
        if (!nav) {
            return;
        }

        nav.style.opacity = "1";
        if (timeoutID) {
            clearTimeout(timeoutID);
        }
        timeoutID = setTimeout(() => {
            if (nav) {
                nav.style.opacity = "0";
            }
        }, 3000);
    }
</script>

<nav bind:this={nav} class="fixed p-4 bottom-0 left-0 flex flex-col z-10 opacity-0">
    <Button variant="outline" class="p-0 w-10 mt-3">
        <ChevronsLeftRightEllipsis class="opacity-60"/>
    </Button>
    <Button variant="outline" class="p-0 w-10 mt-3">
        <Network class="opacity-60"/>
    </Button>
    <Button variant="outline" class="p-0 w-10 mt-3">
        <!-- <Settings class="opacity-60"/> -->
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke-opacity="0.6"
            stroke="currentColor"
            class="size-6"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z"
            />
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
            />
        </svg>
    </Button>

</nav>

<style scoped lang="postcss">
    nav {
        transition: opacity 0.5s;
    }
</style>

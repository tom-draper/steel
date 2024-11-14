<script lang="ts">
    import DirectoryView from "$lib/components/DirectoryView.svelte";
    import { onMount } from "svelte";

    let directoryContents: { path: string; is_directory: boolean }[] | null =
        null;

    onMount(() => {
        fetchRoot();
    });

    async function fetchRoot() {
        try {
            const response = await fetch("http://localhost:3000/_path");
            if (response.ok && response.status === 200) {
                const data = await response.json();
                if (data instanceof Array) {
                    directoryContents = data;
                    console.log("directoryContents:", directoryContents);
                }
            } else {
                resetState();
            }
        } catch (error) {
            console.error(error);
            resetState();
        }
    }

    function resetState() {
        directoryContents = null;
    }
</script>

<main class="grid place-items-center w-full">
    <div class="grid place-items-center h-[25vh] w-full mt-[68px]">
        <div>
            <h1
                class="bg-accent px-2 py-0.5 rounded-sm mb-2 text-muted-foreground"
            >
                steel
            </h1>
            <div class="text-xs text-muted-foreground justify-self-center">
                v0.1.0
            </div>
        </div>
    </div>

    {#if directoryContents !== null}
        <div
            class="grid place-items-center w-[90%] place-self-center pb-[5em] max-w-[500px]"
        >
            <DirectoryView bind:nodes={directoryContents} />
        </div>
    {/if}
</main>

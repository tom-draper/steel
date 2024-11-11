<script lang="ts">
    import DirectoryView from "$lib/components/DirectoryView.svelte";
    import { onMount } from "svelte";

    let fileText: string | null = null;
    let directoryContents: string[] | null = null;

    onMount(() => {
        fetchRoot();
    });

    async function fetchRoot() {
        try {
            const response = await fetch("http://localhost:3000/_path");
            console.log(response);
            if (response.ok && response.status === 200) {
                const data = await response.json();
                if (data instanceof Array) {
                    fileText = "";
                    directoryContents = data;
                    console.log("directoryContents:", directoryContents);
                    console.log("file:", fileText);
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
        fileText = null;
        directoryContents = null;
    }
</script>

<main>
    <div class="title-container">
        <div>
            <h1 class="title text-muted-foreground bg-accent">steel</h1>
            <div class="text-xs text-muted-foreground justify-self-center">v0.1.0</div>
        </div>
    </div>

    {#if directoryContents !== null}
        <div class="directory-view-container">
            <DirectoryView bind:directory={directoryContents} />
        </div>
    {/if}
</main>

<style scoped lang="postcss">
    main {
        display: grid;
        place-items: center;
        width: 100%;
    }
    .title-container {
        display: grid;
        place-items: center;
        height: 25vh;
        width: 100%;
        margin-top: 60px;
    }
    .title {
        padding: 0.1em 0.5em;
        color: var(--muted-foreground);
        border-radius: 5px;
        margin-bottom: 0.5em;
    }
    .directory-view-container {
        display: grid;
        place-items: center;
        width: 90%;
        place-self: center;
        padding-bottom: 5em;
        max-width: 500px;
    }
</style>

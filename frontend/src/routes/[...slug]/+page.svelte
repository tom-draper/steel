<script lang="ts">
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import Nav from "../../lib/components/Nav.svelte";
    import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import DirectoryView from "$lib/components/DirectoryView.svelte";

    type File = {
        path: string;
        is_directory: boolean;
    };

    let updateID: number;
    let slug: string;
    let fileText: string | null = null;
    let directoryContents: File[] | null = null;

    $: slug = $page.params.slug;

    onMount(async () => {
        fetchPath(slug);
    });

    $: if (slug) {
        console.log("slug:", slug);
        fetchPath(slug);
    }

    async function fetchPath(path: string) {
        try {
            const response = await fetch("http://localhost:3000/_path/" + path);
            if (response.ok && response.status === 200) {
                const data = await response.json();
                const isDirectory = data instanceof Array;
                if (isDirectory) {
                    directoryContents = data;
                    fileText = "";
                    console.log("directoryContents:", directoryContents);
                    console.log("file:", fileText);
                } else {
                    fileText = data;
                    directoryContents = null;
                    setTimeout(() => {
                        autoResize({
                            target: document.getElementById("fileText"),
                        });
                    });
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

    function handleInput(event) {
        autoResize(event);
        triggerUpdate();
    }

    function triggerUpdate() {
        clearInterval(updateID);
        updateID = setTimeout(() => {
            console.log("Posting update...");
        }, 1000);
    }

    function autoResize(event: any) {
        const textarea = event.target;
        if (textarea) {
            textarea.style.height = "auto";
            textarea.style.height = `${textarea.scrollHeight}px`; // Set the height to the scrollHeight of the textarea
        }
    }

    function resetState() {
        fileText = null;
        directoryContents = null;
        console.log(directoryContents);
    }
</script>

<main>
    <Nav bind:path={slug}></Nav>

    {#if directoryContents !== null}
        <div class="directory-view-container">
            <DirectoryView bind:nodes={directoryContents} />
        </div>
    {:else if fileText !== null}
        <div class="text-container">
            <Textarea
                id="fileText"
                class="resize-none"
                bind:value={fileText}
                placeholder="Empty file..."
                oninput={handleInput}
                onpaste={handleInput}
            />
        </div>
    {:else if fileText !== null || directoryContents !== null}
        <div class="file-not-found text-muted-foreground text-md">
            File not found.
        </div>
    {/if}
</main>

<style scoped lang="postcss">
    :global(textarea) {
        margin-top: 8em !important;
        margin-bottom: 5em !important;
        width: 80%;
        max-width: 800px;
        font-size: 1.8em;
        padding: 1em;
        border: none;
        border-radius: 5px;
        background: transparent !important;
    }
    main {
        font-family:
            system-ui,
            -apple-system,
            BlinkMacSystemFont,
            "Segoe UI",
            Roboto,
            Oxygen,
            Ubuntu,
            Cantarell,
            "Open Sans",
            "Helvetica Neue",
            sans-serif;
    }
    .text-container {
        display: grid;
        place-items: center;
        margin: 0 0.5em;
    }
    .directory-view-container {
        display: grid;
        place-items: center;
        width: 90%;
        place-self: center;
        padding-top: 25vh;
        padding-bottom: 5em;
        max-width: 500px;
    }
    .file-not-found {
        display: grid;
        place-items: center;
        height: 80vh;
        font-size: 1em;
    }
</style>

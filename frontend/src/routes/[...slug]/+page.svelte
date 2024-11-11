<script lang="ts">
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import Nav from "../../lib/components/Nav.svelte";
    import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import DirectoryView from "$lib/components/DirectoryView.svelte";

    let updateID: number;
    let fileText: string = "";
    let slug: string;
    let directoryContents: string[] | null = null;

    $: slug = $page.params.slug;

    onMount(async () => {
        fetchPath(slug);
    });

    $: if (slug) {
        console.log('slug:', slug);
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
                } else {
                    fileText = data;
                    directoryContents = null;
                    setTimeout(() => {
                        autoResize({
                            target: document.getElementById("fileText"),
                        });
                    });
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
        fileText = "";
        directoryContents = null;
        console.log(directoryContents)
    }
</script>

<main>
    <Nav bind:path={slug}></Nav>

    {#if directoryContents !== null}
        <div class="directory-view-container">
            <DirectoryView bind:directory={directoryContents} />
        </div>
    {:else}
        <div class="text-container">
            <Textarea
                id="fileText"
                bind:value={fileText}
                placeholder="Empty file..."
                oninput={handleInput}
                onpaste={handleInput}
            />
        </div>
    {/if}
</main>

<style scoped lang="postcss">
    :global(textarea) {
        margin-top: 30vh;
        width: 80%;
        max-width: 800px;
        font-size: 1.8em;
        padding: 10px;
        border: none;
        border-radius: 5px;
    }
    main,
    textarea {
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
    h3 {
        padding: 0 0.5em;
    }
    .dynamic-textarea {
        overflow: hidden; /* Hide overflow to allow auto-resizing */
        min-height: 100px;
        width: 100%;
    }
    .hidden {
        display: none;
    }
</style>

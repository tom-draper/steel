<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { goto } from "$app/navigation";
    import * as Command from "$lib/components/ui/command";

    let finderOpen = false;
    let search: string;
    let options: string[];

    export let files;

    onMount(async () => {
        // listen for command + J to open the finder
        window.addEventListener("keydown", handleKeyDown);

        onDestroy(() => {
            window.removeEventListener("keydown", handleKeyDown);
        });
    })

    function handleKeyDown(event) {
        if (event.key === "j" && event.ctrlKey) {
            event.preventDefault();
            toggleFinder();
        }
    }

    function toggleFinder() {
        finderOpen = !finderOpen;
        if (finderOpen) {
            const input = document.querySelector("input");
            console.log(input);
            if (input) {
                input.focus();
            }
        } else {
            search = "";
            options = [];
        }
    }

    function handleSearch() {
        if (!search) {
            options = [];
            return;
        }

        let filtered = [];
        for (let i = 0; i < files.length; i++) {
            if (files[i].toLowerCase().includes(search.toLowerCase())) {
                filtered.push(files[i]);
            }
        }
        if (filtered.length > 10) {
            options = filtered.slice(0, 10);
        } else {
            options = filtered;
        }
    }

    function getFilename(path: string) {
        return path.split("/").pop();
    }

    function getDirectory(path: string) {
        return path.split("/").slice(0, -1).join("/");
    }

    async function fetchMap() {
        try {
            const response = await fetch("http://localhost:3000/_map");
            if (response.ok && response.status === 200) {
                const data = await response.json();
                if (data instanceof Array) {
                    return data;
                }
            }
        } catch (error) {
            console.error(error);
        }
        return [];
    }

    function searchResult(path: string, search: string) {
        const index = path.toLowerCase().indexOf(search.toLowerCase());
        if (index === -1) {
            return `<span class="text-muted-foreground">${path}</span>`;
        }
        return (
            '<span class="text-muted-foreground">' +
            path.substring(0, index) +
            "</span>" +
            "<span>" +
            path.substring(index, index + search.length) +
            "</span>" +
            '<span class="text-muted-foreground">' +
            path.substring(index + search.length) +
            "</span>"
        );
    }
</script>

<div
    class:hidden={!finderOpen}
    class="bg-white mt-[68px] fixed w-full h-full top-0"
>
    <div class="directory-view-container">
        <Command.Root shouldFilter={false}>
            <Command.Input
                placeholder="Search..."
                bind:value={search}
                oninput={handleSearch}
            />
            <Command.List style="min-height: fit-content">
                {#each options as path (path)}
                    <Command.Item
                        value={path}
                        style="cursor: pointer; white-space: pre"
                        onclick={() => {
                            toggleFinder();
                            goto("/" + path);
                        }}
                        ><p class="whitespace-pre-wrap">
                            {@html searchResult(path, search)}
                            <!-- <span class="text-muted-foreground">{getDirectory(path)}/</span><span>{getFilename(path)}</span> -->
                        </p></Command.Item
                    >
                {/each}
            </Command.List>
        </Command.Root>
    </div>
</div>

<style scoped lang="postcss">
    .hidden {
        display: none;
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
</style>

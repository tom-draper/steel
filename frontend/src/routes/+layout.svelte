<script lang="ts">
    import "../app.css";
    import Finder from "$lib/components/Finder.svelte";
    import Actions from "$lib/components/Actions.svelte";
    import Network from "$lib/components/Network.svelte";
    import { onMount } from "svelte";
    import { page } from "$app/stores";

    export let children;

    let allPaths: string[] = [];
    let files: string[] = [];
    let settings: { showNetwork: boolean } = { showNetwork: true };

    $: files = filterByPath(allPaths, $page.params.slug).sort((a, b) => a.length - b.length).slice(0, 500);
    $: allPaths;
    $: settings;

    onMount(async () => {
        allPaths = await fetchMap();
    });

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

    function filterByPath(files: string[], path: string) {
        if (!path) {
            return files;
        }
        return files.filter((p) => p.startsWith(path));
    }
</script>

{@render children()}
<Finder bind:files={allPaths} />
<Actions bind:settings />
<div class:hidden={!settings.showNetwork}>
    <Network bind:files={files} />
</div>

<style scoped lang="postcss">
    .hidden {
        display: none;
    }
</style>

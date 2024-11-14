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
    let settings: { showNetwork: boolean } = { showNetwork: false };

    $: files = filterByPath(allPaths, $page.params.slug);
    $: allPaths;
    $: settings;

    onMount(async () => {
        allPaths = await fetchMap();
        console.log(allPaths.length);
        // files = filterByPath(allPaths, "/");
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
        console.log(path)
        console.log('here', files.filter((p) => p.startsWith(path)));
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

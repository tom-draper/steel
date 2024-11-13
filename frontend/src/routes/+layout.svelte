<script lang="ts">
    import "../app.css";
    import Finder from "$lib/components/Finder.svelte";
    import Actions from "$lib/components/Actions.svelte";
    import Network from "$lib/components/Network.svelte";
    import { onMount } from "svelte";

    export let children;

    let files: {path: string, is_directory: boolean}[] = [];
    let settings: {showNetwork: boolean} = {showNetwork: false};

    $: files;
    $: settings;

    onMount(async () => {
        files = await fetchMap();
        console.log(files.length)
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
</script>

{@render children()}
<Finder bind:files={files} />
<Actions bind:settings={settings} />
<div class:hidden={!settings.showNetwork} >
    <Network bind:files={files} />
</div>

<style scoped lang="postcss">
    .hidden {
        display: none;
    }
</style>
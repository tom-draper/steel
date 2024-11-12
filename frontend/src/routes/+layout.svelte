<script lang="ts">
    import "../app.css";
    import Finder from "$lib/components/Finder.svelte";

	let { children } = $props();
</script>

{@render children()}
<Finder />
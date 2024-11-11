<script lang="ts">
	import { goto } from "$app/navigation";
	import * as Command from "$lib/components/ui/command";

	function getFilename(path: string) {
		return path.split("/").pop();
	}

	let options: string[];
	let search: string = "";

	function handleSearch() {
		let filtered = [];
		for (let i = 0; i < directory.length; i++) {
			if (directory[i].toLowerCase().includes(search.toLowerCase())) {
				filtered.push(directory[i]);
			}
		}
		options = filtered;
	}

	export let directory: string[];

	$: options = directory;
</script>

<Command.Root shouldFilter={false}>
	<Command.Input
		placeholder="Search..."
		bind:value={search}
		oninput={handleSearch}
	/>
	<Command.List style="min-height: fit-content">
		<Command.Empty>No results found.</Command.Empty>
		{#each options as path (path)}
			<Command.Item
				value={path}
				style="cursor: pointer"
				onclick={() => goto("/" + path)}
				>{getFilename(path)}</Command.Item
			>
		{/each}
	</Command.List>
</Command.Root>

<script lang="ts">
	import { goto } from "$app/navigation";
	import * as Command from "$lib/components/ui/command";

	function getFilename(path: string) {
		return path.split("/").pop();
	}

	let options: string[];


	export let directory: string[];

	$: options = directory;
</script>

<Command.Root shouldFilter={false}>
		<div class="mt-[44px]">
			<Command.Separator />
		</div>
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

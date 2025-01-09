<script lang="ts">
	import { goto } from "$app/navigation";
	import * as Command from "$lib/components/ui/command";
	import { File, Folder } from 'lucide-svelte';

	function getFilename(path: string) {
		return path.split("/").pop();
	}

	export let nodes: { path: string; is_directory: boolean }[];

	let options: { path: string; is_directory: boolean }[];
	$: options = nodes.sort((a, b) =>
		a.is_directory === b.is_directory ? 0 : a.is_directory ? -1 : 1,
	);
</script>

<Command.Root shouldFilter={false} class="z-50" style="background: rgba(255, 255, 255, 0.7);">
	<div class="mt-[44px]">
		<Command.Separator />
	</div>
	<Command.List style="min-height: fit-content">
		<Command.Empty class="text-muted-foreground">No results found.</Command.Empty>
		{#each options as option (option.path)}
			<Command.Item
				value={option.path}
				class="item"
				style="cursor: pointer;"
				onclick={() => goto("/" + option.path)}
			>
				<div class="mr-2">
					{#if option.is_directory}
						<Folder class="size-4 opacity-75" />
					{:else}
						<File class="size-4 opacity-75" />
					{/if}
				</div>
				{getFilename(option.path)}</Command.Item
			>
		{/each}
	</Command.List>
</Command.Root>

<style scoped lang="postcss">
	:global(.item:hover) {
		background: rgba(241, 245, 249, 0.8);
	}
</style>
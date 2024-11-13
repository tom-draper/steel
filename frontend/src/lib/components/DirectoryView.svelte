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

<Command.Root shouldFilter={false}>
	<div class="mt-[44px]">
		<Command.Separator />
	</div>
	<Command.List style="min-height: fit-content">
		<Command.Empty class="text-muted-foreground">No results found.</Command.Empty>
		{#each options as option (option.path)}
			<Command.Item
				value={option.path}
				style="cursor: pointer"
				onclick={() => goto("/" + option.path)}
			>
				<div class="mr-2">
					{#if option.is_directory}
						<!-- <svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke-opacity="0.8"
							stroke="currentColor"
							class="size-4"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M2.25 12.75V12A2.25 2.25 0 0 1 4.5 9.75h15A2.25 2.25 0 0 1 21.75 12v.75m-8.69-6.44-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z"
							/>
						</svg> -->
						<Folder class="size-4 opacity-75" />
					{:else}
						<File class="size-4 opacity-75" />
						<!-- <svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke-opacity="0.8"
							stroke="currentColor"
							class="size-4"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z"
							/>
						</svg> -->
					{/if}
				</div>
				{getFilename(option.path)}</Command.Item
			>
		{/each}
	</Command.List>
</Command.Root>

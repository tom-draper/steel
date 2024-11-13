<script lang="ts">
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { onMount, onDestroy } from "svelte";

    let nav: HTMLElement;
    let timeoutID: number;
    let parts: string[] = [];

    $: if (path) {
        parts = path.split("/");
    }

    onMount(() => {
        // Add the event listener
        window.addEventListener("mousemove", handleMouseMove);

        // Cleanup event listener on component destroy
        onDestroy(() => {
            window.removeEventListener("mousemove", handleMouseMove);
        });
    });

    function handleMouseMove(event: MouseEvent) {
        if (!nav) {
            return;
        }

        nav.style.opacity = "1";
        if (timeoutID) {
            clearTimeout(timeoutID);
        }
        timeoutID = setTimeout(() => {
            if (nav) {
                nav.style.opacity = "0";
            }
        }, 3000);
    }

    export let path: string;
</script>

<nav bind:this={nav}>
    <div class="directory">
        <Breadcrumb.Root>
            <Breadcrumb.List>
                <Breadcrumb.Item>
                    <Breadcrumb.Link href="/">Home</Breadcrumb.Link>
                </Breadcrumb.Item>

                <Breadcrumb.Separator />

                {#if parts.length > 2}
                    <Breadcrumb.Item>
                        <DropdownMenu.Root>
                            <DropdownMenu.Trigger
                                class="flex items-center gap-1"
                                aria-label="Toggle menu"
                            >
                                <Breadcrumb.Ellipsis class="h-4 w-4" />
                            </DropdownMenu.Trigger>
                            <DropdownMenu.Content align="start">
                                {#each parts.slice(0, -2).reverse() as _, i}
                                    <DropdownMenu.Item
                                        class="cursor-pointer p-0"
                                    >
                                        <a
                                            class="w-full px-2 py-1.5"
                                            href={"/" +
                                                parts
                                                    .slice(
                                                        0,
                                                        parts.length - i - 2,
                                                    )
                                                    .join("/")}
                                        >
                                            {parts[parts.length - i - 3]}
                                        </a>
                                    </DropdownMenu.Item>
                                {/each}
                            </DropdownMenu.Content>
                        </DropdownMenu.Root>
                    </Breadcrumb.Item>

                    <Breadcrumb.Separator />
                {/if}

                {#if parts.length > 1}
                    <Breadcrumb.Item>
                        <Breadcrumb.Link
                            href={"/" +
                                parts.slice(0, parts.length - 1).join("/")}
                            >{parts[parts.length - 2]}</Breadcrumb.Link
                        >
                    </Breadcrumb.Item>

                    <Breadcrumb.Separator />
                {/if}

                {#if parts.length > 0}
                    <Breadcrumb.Item>
                        <Breadcrumb.Page
                            >{parts[parts.length - 1]}</Breadcrumb.Page
                        >
                    </Breadcrumb.Item>
                {/if}
            </Breadcrumb.List>
        </Breadcrumb.Root>
    </div>
</nav>

<style scoped lang="postcss">
    nav {
        padding: 1em 2em;
        opacity: 0;
        transition: opacity 0.5s;
    }
    .directory {
        align-items: center;
        display: flex;
        font-size: 12px;
        height: 36px;
    }
</style>

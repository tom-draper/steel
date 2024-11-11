<script lang="ts">
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";

    let parts: string[] = [];
    
    $: if (path) {
        parts = path.split("/");
    }

    export let path: string;
</script>

<nav>
    <div class="directory">
        <Breadcrumb.Root>
            <Breadcrumb.List>
                <Breadcrumb.Item>
                    <Breadcrumb.Link href="/">Home</Breadcrumb.Link>
                </Breadcrumb.Item>

                <Breadcrumb.Separator />

                {#if parts.length > 2}
                    <Breadcrumb.Item>
                        <Breadcrumb.Ellipsis />
                    </Breadcrumb.Item>

                    <Breadcrumb.Separator />
                {/if}

                {#if parts.length > 1}
                    <Breadcrumb.Item>
                        <Breadcrumb.Link href={"/" + parts.slice(0, parts.length - 1).join("/")}>{parts[parts.length - 2]}</Breadcrumb.Link>
                    </Breadcrumb.Item>

                    <Breadcrumb.Separator />
                {/if}

                {#if parts.length > 0}
                    <Breadcrumb.Item>
                        <Breadcrumb.Page>{parts[parts.length - 1]}</Breadcrumb.Page>
                    </Breadcrumb.Item>
                {/if}

            </Breadcrumb.List>
        </Breadcrumb.Root>
    </div>
</nav>

<style scoped lang="postcss">
    .directory {
        margin: 1em 2em;
        align-items: center;
        display: flex;
        font-size: 12px;
        height: 36px;
    }
</style>

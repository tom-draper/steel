<script lang="ts">
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    
    onMount(() => {
        // print the page url
        console.log($page.url);
    });

</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

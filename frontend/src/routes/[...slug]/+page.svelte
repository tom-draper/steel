<script lang="ts">
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    
    // This will hold the slug value, such as "my-awesome-post"
    let text: string;
    let slug = $page.params.slug;

    onMount(async () => {
        console.log(slug);
        const response = await fetch('http://localhost:3000/_files/' + slug)
        if (response.ok && response.status === 200) {
            const data = await response.json()
            text = data;
        }
    });
</script>

<main>
    <h3>{slug}</h3>

    <div class="text-container">
        <textarea name="" id="">{text}</textarea>
    </div>
</main>


<style>
    main, textarea {
        font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif
    }
    .text-container {
        display: grid;
        place-items: center;
        width: 100%;
    }
    textarea {
        margin-top: 30vh;
        width: fit-content;
        max-width: 800px;
        font-size: 2em;
        height: 100vh;
        padding: 10px;
        border: none;
        border-radius: 5px;
    }
    h3 {
        padding: 0 0.5em;
    }
</style>
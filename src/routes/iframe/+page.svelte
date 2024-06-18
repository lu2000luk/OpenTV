<script lang="ts">
    //@ts-nocheck
    import { page } from '$app/stores'
    const url = $page.url.searchParams.get('url')
    async function fetchContent() {
        const res = await fetch(url)
        content = await res.text()
    }
    let content = "Loading...";
    fetchContent()
</script>
{@html content}
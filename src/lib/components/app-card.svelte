<script>
// @ts-nocheck

    import { onMount } from "svelte";
    import { invoke } from '@tauri-apps/api/core'

    export let title = "My App";
    export let index = 0;
    export let url = "https://google.com/"

    let card;

    onMount(() => {
        if (index === 0) {
            card.focus();
        }
    });
    
    function navigate() {
      invoke('navigate', { url: url })
      console.log("Moving to "+url)
    }

    let loader = false;
</script>

<style>
    :focus { outline: none; }
</style>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<div class="group w-[300px] h-[200px] rounded-xl shadow-lg cursor-pointer transition-all duration-300 ease-in-out transform-gpu focus:scale-105 focus:shadow-2xl py-4 px-3" tabindex="-{index}" bind:this={card} id="app_card_index_{index}" data-selectlist="true" data-prev="app_card_index_{index-1}" data-next="app_card_index_{index+1}" on:focus={navigate}>
  <div class="relative w-full h-full rounded-xl overflow-hidden group-focus:rotate-3 group-focus:-translate-y-1 group-focus:translate-x-1 transition-transform duration-300 ease-in-out">
    {#if loader}
      <iframe src="/loading" frameborder="0" title="Loader" width="300" height="200" class="absolute inset-0 w-full h-full object-cover pointer-events-none" style="aspect-ratio: 300 / 200; object-fit: cover;">Loading...</iframe>
    {:else}
      <img
        src="https://unsplash.it/300/200?seed={index}"
        alt="App"
        class="absolute inset-0 w-full h-full object-cover"
        width="300"
        height="200"
        style="aspect-ratio: 300 / 200; object-fit: cover;"
      />
    {/if}
    <div class="absolute inset-0 flex justify-center items-center opacity-0 group-focus:opacity-100 transition-opacity duration-300 ease-in-out">
      <h2 class="absolute bottom-4 right-4 bg-black bg-opacity-50 px-4 py-2 rounded-md text-white">{title}</h2>
    </div>
  </div>
</div>
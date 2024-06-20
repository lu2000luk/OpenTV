<script>
// @ts-nocheck

  import AppCard from "$lib/components/app-card.svelte";
  import { onMount } from "svelte";
  import { invoke } from '@tauri-apps/api/core'

  import ChevronsRight from "lucide-svelte/icons/chevrons-right"

  onMount(() => {
    function keyPressed(e) {
      var srcElement = e.target;    // get the element that fired the onkeydown function
      var dataset = false;
      var selectList = false;
      var next = "";
      var prev = "";

      console.log(srcElement);
      console.log(e.keyCode);

      if (srcElement.dataset) {        // can we use HTML5 dataset?
        dataset = true;              // remember for later
        // is this an element for which we care
        if (srcElement.dataset.selectlist == 'true') {
            selectList = true;
        }
      } else {    // can't use HTML5 dataset, use getAttribute
        if (srcElement.getAttribute('data-selectlist') == 'true') {
            selectList = true;
        }
      }
    // is it a select element and the user pressed either up arrow or down arrow
      if (selectList && (e.keyCode == '38' || e.keyCode == '40')) {
        // get the next and prev navigation options for this element
        if (dataset) {
            next = srcElement.dataset.next;
            prev = srcElement.dataset.prev;
        } else {
            next = srcElement.getAttribute('data-next');
            prev = srcElement.getAttribute('data-prev');
        }
        // up arrow was pressed and a prev element is defined
        if (e.keyCode == '38' && prev != '') {
            document.getElementById(prev).focus();
        }
        // down arrow was pressed and a next element is defined
        if (e.keyCode == '40' && next != '') {
            document.getElementById(next).focus();
        }
        // don't do native processing of the up or down arrow (page scrolling)
        e.preventDefault;
      }

      if (e.keyCode == '37' || e.keyCode == '39') {
        if (e.keyCode == '37') {
          invoke('resize_webview', { size: webview_sizes.closed });
          invoke('reposition_webview', { pos: webview_positions.closed });
          opened = false;
        } else {
          invoke('resize_webview', { size: webview_sizes.open });
          invoke('reposition_webview', { pos: webview_positions.open });
          opened = true;
        }
      }
      }

      document.onkeydown = keyPressed;
    }
  );

  var opened = true;

  var screenSize;
  invoke('get_size').then((message) => {screenSize = message; console.log(screenSize); generate_webview_sizes();});

  var webview_sizes = {};
  var webview_positions = {};

  function generate_webview_sizes() {
    webview_sizes = {
      "closed": {
        width: screenSize.width - 25,
        height: screenSize.height
      },
      "open": {
        width: screenSize.width / 5 * 4,
        height: screenSize.height
      }
    }

    webview_positions = {
      "closed": {
        x: 25,
        y: 0
      },
      "open": {
        x: screenSize.width / 5,
        y: 0
      }
    }
  }


</script>

<div class="flex h-full w-full">
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="flex h-full flex-col justify-center cursor-pointer" on:click={() => {
    invoke('resize_webview', { size: webview_sizes.open });
    invoke('reposition_webview', { pos: webview_positions.open });
    opened = true;
  }}>
    {#if !opened}
      <ChevronsRight />
    {/if}
  </div>

    <div class="scene h-full flex flex-col-reverse p-4 w-1/5 justify-center">
      <div class="flex w-full overflow-x-hidden overflow-y-hidden scroll-smooth">
        <div class="">
          <AppCard index={0} url="https://google.com" title="Google"/>
          <AppCard index={1} url="http://pescebeddo.com" title="Pesce Beddo"/>
          <AppCard index={2} url="http://netflix.com/browse" title="Netflix"/>
          <AppCard index={3} url="https://www.disneyplus.com/home" title="Disney+"/>
          <AppCard index={4}/>
          <AppCard index={5}/>
          <AppCard index={6}/>
          <AppCard index={7}/>
          <AppCard index={8}/>
        </div>
      </div>
    </div>

  <div class="mainview rounded w-full h-full bg-transparent">
    <!-- <iframe src="http://example.com" title="App Window" frameborder="0" class="w-full h-full">Oppsie, iFrames are not supported.</iframe> -->
  </div>
</div>

<script>
// @ts-nocheck

  import AppCard from "$lib/components/app-card.svelte";
  import { onMount } from "svelte";

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
}
    document.onkeydown = keyPressed;
  });
</script>

<div class="flex h-full w-full">
  <div class="scene h-full flex flex-col-reverse p-4 w-1/5 justify-center">
    <div class="flex w-full overflow-x-hidden overflow-y-hidden scroll-smooth">
      <div class="">
        <AppCard index={0}/>
        <AppCard index={1}/>
        <AppCard index={2}/>
        <AppCard index={3}/>
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

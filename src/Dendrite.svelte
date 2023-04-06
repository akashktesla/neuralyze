<script lang="ts">
  import {mode} from "./store";
  import type dendrite from "./dendrite";
  import { coords } from "./store";
  import { selDendrite } from "./store";
  import { nodeClick } from "./store";
  import { selType } from "./store";
  import { isSel } from "./store";
  import { inputValue } from "./store";
  export let line:dendrite;
  export let index;
  $: x1 = line.node1.x
  $: x2 = line.node2.x

  $: y1 = line.node1.y
  $: y2 = line.node2.y
  $: x3 = (x1+x2)/2
  $: y3 = (y1+y2)/2

  function measureText(text,textSize) {
    let span = document.createElement('span');
    span.style.fontSize = `${textSize}px`;
    span.style.visibility = 'hidden';
    span.textContent = text;
    document.body.appendChild(span);
    var textWidth = span.getBoundingClientRect().width;
    document.body.removeChild(span);
    return textWidth;
  }

  function onClickDendrite(event){
    const dendrite_id = event.target.dataset.value;
    $selDendrite = dendrite_id;
    $selType = "dendrite"
    $nodeClick = true;
    $isSel = true;
    if ($mode === "Edit"){
      $mode = "edit_dendrite";
      $inputValue = line.data;
    }
  }


  function handleDblClickDendrite(){
    console.log("double click");
    console.log("node"+$selDendrite+"is selected")
    $inputValue = line.data;
    $mode = "edit_dendrite";
  }


</script>
<style>
  line{
    stroke-width: 5px;
    stroke: white;
  }
  text{
    margin:0;
  }
</style>

<line 
  x1={x1+$coords.x}
  y1={y1+$coords.y}
  x2={x2+$coords.x}
  y2={y2+$coords.y} 
  data-value={index}
  on:click={onClickDendrite}
  on:dblclick={handleDblClickDendrite}
  on:keypress={()=>{}}
  />

<rect 
  x = {(x3 + $coords.x)-2}
  y = {y3 + $coords.y -13}
  width = {measureText(line.data,15)+4} height = 15 
  data-value={index}
  on:mousedown={onClickDendrite}
  on:dblclick={handleDblClickDendrite}
  on:keypress={()=>{}}
  fill = white/>

<text 
  x = {((line.node1.x+line.node2.x)/2 + $coords.x)}
  y = {(line.node1.y+line.node2.y)/2 + $coords.y}
  data-value={index}
  on:mousedown={onClickDendrite}
  on:dblclick={handleDblClickDendrite}
  on:keypress={()=>{}}
  >
{line.data}
</text>


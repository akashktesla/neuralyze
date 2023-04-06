<script lang="ts">
  import {mode} from "./store";
  import { hData } from "./store";
  import type  node from "./node";
  import { selType } from "./store";
  import {selNode} from "./store"
  import {nodeClick} from "./store"
  import {preSelNode} from "./store"
  import {isSel} from "./store"
  import {inputValue} from "./store"
  import { coords } from "./store";
  export let _node:node;
  export let data:string;
  export let index:string;
  let cx = _node.x;
  let cy = _node.y;

  function handleNodeClick(event:any){
    console.log("Node click triggered");
    const node_id = event.target.dataset.value;
    $nodeClick = true;
    $selType = "node";
    $preSelNode = $selNode;
    $selNode = node_id;
    $isSel = true;
    $hData=data;
    if ($mode ==="Edit"){
      $mode = "edit_node";
      $inputValue = data;
    }

  }

  function handleDblClickNode(){
    console.log("double click");
    console.log("node"+$selNode+"is selected")
    $inputValue = data;
    $mode = "edit_node";
  }
</script>
<style>
  circle{
    stroke: blueviolet;
    fill: #000000;
    stroke-width:5px
  }
  text{
    position: absolute;
    fill:white;
    margin:0;
  }
</style>

<circle 
  cx = {cx+$coords.x}
  cy = {cy+$coords.y}
  r = "40" 
  data-value = {index} 
  on:mousedown = {handleNodeClick}
  on:dblclick = {handleDblClickNode}
  on:keypress = {()=>{}}
  on:mouseover={()=>{$hData=data}}
  on:focus={()=>{}}
/>

<text 
  x = {cx+$coords.x}
  y = {cy+$coords.y}
  on:mousedown = {handleNodeClick}
  on:dblclick = {handleDblClickNode}
  on:keypress={()=>{}}
  data-value={index} 
  text-anchor="middle"
  alignment-baseline="auto" 
  on:mouseover={()=>{$hData=data}}
  on:focus={()=>{}}
>
{_node.data}
</text>

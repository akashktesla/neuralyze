<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {mousePos} from "./store";
  import node from "./node"
  import dendrite from "./dendrite";
  import Node from "./Node.svelte";
  import {mode} from "./store"
  import {selNode} from "./store"
  import { selType } from "./store";
  import {selDendrite} from "./store"
  import {coords} from "./store"
  import {preSelNode} from "./store"
  import {isSel} from "./store"
  import { inputValue } from "./store";
  import { nodeClick } from "./store";
  import Dendrite from "./Dendrite.svelte";

  let nodes:{[key:string]:node} = {};
  let lines:{[key:string]:dendrite} = {};
  let neurons = {};
  let update = false;
  let next_node_id = 0;
  let next_line_id = 0;

  let isDragging = false;
  let x = 0;
  let y = 0;

  $: nodes = nodes;

  function handleMouseDown() {
    console.log("mouse down");
    isDragging = true;
    if ($mode == "move"){
      x = $mousePos.x;
      y = $mousePos.y;
    }
  }

  function handleMouseMove() {
    console.log("mouse move");
    if (isDragging) {
      if ($mode ==="move"){
        const dx = $mousePos.x - x;
        const dy = $mousePos.y - y;
        x = $mousePos.x;
        y = $mousePos.y;
        $coords = { x:$coords.x+dx, y:$coords.y+dy };
      }
      if($mode ==="rope"){
        const cx = $mousePos.x-$coords.x;
        const cy = $mousePos.y-$coords.y;
        nodes[$selNode].x = cx;
        nodes[$selNode].y = cy;
      }
      update = !update;
    }
  }

  function handleMouseUp() {
    console.log("mouse up");
    isDragging = false;
  }

  function handleClick(){
    if (!$nodeClick){
      $isSel = false;
    }
    console.log("handleClick triggered");
      switch ($mode){
        case "node":
          spawnCircle();
          break;
        case "edit_node":
          nodes[$selNode].data = $inputValue;
          $mode = "move"
          $isSel = false;
          $preSelNode = $selNode;
          $selNode = undefined;
          break;
        case "edit_dendrite":
          lines[$selDendrite].data = $inputValue;
          $mode = "move"
          $isSel = false;
          $selDendrite = undefined;
          break;
        default:
          break;
      }
    $nodeClick = false;
    }

  function deleteNode(node_id:number){
    if ($isSel){
      let neuron = neurons[node_id];
      console.log("neuron",neuron);
      let _len = neuron.dnd.length;
      for ( var i=0; i<_len; i++){
        console.log("i",i);
        deleteDendrite(neuron.dnd[i]);
      }
      delete nodes[node_id];
      delete neurons[node_id];
      $isSel = false;
      $preSelNode = $selNode;
      $selNode = undefined;
      console.log(nodes);
    }
  }

  function deleteDendrite(node_id:number){
    if ($isSel){
      delete lines[node_id];
      $selDendrite = undefined;
      console.log(lines);
    }
  }

  function handleRClick(event:MouseEvent){
    event.preventDefault();
    console.log("right click");
  }

  function handleKeyDown(event:KeyboardEvent){
    if (event.key==="Escape"){
      $mode = "command";
    }
  }
  function handleKeyPress(event:KeyboardEvent) {
    console.log("keypress: ",event.key);
    if (event.key==="f"){
      $mode = "move";
    }
    if (event.key==="d"){
      if ($selType ==="node"){
        deleteNode($selNode);
      }
      if ($selType ==="dendrite"){
        deleteDendrite($selDendrite);
      }
      update = !update;
    }
    if (event.key==="a"){
      $mode = "node";
    }
    if (event.key==="t"){
      if ($preSelNode!=undefined && $selNode!=undefined){
        spawnLine();
      }
    }
    if(event.key==="r"){
      $mode = "rope";
    }
    if (event.key==="p"){
      invoke('greet', { })
      console.log("neural network:",neurons);
      console.log("nodes",nodes);
      console.log("lines",lines);
    }
  }

  function handlekeyPressInput(event:KeyboardEvent){
    if (event.key==="Enter"){
      console.log("enter is pressed: "+$inputValue);
      if ($mode==="edit_node"){
        nodes[$selNode].data = $inputValue;
        $mode = "move"
        $isSel = false;
        $preSelNode = $selNode;
        $selNode = undefined;
      }
      if ($mode==="edit_dendrite"){
        lines[$selDendrite].data = $inputValue;
        $mode = "move"
        $isSel = false;
        $selDendrite = undefined;
      }
    }
  }

  function spawnCircle() {
    if (!$nodeClick){
      const cx = $mousePos.x-$coords.x;
      const cy = $mousePos.y-$coords.y;
      const data = next_node_id.toString();
      nodes[next_node_id] = new node(cx,cy,data);
      neurons[next_node_id] = {weight:0,in:[],out:[],dnd:[]}
      next_node_id +=1;
    }
  }

  function spawnLine(){
    let node1 = nodes[$preSelNode];
    let node2 = nodes[$selNode];
    lines[next_line_id] = new dendrite(node1,node2,"");
    //updating neural network
    neurons[$preSelNode].out.push($selNode);
    neurons[$selNode].in.push($preSelNode);
    //to manipulate dendrites later (delete)
    neurons[$preSelNode].dnd.push(next_line_id);
    neurons[$selNode].dnd.push(next_line_id);
    next_line_id +=1;
    console.log("neurons",neurons,node1);
    console.log("lines",lines);
  }

</script>

<style>
  div{
    position: absolute;
    width:100%;
    height:100%;
    background-color: #181a1b;
  }
  svg{
    position: absolute;
    width:100%;
    height:100%;
  }

  div.main{
    position: absolute;
    color: white;
    background-color: #181a1b;
    width:100%;
    height:100%;
    padding:0;
    margin:0;
  }
  input{
    background-color: white;
    border-color:blueviolet;
    margin: 15px;
    border-width:5px;
    font-size: 30px;

  }
  .inputBar{
    display:flex;
    margin: 0;
    margin-left:5px;
    justify-content: flex-start;
    height:fit-content;
    width:fit-content;
    flex-direction:column;
  }
  .infoBar{
    height:fit-content;
    width:fit-content;
    font-size: 20px;
    right:10px;
    bottom:20px;
    border-width:5px;
    border-color:blueviolet;
  }
  p{
    color:white;
    margin: 5px;
  }
</style>

<div class="main" 
     on:click={handleClick} 
     tabindex=-1 
     on:keypress={handleKeyPress} 
     on:keydown={handleKeyDown}
     on:contextmenu={handleRClick} 
     on:mousedown={handleMouseDown} 
     on:mousemove={handleMouseMove} 
     on:mouseup={handleMouseUp}>
  <svg>
    {#key update}
    {#each Object.entries(lines) as [key,value]}
      <Dendrite 
      line = {value}
      index = {key}
      />
    {/each}
    {#each Object.entries(nodes) as [key,value]}
      <Node
      _node = {value}
      index = {key}
      />
    {/each}
    {/key}
  </svg>
</div>

<div class = "infoBar">
  <p> mode: {$mode} </p>
  <p> Selected Node: {$selNode} </p>
  <p> Prev Selected Node: {$preSelNode} </p>
  <p> Selected dendrite: {$selDendrite} </p>
  <p> Selected type: {$selType} </p>
  <p> is selected: {$isSel} </p>
  <p> coords: {$coords.x},{$coords.y} </p>
  <p> mouse pos: {$mousePos.x},{$mousePos.y} </p>
  <p> isDragging:{isDragging} </p>

</div>
{#if $mode.startsWith("edit")}
<div class="inputBar">
  <input bind:value={$inputValue} on:keypress={handlekeyPressInput}/>
</div>
{/if}

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { hData } from "./store";
  import {mousePos} from "./store";
  import node from "./node";
  import neuron from "./neuron";
  import dendrite from "./dendrite";
  import tdendrite from "./tdendrite";
  import rdendrite from "./rdendrite";
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
  import { tick } from "svelte";
  let inputBox:HTMLInputElement;
  let divv;
  let nodes:{[key:string]:node} = {};
  let lines:{[key:string]:dendrite} = {};
  let neuron_map:{[key:string]:neuron} = {};
  let tdendrite_map:{[key:string]:tdendrite} = {};
  let rdendrite_map:{[key:string]:rdendrite} = {};
  let neurons = {};
  let update = false;
  let next_node_id = 0;
  let next_line_id = 0;
  let flag_focus = false;

  $: ifocus(inputBox);
  async function ifocus(ib){
    if (ib!=null){
       ib.focus();
        }
    }


  let isDragging = false;
  let x = 0;
  let y = 0;

  $: nodes = nodes;



  function save(path:string){
      let data = "";
      for (const [key,value] of Object.entries(neuron_map)){
          let t_term = "["+value.t_term.join(",").toString()+"]"
          let r_term = "["+value.t_term.join(",").toString()+"]";
          let nstr = value.id.toString()+":"+value.value+":"+t_term+":"+r_term
          data+="|"+nstr;
      }
      data+="#";
      for (const [key,value] of Object.entries(tdendrite_map)){
        let tstr = value.id.toString()+":"+value.output.toString()+":"+value.weight.toString();
        data +="|"+tstr;
      }
      data+="#";
      for (const [key,value] of Object.entries(rdendrite_map)){
        let tstr = value.id.toString()+":"+value.input.toString()+":"+value.weight.toString();
        data +="|"+tstr;
      }
      console.log("data:",data);
      invoke('save',{path:path,data:data});

  }




  
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
          neuron_map[$selNode].value = $inputValue;
          $mode = "command"
          divv.focus();
          $isSel = false;
          $preSelNode = $selNode;
          $selNode = undefined;
          break;
        case "edit_dendrite":
          lines[$selDendrite].data = $inputValue;
          tdendrite_map[$selDendrite].weight = parseFloat($inputValue);
          rdendrite_map[$selDendrite].weight = parseFloat($inputValue);
          $mode = "command"
          divv.focus();
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
      delete neuron_map[node_id];
      $isSel = false;
      $preSelNode = $selNode;
      $selNode = undefined;
      console.log(nodes);
    }
  }

  function deleteDendrite(node_id:number){
    if ($isSel){
      delete lines[node_id];
      delete tdendrite_map[node_id];
      delete rdendrite_map[node_id];
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
      divv.focus();
    }
  }
  function handleKeyPress(event:KeyboardEvent) {
    console.log("keypress: ",event.key);
    if (event.key===":"){
        $mode = "edit_command";
    }
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
      console.log("nodes",nodes);
      console.log("lines",lines);
      console.log("neuron_map",neuron_map);
      console.log("tdendrite_map",tdendrite_map);
      console.log("rdendrite_map",rdendrite_map);

    }
    if (event.key==="g"){
      let data = "";
      for (const [key,value] of Object.entries(neuron_map)){
          let t_term = "["+value.t_term.join(",").toString()+"]"
          let r_term = "["+value.t_term.join(",").toString()+"]";
          let nstr = value.id.toString()+":"+value.value+":"+t_term+":"+r_term
          data+="|"+nstr;
      }
      data+="#";
      for (const [key,value] of Object.entries(tdendrite_map)){
        let tstr = value.id.toString()+":"+value.output.toString()+":"+value.weight.toString();
        data +="|"+tstr;
      }
      data+="#";
      for (const [key,value] of Object.entries(rdendrite_map)){
        let tstr = value.id.toString()+":"+value.input.toString()+":"+value.weight.toString();
        data +="|"+tstr;
      }
      console.log("data:",data);
      invoke('save',{path:"/home/aki/projects/projectD/neuralyze/test.ddb",data:data});
    }
  }

  function handleDblClick(){
    if (inputBox!=null){
      inputBox.focus();
    }
  }

  function handlekeyPressInput(event:KeyboardEvent){
    if (event.key==="Enter"){
      console.log("enter is pressed: "+$inputValue);
      if ($mode==="edit_node"){
        neuron_map[$selNode].value=$inputValue;
        $mode = "command"
        divv.focus();
        $isSel = false;
        $preSelNode = $selNode;
        $selNode = undefined;
      }
      if ($mode==="edit_dendrite"){
        lines[$selDendrite].data = $inputValue;
        tdendrite_map[$selDendrite].weight = parseFloat($inputValue);
        rdendrite_map[$selDendrite].weight = parseFloat($inputValue);
        console.log("ival:",$inputValue);
        $mode = "command"
        divv.focus();
        $isSel = false;
        $selDendrite = undefined;
      }
      if ($mode==="edit_command"){
        let input = $inputValue;
        let cmd_array = input.split(" ");
        let command  = cmd_array[0];
        let param  = cmd_array[1];
        console.log("command",command);
        console.log("param",param);
        switch(command){
          case "save":
            save(param);
            break;
          case "load":
            break;
          case "gen":
            break;
          default:
            break;

          
        }



        $mode = "command"
        divv.focus();
      }
    }
  }

  function spawnCircle() {
    if (!$nodeClick){
      const cx = $mousePos.x-$coords.x;
      const cy = $mousePos.y-$coords.y;
      const data = next_node_id.toString();
      nodes[next_node_id] = new node(cx,cy,data);
      neuron_map[next_node_id] = new neuron(next_node_id,"",[],[]);
      neurons[next_node_id] = {weight:0,in:[],out:[],dnd:[]}
      next_node_id +=1;
    }
  }

  function spawnLine(){
    let node1 = nodes[$preSelNode];
    let node2 = nodes[$selNode];
    lines[next_line_id] = new dendrite(node1,node2,"");
    tdendrite_map[next_line_id] = new tdendrite(next_line_id,$selNode,0);
    rdendrite_map[next_line_id] = new rdendrite(next_line_id,$preSelNode,0);
    //updating neural network
    neurons[$preSelNode].out.push($selNode);
    neurons[$selNode].in.push($preSelNode);
    neuron_map[$preSelNode].t_term.push($selNode);
    neuron_map[$selNode].r_term.push($preSelNode);
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
    background-color: #000000;
  }
  svg{
    position: absolute;
    width:100%;
    height:100%;
  }

  div.main{
    position: absolute;
    color: white;
    background-color: #000000;
    width:100%;
    height:100%;
    padding:0;
    margin:0;
  }
  input{
    background-color: #000000;
    color: white;
    border-color:blueviolet;
    margin: 15px;
    border-width:3px;
    font-size: 30px;

  }
  .inputBar{
    display:flex;
    margin: 0;
    margin-left:5px;
    justify-content: center;
    height:fit-content;
    width:fit-content;
    top:10%;
    left:20%;
    transform: translate(-50%, -50%);
  }
  .infoBar{
    display: flex;
    height:fit-content;
    bottom:20px;
    justify-content: space-evenly;
  }
  p{
    color:white;
    font-size:16px;
  }
</style>

<div class="main" 
     on:click={handleClick} 
     bind:this ={divv}
     tabindex=-1 
     on:keypress={handleKeyPress} 
     on:keydown={handleKeyDown}
     on:contextmenu={handleRClick} 
     on:mousedown={handleMouseDown} 
     on:mousemove={handleMouseMove} 
     on:dblclick={handleDblClick}
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
      data = {neuron_map[key].value}
      index = {key}
      />
    {/each}
    {/key}
  </svg>
</div>

<div class = "infoBar">
  <p> {$mode}</p>
  <p> | </p>
  <p> D~ {$hData} </p>
  <p> | </p>
  <p> S~ {$preSelNode} | {$selNode} D:{$selDendrite} T:{$selType}</p>
  <p> | </p>
  <p> C:({$coords.x},{$coords.y}) M:({$mousePos.x},{$mousePos.y}) </p>

</div>
{#if $mode.startsWith("edit")}
<div class="inputBar">
  <input 
    bind:value={$inputValue} 
    on:keypress={handlekeyPressInput}
    on:keydown={handleKeyDown}
    bind:this = {inputBox}
    />
</div>
{/if}

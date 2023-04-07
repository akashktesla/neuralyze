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
  let inputBox:HTMLInputElement;
  let divv;
  let default_path;
  let lines_info = [];
  let nodes_data_info = {};
  let lines_data_info = {};
  let nodes:{[key:string]:node} = {};
  let lines:{[key:string]:dendrite} = {};
  let neuron_map:{[key:string]:neuron} = {};
  let tdendrite_map:{[key:string]:tdendrite} = {};
  let rdendrite_map:{[key:string]:rdendrite} = {};
  let neurons = {};
  let update = false;
  let next_node_id = 1;
  let next_line_id = 1;

  $: ifocus(inputBox);
  dp();
  async function ifocus(ib){
    if (ib!=null){
       ib.focus();
        }
    }
  
  async function dp(){
    default_path = await invoke('dp',{});
  }

  let isDragging = false;
  let x = 0;
  let y = 0;

  $: nodes = nodes;

  function clear(){
     nodes_data_info = {};
     lines_data_info = {};
     lines_info = [];
     nodes= {};
     lines= {};
     neuron_map= {};
     tdendrite_map= {};
     rdendrite_map= {};
     neurons = {};
     update = false;
     next_node_id = 1;
     next_line_id = 1;
  }

  //TODO: write load for .ddb file too and store it as a part of .nz file (test case... delete panna proper ah aganum)
  function save(path:string){
    let nodes_data = JSON.stringify(nodes);
    let lines_info_data = JSON.stringify(lines_info);
    let nd_info = JSON.stringify(nodes_data_info)
    let ld_info = JSON.stringify(lines_data_info)
    let data = nodes_data+"#"+lines_info_data+"#"+next_node_id+"#"+nd_info+"#"+ld_info;
    invoke('save',{path:default_path+path,data:data});
  }

  async function load(path:string){
    let data:string = await invoke('load',{path:default_path+path});
    let sdata = data.split("#");
    let nodes_data = JSON.parse(sdata[0]);
    let lines_info_data = JSON.parse(sdata[1]);
    let nd_info = JSON.parse(sdata[3]);
    let ld_info = JSON.parse(sdata[4]);
    clear()
    next_node_id = parseInt(sdata[2]);
    for (let key in nodes_data){
      let cx = nodes_data[key].x;
      let cy = nodes_data[key].y;
      let data = nodes_data[key].data;
      let _key = parseInt(key);
      nodes[_key] = new node(cx,cy,data);
      neuron_map[_key] = new neuron(_key,data,[],[]);
      neurons[_key] = {weight:0,in:[],out:[],dnd:[]}
    }
    for (let key in lines_info_data){
      let a = lines_info_data[key].a
      let b = lines_info_data[key].b
      let node1 = nodes[a];
      let node2 = nodes[b];
      lines_info = [...lines_info, {a:a,b:b}];
      lines[next_line_id] = new dendrite(node1,node2,"");
      tdendrite_map[next_line_id] = new tdendrite(next_line_id,b,0);
      rdendrite_map[next_line_id] = new rdendrite(next_line_id,a,0);
      neurons[a].out.push(b);
      neurons[b].in.push(a);
      neuron_map[a].t_term.push(next_line_id);
      neuron_map[b].r_term.push(next_line_id);
      neurons[a].dnd.push(next_line_id);
      neurons[b].dnd.push(next_line_id);
      next_line_id +=1;
    }
    for (let key in nd_info){
      let val = nd_info[key];
      neuron_map[key].value = val;
      nodes_data_info[key] = val;
    }
    for (let key in ld_info){
      let val = ld_info[key];
      lines[key].data = val;
      lines_data_info[key] = val;
      tdendrite_map[key].weight = parseFloat(val);
      rdendrite_map[key].weight = parseFloat(val);
    }

    await invoke("print",{data:"nodes:"+JSON.stringify(nodes)});
    await invoke("print",{data:"lines_info:"+JSON.stringify(lines_info_data)});
    await invoke("print",{data:"lines:"+JSON.stringify(lines)});
    await invoke("print",{data:"neuron_map:"+JSON.stringify(neuron_map)});
    await invoke("print",{data:"tden:"+JSON.stringify(tdendrite_map)});
    await invoke("print",{data:"rden:"+JSON.stringify(rdendrite_map)});
    await invoke("print",{data:"neurons:"+JSON.stringify(neurons)});
    await invoke("print",{data:"next_node_id:"+JSON.stringify(next_node_id)});
    await invoke("print",{data:"next_line_id:"+JSON.stringify(next_line_id)});
    await invoke("print",{data:"nd_info:"+JSON.stringify(nd_info)});
    await invoke("print",{data:"ld_info:"+JSON.stringify(ld_info)});

    update = !update;
  }

  function gen(path:string){
      let data = "";
      for (const [_,value] of Object.entries(neuron_map)){
          let t_term = "["+value.t_term.join(",").toString()+"]"
          let r_term = "["+value.r_term.join(",").toString()+"]";
          let nstr = value.id.toString()+":"+value.value+":"+t_term+":"+r_term
          data+="|"+nstr;
      }
      data+="#";
      for (const [_,value] of Object.entries(tdendrite_map)){
        let tstr = value.id.toString()+":"+value.output.toString()+":"+value.weight.toString();
        data +="|"+tstr;
      }
      data+="#";
      for (const [_,value] of Object.entries(rdendrite_map)){
        let tstr = value.id.toString()+":"+value.input.toString()+":"+value.weight.toString();
        data +="|"+tstr;
      }
      console.log("data:",data);
      invoke('save',{path:default_path+path,data:data});
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
    if (inputBox!=null){
      inputBox.focus();
    }

  }

  function handleClick(){
    if (!$nodeClick){
      $isSel = false;
    }
    console.log("handleClick triggered");
      switch ($mode){
        case "add_node":
          spawnCircle();
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
    switch (event.key){
        case ":":
          $mode = "edit_command";
          $inputValue = "";
          break;
        case "f":
          $mode = "move";
          break;
        case "e":
          $mode = "Edit"
          break;
        case "d":
          if ($selType === "node"){
            deleteNode($selNode);
          }
          if ($selType ==="dendrite"){
            deleteDendrite($selDendrite);
          }
          update = !update;
          break;
    
        case"a":
          $mode = "add_node";
          break;
    
        case "t":
        if ($preSelNode!=undefined && $selNode!=undefined){
          spawnLine();
        }
        break;
    
        case "r":
          $mode = "rope";
          break;
    
        case "p":
          console.log("nodes",nodes);
          console.log("lines",lines);
          console.log("neuron_map",neuron_map);
          console.log("tdendrite_map",tdendrite_map);
          console.log("rdendrite_map",rdendrite_map);
          console.log("default path",default_path);
          invoke("print",{data:"nodes:"+JSON.stringify(nodes)});
          invoke("print",{data:"lines:"+JSON.stringify(lines)});
          invoke("print",{data:"neuron_map:"+JSON.stringify(neuron_map)});
          invoke("print",{data:"tden:"+JSON.stringify(tdendrite_map)});
          invoke("print",{data:"rden:"+JSON.stringify(rdendrite_map)});
          invoke("print",{data:"neurons:"+JSON.stringify(neurons)});
          invoke("print",{data:"next_node_id:"+JSON.stringify(next_node_id)});
          invoke("print",{data:"next_line_id:"+JSON.stringify(next_line_id)});
          invoke("print",{data:"lines_info:"+JSON.stringify(lines_info)});
          invoke("print",{data:"nd_info:"+JSON.stringify(nodes_data_info)});
          invoke("print",{data:"ld_info:"+JSON.stringify(lines_data_info)});
          invoke("print",{data:"default path:"+default_path});
          break;
        default:
          break;

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
        nodes_data_info[$selNode] = $inputValue;
        $mode = "command"
        divv.focus();
        $isSel = false;
        $preSelNode = $selNode;
        $selNode = undefined;
      }
      if ($mode==="edit_dendrite"){
        let value = parseFloat(eval($inputValue));
        lines[$selDendrite].data = value.toString();
        lines_data_info[$selDendrite] = value;
        tdendrite_map[$selDendrite].weight = value;
        rdendrite_map[$selDendrite].weight = value;
        console.log("ival:",$inputValue);
        $mode = "command"
        divv.focus();
        $isSel = false;
        $selDendrite = undefined;
      }
      if ($mode==="edit_command"){
        let input = $inputValue;
        let cmd_array = input.split(" ");
        let command  = cmd_array[0].replace(":","");
        let param  = cmd_array[1];
        console.log("command",command);
        console.log("param",param);
        switch(command){
          case "save":
            save(param);
            break;
          case "load":
            load(param);
            break;
          case "gen":
            gen(param);
            break;
          case "dp":
            default_path = param;
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
    lines_info = [...lines_info, {a:$preSelNode,b:$selNode}];
    lines[next_line_id] = new dendrite(node1,node2,"");
    tdendrite_map[next_line_id] = new tdendrite(next_line_id,$selNode,0);
    rdendrite_map[next_line_id] = new rdendrite(next_line_id,$preSelNode,0);
    //updating neural network
    neurons[$preSelNode].out.push($selNode);
    neurons[$selNode].in.push($preSelNode);
    neuron_map[$preSelNode].t_term.push(next_line_id);
    neuron_map[$selNode].r_term.push(next_line_id);
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
    border:none;
    color: white;
    width: 100%;
    margin: 15px;
    font-size: 15px;
  }
  input:focus {
    border:none;
    outline: none;
  }
  .inputBar{
    display:flex;
    margin: 0;
    margin-left:5px;
    justify-content: center;
    height:fit-content;
    width:99%;
    bottom:0px;
  }
  .infoBar{
    display: flex;
    height:fit-content;
    justify-content: space-evenly;
    bottom:30px;
  }
  p{
    color:white;
    font-size:15px;
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

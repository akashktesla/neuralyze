<script lang="ts">
  import {mode} from "./store";
  import type dendrite from "./dendrite";
  import { hData } from "./store";
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
  /* $: x3 = (x1+x2)/2 */
  /* $: y3 = (y1+y2)/2 */

  const arrowSize = 17;
  const offset = 39;

  let rect_offset = 80;
  let yoffset = 30;
  $: cirx = x2 - rect_offset * Math.cos(angle) + $coords.x + yoffset * Math.cos(angle+ Math.PI/2 );
  $: ciry = y2 - rect_offset * Math.sin(angle) + $coords.y + yoffset * Math.sin(angle+ Math.PI/2 );

  $: angle = Math.atan2(y2 - y1, x2 - x1);
  $: arrowX1 = x2 - offset * Math.cos(angle) + $coords.x;
  $: arrowY1 = y2 - offset * Math.sin(angle) + $coords.y ;

  $: arrow1X2 = arrowX1 - arrowSize * Math.cos(angle - Math.PI / 6) ;
  $: arrow1Y2 = arrowY1 - arrowSize * Math.sin(angle - Math.PI / 6) ;
  $: arrow2X2 = arrowX1 - arrowSize * Math.cos(angle + Math.PI / 6) ;
  $: arrow2Y2 = arrowY1 - arrowSize * Math.sin(angle + Math.PI / 6) ;

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
    stroke-width: 2px;
    stroke: white;
  }
  circle{
    stroke: red;
    fill: #000000;
    /* stroke-width:5px; */
  }
  
</style>

<line 
  x1={x1+$coords.x}
  y1={y1+$coords.y}
  x2={x2+$coords.x}
  y2={y2+$coords.y} 
  data-value={index}
  on:mousedown={onClickDendrite}
  on:dblclick={handleDblClickDendrite}
  on:mouseover={()=>{$hData=line.node1.data+" => "+line.node2.data+" = "+line.data}}
  on:focus={()=>{}}
  on:keypress={()=>{}}
  />

<polygon 
  points=
  "{arrowX1},{arrowY1} 
  {arrow1X2},{arrow1Y2}
  {arrow2X2},{arrow2Y2}" 
  data-value={index}
  on:mousedown={onClickDendrite}
  on:dblclick={handleDblClickDendrite}
  on:keypress={()=>{}}
  on:mouseover={()=>{$hData=line.node1.data+" => "+line.node2.data+" = "+line.data}}
  on:focus={()=>{}}
  fill="white" />

<circle 
  cx = {cirx}
  cy = {ciry}
  data-value={index}
  on:mousedown={onClickDendrite}
  on:dblclick={handleDblClickDendrite}
  on:keypress={()=>{}}
  on:mouseover={()=>{$hData=line.node1.data+" => "+line.node2.data+" = "+line.data}}
  on:focus={()=>{}}
  r = "20" />

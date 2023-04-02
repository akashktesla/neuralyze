import {readable, writable} from "svelte/store";

export var mode = writable("");
export var hData = writable("");
export var selType= writable("");
export var selNode= writable<number>();
export var selDendrite = writable<number>();
export var preSelNode = writable<number>();
export var isSel = writable<boolean>();
export var inputValue = writable("");
export var nodeClick = writable(false);
export var coords = writable({x:0,y:0});


export let mousePos = readable({x:0,y:0},(set)=>{
	document.addEventListener("mousemove", move);
	function move(event){
		set({x:event.clientX,y:event.clientY,});
	}
	return ()=>{
		document.removeEventListener("mousemove", move);
	}
});


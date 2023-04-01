import type node from "./node";
export default class dendrite{
  node1:node;
  node2:node;
  data:string;

  constructor(node1:node,node2:node,data:string){
    this.node1 = node1;
    this.node2 = node2;
    this.data = data;
  }
}

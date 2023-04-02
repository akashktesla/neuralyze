export default class neuron{
  id:number;
  value:string;
  t_term:number[];
  r_term:number[];

  constructor(id:number,value:string,t_term:number[],r_term:number[]){
    this.id = id;
    this.value = value;
    this.t_term = t_term;
    this.r_term = r_term;
  }
}

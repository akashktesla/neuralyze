    // _    _   _ _   _
   // / \  | \ | | \ | |
  // / _ \ |  \| |  \| |
 // / ___ \| |\  | |\  |
// /_/   \_\_| \_|_| \_|.rs
// Artificial Neural Network
#![allow(warnings)]
use std::{collections::HashMap, ops::IndexMut};
use log::debug;

pub fn main(){
  let n = Neuron::new(1,0., vec![1,2,3], vec![5,2,3]);
  let _str = n.to_string();
  print!("{}",_str);
}


#[derive(Debug)]
pub struct Neuron{
    pub id:i32,
    pub value:f32, //represents confidence ig
    pub t_term:Vec<i32>, //transmitting terminal dendrite id
    pub r_term:Vec<i32>, //recieving terminal dendrite id
}

#[derive(Debug)]
pub struct Dendrite{
    pub id:i32,
    input:i32,//neuron id
    output:i32,//neuron id
    weight:f32,//weight
}
#[derive(Debug)]
pub struct TDendrite{
    pub id:i32,
    pub output:i32,//neuron id
    pub weight:f32,//weight
}
#[derive(Debug)]
pub struct RDendrite{
    pub id:i32,
    pub input:i32,//neuron id
    pub weight:f32,//weight
}

#[derive(Debug)]
pub struct NeuralNetwork{
    pub neuron_map:HashMap<i32,Neuron>,//dictionary of neurons
    pub rdendrite_map:HashMap<i32,RDendrite>,//dictionary of rdendrites
    pub tdendrite_map:HashMap<i32,TDendrite>,//dictionary of tdendrites
}

impl Neuron{
  pub fn new(id:i32,value:f32,t_term:Vec<i32>,r_term:Vec<i32>)->Neuron{
    Neuron { 
      id ,
      value,
      t_term, 
      r_term }
  }
}

impl ToString for Neuron{
  fn to_string(&self) -> String {
      format!("{}:{}:{:?}:{:?}",self.id,self.value,self.t_term,self.r_term)
  }
}

impl TDendrite{
  pub fn new(id:i32,output:i32,weight:f32)->TDendrite{
    TDendrite { 
      id,
      output,
      weight }
  }
}

impl ToString for TDendrite{
  fn to_string(&self) -> String {
      format!("{}:{}:{}",self.id,self.output,self.weight)
  }
}

impl RDendrite{
  pub fn new(id:i32,input:i32,weight:f32)->RDendrite{
    RDendrite { 
      id,
      input,
      weight }
  }
}

impl ToString for RDendrite{
  fn to_string(&self) -> String {
      format!("{}:{}:{}",self.id,self.input,self.weight)
  }
}

impl NeuralNetwork{
  pub fn new()->NeuralNetwork{
    NeuralNetwork { 
      neuron_map: HashMap::new(),
      tdendrite_map: HashMap::new(),
      rdendrite_map: HashMap::new(),
    }
  }

  //change all methods and instances to make up for the new classes
  pub fn insert_rdendrite(&mut self,id:&i32,dendrite:RDendrite){
    debug!("inserting rdendrite: {:?}",dendrite);
    let neuron = self.neuron_map.get_mut(id).unwrap();
    neuron.r_term.push(dendrite.id);
    self.rdendrite_map.insert(dendrite.id, dendrite);
  }

  pub fn insert_tdendrite(&mut self,id:&i32,dendrite:TDendrite){
    debug!("inserting tdendrite: {:?}",dendrite);
    let neuron = self.neuron_map.get_mut(id).unwrap();
    neuron.t_term.push(dendrite.id);
    self.tdendrite_map.insert(dendrite.id, dendrite);
  }

  pub fn insert_neuron(&mut self,neuron:Neuron){
    self.neuron_map.insert(neuron.id, neuron);
  }
  pub fn get_neuron(&self,index:&i32)->Neuron{
    let _neuron =  self.neuron_map.get(index).unwrap();
    let neuron = Neuron::new(_neuron.id.clone(), _neuron.value.clone(), _neuron.t_term.clone(), _neuron.r_term.clone());
    return neuron
  }
  pub fn get_rdendrite(&self,index:&i32)->RDendrite{
    let _dendrite =  self.rdendrite_map.get(index).unwrap();
    let dendrite = RDendrite::new(_dendrite.id, _dendrite.input, _dendrite.weight);
    return dendrite
  }
  pub fn get_tdendrite(&self,index:&i32)->TDendrite{
    let _dendrite =  self.tdendrite_map.get(index).unwrap();
    let dendrite = TDendrite::new(_dendrite.id, _dendrite.output, _dendrite.weight);
    return dendrite
  }
}

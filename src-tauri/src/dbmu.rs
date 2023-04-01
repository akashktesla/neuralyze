#![allow(warnings)]
use log::{LevelFilter, debug, error, warn, info};
use std::sync::{Arc,RwLock,Mutex};
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use std::mem::drop;
use prettytable::{Table,Row,Cell,row};
use chrono::prelude::*;
use regex::Regex;
use regex::CaptureMatches;
use crate::ann::{Dendrite,TDendrite,RDendrite,NeuralNetwork,Neuron};
use crate::rustpp::{read_file,write_file};
//NeuralNetwork Database
#[derive(Debug)]
pub struct NnDatabase{
    pub data:Arc<RwLock<NeuralNetwork>>,
    pub path:String,
    pub neuron_counter:i32,
    pub tdendrite_counter:i32,
    pub rdendrite_counter:i32,
}

impl NnDatabase{

    pub fn new(path:String)->NnDatabase{
        let data = Arc::new(RwLock::new(NeuralNetwork::new()));
        NnDatabase{
            data,
            path,
            neuron_counter:1,
            tdendrite_counter:1,
            rdendrite_counter:1,
       }
    }

    pub fn insert_neuron(&self, neuron:Neuron){
        let mut lock = self.data.write().unwrap();
        lock.insert_neuron(neuron);
        drop(lock);
    }

    pub fn insert_rdendrite(&self,nid:&i32, dendrite:RDendrite){
        let mut lock = self.data.write().unwrap();
        lock.insert_rdendrite(&nid,dendrite);
        drop(lock);
    }
    pub fn insert_tdendrite(&self,nid:&i32, dendrite:TDendrite){
        let mut lock = self.data.write().unwrap();
        lock.insert_tdendrite(&nid,dendrite);
        drop(lock);
    }
    
    pub fn delete_neuron(&self,id:&i32){
        let mut lock = self.data.write().unwrap();
        lock.neuron_map.remove(id);
    }

    pub fn delete_tdendrite(&self,id:&i32){
        let mut lock = self.data.write().unwrap();
        lock.tdendrite_map.remove(id);
    }

    pub fn delete_rdendrite(&self,id:&i32){
        let mut lock = self.data.write().unwrap();
        lock.rdendrite_map.remove(id);
    }

    pub fn insert(&mut self,id:&i32,data:i32){
        let mut lock = self.data.write().unwrap();
        // lock.neuron_map[id].t_term.push(data);
    }

    pub fn save_database(&self){
        let lock = self.data.read().unwrap();
        let mut data = String::new();
        for k in lock.neuron_map.keys(){
            let v = lock.neuron_map.get(k).unwrap().to_string();
            data = format!("{}|{}",data,v);
        }
        data = format!("{}#",data);
        for k in lock.tdendrite_map.keys(){
            let v = lock.tdendrite_map.get(k).unwrap().to_string();
            data = format!("{}|{}",data,v);
        }
        data = format!("{}#",data);
        for k in lock.rdendrite_map.keys(){
            let v = lock.rdendrite_map.get(k).unwrap().to_string();
            data = format!("{}|{}",data,v);
        }
        write_file(&self.path,&data);
        debug!("saving data:{:?}",data);
        debug!("sucessfully saved");
    }

    pub fn load_database(&self){
        let data = read_file(&self.path);
        let (neuron_map,dendrite_map) = data.split_once("#").unwrap();
        let mut split = data.split("#");
        let neuron_map = split.next().unwrap();
        let tdendrite_map = split.next().unwrap();
        let rdendrite_map = split.next().unwrap();
        debug!("nm{:?}",neuron_map);
        debug!("dm{:?}",dendrite_map);
        //neurons load
        let neuron_map = neuron_map.replacen("|", "", 1);
        let nm_split = neuron_map.split("|");
        let mut neuron_map = HashMap::new();
        for i in nm_split{
            debug!("neuron str: {:?}",i);
            let mut n_split = i.split(":");
            let id  = n_split.next().unwrap().parse::<i32>().unwrap();
            let value = n_split.next().unwrap().parse().unwrap();
            let key = id.clone();

            let t_term  = n_split.next().unwrap();
            let mut t_term = t_term.replace("[","");
            t_term = t_term.replace("]","");
            let t_term_split = t_term.split(",");
            let mut t_term = Vec::new();
            for i in t_term_split{
                t_term.push(i.parse::<i32>().unwrap());
            }

            let r_term  = n_split.next().unwrap();
            let mut r_term = r_term.replace("[","");
            r_term = r_term.replace("]","");
            let r_term_split = r_term.split(",");
            let mut r_term = Vec::new();
            for i in r_term_split{
                r_term.push(i.parse::<i32>().unwrap());
            }
            let neuron = Neuron::new(id, value, t_term, r_term);
            neuron_map.insert(key, neuron);
        }

        //tdendrites load
        let tdendrite_map = tdendrite_map.replacen("|", "", 1);
        let tdm_split = tdendrite_map.split("|");
        let mut tdendrite_map = HashMap::new();
        for i in tdm_split{
            debug!("dendrite str: {:?}",i);
            let mut td_split = i.split(":");
            let id:i32 = td_split.next().unwrap().parse().unwrap();
            let key = id.clone();
            let output = td_split.next().unwrap().parse().unwrap();
            let weight = td_split.next().unwrap().parse().unwrap();
            let dendrite = TDendrite::new(id, output, weight);
            tdendrite_map.insert(key, dendrite);
        }
        //rdendtire load
        let rdendrite_map = rdendrite_map.replacen("|", "", 1);
        let rdm_split = rdendrite_map.split("|");
        let mut rdendrite_map = HashMap::new();
        for i in rdm_split{
            debug!("dendrite str: {:?}",i);
            let mut rd_split = i.split(":");
            let id:i32 = rd_split.next().unwrap().parse().unwrap();
            let key = id.clone();
            let input = rd_split.next().unwrap().parse().unwrap();
            let weight = rd_split.next().unwrap().parse().unwrap();
            let dendrite = RDendrite::new(id, input, weight);
            rdendrite_map.insert(key, dendrite);
        }


        let mut lock = self.data.write().unwrap(); 
        lock.neuron_map = neuron_map;
        lock.tdendrite_map = tdendrite_map;
        lock.rdendrite_map = rdendrite_map;

    }
    pub fn get_neuron(&self,index:&i32) -> Neuron{
        let lock = self.data.read().unwrap();
        let neuron = lock.get_neuron(index);
        drop(lock);
        return neuron

    }
    pub fn get_tdendrite(&self,index:&i32)->TDendrite{
        let lock = self.data.read().unwrap();
        let dendrite = lock.get_tdendrite(index);
        drop(lock);
        return dendrite
    }
    pub fn get_rdendrite(&self,index:&i32)->RDendrite{
        let lock = self.data.read().unwrap();
        let dendrite = lock.get_rdendrite(index);
        drop(lock);
        return dendrite
    }

    pub fn print_data(&self){
        let lock = self.data.read().unwrap();
        let mut table = Table::new();
        println!("Neuron Map");
        table.add_row(row!["index","data"]);
        for i in lock.neuron_map.keys(){
            let data = lock.neuron_map.get(i).unwrap().to_string();
            table.add_row(row![i,data]);
        }
        table.printstd();

        println!("TDednrite Map");
        let mut table = Table::new();
        table.add_row(row!["TDendrite"]);
        table.add_row(row!["index","data"]);
        for i in lock.tdendrite_map.keys(){
            let data = lock.get_tdendrite(i).to_string();
            table.add_row(row![i,data]);
        }
        table.printstd();

        println!("RDednrite map");
        let mut table = Table::new();
        table.add_row(row!["index","data"]);
        for i in lock.rdendrite_map.keys(){
            let data = lock.get_rdendrite(i).to_string();
            table.add_row(row![i,data]);
        }
        table.printstd();
        drop(lock);
}

    pub fn clear(&self){
        let mut lock = self.data.write().unwrap();
        lock.neuron_map = HashMap::new(); 
        lock.rdendrite_map = HashMap::new(); 
        lock.tdendrite_map = HashMap::new(); 
        drop(lock);
    }
}


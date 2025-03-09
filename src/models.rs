use serde::{Serialize,Deserialize};

#[derive(Serialize , Deserialize , Debug)]
//tells the rust compiler to automatically generate the code for serializing and deserializing
pub struct Task{
    pub description : String,
    pub completed : bool,
}
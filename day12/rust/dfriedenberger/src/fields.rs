//Playing fields
use array2d::Array2D;
use std::collections::HashSet;
//use regex::Regex;
use std::io::{Error, ErrorKind};
use gamma::graph::{ Graph, DefaultGraph  };


//Array2D based field
pub struct Caves {
    graph : DefaultGraph,
    nodes : Vec<String>
}

pub fn find(nodes : &Vec<String>,name : &String) -> std::io::Result<usize> {
    let l = nodes.len();

    for i in 0..l {
        let node = nodes.get(i).unwrap();
        if node == name {
            return Ok(i);
        }
    }

    Err(Error::new(ErrorKind::Other, "oh no!"))
}

impl Caves {
    pub fn new(rows : Vec<Vec<String>>) -> Caves
    {
        let mut nodes = Vec::new();

        let mut graph = DefaultGraph::new();
        let mut set : HashSet<String> = HashSet::new();

        for path in rows {

            for node in &path {
                let name = node.to_string(); //Create Copy
                if set.contains(&name) {
                    continue;
                }
                set.insert(name.to_string());
                nodes.push(name.to_string());
                let id = nodes.len() - 1;
                graph.add_node(id);
            }

            let id1 = find(&nodes,path.get(0).unwrap()).unwrap();
            let id2 = find(&nodes,path.get(1).unwrap()).unwrap();

            graph.add_edge(id1,id2);

        }


        Caves {
            graph : graph,
            nodes : nodes
        }
    }

 
    pub fn getNext(&self,name : &String) -> Vec<String> {

        let id = find(&self.nodes,name).unwrap();
        let mut v = Vec::new();

        for nb in self.graph.neighbors(id).unwrap()
        {
            let nb_name = self.nodes.get(nb).unwrap().to_string();
            v.push(nb_name);
        }

        return v;
    }

   

    
}


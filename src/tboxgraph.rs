use std::fmt;
use arrayvec::ArrayString;

// for json files
// use serde::{Deserialize, Serialize};
// use serde_json::Result;
// use serde_json::json;


/*================================================================================================*/
pub fn index_of<T: PartialEq>(v: &Vec<T>, el: T) -> usize {
    let index = v.iter().position(|x| *x == el).unwrap();

    index
}


/*================================================================================================*/
const STRSIZE: usize= 100;

// to create a more compact and fast implementation, in the future we will store names as
// numbers
#[derive(PartialEq, Copy, Clone)]
pub struct Name {
    name: ArrayString<[u8; STRSIZE]>, // dont't forget the change of 'char' by 'u8'
}

impl Name {
    pub fn validate_name(s: &str) -> Name {
        let copy_s = match s.char_indices().nth(STRSIZE) {
            None => s,
            Some((idx, _)) => &s[..idx]
        };

        let name = ArrayString::from(copy_s)
            .expect("Failed to create ArrayString, verify size.");

        Name { name }
    }

    pub fn from_name(n: &Self) -> Name {
        Name::validate_name(&n.name.clone().to_string())
    }

    pub fn push_str(&mut self, s: &str) -> () {
        let mut old_name = self.name.to_string();
        old_name.push_str(s);

        self.name = ArrayString::from(match old_name.char_indices().nth(STRSIZE) {
            None => s,
            Some((idx, _)) => &s[..idx]
        })
            .expect("Failed to create ArrayString, verify size.");
    }

    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
/*================================================================================================*/

#[derive(PartialEq, Copy, Clone)]
pub enum Kind {
    R, // for role
    C, // for concept
    I, // for individual (constant)
}

impl fmt::Display for Kind {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::R => write!(f, "r"),
            Kind::C => write!(f, "c"),
            Kind::I => write!(f, "i"),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Polarity {
    P, // positive polarity
    N, // negative polarity
}

impl fmt::Display for Polarity {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Polarity::P => write!(f, "(+)"),
            Polarity::N => write!(f, "(-)"),
        }
    }
}

/*================================================================================================*/
#[derive(PartialEq, Copy, Clone)]
struct TBoxNode {
    name: Name,
    k: Kind,
}

impl fmt::Display for TBoxNode {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}-{})", self.name, self.k)
    }
}

impl  TBoxNode {
    fn from_string(s: String) -> Option<TBoxNode> {
        let split = s.split(" ");
        let vec = split.collect::<Vec<&str>>();
        if vec.len() != 2 {
            None
        } else {
            match vec[1] {
                "r" => Some(TBoxNode{name: Name::validate_name(vec[0]), k:Kind::R}),
                "c" => Some(TBoxNode{name: Name::validate_name(vec[0]), k:Kind::C}),
                _ => None
            }
        }
    }

    fn from_node(n: &TBoxNode) -> Option<TBoxNode> {
        Some(TBoxNode{name: (*n).name, k: (*n).k})
    }

    fn kind(&self) -> Kind {
        self.k
    }
}

/*================================================================================================*/

#[derive(PartialEq, Copy, Clone)]
struct TBoxEdge {
    n1: TBoxNode,
    n2: TBoxNode,
    k: Kind,
    p: Polarity,
}

impl TBoxEdge {
    fn from_string(s: String) -> Option<TBoxEdge> {
        let split = s.split(",");
        let vec = split.collect::<Vec<&str>>();
        if vec.len() != 4 {
            None
        } else {
            let ok = match vec[2] {
                "r" => Some(Kind::R),
                "c" => Some(Kind::C),
                _ => None,
            };
            let op = match vec[3] {
                "p" => Some(Polarity::P),
                "n" => Some(Polarity::N),
                _ => None,
            };
            let on1 = TBoxNode::from_string(vec[0].to_string());
            let on2 = TBoxNode::from_string(vec[2].to_string());
            match (on1, on2, ok, op) {
                (Some(n1), Some(n2), Some(k), Some(p)) => Some(TBoxEdge{n1:n1, n2:n2, k:k, p:p}),
                _ => None
            }
        }
    }

    fn from_edge(e: &Self) -> Option<TBoxEdge> {
        Some(TBoxEdge{n1: (*e).n1, n2: (*e).n2, k: (*e).k, p: (*e).p})
    }

    fn kind(&self) -> Kind {self.k}
    fn polarity(&self) -> Polarity {self.p}
}


/*================================================================================================*/

struct TBoxGraph {
    nodes: Vec<TBoxNode>,
    edges: Vec<TBoxEdge>,
}

impl TBoxGraph{
    fn new() -> TBoxGraph {
        TBoxGraph { nodes: Vec::new(), edges: Vec::new() }
    }

    fn add_node(&mut self, n: TBoxNode) -> bool {
        if self.node_exists(&n) {
            false
        } else {
            self.nodes.append(&mut vec![n]);
            true
        }
    }

    fn add_edge(&mut self, e: TBoxEdge) -> bool {
        if self.edge_exists(&e) {
            false
        } else {
            self.edges.append(&mut vec![e]);
            true
        }
    }

    fn rem_node(&mut self, n: &TBoxNode) -> bool {
        if self.node_exists(n) {
            false
        } else {
            // remove all edges with this node
            for e in self.edges.clone() { // I had to clone the vector here because the compiler wasn't happy
               if *n == e.n1 || *n == e.n2 {
                   let _ = self.rem_edge(&e);
               }
            }
            // remove node
            self.nodes.remove(index_of(&self.nodes, *n));
            true
        }
    }

    fn rem_edge(&mut self, e: &TBoxEdge) -> bool {
        if self.edge_exists(e) {
            self.edges.remove(index_of(&self.edges, *e));
            true
        } else {
            false
        }
    }

    fn node_exists(&self, n: &TBoxNode) -> bool {
        self.nodes.contains(n)
    }

    fn edge_exists(&self, e: &TBoxEdge) -> bool { self.edges.contains(e) }

    fn node_size(&self) -> usize {
        self.nodes.len()
    }

    fn edge_size(&self) -> usize {
        self.edges.len()
    }

    fn parse_json(_filename: &str) -> Option<TBoxGraph> {
        None
    }

    fn parse_xml(_filename: &str) -> Option<TBoxGraph> {
        None
    }
}

/*================================================================================================*/

pub fn print_hello() {
    println!("Hello World from the 'tboxgraph' module!")
}

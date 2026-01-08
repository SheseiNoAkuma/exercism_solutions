pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())));
                    self.clone()
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_ref())
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_ref())
                }

                pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())));
                    self.clone()
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }
    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(&mut self, nodes: &Vec<Node>) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self.clone()
        }

        pub fn with_edges(&mut self, edges: &Vec<Edge>) -> Self {
            self.edges.extend(edges.iter().cloned());
            self.clone()
        }
        pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs
                .extend(attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())));
            self.clone()
        }
        pub fn node(&self, name: &str) -> Option<Node> {
            self.nodes
                .iter()
                .find(|n| &n.name == name)
                .map(|n| n.clone())
        }
    }
}

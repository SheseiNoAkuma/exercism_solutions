pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

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
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())));
                    self
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

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())));
                    self
                }
            }
        }
    }

    #[derive(Debug, PartialEq)]
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

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|&(a, b)| (a.to_owned(), b.to_owned()))
                .collect();
            self
        }
        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }
    }
}

use std::collections::{HashMap, VecDeque};

pub struct NamespaceContext {
    named_ns: VecDeque<HashMap<String, String>>,
    default_ns: VecDeque<String>,
}

impl NamespaceContext {
    pub fn new() -> Self {
        Self {
            named_ns: VecDeque::new(),
            default_ns: VecDeque::new(),
        }
    }

    #[allow(dead_code)]
    pub fn push_scope(&mut self) {
        self.named_ns.push_back(HashMap::new());
    }

    #[allow(dead_code)]
    pub fn pop_scope(&mut self) -> Option<HashMap<String, String>> {
        self.named_ns.pop_back()
    }

    pub fn push_default_ns(&mut self, ns: String) {
        self.default_ns.push_back(ns);
    }

    pub fn pop_default_ns(&mut self) -> Option<String> {
        self.default_ns.pop_back()
    }

    pub fn get_ns(&self, name: &str) -> Option<&String> {
        for ns in self.named_ns.iter().rev() {
            if let Some(ns) = ns.get(name) {
                return Some(ns);
            }
        }
        None
    }

    pub fn default_ns(&self) -> Option<&String> {
        self.default_ns.back()
    }
}

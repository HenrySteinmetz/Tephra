use serde::{Serialize, Deserialize};

use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    name: String,
    children: Vec<Tag>,
    parent: Option<String>,
}

impl Tag {
    fn new(name: String) -> Self {
        Tag {
            name,
            children: Vec::new(),
            parent: None,
        }
    }
    
    fn add_child(&mut self, mut child: Tag) {
        child.parent = Some(self.name.clone());
        self.children.push(child);
    }

    fn remove_child(&mut self, child: Tag) {
        if let Some(index) = self.children.iter().position(|c| c.name == child.name) {
            self.children.remove(index);
        } else {
            println!("Child not found");
        }
    }

    fn get_children(&self) -> Vec<&Tag> {
        self.children.iter().collect()
    }

    fn get_parent(&self) -> Option<&String> {
        self.parent.as_ref()
    }
}

pub struct File {
    tags: Vec<Tag>,
    location: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
impl File {
    fn new(location: PathBuf) -> Self {
        Self { 
            tags: vec![],
            location
        }
    }

    fn get_tags(&self) -> Vec<&Tag> {
        self.tags.iter().collect()
    }

    fn get_location(&self) -> PathBuf {
        self.location.clone()
    }
}

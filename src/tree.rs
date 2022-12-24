use std::fmt;
use std::mem;

pub type Link = Option<Box<Node>>;

pub struct Node {
  age: u32, name: String,
  left: Link,
  right: Link,
}

impl Node {
  pub fn new<S: Into<String>>(age: u32, name: S) -> Node {
    Self { 
      age, name: name.into().clone(), 
      left: Link::None, right: Link::None,
    }
  }

  pub fn link<S: Into<String>>(age: u32, name: S) -> Link {
    Link::Some(Box::new(Self::new(age, name)))
  }

  pub fn equal<S: Into<String>>(&self, age: u32, name: S) -> bool {
    self.age == age && self.name == name.into()
  }
}

pub struct Tree {
  root: Link,
}

impl Tree {
  pub fn empty() -> Tree {
    Self { root: Link::None }
  }

  pub fn with_root<S: Into<String>>(age: u32, name: S) -> Tree {
    Self { root: Node::link(age, name) } 
  }

  pub fn insert<S: Into<String>>(&mut self, age: u32, name: S) { 
    let name = &name.into();
    let location = Self::locate_mut(&mut self.root, age, name);
    let _inserted = match location.as_ref() {
      Some(_) => false, 
      None => {
        *location = Node::link(age, name);
        true
      }
    };
  }

  pub fn erase<S: Into<String>>(&mut self, age: u32, name: S) {
    let name = &name.into();
    let location = Self::locate_mut(&mut self.root, age, name);
    let _removed = match location.take() {
      Some(mut nd) => {
        match (nd.left.take(), nd.right.take()) {
          (None, None) => *location = None,
          (None, Some(ndr)) => *location = Some(ndr),
          (Some(ndl), None) => *location = Some(ndl),
          (Some(ndl), Some(ndr)) => {
            (nd.left, nd.right) = (Some(ndl), Some(ndr));
            let suc_link = Self::successor(&mut nd.right);
            let suc = &mut suc_link.as_mut().unwrap();
            mem::swap(&mut nd.age, &mut suc.age);
            mem::swap(&mut nd.name, &mut suc.name);
            *suc_link = suc.right.take(); 
            *location = Some(nd);
          },
        };
        true
      },
      None => false,
    };
  }

  pub fn contains<S: Into<String>>(&self, age: u32, name: S) -> bool {
    let name = &name.into();
    Self::locate(&self.root, age, name).is_some()
  }

  pub fn delete(&mut self) {
    self.root = Link::None;
  }
   
  // Helpers

  fn locate<'a>(link: &'a Link, age: u32, name: &String) -> &'a Link {
    match link {
      Some(nd) if !nd.equal(age, name) => {
        if age < nd.age || age == nd.age {
          Self::locate(&nd.left, age, name)
        } else {
          Self::locate(&nd.right, age, name)
        }
      }
      _ => link
    }
  }

  fn locate_mut<'a>(link: &'a mut Link, age: u32, name: &String) -> &'a mut Link {
    match link.as_ref() {
      Some(nd) if !nd.equal(age, name) => {
        let node = link.as_mut().unwrap();
        if age < node.age || age == node.age {
          Self::locate_mut(&mut node.left, age, name)
        } else {
          Self::locate_mut(&mut node.right, age, name)
        }  
      }
      _ => link,
    } 
  }

  fn successor(root: &mut Link) -> &mut Link {
    if root.is_none() || root.as_ref().unwrap().left.is_none() { return root; };

    let mut node = root.as_mut().unwrap();
    loop {
      let left = node.left.as_ref().unwrap();
      if left.left.is_none() { return &mut node.left; };
      node = node.left.as_mut().unwrap();
    } 
  }
}

// Print implementation
impl Tree {
  fn _print(link: &Link) -> String {
    match link {
      None => String::from("null"),
      Some(nd) => 
        format!("[{{\"{}\":\"{}\"}},{},{}]", 
          nd.age, nd.name, Self::_print(&nd.left), Self::_print(&nd.right)), 
    }
  }
}

impl fmt::Display for Tree  {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", Self::_print(&self.root))
  }
}


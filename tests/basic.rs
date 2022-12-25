use tree::Tree;

#[test]
fn insert_and_reverse_erase() {
  let mut root = Tree::with_root(32, "Erik");
  root.insert(55, "Mykola");  
  root.insert(12, "Crypto");  
  root.insert(34, "Luna");  
  root.insert(19, "Terra");  
  root.insert(32, "Victor");  

  root.erase(32, "Erik");  
  root.erase(55, "Mykola");  
  root.erase(12, "Crypto");  
  root.erase(34, "Luna");  
  root.erase(19, "Terra");  
  root.erase(32, "Victor");  

  let empty = format!("{}", Tree::empty());
  assert!(empty == format!("{root}"));
  assert!(empty == "null");
}

#[test]
fn contains_check() {
  let mut root = Tree::with_root(32, "Erik");
  root.insert(55, "Mykola");  
  root.insert(12, "Crypto");  
  root.insert(34, "Luna");  
  root.insert(19, "Terra");  
  root.insert(32, "Victor");  

  for (age, name) in [(55, "Mykola"), (12, "Crypto")] {
    assert!(root.contains(age, name));
  }
}


use tree::*;

fn main() {
  let mut root = Tree::with_root(32, "Erik");
  println!("{root}\n");
  root.insert(55, "Mykola");  
  println!("{root}\n");
  root.insert(12, "Crypto");  
  println!("{root}\n");
  root.insert(34, "Luna");  
  println!("{root}\n");
  root.insert(19, "Terra");  
  println!("{root}\n");
  root.insert(32, "Victor");  
  println!("{root}\n");

  for (age, name) in [(55, "Mykola"), (12, "Crypto"), (55, "Skola")] {
    let contains = root.contains(age, name);
    let msg = if contains { "contains" } else { "doesn't contain" };
    println!("Tree {} element {{{}:{}}}", msg, age, name);
  }

  root.erase(55, "Mykola"); 
  println!("\n{root}\n");
}


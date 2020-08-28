pub fn vec_example() {
   // create a vector
   let mut v = vec!["Hello","how","are","you"];
   println!("{}",v.len());

   // push
   v.push("today");
   println!("{:?}",v);

   // pop
   v.pop();
   println!("{:?}",v);
   
   // insert a value in a particular position
   v.insert(4, "sir");  
   println!("{:?}",v);

   // remove a value by its index
   v.remove(0);
   println!("{:?}",v);

   // remove a value
   let index = v.iter().position(|x| x == &"sir" ).unwrap();	// remove element
	v.remove(index);
   println!("{:?}",v);
   
   // iterate
   for i in &v {
       println!("{}", i);
   }
}

use std::collections::HashMap;
pub fn hm_example() {
   let mut hm: HashMap<String,String> = HashMap::new();
   hm.insert("MA".to_string(),"Massachusetts".to_string());
   hm.insert("NY".to_string(),"New York".to_string());
   hm.insert("CA".to_string(),"California".to_string());

   for (key, val) in hm.iter() {
      println!("key: {} val: {}", key, val);
   }
   println!("");

   *hm.get_mut("MA").unwrap() = "MASSACHUSETTS".to_string();

   hm.remove("CA");
   for (key, val) in hm.iter() {
      println!("key: {} val: {}", key, val); 
   }   
   println!("{:?}", hm.len());
}

use std::collections::HashSet;
pub fn hs_example() {
   let mut hs: HashSet<String> = HashSet::new();
   hs.insert("Tennis".to_string());
   hs.insert("Tennis".to_string());
   hs.insert("Soccer".to_string());
   hs.insert("Badminton".to_string());

   for val in hs.iter() {
      println!("val: {}", val); 
   }   
   println!("");

   hs.remove("Badminton");
   for val in hs.iter() {
      println!("val: {}", val); 
   }   
   println!("{:?}", hs.len());
}
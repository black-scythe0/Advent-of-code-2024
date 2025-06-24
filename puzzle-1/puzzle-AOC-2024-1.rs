use std::fs;
//use std::str::FromStr;
//use std::str::Split;

//use std::env;
//use std::any::type_name;
fn main(){
   //let mut  list_left = [0];
   //let mut list_right = [0];
   //list_left.sort();
   //println!("sorted list: {:?}", list_left);
   //
   let content: &str = &fs::read_to_string("AOC-2024-1.txt").expect("file must have been read");
   let splited = content.split_terminator("\n");
   //let splited = splited.split(" ");
   let mut list = vec![0];
   for i in splited{
       list.push(i.parse::<i64>());
       println!("{}", i);
       break;

   }
   //println!("file content:\n {:#?}", splited);


} 



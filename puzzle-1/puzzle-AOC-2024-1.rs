use std::fs;

/*fn print_type<T>(_:&T){
	println!("{}", std::any::type_name::<T>())
}
*/
fn square_root<T: Into<f64> + Copy>(num: T) -> f64{
	num.into().sqrt()
}

fn main(){

   let mut left_list = vec![];
   let mut right_list = vec![];
   let content: &str = &fs::read_to_string("AOC-2024-1.txt").expect("file must have been read");
   let splited = content.split_terminator("\n");
   let mut em_list = vec![];
   
   let mut diff_sum: i32 = 0;
   let mut c = 0;
   for i in splited{
        let  i = i.trim()
                 .split(" ")
                 .filter(|s| !s.is_empty())
                 .collect::<Vec<_>>()
                 .join(" ");

        let j: Vec<&str> = i.split_whitespace().collect();
        if j.len() != 0{
        left_list.push(j[0].parse::<i32>().unwrap());
        right_list.push(j[1].parse::<i32>().unwrap());
        em_list.push(c);
        c +=1
        }
    
   }
   
  left_list.sort();
  right_list.sort();
 
 let mut diff: i32; 
 for i in em_list{
        diff = left_list[i] - right_list[i];
        println!("left: {},right: {},diff: {}", left_list[i], right_list[i], diff);
      	
      	diff_sum += square_root(diff * diff) as i32;  // as asked to calculate how far apart they are. 
 }
println!("diff_sum: {}", diff_sum);

} 



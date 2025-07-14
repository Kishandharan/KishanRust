use prompt_input::prelude::*;

fn main() {
   let multiplicand_s: i32 = String::prompt("Table start: ").parse().unwrap();
   let multiplicand_e: i32 = String::prompt("Table End: ").parse().unwrap();
   let multiplier_s: i32 = String::prompt("Multiplier start: ").parse().unwrap();
   let multiplier_e: i32 = String::prompt("Multiplier end: ").parse().unwrap();

   let mut i: i32 = multiplicand_s;
   let mut j: i32 = multiplier_s;
   while i<=multiplicand_e{
      j=multiplier_s;
      while j<=multiplier_e{
         println!("{}x{}={}", j, i, j*i);
         j+=1;
      }
      println!("  ");
      i+=1;
   }
}

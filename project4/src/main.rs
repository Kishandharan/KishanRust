fn main() {
    println!("{}", choose_day(1_i64));
}

fn choose_day(day_num: i64) -> String{
    match day_num {
         1 => "Sunday".to_string(),
         2 => "Monday".to_string(),
         3 => "Tuesday".to_string(),
         4 => "Wednesday".to_string(),
         5 => "Thursday".to_string(),
         6 => "Friday".to_string(),
         7 => "Saturday".to_string(),
         _ => "Null".to_string()
    }
}

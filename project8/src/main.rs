fn main(){
    let mut prison_cells = vec!["C","C","C","C","C","C","C","C","C","C"];
    
    for x in 0..prison_cells.len(){
        prison_cells[x] = "O";
    }
    println!("{:?}", prison_cells);

    for x in (1..prison_cells.len()).step_by(2){
        prison_cells[x] = "C";
    }
    println!("{:?}", prison_cells);

    for x in 3..=prison_cells.len(){
        for y in ((x-1)..prison_cells.len()).step_by(x){
            if prison_cells[y] == "C"{
                prison_cells[y] = "O";
            }else{
                prison_cells[y] = "C";
            }
        }
        println!("{:?}", prison_cells);
    }
}
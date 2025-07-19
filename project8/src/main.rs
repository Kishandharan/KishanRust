fn main() {
    let mut prison_cells = vec![];
    let mut iter = 0;
    println!("{:?}", prison_cells);

    while iter < 10{
        prison_cells.push("C");
        iter += 1;
    }
    iter = 0;
    println!("{:?}", prison_cells);

    while iter < prison_cells.len(){
        prison_cells[iter] = "O";
        iter += 1;
    }
    iter = 0;
    println!("{:?}", prison_cells);

    while iter < prison_cells.len(){
         iter += 2;
         prison_cells[iter-1] = "C";
    }
    iter = 0;
    println!("{:?}", prison_cells);

    while iter < prison_cells.len(){
        iter += 3;
        if iter > prison_cells.len(){ break; }

        if prison_cells[iter-1] == "C"{
            prison_cells[iter-1] = "O";
        }else{
            prison_cells[iter-1] = "C";
        }
    }
    iter = 0;

    while iter < prison_cells.len(){
        iter += 4;
        if iter > prison_cells.len() { break; }

        if prison_cells[iter-1] == "C"{
            prison_cells[iter-1] = "O";
        }else{
            prison_cells[iter-1] = "C";
        }
    }

    // Completed Four Rounds, Need to change this to use for loop
    println!("{:?}", prison_cells);
}

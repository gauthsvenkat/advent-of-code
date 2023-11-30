use std::io;

fn main(){
    let mut top_1 = 0;
    let mut top_2 = 0;
    let mut top_3 = 0;
    let mut elf_agg = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty()  {
            if elf_agg > top_1{
                top_3 = top_2;
                top_2 = top_1;
                top_1 = elf_agg;
            } else if elf_agg > top_2 {
                top_3 = top_2;
                top_2 = elf_agg;
            } else if elf_agg > top_3 {
                top_3 = elf_agg;
            }
            elf_agg = 0;
        } else {
            elf_agg += line.parse::<i32>().unwrap();
        }
    }
    println!("{top_1}");
    println!("{}", top_1 + top_2 + top_3)
}

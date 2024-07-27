mod lcg;

fn main() {
    let mut rangen = lcg::LCG::new();
    let mut i = 0;
    loop {
        i = i + 1;
        println!("Random u128 test {}: {}", i, rangen.random());
        if i == 25 {
            break;
        }
    }
    i = 0;
    loop {
        i = i + 1;
        let random = rangen.random_between(300, 90000).unwrap();
        println!("Random u128 between 300 and 90000 {}: {}", i, random);
        rangen.add_result(random);
        if i == 1000 {
            break;
        }
    }
    let _ = rangen.save_to_file_seperated("numbers_s.txt", ",");
    println!("Tests finished!");
}

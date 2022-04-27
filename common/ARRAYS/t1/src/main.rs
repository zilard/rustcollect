fn main() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropodia", "Insecta"];

    println!("lazy_caterer {:?}", lazy_caterer);

    println!("taxonomy {:?}", taxonomy);

    let mut sieve = [true; 10000];
    //println!("sieve {:?}", sieve);

    for i in 2..100 {
        if sieve[i] {
            let j = i * i;
            while j < 10000 {
                sieve[j] = false;
            }
        }

        println!("sieve {:?}", sieve);
    }
}

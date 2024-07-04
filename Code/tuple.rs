fn main(){
    let tup1 = (12,12,1,31,41,5,16,111,112,1,144,134);

    match tup1 {
        (first, .., last) => {
            println!("Success");
        }
    }
}
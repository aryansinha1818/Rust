fn main(){
    let mut counter = 0;

    let res = loop{
        counter += 1;
        println!("{}", counter);
        if counter == 10{
            break counter+1;
        }
    };
    println!("{}", res);
 }
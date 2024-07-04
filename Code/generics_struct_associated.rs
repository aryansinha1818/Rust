struct Jam<T>{
    flavour: T,
}

struct Coffee<U>{
    brand: U
}

impl<U> Coffee<U>{
    fn company(&self) -> &U{
        &self.brand
    }
}

impl<T> Jam<T>{
    fn product(&self) -> &T{
        &self.flavour
    }
}

fn main(){
    let item_1 = Jam{flavour: "Mixed-Fruit".to_string()};
    println!("The first item is Jam of flavour -- {}", item_1.product());
    

    let item_2 = Coffee{brand: "Nescafe"};
    println!("The second item is coffee of brand -- {}", item_2.company());
}
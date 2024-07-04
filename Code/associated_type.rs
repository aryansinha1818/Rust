trait TraitName{
    type TheType;

    fn function_for(&self) -> Self::TheType;
}

struct MyStruct{}

impl TraitName for MyStruct{
    type TheType = i32;

    fn function_for(&self) -> Self::TheType{
        return 45;
    }
}
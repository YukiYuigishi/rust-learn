macro_rules! add{
    ($x:expr) =>{
        $x + 1
    }
}

fn main(){
    println!("{}",add!(1));
}

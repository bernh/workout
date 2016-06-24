extern crate workout;


fn main() 
{
    let header = workout::Fit_File_Header::new();
    let array = header.bin();
    println!("{:?}", array);

}

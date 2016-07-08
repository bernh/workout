use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Write;

extern crate workout;


fn write_file(data: &[u8], path : &Path) {
    let mut file = match File::create(&path) {
        Err(why) => {
            panic!("couldn't create {}: {}",
                   path.display(),
                   Error::description(&why))
        }
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(data) {
        panic!("couldn't write to {}: {}",
               path.display(),
               Error::description(&why));
    };
}


fn main() 
{
    let mut header = workout::FitFileHeader::new();
    header.calc_crc();
    let array = header.bin();
    println!("{:?}", array);
    write_file(&array, Path::new("workout.fit"))
}

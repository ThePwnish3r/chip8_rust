use std::io::Read;
use std::fs::File;
use chip8::Chip8;

mod ram;
mod chip8;
fn main() {


    let mut file = File::open("data/INVADERS").unwrap();
    
    let  mut data:Vec<u8> = Vec::new();

    file.read_to_end(&mut data);
    print!("data : {:?}",data);

    let mut chip8 = Chip8::new();

    chip8.load_rom(&data);
}

use std::fs::File;
use std::io::Read;

type BankSizeType = u16;
type NumBanksType = u8;

const BANK_SIZE: BankSizeType = 0x0400;
const MAX_BANKS: NumBanksType = 8;

#[derive(Copy, Clone)]
struct Bank {
    data: [u8; BANK_SIZE as usize],
}


pub trait Cartridge {
    fn load(&mut self) -> std::io::Result<()>;
    fn print(&self);
}

pub struct GenericCartridge {
    filename: String,
    pub num_banks: NumBanksType,
    rom: Box<[Bank; MAX_BANKS as usize]>,
}

impl GenericCartridge {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            num_banks: 0,
            rom: Box::new(
                [Bank {
                    data: [0; BANK_SIZE as usize],
                }; MAX_BANKS as usize],
            ),
        }
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        let mut file = File::open(&self.filename)?;

        self.load_banks(&mut file);

        self.print();

        Ok(())
    }

    pub fn load_banks(&mut self, source: &mut dyn Read) {
    }
}

impl Cartridge for GenericCartridge {
    fn load(&mut self) -> std::io::Result<()> {
        self.load()
    }

    fn print(&self) {
        println!("read: {}", self.filename);
        println!("Num banks: {}", self.num_banks);
    }

}

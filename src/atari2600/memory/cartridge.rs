use std::fs::File;
use std::io::Read;
use strum_macros::EnumIter;
use strum_macros::EnumString;

type BankSizeType = u16;
type NumBanksType = u8;

const BANK_SIZE: BankSizeType = 0x0400;
const MAX_BANKS: NumBanksType = 8;

#[derive(Debug, EnumIter, EnumString)]
pub enum CartridgeType {
    Default,
    F4,
    F4SC,
    F6,
    F6SC,
    F8,
    F8SC,
    CBS,
    SUPER,
}

#[derive(Clone)]
pub struct Bank {
    data: Vec<u8>,
}

impl Bank {
    fn new(bank_size: u16) -> Self {
        Self { data: vec![0; bank_size as usize] }
    }
}

pub trait Cartridge {
    fn load(&mut self) -> std::io::Result<()>;

    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);

    fn summary(&self);
}

pub struct GenericCartridge {
    filename: String,
    pub num_banks: NumBanksType,
    cartridge_banks: Vec<Bank>,

    max_banks: u8,
    bank_size: u16,
    ram_size: u16,

    ram_addr_mask: u16,

    ram: Vec<u8>,

    current_bank: u8,
    bank_select: u8,

    /// 0xFF8 == address: Last bank - 2
    /// 0xFF9 == address: Last bank - 1
    /// 0xFFA == address: Last bank
    hot_swap: u16,
}

impl GenericCartridge {
    pub fn new(filename: &str, max_banks: u8, bank_size: u16, hot_swap: u16, ram_size: u16) -> Self {
        Self {
            filename: filename.to_string(),
            cartridge_banks: Vec::new(),
            ram: vec![0; ram_size as usize],
            bank_size,
            max_banks,
            hot_swap,
            ram_size,
            ram_addr_mask: ram_size.wrapping_sub(1),
            num_banks: 0,
            current_bank: 0,
            bank_select: 0,
        }
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        let f = File::open(&self.filename);
        match f {
            Ok(mut file) => {
                self.load_banks(&mut file);
                self.summary();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn load_banks(&mut self, source: &mut dyn Read) {
        let total_bytes_read = 0;

        for i in 0..self.max_banks {
            if let (Some(bank), _n) = self.load_bank(source) {
                // Grow the banks as they are read.
                self.cartridge_banks.push(bank);
                self.num_banks += 1;
            }
        }

        // Consumes and counts the remaining bytes.
        let remaining_bytes = source.bytes().count();
        if remaining_bytes > 0 {
            println!("Extra bytes in cartridge: {} bytes", remaining_bytes);
        }
    }

    fn load_bank(&mut self, source: &mut dyn Read) -> (Option<Bank>, NumBanksType) {
        let mut bank = Bank::new(self.bank_size);

        // Try to read an entire bank.
        match source.read(&mut bank.data) {
            Ok(0) => (None, 0),
            Ok(n) if 2048 == n && 0 == self.num_banks => {
                println!("Assuming this to be a '2k' cartridge with no bank switching.");
                self.bank_size = n as u16;
                (Some(bank), n as NumBanksType)
            }
            Ok(n) if n < bank.data.len() => {
                self.bank_size = bank.data.len() as u16;
                println!("Bank incomplete ({} bytes found in last bank), will be padded with zeros", n);
                (Some(bank), n as NumBanksType)
            }
            Ok(n) => (Some(bank), n as NumBanksType),
            _ => (None, 0),
        }
    }

    fn read(&mut self, address: u16) -> u8 {
        // Mask the 'address' with the bank size (so the highest address lines are ignored).
        let address = address & (self.bank_size - 1);
        if (self.ram_size > 0) && (address < 2 * self.ram_size) && (address >= self.ram_size) {
            self.ram[(address & self.ram_addr_mask) as usize]
        } else {
            // 0xFF8 == address: Last bank - 2
            // 0xFF9 == address: Last bank - 1
            // 0xFFA == address: Last bank
            if self.num_banks > 1 && (((self.hot_swap + 1) - self.num_banks as u16) <= address) && ((self.hot_swap + 1) > address) {
                self.current_bank = self.num_banks - ((self.hot_swap + 1) - address) as u8;
            }

            self.cartridge_banks[self.current_bank as usize].data[address as usize]
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        let address = address & (self.bank_size - 1);
        if (self.ram_size > 0) && (address < self.ram_size) {
            self.ram[(address & self.ram_addr_mask) as usize] = data;
        }

        if self.num_banks > 1 && (((self.hot_swap + 1) - self.num_banks as u16) <= address) && ((self.hot_swap + 1) > address) {
            self.current_bank = self.num_banks - ((self.hot_swap + 1) - address) as u8;
        }
    }
}

impl Cartridge for GenericCartridge {
    fn load(&mut self) -> std::io::Result<()> {
        self.load()
    }

    fn summary(&self) {
        println!("cartridge read: {}", self.filename);
        println!(" num banks: {}", self.num_banks);
        println!(" bank size = {}", self.cartridge_banks[0].data.len());
    }

    fn read(&mut self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.write(address, data);
    }
}

pub fn get_new_carterage(filename: String, cartridge_type: CartridgeType) -> GenericCartridge {
    const NO_RAM: u16 = 0x0000;
    const RAM_128_BYTES: u16 = 0x0080;
    const RAM_256_BYTES: u16 = 0x0100;
    match cartridge_type {
        // filename,  max_banks (4K banks), bank_size, hot_swap, ram_size
        // 'hot_swap' values is the 'upper' value, generally, subsequent banks are selected via 'value - 1'.
        CartridgeType::Default => GenericCartridge::new(&filename, 8, 0x1000, 0xFF9, NO_RAM),
        CartridgeType::F4 => GenericCartridge::new(&filename, 8, 0x1000, 0xFFB, NO_RAM),
        CartridgeType::F4SC => GenericCartridge::new(&filename, 8, 0x1000, 0xFFB, RAM_128_BYTES),

        CartridgeType::F8 => GenericCartridge::new(&filename, 2, 0x1000, 0xFF9, NO_RAM),
        CartridgeType::F8SC => GenericCartridge::new(&filename, 2, 0x1000, 0xFF9, RAM_128_BYTES),

        CartridgeType::F6 => GenericCartridge::new(&filename, 4, 0x1000, 0xFF9, NO_RAM),
        CartridgeType::F6SC => GenericCartridge::new(&filename, 4, 0x1000, 0xFF9, RAM_128_BYTES),

        CartridgeType::CBS => GenericCartridge::new(&filename, 3, 0x1000, 0xFFA, RAM_256_BYTES),
        CartridgeType::SUPER => GenericCartridge::new(&filename, 4, 0x1000, 0xFF9, NO_RAM),
    }
}

#[cfg(test)]
mod tests {
    use crate::atari2600::memory::cartridge::GenericCartridge;
    #[test]
    fn test_simple_generic_cartridge() {
        let mut sample_cartridge = GenericCartridge::new("dummy", 3, 0x200, 0xF9, 40);
        assert_eq!(sample_cartridge.cartridge_banks.len(), 0);

        // A slice implements 'Read'
        let dummy_file_data = vec![0 as u8; 0x100000];

        sample_cartridge.load_banks(&mut &dummy_file_data[..]);

        assert_eq!(sample_cartridge.cartridge_banks.len(), 3);
        assert_eq!(sample_cartridge.cartridge_banks[0].data.len(), 0x200);
    }
}

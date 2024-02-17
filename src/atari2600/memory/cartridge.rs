use std::str::FromStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;

type BankSizeType = u16;
type NumBanksType = u8;

const BANK_SIZE: BankSizeType = 0x0400;
const MAX_BANKS: NumBanksType = 8;

#[derive(Debug, EnumIter, EnumString, Clone, Copy)]
pub enum CartridgeType {
    Default,
    F4,
    F4SC,
    F6,
    F6SC,
    F8,
    F8SC,
    Cbs,
    Super,
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
    pub fn new(filename: &str, max_banks: u8, current_bank: u8, bank_size: u16, hot_swap: u16, ram_size: u16) -> Self {
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
            current_bank,
            bank_select: 0,
        }
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        let mut buffer = Vec::new();

        #[cfg(not(target_os = "emscripten"))]
        {
            use std::fs::File;
            use std::io::Read;

            let f = File::open(&self.filename);
            match f {
                Ok(mut file) => {
                    file.read_to_end(&mut buffer)?;
                }
                Err(e) => return Err(e),
            }
        }

        #[cfg(target_os = "emscripten")]
        {
            JAVASCRIPT_DATA_STORE.with(|ref_cell_data| {
                buffer = ref_cell_data.borrow().raw_cart_data.clone();
            });
        }

        self.load_banks(&mut buffer);
        self.summary();

        Ok(())
    }

    fn load_banks(&mut self, source: &mut Vec<u8>) {
        let total_bytes_read = 0;

        for i in 0..self.max_banks {
            if let (Some(bank), _n) = self.load_bank(source) {
                // Grow the banks as they are read.
                self.cartridge_banks.push(bank);
                self.num_banks += 1;
            }
        }

        if self.current_bank >= self.num_banks {
            println!("Default 'current_bank:{}' exceeds number of banks {}. Setting current_bank to '0'", self.current_bank, self.num_banks);
            self.current_bank = 0;
        }

        // Consumes and counts the remaining bytes.
        let remaining_bytes = source.len();
        if remaining_bytes > 0 {
            println!("Extra bytes in cartridge: {} bytes", remaining_bytes);
        }
    }

    fn load_bank(&mut self, source: &mut Vec<u8>) -> (Option<Bank>, NumBanksType) {
        let mut bank = Bank::new(self.bank_size);

        // Try to read an entire bank.
        match source.len() {
            0 => (None, 0),
            n if 2048 == n && 0 == self.num_banks => {
                println!("Assuming this to be a '2k' cartridge with no bank switching.");
                bank.data = source[0..self.bank_size as usize].into();
                self.bank_size = n as u16;
                (Some(bank), n as NumBanksType)
            }
            n if n < bank.data.len() => {
                bank.data = source[0..n].into();
                source.drain(0..n);
                self.bank_size = bank.data.len() as u16;
                println!("Bank incomplete ({} bytes found in last bank), will be padded with zeros", n);
                (Some(bank), n as NumBanksType)
            }
            n => {
                bank.data = source[0..self.bank_size as usize].into();
                source.drain(0..self.bank_size as usize);
                self.bank_size = bank.data.len() as u16;
                (Some(bank), self.bank_size as NumBanksType)
            }
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

            if !self.cartridge_banks.is_empty() {
                self.cartridge_banks[self.current_bank as usize].data[address as usize]
            } else {
                0
            }
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
        if !self.cartridge_banks.is_empty() {
            println!(" bank size = {}", self.cartridge_banks[0].data.len());
        }
    }

    fn read(&mut self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.write(address, data);
    }
}

struct JavaScriptData {
    pub raw_cart_data: Vec<u8>,
    pub raw_cart_type: CartridgeType,
}
impl JavaScriptData {
    pub fn new() -> Self {
        Self {
            raw_cart_data: Vec::new(),
            raw_cart_type: CartridgeType::Default,
        }
    }
}

use std::cell::RefCell;

thread_local! {
    static JAVASCRIPT_DATA_STORE: RefCell<JavaScriptData> = RefCell::new(JavaScriptData::new());
}

pub fn is_cart_ready() -> bool {
    let mut is_ready = false;
    JAVASCRIPT_DATA_STORE.with(|ref_cell_data| {
        is_ready = !ref_cell_data.borrow().raw_cart_data.is_empty();
    });
    is_ready
}

pub fn get_cart_type() -> CartridgeType {
    let mut cart_type = CartridgeType::Default;
    JAVASCRIPT_DATA_STORE.with(|ref_cell_data| {
        cart_type = ref_cell_data.borrow().raw_cart_type;
    });
    cart_type
}

#[no_mangle]
pub extern "C" fn display_data(raw_data_ptr: *const u8, raw_data_length: usize, cart_type_char_ptr: *const std::ffi::c_char) {
    // TODO: Although it's possible there's another way (alternate arguments), I'll just use the unsafe option for now.
    let v = unsafe { std::slice::from_raw_parts(raw_data_ptr, raw_data_length) };
    let cart_type_string = unsafe { std::ffi::CStr::from_ptr(cart_type_char_ptr) }.to_str().unwrap();
    println!("Called from javascript. Rom size: {}, Cartridge Type: {}", v.len(), cart_type_string);

    let cart_type = CartridgeType::from_str(cart_type_string).expect("Couldn't convert from string to CartType.");

    JAVASCRIPT_DATA_STORE.with(|ref_cell_data| {
        ref_cell_data.borrow_mut().raw_cart_data = v.to_vec();
        ref_cell_data.borrow_mut().raw_cart_type = cart_type
    });
}

pub fn get_new_cartridge(filename: &String, cartridge_type: &CartridgeType) -> Box<GenericCartridge> {
    const NO_RAM: u16 = 0x0000;
    const RAM_128_BYTES: u16 = 0x0080;
    const RAM_256_BYTES: u16 = 0x0100;
    let mut new_cartridge = match cartridge_type {
        // filename,  max_banks (4K banks), bank_size, hot_swap, ram_size
        // 'hot_swap' values is the 'upper' value, generally, subsequent banks are selected via 'value - 1'.
        // TODO: Confirm initial/starting bank for each type.
        CartridgeType::Default => Box::new(GenericCartridge::new(filename, 8, 1, 0x1000, 0xFF9, NO_RAM)),
        CartridgeType::F4 => Box::new(GenericCartridge::new(filename, 8, 0, 0x1000, 0xFFB, NO_RAM)),
        CartridgeType::F4SC => Box::new(GenericCartridge::new(filename, 8, 0, 0x1000, 0xFFB, RAM_128_BYTES)),

        CartridgeType::F8 => Box::new(GenericCartridge::new(filename, 2, 1, 0x1000, 0xFF9, NO_RAM)),
        CartridgeType::F8SC => Box::new(GenericCartridge::new(filename, 2, 1, 0x1000, 0xFF9, RAM_128_BYTES)),

        CartridgeType::F6 => Box::new(GenericCartridge::new(filename, 4, 0, 0x1000, 0xFF9, NO_RAM)),
        CartridgeType::F6SC => Box::new(GenericCartridge::new(filename, 4, 0, 0x1000, 0xFF9, RAM_128_BYTES)),

        CartridgeType::Cbs => Box::new(GenericCartridge::new(filename, 3, 0, 0x1000, 0xFFA, RAM_256_BYTES)),
        CartridgeType::Super => Box::new(GenericCartridge::new(filename, 4, 0, 0x1000, 0xFF9, NO_RAM)),
    };

    // Load the cartridge.
    match new_cartridge.load() {
        Ok(()) => {
            println!("Ok");
        }
        Err(e) => {
            panic!("Error loading cartridge \"{}\".\n {}", filename, e);
        }
    }

    new_cartridge
}

#[cfg(test)]
mod tests {
    use crate::atari2600::memory::cartridge::GenericCartridge;
    #[test]
    fn test_simple_generic_cartridge() {
        let mut sample_cartridge = GenericCartridge::new("dummy", 3, 0, 0x200, 0xF9, 40);
        assert_eq!(sample_cartridge.cartridge_banks.len(), 0);

        // A slice implements 'Read'
        let mut dummy_file_data = vec![0_u8; 0x100000];

        sample_cartridge.load_banks(&mut dummy_file_data);

        assert_eq!(sample_cartridge.cartridge_banks.len(), 3);
        assert_eq!(sample_cartridge.cartridge_banks[0].data.len(), 0x200);
    }
}

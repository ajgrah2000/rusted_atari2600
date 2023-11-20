pub struct Constants {}

impl Constants {
    pub const ATARI2600_WIDTH:  u16 = 160;
    pub const ATARI2600_HEIGHT: u16 = 280;

    pub const PIXEL_WIDTH:  u8 = 4;
    pub const PIXEL_HEIGHT: u8 = 2;

    pub const BLIT_WIDTH:  u16 = Constants::ATARI2600_WIDTH  * (Constants::PIXEL_WIDTH  as u16);
    pub const BLIT_HEIGHT: u16 = Constants::ATARI2600_HEIGHT * (Constants::PIXEL_HEIGHT as u16);
}

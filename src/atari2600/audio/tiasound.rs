use super::sound;
use super::soundchannel;

pub struct TiaSound {
}

impl TiaSound {
    // CPU Clock rate, used to scale to real time.
    pub const CPU_CLOCK_RATE:u32 = 3580000;

    pub const SAMPLERATE:u16 = 32050;
    pub const CHANNELS:u8 = 2;
    pub const FREQ_DATA_MASK:u8 = 0x1F;
    pub const BITS:u8 = 8;

    pub fn new() -> Self {
        Self {
        }
    }

    pub fn get_next_audio_chunk(&mut self, length: u32) -> Vec<soundchannel::PlaybackType> {
        let mut stream = Vec::with_capacity((2*length) as usize);

        if length > 0 {
            for i in 0..(length * (sound::SDLUtility::MONO_STERO_FLAG as u32)) {
                stream.push(0x0); // Neutral volume
            }
        }

        stream
    }
}

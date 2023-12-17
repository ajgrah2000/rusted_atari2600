use super::super::clocks;
use super::sound;
use super::soundchannel;
use super::super::cpu::core;

pub struct TiaSound {
        volume:Vec<u8>,
        freq:Vec<u8>,
        poly4state:Vec<u8>,
        poly5state:Vec<u8>,
        wave_form:Vec<u8>,

        freq_pos:Vec<u32>,

        last_update_time: clocks::ClockType,

        working_stream: Vec::<soundchannel::PlaybackType>,
}

impl TiaSound {
    // CPU Clock rate, used to scale to real time.
    pub const CPU_CLOCK_RATE:u32 = core::Constants::CLOCK_HZ;

    pub const SAMPLERATE:u16 = 32050;
    pub const CHANNELS:u8 = 2;
    pub const FREQ_DATA_MASK:u8 = 0x1F;
    pub const BITS:u8 = 8;

    pub fn new() -> Self {
        Self {
            volume:vec![0; TiaSound::CHANNELS as usize],
            freq:vec![0; TiaSound::CHANNELS as usize],
            poly4state:vec![0; TiaSound::CHANNELS as usize],
            poly5state:vec![0; TiaSound::CHANNELS as usize],
            wave_form:vec![0; TiaSound::CHANNELS as usize],

            freq_pos:vec![0; TiaSound::CHANNELS as usize],

            last_update_time: 0,

            working_stream: Vec::new(),
        }
    }

    pub fn get_next_audio_chunk(&mut self, length: u32) -> Vec<soundchannel::PlaybackType> {
        let mut stream = Vec::with_capacity((2*length) as usize);

        if length > 0 {
            for i in 0..(length * (sound::SDLUtility::MONO_STERO_FLAG as u32)) {
                if !self.working_stream.is_empty() {
                    for j in 0..sound::SDLUtility::MONO_STERO_FLAG {
                        stream.push(self.working_stream.remove(0)); // Neutral volume
                    }
                }
            }
        }

        stream
    }

    // Clock poly 4, return new poly4 state
    // @staticmethod
    pub fn poly4(audio_ctrl:u8, poly5_state:u8, poly4_state:u8) -> u8 {

        let i = (0 == audio_ctrl & 0xF) ||
                ((0 == audio_ctrl & 0xC) && (((poly4_state & 0x3) != 0x3) && (0 != poly4_state & 0x3) && ((poly4_state & 0xF) != 0xA))) ||
                (((audio_ctrl & 0xC) == 0xC) &&  (0 != poly4_state & 0xC) && (0 == poly4_state & 0x2)) ||
                (((audio_ctrl & 0xC) == 0x4) && (0 == poly4_state & 0x8)) ||
                (((audio_ctrl & 0xC) == 0x8) && (0 == poly5_state & 0x1));

        let poly4output = (0x7 ^ (poly4_state >> 1)) | (i as u8) << 3;

        poly4output
    }

    // Clock poly 5, return new poly5 state
    // @staticmethod
    pub fn poly5(audio_ctrl:u8, poly5_state:u8, poly4_state:u8) -> u8{

        let in_5 = (0 == audio_ctrl & 0xF) ||
                   (((0 != audio_ctrl & 0x3) || ((poly4_state & 0xF) == 0xA)) && (0 == poly5_state & 0x1F)) ||
                   !((((0 != audio_ctrl & 0x3) || (0 == poly4_state & 0x1)) && ((0 == poly5_state & 0x8) || (0 == audio_ctrl & 0x3))) ^ (0 != poly5_state & 0x1));

        let poly5output = (poly5_state >> 1) | ((in_5 as u8) << 4);

        poly5output
    }

    // @staticmethod
    pub fn poly5clk(audio_ctrl:u8, poly5_state:u8) -> bool {
        let clockoutput = (((audio_ctrl & 0x3) != 0x2) || (0x2 == (poly5_state & 0x1E))) &&
                          (((audio_ctrl & 0x3) != 0x3) || (0 != poly5_state & 0x1));

        clockoutput
    }


    pub fn get_channel_data(&mut self, channel:u8, length:u16) -> Vec<u8> {
        // Stereo callback encodes left and right by using even/odd entries in the
        // stream.
        let mut stream = vec![0; length as usize];
        for i in 0..length {
    
            if 0 == self.freq_pos[channel as usize] % (self.freq[channel as usize] as u32 + 1) {
                let next_poly5 = TiaSound::poly5(self.wave_form[channel as usize], self.poly5state[channel as usize], self.poly4state[channel as usize]);

                if TiaSound::poly5clk(self.wave_form[channel as usize], self.poly5state[channel as usize]) {
                    self.poly4state[channel as usize] = TiaSound::poly4(self.wave_form[channel as usize], self.poly5state[channel as usize], self.poly4state[channel as usize]);
                }

                self.poly5state[channel as usize] = next_poly5;
            }
    
            if 0 != self.poly4state[channel as usize] & 1 {
                stream[i as usize] = (self.volume[channel as usize] & 0xF) * 0x7 & 0xFF;
            }
    
            self.freq_pos[channel as usize] += 1;
        }

        stream
    }

    
    // Update the current state of the emulated sound data before control
    // change, so previous wave form can be stopped at correct time before
    // control change.

    pub fn write_audio_ctrl_0(&mut self, clock: &mut clocks::Clock, address: u16, data:u8) {
        self.pre_write_generate_sound(clock);
        self.wave_form[0] = data & 0xFF;
    }

    pub fn write_audio_ctrl_1(&mut self, clock: &mut clocks::Clock, address: u16, data:u8) {
        self.pre_write_generate_sound(clock);
        self.wave_form[1] = data & 0xFF;
    }

    pub fn write_audio_freq_0(&mut self, clock: &mut clocks::Clock, address: u16, data:u8) {
        self.pre_write_generate_sound(clock);
        self.freq[0]     = data & TiaSound::FREQ_DATA_MASK;
    }

    pub fn write_audio_freq_1(&mut self, clock: &mut clocks::Clock, address: u16, data:u8) {
        self.pre_write_generate_sound(clock);
        self.freq[1]     = data & TiaSound::FREQ_DATA_MASK;
    }

    pub fn write_audio_vol_0(&mut self, clock: &mut clocks::Clock, address: u16, data:u8) {
        self.pre_write_generate_sound(clock);
        self.volume[0]   = data;
        self.post_write_generate_sound();
    }

    pub fn write_audio_vol_1(&mut self, clock: &mut clocks::Clock, address: u16, data:u8) {
        self.pre_write_generate_sound(clock);
        self.volume[1]   = data;
        self.post_write_generate_sound();
    }

    pub fn step(&mut self, clock: &clocks::Clock) {
    }

    // Wav
    fn pre_write_generate_sound(&mut self, clock: &mut clocks::Clock) {
        let audio_ticks:u32 = (clock.ticks - self.last_update_time) as u32;

        let mut raw_audio:(Vec<u8>, Vec<u8>) = (Vec::new(), Vec::new());

        raw_audio.0.append(&mut self.get_channel_data(0, ((TiaSound::SAMPLERATE as u64 * audio_ticks as u64)/TiaSound::CPU_CLOCK_RATE as u64) as u16));
        raw_audio.1.append(&mut self.get_channel_data(1, ((TiaSound::SAMPLERATE as u64 * audio_ticks as u64)/TiaSound::CPU_CLOCK_RATE as u64) as u16));

        self.last_update_time = clock.ticks;

        while !raw_audio.0.is_empty() && !raw_audio.1.is_empty() {
            if 2 == sound::SDLUtility::MONO_STERO_FLAG {
                self.working_stream.push(raw_audio.0.remove(0));
                self.working_stream.push(raw_audio.1.remove(0));
            } else {
                self.working_stream.push(((raw_audio.0.remove(0) as u16 + raw_audio.1.remove(0) as u16)/2) as u8);
            }
        }
    }

    fn post_write_generate_sound(&mut self) {
    }
}

use super::super::audio::tiasound;
use super::soundchannel;
use sdl2::audio;

pub trait SoundQueue {
    fn add_audio(&mut self, new_audio_data: &Vec<soundchannel::PlaybackType>);
    fn size(&self) -> usize;
}

impl SoundQueue for audio::AudioQueue<soundchannel::PlaybackType> {
    fn add_audio(&mut self, new_audio_data: &Vec<soundchannel::PlaybackType>) {
        self.queue_audio(&new_audio_data).unwrap();
    }

    fn size(&self) -> usize {
        self.size() as usize
    }
}

pub type SoundQueueType = audio::AudioQueue<soundchannel::PlaybackType>;

pub struct HoundOutput {
    spec: hound::WavSpec,
    out_file: hound::WavWriter<std::io::BufWriter<std::fs::File>>,
}

impl HoundOutput {
    pub fn new(filename: &str) -> Self {
        let wav_spec = hound::WavSpec {
            channels: SDLUtility::MONO_STERO_FLAG as u16,
            sample_rate: tiasound::TiaSound::SAMPLERATE as u32, // Setting to 'chip' frequency, to avoid conversion.
            bits_per_sample: std::mem::size_of::<soundchannel::PlaybackType>() as u16 * 8,
            sample_format: hound::SampleFormat::Int,
        };
        Self {
            spec: wav_spec,
            out_file: hound::WavWriter::create(std::path::Path::new(&filename), wav_spec).unwrap(),
        }
    }

    pub fn write(&mut self, data: &Vec<soundchannel::PlaybackType>) {
        for d in data {
            self.out_file.write_sample(*d as i8).unwrap();
        }
    }
}

impl SoundQueue for HoundOutput {
    fn add_audio(&mut self, new_audio_data: &Vec<soundchannel::PlaybackType>) {
        self.write(new_audio_data);
    }

    fn size(&self) -> usize {
        // Arbitrary number, maximum size to write in once go.
        1_000_000
    }
}

pub struct SDLUtility {}

impl SDLUtility {
    // TODO: Fix up values, make them more dynamic, do better comparisons
    // Not sure how they compare on different PCs
    const TARGET_QUEUE_LENGTH: u32 = 4096; // This drives the 'delay' in audio, but too small for the speed and they aren't filled fast enough
    const AUDIO_SAMPLE_SIZE: u16 = 1024; // 'Desired' sample size, too small and SDL buffer doesn't stay filled (pops/crackles).
    const FRACTION_FILL: f32 = 0.05; // TODO: FUDGE FACTOR.  Don't completely fill, samples a removed 1 at a time, don't fill them immediately.

    pub const MONO_STERO_FLAG: u8 = 2; // TODO: Make this configurable 1 - mono, 2 - stereo

    pub fn get_audio_queue(sdl_context: &mut sdl2::Sdl) -> Box<dyn SoundQueue> {
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = audio::AudioSpecDesired {
            freq: Some(Sound::SAMPLERATE as i32),
            channels: Some(SDLUtility::MONO_STERO_FLAG), // mono
            samples: Some(SDLUtility::AUDIO_SAMPLE_SIZE),
        };

        let audio_queue = audio_subsystem.open_queue::<soundchannel::PlaybackType, _>(None, &desired_spec).unwrap();

        audio_queue.clear();
        audio_queue.resume(); // Start the audio (nothing in the queue at this point).

        Box::new(audio_queue)
    }

    pub fn top_up_audio_queue<F>(audio_queue: &mut dyn SoundQueue, mut get_additional_buffer: F)
    where
        F: FnMut(u32) -> Vec<soundchannel::PlaybackType>,
    {
        let fill_size = if SDLUtility::TARGET_QUEUE_LENGTH as usize > audio_queue.size() {
            ((SDLUtility::TARGET_QUEUE_LENGTH - audio_queue.size() as u32) as f32 * SDLUtility::FRACTION_FILL) as u32
        } else {
            SDLUtility::TARGET_QUEUE_LENGTH
        };
        // If 'stereo' the buffer is twice as large, so just as for half as much.
        let sound_buffer = get_additional_buffer(fill_size / (SDLUtility::MONO_STERO_FLAG as u32));
        audio_queue.add_audio(&sound_buffer);
    }
}

pub struct Sound {}

impl Sound {
    const SAMPLERATE: u32 = 32050;
    const BITS: u8 = 8;
}

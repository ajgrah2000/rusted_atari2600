use sdl2::audio;
use super::soundchannel;

pub type SoundQueueType = audio::AudioQueue<soundchannel::PlaybackType>;
pub struct SDLUtility {}

impl SDLUtility {
    // TODO: Fix up values, make them more dynamic, do better comparisons
    // Not sure how they compare on different PCs
    const TARGET_QUEUE_LENGTH:u32 = 2048; // This drives the 'delay' in audio, but too small for the speed and they aren't filled fast enough
    const AUDIO_SAMPLE_SIZE:u16 = 1024; // 'Desired' sample size, too small and SDL buffer doesn't stay filled (pops/crackles).
    const FRACTION_FILL:f32 = 0.05; // TODO: FUDGE FACTOR.  Don't completely fill, samples a removed 1 at a time, don't fill them immediately.

    pub const MONO_STERO_FLAG:u8 = 1; // TODO: Make this configurable 1 - mono, 2 - stereo

    pub fn get_audio_queue (
        sdl_context: &mut sdl2::Sdl,
    ) -> Result<SoundQueueType, String> {
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = audio::AudioSpecDesired {
            freq: Some(Sound::SAMPLERATE as i32),
            channels: Some(SDLUtility::MONO_STERO_FLAG), // mono
            samples: Some(SDLUtility::AUDIO_SAMPLE_SIZE),
        };

        audio_subsystem.open_queue::<soundchannel::PlaybackType,_>(None, &desired_spec)
    }

    pub fn top_up_audio_queue<F>(audio_queue: &mut SoundQueueType, mut get_additional_buffer:F)
        where F: FnMut(u32) ->Vec<soundchannel::PlaybackType> {
            assert!(audio_queue.size() <= SDLUtility::TARGET_QUEUE_LENGTH as u32);
            let fill_size = ((SDLUtility::TARGET_QUEUE_LENGTH - audio_queue.size()) as f32 * SDLUtility::FRACTION_FILL) as u32;
            // If 'stereo' the buffer is twice as large, so just as for half as much.
            let sound_buffer = get_additional_buffer(fill_size/(SDLUtility::MONO_STERO_FLAG as u32));
            audio_queue.queue_audio(&sound_buffer).unwrap();
    }
}

pub struct Sound {
}

impl Sound {
    const SAMPLERATE: u32 = 44100;
    const BITS: u8 = 8;
}


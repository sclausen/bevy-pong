use rodio::Source;
use std::time::Duration;

pub struct SfxrDecoder {
	generator: sfxr::Generator,
}

impl SfxrDecoder {
	pub fn new(sample: sfxr::Sample) -> Self {
		Self {
			generator: sfxr::Generator::new(sample),
		}
	}
}

impl Iterator for SfxrDecoder {
	type Item = f32;

	fn next(&mut self) -> Option<Self::Item> {
		self.generator.next()
	}
}

impl Source for SfxrDecoder {
	fn current_frame_len(&self) -> Option<usize> {
		None
	}

	fn channels(&self) -> u16 {
		1
	}

	fn sample_rate(&self) -> u32 {
		44_100
	}

	fn total_duration(&self) -> Option<Duration> {
		None
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCodec {
	Opus,
	Vorbis
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioEncoding {
	Pcm,
	ALaw,
	MuLaw
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioChannels {
	/// Single channel (mono) audio.
	Mono,
	/// 2 channel (stereo) audio.
	Stereo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioContainer {
	/// Containerless audio, only used with PCM, A-law, and mu-law encodings.
	Raw(AudioEncoding),
	/// RIFF format, aka .WAV lossless audio.
	Riff(AudioEncoding),
	/// MP3 format audio.
	Mp3,
	/// OGG format audio.
	Ogg(AudioCodec),
	/// WEBM format audio.
	Webm(AudioCodec)
}

/// Struct used for negotiating an audio format supported by both the application and the speech synthesiser.
#[derive(Debug, Default, Clone)]
pub struct AudioFormatPreference {
	pub sample_rates: Option<Vec<u32>>,
	pub channels: Option<Vec<AudioChannels>>,
	pub bitrates: Option<Vec<u16>>,
	pub containers: Option<Vec<AudioContainer>>
}

impl AudioFormatPreference {
	pub fn with_prefer_sample_rates(mut self, pref: impl IntoIterator<Item = u32>) -> Self {
		match self.sample_rates.as_mut() {
			None => self.sample_rates = Some(pref.into_iter().collect()),
			Some(sample_rates) => sample_rates.extend(pref)
		}
		self
	}

	pub fn with_prefer_channels(mut self, pref: impl IntoIterator<Item = AudioChannels>) -> Self {
		match self.channels.as_mut() {
			None => self.channels = Some(pref.into_iter().collect()),
			Some(channels) => channels.extend(pref)
		}
		self
	}

	pub fn with_prefer_bitrates(mut self, pref: impl IntoIterator<Item = u16>) -> Self {
		match self.bitrates.as_mut() {
			None => self.bitrates = Some(pref.into_iter().collect()),
			Some(bitrates) => bitrates.extend(pref)
		}
		self
	}

	pub fn with_prefer_containers(mut self, pref: impl IntoIterator<Item = AudioContainer>) -> Self {
		match self.containers.as_mut() {
			None => self.containers = Some(pref.into_iter().collect()),
			Some(containers) => containers.extend(pref)
		}
		self
	}
}

#[derive(Debug, Clone)]
pub struct AudioFormat {
	name: Option<Box<str>>,
	sample_rate: u32,
	channels: AudioChannels,
	bitrate: Option<u16>,
	container: AudioContainer
}

impl AudioFormat {
	pub fn new(sample_rate: u32, channels: AudioChannels, bitrate: Option<u16>, container: AudioContainer) -> Self {
		AudioFormat {
			name: None,
			sample_rate,
			channels,
			bitrate,
			container
		}
	}

	pub fn new_named(name: impl Into<Box<str>>, sample_rate: u32, channels: AudioChannels, bitrate: Option<u16>, container: AudioContainer) -> Self {
		AudioFormat {
			name: Some(name.into()),
			sample_rate,
			channels,
			bitrate,
			container
		}
	}

	pub fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}

	pub fn sample_rate(&self) -> u32 {
		self.sample_rate
	}

	pub fn channels(&self) -> AudioChannels {
		self.channels
	}

	pub fn bitrate(&self) -> Option<u16> {
		self.bitrate
	}

	pub fn container(&self) -> AudioContainer {
		self.container
	}
}

use core::future::Future;

pub use ::ssml;

mod audio;
pub use self::audio::{AudioChannels, AudioCodec, AudioContainer, AudioEncoding, AudioFormat, AudioFormatPreference};
mod event;
pub use self::event::{BasicViseme, BasicVisemeFrame, BlendShape, BlendShapeVisemeFrame, UtteranceEvent, UtteranceEventStream};

/// Configuration for a single speech synthesis utterance.
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct UtteranceConfig {
	/// Whether to emit [`UtteranceEvent::WordBoundary`] events.
	pub emit_word_boundary_events: bool,
	/// Whether to emit [`UtteranceEvent::SentenceBoundary`] events.
	pub emit_sentence_boundary_events: bool,
	/// Whether to emit [`UtteranceEvent::VisemesChunk`]/[`UtteranceEvent::BlendShapeVisemesChunk`] events.
	pub emit_visemes: bool,
	/// The name of the voice to use for synthesis.
	pub voice: Option<Box<str>>,
	/// The language to use for raw text synthesis.
	pub language: Option<Box<str>>
}

impl UtteranceConfig {
	/// Configures whether to emit [`UtteranceEvent::WordBoundary`] events.
	pub fn with_emit_word_boundary_events(mut self, x: bool) -> Self {
		self.emit_word_boundary_events = x;
		self
	}

	/// Configures whether to emit [`UtteranceEvent::SentenceBoundary`] events.
	pub fn with_emit_sentence_boundary_events(mut self, x: bool) -> Self {
		self.emit_sentence_boundary_events = x;
		self
	}

	/// Configures whether to emit [`UtteranceEvent::VisemesChunk`]/[`UtteranceEvent::BlendShapeVisemesChunk`] events.
	pub fn with_emit_visemes(mut self, x: bool) -> Self {
		self.emit_visemes = x;
		self
	}

	/// Configures the name of the voice to use for synthesis.
	///
	/// This is generally only used for [text synthesis](SpeechSynthesiser::synthesise_text_stream) and will be ignored
	/// with [SSML synthesis](SpeechSynthesiser::synthesise_ssml_stream).
	pub fn with_voice(mut self, x: impl Into<Box<str>>) -> Self {
		self.voice = Some(x.into());
		self
	}

	/// Configures the language to use for raw text synthesis.
	///
	/// This is generally only used for [text synthesis](SpeechSynthesiser::synthesise_text_stream) and will be ignored
	/// with [SSML synthesis](SpeechSynthesiser::synthesise_ssml_stream).
	pub fn with_language(mut self, x: impl Into<Box<str>>) -> Self {
		self.language = Some(x.into());
		self
	}
}

/// Common trait for a speech synthesiser.
pub trait SpeechSynthesiser {
	type Error: std::error::Error + Send + Sync + 'static;

	/// Negotiate an audio format supported by both the application and this synthesiser. The synthesiser returns `None`
	/// if:
	/// - Any requested sample rate is not supported.
	/// - Any requested container is not supported.
	/// - Any requested channel count is not supported.
	///
	/// If multiple values are provided for a preference by the application, the synthesiser should prioritise the
	/// highest quality configuration. For optional properties (such as bitrate), this should **not** fail, and instead
	/// return the highest quality bitrate closest to the user's preference.
	///
	/// i.e., for a synthesiser that only supports 44100 Hz, stereo MP3 at either 128 or 192 Kbps:
	/// - requesting a sample rate of `48000` or `22050` should return `None`,
	/// - requesting [`AudioChannels::Mono`] should return `None`,
	/// - requesting OGG format should return `None`,
	/// - and requesting 44100 Hz stereo MP3 at 160 Kbps should return an audio format of 44100 Hz stereo MP3 **at 192
	///   Kbps**.
	fn negotiate_audio_format(&self, pref: AudioFormatPreference) -> Option<AudioFormat>;

	/// Stream the synthesis of an [`ssml`] document.
	///
	/// Audio will be streamed in chunks, in the format specified by the given [`AudioFormat`]. You can negotiate an
	/// audio format that both your application and the synthesiser supports via
	/// [`SpeechSynthesiser::negotiate_audio_format`].
	///
	/// You'll need to configure whether to receive events like visemes or boundaries with an [`UtteranceConfig`].
	fn synthesise_ssml_stream(
		&self,
		input: ssml::Speak,
		audio_format: &AudioFormat,
		config: &UtteranceConfig
	) -> impl Future<Output = Result<impl UtteranceEventStream<Self::Error>, Self::Error>> + Send;

	/// Stream the synthesis of **raw text**.
	///
	/// Note that text is hardly controllable. For more advanced control of the synthesised speech, including prosody,
	/// pitch contour, or pronunciation of words, see [`SpeechSynthesiser::synthesise_ssml_stream`] and [`ssml`].
	///
	/// This method should **not** be able to accept a raw string of SSML. SSML should be handled exclusively through
	/// [`SpeechSynthesiser::synthesise_ssml_stream`].
	///
	/// Audio will be streamed in chunks, in the format specified by the given [`AudioFormat`]. You can negotiate an
	/// audio format that both your application and the synthesiser supports via
	/// [`SpeechSynthesiser::negotiate_audio_format`].
	///
	/// You'll need to configure whether to receive events like visemes or boundaries with an [`UtteranceConfig`].
	fn synthesise_text_stream(
		&self,
		input: impl AsRef<str> + Send,
		audio_format: &AudioFormat,
		config: &UtteranceConfig
	) -> impl Future<Output = Result<impl UtteranceEventStream<Self::Error>, Self::Error>> + Send;
}

use futures_core::Stream;

/// Key & weight information for a single blend shape as part of a [`BlendShapeVisemeFrame`].
#[derive(Debug, Clone)]
pub struct BlendShape {
	/// Blend shape key, typically as an [ARKit](https://developer.apple.com/documentation/arkit/arblendshapelocation?language=objc) blend shape.
	pub key: Box<str>,
	/// Weight of the blend shape from `0.0` (no influence) to `1.0` (full influence).
	pub weight: f32
}

/// A single frame for visemes in blend shape format.
#[derive(Debug, Clone)]
pub struct BlendShapeVisemeFrame {
	pub blendshapes: Box<[BlendShape]>,
	/// Offset of this blendshape frame relative to the beginning of the audio stream.
	pub frame_offset: f32
}

/// A 'basic' viseme.
///
/// The format for basic visemes is not currently defined due to conflicts between Azure Cognitive Speech Services &
/// Amazon Polly's viseme mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicViseme(pub char);

/// A single frame of 'basic' visemes.
#[derive(Debug, Clone)]
pub struct BasicVisemeFrame {
	pub viseme: BasicViseme,
	/// Offset of this viseme frame relative to the beginning of the audio stream.
	pub frame_offset: f32
}

/// An event emitted by a speech synthesiser's [`UtteranceEventStream`].
#[derive(Debug)]
#[non_exhaustive]
pub enum UtteranceEvent {
	/// Marks the audio offset of an [`ssml::Mark`].
	SsmlMark {
		/// The position in milliseconds the mark occurred, relative to the beginning of the audio stream.
		at_millis: f32,
		/// The name of the mark in SSML.
		mark: Box<str>
	},
	/// Marks the time boundary of a spoken word in the audio.
	WordBoundary {
		/// The position in milliseconds the spoken word begun, relative to the beginning of the audio stream.
		from_millis: f32,
		/// The position in milliseconds the spoken word ended, relative to the beginning of the audio stream.
		to_millis: f32,
		/// The text of the single word spoken between this boundary.
		text: Box<str>
	},
	/// Marks the time boundary of a sentence in the audio.
	SentenceBoundary {
		/// The position in milliseconds the sentence begun, relative to the beginning of the audio stream.
		from_millis: f32,
		/// The position in milliseconds the sentence ended, relative to the beginning of the audio stream.
		to_millis: f32,
		/// The text of the sentence spoken between this boundary.
		text: Box<str>
	},
	/// A chunk of viseme frames in blend shape format.
	BlendShapeVisemesChunk(Box<[BlendShapeVisemeFrame]>),
	/// A chunk of frames of 'basic' visemes.
	VisemesChunk(Box<[BasicVisemeFrame]>),
	/// A chunk of synthesised speech audio in the requested format.
	AudioChunk(Box<[u8]>)
}

/// A stream of [`UtteranceEvent`]s returned by the synthesiser.
///
/// May be an [`Err`][Result::Err] if an error was encountered during synthesis (i.e. a socket disconnect).
pub trait UtteranceEventStream<E>: Stream<Item = Result<UtteranceEvent, E>> + Send {}

impl<E, T: Stream<Item = Result<UtteranceEvent, E>> + Send> UtteranceEventStream<E> for T {}

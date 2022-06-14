use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Audio {
	MP3,
	Dolby51,
	Dual,
	Line,
	DTS,
	AAC,
	AC3
}

static AUDIOS: phf::Map<&'static str, Audio> = phf::phf_map! {
	"MP3" => Audio::MP3,
	"DD51" => Audio::Dolby51,
	"DD5.1" => Audio::Dolby51,
	"Dual-Audio" => Audio::Dual,
	"Dual Audio" => Audio::Dual,
	"LiNE" => Audio::Line,
	"DTS" => Audio::DTS,
	"AAC" => Audio::AAC,
	"AAC2.0" => Audio::AAC,
	"AAC.2.0" => Audio::AAC,
	"AC3" => Audio::AC3,
	"AC35.1" => Audio::AC3,
	"AC3.5.1" => Audio::AC3
};

impl FromStr for Audio {
	type Err = Error;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		AUDIOS.get(input).cloned().ok_or_else(|| Error::InvalidAudio(input.into()))
	}
}


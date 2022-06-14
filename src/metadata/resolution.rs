use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resolution {
    R240i,
    R240p,
    R320i,
    R320p,
    R480i,
    R480p,
    R576i,
    R576p,
    R720i,
    R720p,
    R1080i,
    R1080p,
    R2160i,
    R2160p,
    R4k,
}

static RESOLUTIONS: phf::Map<&'static str, Resolution> = phf::phf_map! {
    "240i" => Resolution::R240i,
    "240I" => Resolution::R240i,
    "240p" => Resolution::R240p,
    "240P" => Resolution::R240p,
    "320i" => Resolution::R320i,
    "320I" => Resolution::R320i,
    "320p" => Resolution::R320p,
    "320P" => Resolution::R320p,
    "480i" => Resolution::R480i,
    "480I" => Resolution::R480i,
    "480p" => Resolution::R480p,
    "480P" => Resolution::R480p,
    "576i" => Resolution::R576i,
    "576I" => Resolution::R576i,
    "576p" => Resolution::R576p,
    "576P" => Resolution::R576p,
    "720i" => Resolution::R720i,
    "720I" => Resolution::R720i,
    "720p" => Resolution::R720p,
    "720P" => Resolution::R720p,
    "1080i" => Resolution::R1080i,
    "1080I" => Resolution::R1080i,
    "1080p" => Resolution::R1080p,
    "1080P" => Resolution::R1080p,
    "2160i" => Resolution::R2160i,
    "2160I" => Resolution::R2160i,
    "2160p" => Resolution::R2160p,
    "2160P" => Resolution::R2160p,
    "4k" => Resolution::R4k,
    "4K" => Resolution::R4k
};

impl FromStr for Resolution {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        RESOLUTIONS.get(input).cloned().ok_or_else(|| Error::InvalidResolution(input.into()))
    }
}

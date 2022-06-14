use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Codec {
    X264,
    X265,
    Xvid,
}

static CODECS: phf::Map<&'static str, Codec> = phf::phf_map! {
    "x264" => Codec::X264,
    "X264" => Codec::X264,
    "h264" => Codec::X264,
    "H264" => Codec::X264,
    "h.264" => Codec::X264,
    "H.264" => Codec::X264,
    "x265" => Codec::X265,
    "X265" => Codec::X265,
    "h265" => Codec::X265,
    "H265" => Codec::X265,
    "h.265" => Codec::X265,
    "H.265" => Codec::X265,
    "hevc" => Codec::X265,
    "hevC" => Codec::X265,
    "heVc" => Codec::X265,
    "heVC" => Codec::X265,
    "hEvc" => Codec::X265,
    "hEvC" => Codec::X265,
    "hEVc" => Codec::X265,
    "hEVC" => Codec::X265,
    "Hevc" => Codec::X265,
    "HevC" => Codec::X265,
    "HeVc" => Codec::X265,
    "HeVC" => Codec::X265,
    "HEvc" => Codec::X265,
    "HEvC" => Codec::X265,
    "HEVc" => Codec::X265,
    "HEVC" => Codec::X265,
    "xvid" => Codec::Xvid,
    "xviD" => Codec::Xvid,
    "xvId" => Codec::Xvid,
    "xvID" => Codec::Xvid,
    "xVid" => Codec::Xvid,
    "xViD" => Codec::Xvid,
    "xVId" => Codec::Xvid,
    "xVID" => Codec::Xvid,
    "Xvid" => Codec::Xvid,
    "XviD" => Codec::Xvid,
    "XvId" => Codec::Xvid,
    "XvID" => Codec::Xvid,
    "XVid" => Codec::Xvid,
    "XViD" => Codec::Xvid,
    "XVId" => Codec::Xvid,
    "XVID" => Codec::Xvid,
};

impl FromStr for Codec {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        CODECS
            .get(input)
            .cloned()
            .ok_or_else(|| Error::InvalidCodec(input.into()))
    }
}

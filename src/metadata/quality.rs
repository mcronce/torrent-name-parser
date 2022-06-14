use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Quality {
    HDTV,
    Cam,
    BluRay,
    TS,
    HD,
    DVD,
    Web,
}

static QUALITIES: phf::Map<&'static str, Quality> = phf::phf_map! {
    "HDTV" => Quality::HDTV,
    "PPV.HDTV" => Quality::HDTV,
    "PDTV" => Quality::HDTV,
    "PPV.PDTV" => Quality::HDTV,
    "hdtv" => Quality::HDTV,
    "CAM" => Quality::Cam,
    "HDCAM" => Quality::Cam,
    "CamRip" => Quality::Cam,
    "BrRip" => Quality::BluRay,
    "BRRip" => Quality::BluRay,
    "BluRay" => Quality::BluRay,
    "Bluray" => Quality::BluRay,
    "bluRay" => Quality::BluRay,
    "bluray" => Quality::BluRay,
    "TS" => Quality::TS,
    "WEB" => Quality::Web,
    "WEB-" => Quality::Web,
    "WEB-DL" => Quality::Web,
    "WEBDL" => Quality::Web,
    "PPV WEB" => Quality::Web,
    "PPV WEB-" => Quality::Web,
    "PPV WEB-DL" => Quality::Web,
    "PPV WEBDL" => Quality::Web,
    "WEB DVDRip" => Quality::Web,
    "WEB- DVDRip" => Quality::Web,
    "WEB-DL DVDRip" => Quality::Web,
    "WEBDL DVDRip" => Quality::Web,
    "PPV WEB DVDRip" => Quality::Web,
    "PPV WEB- DVDRip" => Quality::Web,
    "PPV WEB-DL DVDRip" => Quality::Web,
    "PPV WEBDL DVDRip" => Quality::Web,
    "WEBRip" => Quality::Web,
    "WEBrip" => Quality::Web,
    "WBBRip" => Quality::Web,
    "WBBrip" => Quality::Web,
    "HDRip" => Quality::HD,
    "HdRip" => Quality::HD,
    "DVDRip" => Quality::DVD,
    "DVDRiP" => Quality::DVD,
    "DVDRIP" => Quality::DVD,
    "DvDScr" => Quality::DVD
};

impl FromStr for Quality {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        QUALITIES
            .get(input)
            .cloned()
            .ok_or_else(|| Error::InvalidQuality(input.into()))
    }
}

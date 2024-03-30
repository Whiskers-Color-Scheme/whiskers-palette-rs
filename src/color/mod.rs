use serde::{Deserialize, Serialize};

pub enum WhiskersColor {
    PantherBanana,
    PantherBlueberry,
    PantherCherry,
    PantherGrape,
    PantherKiwi,
    PantherTangerine,
    PantherNeutral,
    PantherNeutralTwo,
    PantherNeutralThree,
    PantherNeutralFor,
    PantherNeutralFive,
    PantherNeutralSix,
    PantherNeutralSeven,
    PantherNeutralEight,
    PantherText,
    PantherTextTwo,
    PantherTextThree,
    PantherTextFor,
    TigerBanana,
    TigerBlueberry,
    TigerCherry,
    TigerGrape,
    TigerKiwi,
    TigerTangerine,
    TigerNeutral,
    TigerNeutralTwo,
    TigerNeutralThree,
    TigerNeutralFor,
    TigerNeutralFive,
    TigerNeutralSix,
    TigerNeutralSeven,
    TigerNeutralEight,
    TigerText,
    TigerTextTwo,
    TigerTextThree,
    TigerTextFor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Color {
    pub name: String,
    pub hex: String,
    pub rgb: RGB,
    pub hsl: HSL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RGB {
    pub r: String,
    pub g: String,
    pub b: String,
    pub rgb: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HSL {
    pub h: String,
    pub s: String,
    pub l: String,
    pub hsl: String,
}

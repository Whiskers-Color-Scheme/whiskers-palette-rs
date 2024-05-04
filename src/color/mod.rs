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
    PantherNeutralFour,
    PantherNeutralFive,
    PantherNeutralSix,
    PantherNeutralSeven,
    PantherNeutralEight,
    PantherText,
    PantherTextTwo,
    PantherTextThree,
    PantherTextFour,
    TigerBanana,
    TigerBlueberry,
    TigerCherry,
    TigerGrape,
    TigerKiwi,
    TigerTangerine,
    TigerNeutral,
    TigerNeutralTwo,
    TigerNeutralThree,
    TigerNeutralFour,
    TigerNeutralFive,
    TigerNeutralSix,
    TigerNeutralSeven,
    TigerNeutralEight,
    TigerText,
    TigerTextTwo,
    TigerTextThree,
    TigerTextFour,
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

use crate::color::{Color, WhiskersColor};

#[derive(Debug, Clone)]
pub struct WhiskersPalette {
    pub panther: Palette,
    pub tiger: Palette,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub banana: Color,
    pub blueberry: Color,
    pub cherry: Color,
    pub grape: Color,
    pub kiwi: Color,
    pub tangerine: Color,
    pub neutral: Color,
    pub neutral_two: Color,
    pub neutral_three: Color,
    pub neutral_for: Color,
    pub neutral_five: Color,
    pub neutral_six: Color,
    pub neutral_seven: Color,
    pub neutral_eight: Color,
    pub text: Color,
    pub text_two: Color,
    pub text_three: Color,
    pub text_for: Color,
}

pub fn get_panther_palette() -> Palette {
    let panther_json = include_str!("panther.json");
    let colors = serde_json::from_str::<Vec<Color>>(&panther_json).unwrap();

    Palette {
        banana: colors.get(0).unwrap().to_owned(),
        blueberry: colors.get(1).unwrap().to_owned(),
        cherry: colors.get(2).unwrap().to_owned(),
        grape: colors.get(3).unwrap().to_owned(),
        kiwi: colors.get(4).unwrap().to_owned(),
        tangerine: colors.get(5).unwrap().to_owned(),
        neutral: colors.get(6).unwrap().to_owned(),
        neutral_two: colors.get(7).unwrap().to_owned(),
        neutral_three: colors.get(8).unwrap().to_owned(),
        neutral_for: colors.get(9).unwrap().to_owned(),
        neutral_five: colors.get(10).unwrap().to_owned(),
        neutral_six: colors.get(11).unwrap().to_owned(),
        neutral_seven: colors.get(12).unwrap().to_owned(),
        neutral_eight: colors.get(13).unwrap().to_owned(),
        text: colors.get(14).unwrap().to_owned(),
        text_two: colors.get(15).unwrap().to_owned(),
        text_three: colors.get(16).unwrap().to_owned(),
        text_for: colors.get(17).unwrap().to_owned(),
    }
}

pub fn get_tiger_palette() -> Palette {
    let tiger_json = include_str!("tiger.json");
    let colors = serde_json::from_str::<Vec<Color>>(&tiger_json).unwrap();

    Palette {
        banana: colors.get(0).unwrap().to_owned(),
        blueberry: colors.get(1).unwrap().to_owned(),
        cherry: colors.get(2).unwrap().to_owned(),
        grape: colors.get(3).unwrap().to_owned(),
        kiwi: colors.get(4).unwrap().to_owned(),
        tangerine: colors.get(5).unwrap().to_owned(),
        neutral: colors.get(6).unwrap().to_owned(),
        neutral_two: colors.get(7).unwrap().to_owned(),
        neutral_three: colors.get(8).unwrap().to_owned(),
        neutral_for: colors.get(9).unwrap().to_owned(),
        neutral_five: colors.get(10).unwrap().to_owned(),
        neutral_six: colors.get(11).unwrap().to_owned(),
        neutral_seven: colors.get(12).unwrap().to_owned(),
        neutral_eight: colors.get(13).unwrap().to_owned(),
        text: colors.get(14).unwrap().to_owned(),
        text_two: colors.get(15).unwrap().to_owned(),
        text_three: colors.get(16).unwrap().to_owned(),
        text_for: colors.get(17).unwrap().to_owned(),
    }
}

pub fn get_whiskers_palette() -> WhiskersPalette {
    WhiskersPalette {
        panther: get_panther_palette(),
        tiger: get_tiger_palette(),
    }
}

pub fn get_color(color: WhiskersColor) -> Color {
    match color {
        WhiskersColor::PantherBanana => get_panther_palette().banana,
        WhiskersColor::PantherBlueberry => get_panther_palette().blueberry,
        WhiskersColor::PantherCherry => get_panther_palette().cherry,
        WhiskersColor::PantherGrape => get_panther_palette().grape,
        WhiskersColor::PantherKiwi => get_panther_palette().kiwi,
        WhiskersColor::PantherTangerine => get_panther_palette().tangerine,
        WhiskersColor::PantherNeutral => get_panther_palette().neutral,
        WhiskersColor::PantherNeutralTwo => get_panther_palette().neutral_two,
        WhiskersColor::PantherNeutralThree => get_panther_palette().neutral_three,
        WhiskersColor::PantherNeutralFor => get_panther_palette().neutral_for,
        WhiskersColor::PantherNeutralFive => get_panther_palette().neutral_five,
        WhiskersColor::PantherNeutralSix => get_panther_palette().neutral_six,
        WhiskersColor::PantherNeutralSeven => get_panther_palette().neutral_seven,
        WhiskersColor::PantherNeutralEight => get_panther_palette().neutral_eight,
        WhiskersColor::PantherText => get_panther_palette().text,
        WhiskersColor::PantherTextTwo => get_panther_palette().text_two,
        WhiskersColor::PantherTextThree => get_panther_palette().text_three,
        WhiskersColor::PantherTextFor => get_panther_palette().text_for,
        WhiskersColor::TigerBanana => get_tiger_palette().banana,
        WhiskersColor::TigerBlueberry => get_tiger_palette().blueberry,
        WhiskersColor::TigerCherry => get_tiger_palette().cherry,
        WhiskersColor::TigerGrape => get_tiger_palette().grape,
        WhiskersColor::TigerKiwi => get_tiger_palette().kiwi,
        WhiskersColor::TigerTangerine => get_tiger_palette().tangerine,
        WhiskersColor::TigerNeutral => get_tiger_palette().neutral,
        WhiskersColor::TigerNeutralTwo => get_tiger_palette().neutral_two,
        WhiskersColor::TigerNeutralThree => get_tiger_palette().neutral_three,
        WhiskersColor::TigerNeutralFor => get_tiger_palette().neutral_for,
        WhiskersColor::TigerNeutralFive => get_tiger_palette().neutral_five,
        WhiskersColor::TigerNeutralSix => get_tiger_palette().neutral_six,
        WhiskersColor::TigerNeutralSeven => get_tiger_palette().neutral_seven,
        WhiskersColor::TigerNeutralEight => get_tiger_palette().neutral_eight,
        WhiskersColor::TigerText => get_tiger_palette().text,
        WhiskersColor::TigerTextTwo => get_tiger_palette().text_two,
        WhiskersColor::TigerTextThree => get_tiger_palette().text_three,
        WhiskersColor::TigerTextFor => get_tiger_palette().text_for,
    }
}

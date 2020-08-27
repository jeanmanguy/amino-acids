use crate::{
    colors::{Blue, Color},
    Rgb,
};

pub trait Palette {
    const A: &'static Rgb;
    const C: &'static u8;
    const S: &'static u8;
    const T: &'static u8;
}

#[derive(Debug)]
pub struct Clustal;

impl Palette for Clustal {
    const A: &'static Rgb = Blue::RGB;
    const C: &'static u8 = &2u8;
    const S: &'static u8 = &3u8;
    const T: &'static u8 = &4u8;
}

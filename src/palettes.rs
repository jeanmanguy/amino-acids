/// RGB colour
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/* --------------------------------- palette -------------------------------- */

pub trait Colour {
    const RGB: &'static Rgb;
}

macro_rules! palette {
    ($(
        $aa:ident
    ),*) => {
        pub trait Palette {
            $(
                const $aa: &'static Rgb;
            )*
        }
    };
}

macro_rules! impl_palette {
    ($name:ident ($(
        $aa:ident $colour:path
    ),*)) => {
        #[derive(Debug)]
        pub struct $name;

        impl Palette for $name {
            $(
                const $aa: &'static Rgb = $colour;
            )*
        }


    };
}

palette! {A, C, D, E, F, G, H, I, K, L, M, N, P, Q, R, S, T, V, W, Y, GAP}

/* --------------------------------- colours -------------------------------- */

macro_rules! add_colour {
    ($name:ident, $r:expr, $g:expr, $b:expr) => {
        pub struct $name;

        impl Colour for $name {
            const RGB: &'static Rgb = &Rgb {
                r: $r,
                g: $g,
                b: $b,
            };
        }
    };
}

add_colour! {SkyBlue, 131u8, 182u8, 234u8}
add_colour! {Red, 229u8, 69u8, 45u8}
add_colour! {White, 255u8, 255u8, 255u8}
add_colour! {Yellow, 211u8, 211u8, 48u8}
add_colour! {Green, 80u8, 186u8, 80u8}
add_colour! {Salmon, 229u8, 127u8, 127u8}
add_colour! {Orange, 229u8, 161u8, 91u8}
add_colour! {Teal, 0u8, 178u8, 178u8}
add_colour! {Purple, 191u8, 71u8, 191u8}

/* --------------------------------- CLustal -------------------------------- */

impl_palette! {
    Clustal
    (
        A SkyBlue::RGB,
        C Salmon::RGB,
        D Purple::RGB,
        E Purple::RGB,
        F SkyBlue::RGB,
        G Orange::RGB,
        H Teal::RGB,
        I SkyBlue::RGB,
        K Red::RGB,
        L SkyBlue::RGB,
        M SkyBlue::RGB,
        N Green::RGB,
        P Yellow::RGB,
        Q Green::RGB,
        R Red::RGB,
        S Green::RGB,
        T Green::RGB,
        V SkyBlue::RGB,
        W SkyBlue::RGB,
        Y Teal::RGB,
        GAP White::RGB
    )
}

use crate::Rgb;

pub trait Color {
    const RGB: &'static Rgb;
}

/// Blue
pub struct Blue;

impl Color for Blue {
    const RGB: &'static Rgb = &Rgb {
        r: 25u8,
        g: 127u8,
        b: 229u8,
    };
}

// pub struct Red;

// impl Color for Red {
//     const R: &'static str = "229";
//     const G: &'static str = "51";
//     const B: &'static str = "25";
// }

// pub struct Purple;

// impl Color for Purple {
//     const R: &'static str = "140";
//     const G: &'static str = "96";
//     const B: &'static str = "194";
// }

// pub struct Green;

// impl Color for Green {
//     const R: &'static str = "63";
//     const G: &'static str = "190";
//     const B: &'static str = "79";
// }

// pub struct Orange; // cysteine

// impl Color for Orange {
//     const R: &'static str = "229";
//     const G: &'static str = "127";
//     const B: &'static str = "127";
// }

// pub struct Yellow; // proline

// impl Color for Yellow {
//     const R: &'static str = "204";
//     const G: &'static str = "204";
//     const B: &'static str = "0";
// }

// pub struct Teal;

// impl Color for Teal {
//     const R: &'static str = "0";
//     const G: &'static str = "178";
//     const B: &'static str = "178";
// }

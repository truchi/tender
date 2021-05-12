//! Colors ([`Rgb`], [`Rgba`], [`PreRgba`]).

mod ground;
mod pre_rgba;
mod rgb;
mod rgba;
mod with_alpha;

pub use ground::*;
pub use pre_rgba::*;
pub use rgb::*;
pub use rgba::*;
pub use with_alpha::*;

use super::*;
use std::convert::{TryFrom, TryInto};

macro_rules! web_colors {
    ($($Color:ident $R:literal $G:literal $B:literal)*) => {
        impl    Rgb  { $(pub const $Color:    Rgb  =    Rgb ($R, $G, $B);)* }
        impl    Rgba { $(pub const $Color:    Rgba =    Rgba($R, $G, $B, u8::MAX);)* }
        impl PreRgba { $(pub const $Color: PreRgba = PreRgba($R, $G, $B, u8::MAX);)* }
    };
}

web_colors!(
    // PINK COLORS
    MEDIUM_VIOLET_RED      199  21 133
    DEEP_PINK              255  20 147
    PALE_VIOLET_RED        219 112 147
    HOT_PINK               255 105 180
    LIGHT_PINK             255 182 193
    PINK                   255 192 203

    // RED COLORS
    DARK_RED               139   0   0
    RED                    255   0   0
    FIREBRICK              178  34  34
    CRIMSON                220  20  60
    INDIAN_RED             205  92  92
    LIGHT_CORAL            240 128 128
    SALMON                 250 128 114
    DARK_SALMON            233 150 122
    LIGHT_SALMON           255 160 122

    // ORANGE COLORS
    ORANGE_RED             255  69   0
    TOMATO                 255  99  71
    DARK_ORANGE            255 140   0
    CORAL                  255 127  80
    ORANGE                 255 165   0

    // YELLOW COLORS
    DARK_KHAKI             189 183 107
    GOLD                   255 215   0
    KHAKI                  240 230 140
    PEACH_PUFF             255 218 185
    YELLOW                 255 255   0
    PALE_GOLDENROD         238 232 170
    MOCCASIN               255 228 181
    PAPAYA_WHIP            255 239 213
    LIGHT_GOLDENROD_YELLOW 250 250 210
    LEMON_CHIFFON          255 250 205
    LIGHT_YELLOW           255 255 224

    // BROWN COLORS
    MAROON                 128   0   0
    BROWN                  165  42  42
    SADDLE_BROWN           139  69  19
    SIENNA                 160  82  45
    CHOCOLATE              210 105  30
    DARK_GOLDENROD         184 134  11
    PERU                   205 133  63
    ROSY_BROWN             188 143 143
    GOLDENROD              218 165  32
    SANDY_BROWN            244 164  96
    TAN                    210 180 140
    BURLYWOOD              222 184 135
    WHEAT                  245 222 179
    NAVAJO_WHITE           255 222 173
    BISQUE                 255 228 196
    BLANCHED_ALMOND        255 235 205
    CORNSILK               255 248 220

    // PURPLE, VIOLET, AND MAGENTA COLORS
    INDIGO                  75   0 130
    PURPLE                 128   0 128
    DARK_MAGENTA           139   0 139
    DARK_VIOLET            148   0 211
    DARK_SLATE_BLUE         72  61 139
    BLUE_VIOLET            138  43 226
    DARK_ORCHID            153  50 204
    FUCHSIA                255   0 255
    MAGENTA                255   0 255
    SLATE_BLUE             106  90 205
    MEDIUM_SLATE_BLUE      123 104 238
    MEDIUM_ORCHID          186  85 211
    MEDIUM_PURPLE          147 112 219
    ORCHID                 218 112 214
    VIOLET                 238 130 238
    PLUM                   221 160 221
    THISTLE                216 191 216
    LAVENDER               230 230 250

    // WHITE COLORS
    MISTY_ROSE             255 228 225
    ANTIQUE_WHITE          250 235 215
    LINEN                  250 240 230
    BEIGE                  245 245 220
    WHITE_SMOKE            245 245 245
    LAVENDER_BLUSH         255 240 245
    OLD_LACE               253 245 230
    ALICE_BLUE             240 248 255
    SEASHELL               255 245 238
    GHOST_WHITE            248 248 255
    HONEYDEW               240 255 240
    FLORAL_WHITE           255 250 240
    AZURE                  240 255 255
    MINT_CREAM             245 255 250
    SNOW                   255 250 250
    IVORY                  255 255 240
    WHITE                  255 255 255

    // GRAY AND BLACK COLORS
    BLACK                    0   0   0
    DARK_SLATE_GRAY         47  79  79
    DIM_GRAY               105 105 105
    SLATE_GRAY             112 128 144
    GRAY                   128 128 128
    LIGHT_SLATE_GRAY       119 136 153
    DARK_GRAY              169 169 169
    SILVER                 192 192 192
    LIGHT_GRAY             211 211 211
    GAINSBORO              220 220 220

    // GREEN COLORS
    DARK_GREEN               0 100   0
    GREEN                    0 128   0
    DARK_OLIVE_GREEN        85 107  47
    FOREST_GREEN            34 139  34
    SEA_GREEN               46 139  87
    OLIVE                  128 128   0
    OLIVE_DRAB             107 142  35
    MEDIUM_SEA_GREEN        60 179 113
    LIME_GREEN              50 205  50
    LIME                     0 255   0
    SPRING_GREEN             0 255 127
    MEDIUM_SPRING_GREEN      0 250 154
    DARK_SEA_GREEN         143 188 143
    MEDIUM_AQUAMARINE      102 205 170
    YELLOW_GREEN           154 205  50
    LAWN_GREEN             124 252   0
    CHARTREUSE             127 255   0
    LIGHT_GREEN            144 238 144
    GREEN_YELLOW           173 255  47
    PALE_GREEN             152 251 152

    // CYAN COLORS
    TEAL                     0 128 128
    DARK_CYAN                0 139 139
    LIGHT_SEA_GREEN         32 178 170
    CADET_BLUE              95 158 160
    DARK_TURQUOISE           0 206 209
    MEDIUM_TURQUOISE        72 209 204
    TURQUOISE               64 224 208
    AQUA                     0 255 255
    CYAN                     0 255 255
    AQUAMARINE             127 255 212
    PALE_TURQUOISE         175 238 238
    LIGHT_CYAN             224 255 255

    // BLUE COLORS
    NAVY                     0   0 128
    DARK_BLUE                0   0 139
    MEDIUM_BLUE              0   0 205
    BLUE                     0   0 255
    MIDNIGHT_BLUE           25  25 112
    ROYAL_BLUE              65 105 225
    STEEL_BLUE              70 130 180
    DODGER_BLUE             30 144 255
    DEEP_SKY_BLUE            0 191 255
    CORNFLOWER_BLUE        100 149 237
    SKY_BLUE               135 206 235
    LIGHT_SKY_BLUE         135 206 250
    LIGHT_STEEL_BLUE       176 196 222
    LIGHT_BLUE             173 216 230
    POWDER_BLUE            176 224 230
);

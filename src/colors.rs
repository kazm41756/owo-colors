use crate::{BgColorDisplay, FgColorDisplay};
use core::fmt;

macro_rules! colors {
    ($(
        $color:ident $fg:literal $bg:literal
    ),* $(,)?) => {
        use crate::Color;

        $(
            /// A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.
            pub struct $color;

            impl Color for $color {
                const ANSI_FG: &'static str = concat!("\x1b[", $fg, "m");
                const ANSI_BG: &'static str = concat!("\x1b[", $bg, "m");
            }
        )*
    };
}

colors! {
    Black   "30" "40",
    Red     "31" "41",
    Green   "32" "42",
    Yellow  "33" "43",
    Blue    "34" "44",
    Magenta "35" "45",
    Cyan    "36" "46",
    White   "37" "47",

    BrightBlack   "90" "100",
    BrightRed     "91" "101",
    BrightGreen   "92" "102",
    BrightYellow  "93" "103",
    BrightBlue    "94" "104",
    BrightMagenta "95" "105",
    BrightCyan    "96" "106",
    BrightWhite   "97" "107",
}

macro_rules! impl_fmt_for {
    ($(($ty:ident, $trait:path, $const:ident)),* $(,)?) => {
        $(
            impl<'a, Color: crate::Color, T: $trait> $trait for $ty<'a, Color, T> {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str(Color::$const)?;
                    <T as $trait>::fmt(&self.0, f)?;
                    f.write_str("\x1b[0m")
                    //write!(f, "{}{}\x1b[0m", Color::$const, self.0)
                }
            }
        )*
    };
}

impl_fmt_for! {
    // Foreground
    (FgColorDisplay, fmt::Display,  ANSI_FG),
    (FgColorDisplay, fmt::Debug,    ANSI_FG),
    (FgColorDisplay, fmt::UpperHex, ANSI_FG),
    (FgColorDisplay, fmt::LowerHex, ANSI_FG),
    (FgColorDisplay, fmt::Binary,   ANSI_FG),
    (FgColorDisplay, fmt::UpperExp, ANSI_FG),
    (FgColorDisplay, fmt::LowerExp, ANSI_FG),
    (FgColorDisplay, fmt::Octal,    ANSI_FG),
    (FgColorDisplay, fmt::Pointer,  ANSI_FG),
    // Background
    (BgColorDisplay, fmt::Display,  ANSI_BG),
    (BgColorDisplay, fmt::Debug,    ANSI_BG),
    (BgColorDisplay, fmt::UpperHex, ANSI_BG),
    (BgColorDisplay, fmt::LowerHex, ANSI_BG),
    (BgColorDisplay, fmt::Binary,   ANSI_BG),
    (BgColorDisplay, fmt::UpperExp, ANSI_BG),
    (BgColorDisplay, fmt::LowerExp, ANSI_BG),
    (BgColorDisplay, fmt::Octal,    ANSI_BG),
    (BgColorDisplay, fmt::Pointer,  ANSI_BG),
}

macro_rules! xterm_colors {
    ($(
        $xterm_num:literal $name:ident ($r:literal, $g:literal, $b:literal)
    )*) => {
        $(
            pub struct $name;

            impl crate::Color for $name {
                const ANSI_FG: &'static str = concat!("\x1b[38;5;", stringify!($xterm_num), "m");
                const ANSI_BG: &'static str = concat!("\x1b[48;5;", stringify!($xterm_num), "m");
            }
        )*

    };
}

pub mod xterm {
    xterm_colors! {
          0 Black            (0,0,0)
          1 Maroon           (128,0,0)
          2 JapaneseLaurel   (0,128,0)
          3 Olive            (128,128,0)
          4 NavyBlue         (0,0,128)
          5 FreshEggplant    (128,0,128)
          6 Teal             (0,128,128)
          7 Silver           (192,192,192)
          8 Gray             (128,128,128)
          9 Red              (255,0,0)
         10 Green            (0,255,0)
         11 Yellow           (255,255,0)
         12 Blue             (0,0,255)
         13 Magenta          (255,0,255)
         14 Cyan             (0,255,255)
         15 White            (255,255,255)
         16 Black1           (0,0,0)
         17 Stratos          (0,0,95)
         18 NavyBlue1        (0,0,135)
         19 DarkBlue         (0,0,175)
         20 DarkBlue1        (0,0,215)
         21 Blue1            (0,0,255)
         22 Camarone         (0,95,0)
         23 BlueStone        (0,95,95)
         24 Orient           (0,95,135)
         25 Endeavour        (0,95,175)
         26 ScienceBlue      (0,95,215)
         27 BlueRibbon       (0,95,255)
         28 JapaneseLaurel1  (0,135,0)
         29 DeepSea          (0,135,95)
         30 Teal1            (0,135,135)
         31 DeepCerulean     (0,135,175)
         32 Lochmara         (0,135,215)
         33 AzureRadiance    (0,135,255)
         34 JapaneseLaurel2  (0,175,0)
         35 Jade             (0,175,95)
         36 PersianGreen     (0,175,135)
         37 BondiBlue        (0,175,175)
         38 Cerulean         (0,175,215)
         39 AzureRadiance1   (0,175,255)
         40 Green1           (0,215,0)
         41 Malachite        (0,215,95)
         42 CaribbeanGreen   (0,215,135)
         43 CaribbeanGreen1  (0,215,175)
         44 RobinEggBlue     (0,215,215)
         45 Aqua             (0,215,255)
         46 Green2           (0,255,0)
         47 SpringGreen      (0,255,95)
         48 SpringGreen1     (0,255,135)
         49 SpringGreen2     (0,255,175)
         50 BrightTurquoise  (0,255,215)
         51 Cyan2            (0,255,255)
         52 Rosewood         (95,0,0)
         53 Pompadour        (95,0,95)
         54 PigmentIndigo    (95,0,135)
         55 Purple           (95,0,175)
         56 ElectricViolet   (95,0,215)
         57 ElectricViolet1  (95,0,255)
         58 VerdunGreen      (95,95,0)
         59 Scorpion         (95,95,95)
         60 Comet            (95,95,135)
         61 Scampi           (95,95,175)
         62 Indigo           (95,95,215)
         63 CornflowerBlue   (95,95,255)
         64 Limeade          (95,135,0)
         65 GladeGreen       (95,135,95)
         66 Juniper          (95,135,135)
         67 HippieBlue       (95,135,175)
         68 HavelockBlue     (95,135,215)
         69 CornflowerBlue1  (95,135,255)
         70 Limeade1         (95,175,0)
         71 Fern             (95,175,95)
         72 SilverTree       (95,175,135)
         73 Tradewind        (95,175,175)
         74 Shakespeare      (95,175,215)
         75 Malibu           (95,175,255)
         76 BrightGreen      (95,215,0)
         77 PastelGreen      (95,215,95)
         78 PastelGreen1     (95,215,135)
         79 Downy            (95,215,175)
         80 Viking           (95,215,215)
         81 Malibu1          (95,215,255)
         82 BrightGreen1     (95,255,0)
         83 ScreaminGreen    (95,255,95)
         84 ScreaminGreen1   (95,255,135)
         85 Aquamarine       (95,255,175)
         86 Aquamarine1      (95,255,215)
         87 Aquamarine2      (95,255,255)
         88 Maroon1          (135,0,0)
         89 FreshEggplant1   (135,0,95)
         90 FreshEggplant2   (135,0,135)
         91 Purple1          (135,0,175)
         92 ElectricViolet2  (135,0,215)
         93 ElectricViolet3  (135,0,255)
         94 Brown            (135,95,0)
         95 CopperRose       (135,95,95)
         96 Strikemaster     (135,95,135)
         97 Deluge           (135,95,175)
         98 MediumPurple     (135,95,215)
         99 Heliotrope       (135,95,255)
        100 Olive1           (135,135,0)
        101 ClayCreek        (135,135,95)
        102 Gray1            (135,135,135)
        103 WildBlueYonder   (135,135,175)
        104 ChetwodeBlue     (135,135,215)
        105 Malibu2          (135,135,255)
        106 Limeade2         (135,175,0)
        107 ChelseaCucumber  (135,175,95)
        108 BayLeaf          (135,175,135)
        109 GulfStream       (135,175,175)
        110 PoloBlue         (135,175,215)
        111 Malibu3          (135,175,255)
        112 Pistachio        (135,215,0)
        113 PastelGreen2     (135,215,95)
        114 Feijoa           (135,215,135)
        115 VistaBlue        (135,215,175)
        116 Bermuda          (135,215,215)
        117 Anakiwa          (135,215,255)
        118 Chartreuse       (135,255,0)
        119 ScreaminGreen2   (135,255,95)
        120 MintGreen        (135,255,135)
        121 MintGreen1       (135,255,175)
        122 Aquamarine3      (135,255,215)
        123 Anakiwa1         (135,255,255)
        124 BrightRed        (175,0,0)
        125 Flirt            (175,0,95)
        126 Flirt1           (175,0,135)
        127 Flirt2           (175,0,175)
        128 ElectricViolet4  (175,0,215)
        129 ElectricViolet5  (175,0,255)
        130 RoseofSharon     (175,95,0)
        131 Matrix           (175,95,95)
        132 Tapestry         (175,95,135)
        133 FuchsiaPink      (175,95,175)
        134 MediumPurple1    (175,95,215)
        135 Heliotrope1      (175,95,255)
        136 PirateGold       (175,135,0)
        137 Muesli           (175,135,95)
        138 Pharlap          (175,135,135)
        139 Bouquet          (175,135,175)
        140 Lavender         (175,135,215)
        141 Heliotrope2      (175,135,255)
        142 BuddhaGold       (175,175,0)
        143 OliveGreen       (175,175,95)
        144 Hillary          (175,175,135)
        145 SilverChalice    (175,175,175)
        146 Wistful          (175,175,215)
        147 Melrose          (175,175,255)
        148 RioGrande        (175,215,0)
        149 Conifer          (175,215,95)
        150 Feijoa1          (175,215,135)
        151 PixieGreen       (175,215,175)
        152 JungleMist       (175,215,215)
        153 Anakiwa2         (175,215,255)
        154 Lime             (175,255,0)
        155 GreenYellow      (175,255,95)
        156 MintGreen2       (175,255,135)
        157 MintGreen3       (175,255,175)
        158 AeroBlue         (175,255,215)
        159 FrenchPass       (175,255,255)
        160 GuardsmanRed     (215,0,0)
        161 Razzmatazz       (215,0,95)
        162 HollywoodCerise  (215,0,135)
        163 HollywoodCerise1 (215,0,175)
        164 PurplePizzazz    (215,0,215)
        165 ElectricViolet6  (215,0,255)
        166 Tenn             (215,95,0)
        167 Roman            (215,95,95)
        168 Cranberry        (215,95,135)
        169 Hopbush          (215,95,175)
        170 Orchid           (215,95,215)
        171 Heliotrope3      (215,95,255)
        172 MangoTango       (215,135,0)
        173 Copperfield      (215,135,95)
        174 MyPink           (215,135,135)
        175 CanCan           (215,135,175)
        176 LightOrchid      (215,135,215)
        177 Heliotrope4      (215,135,255)
        178 Corn             (215,175,0)
        179 Tacha            (215,175,95)
        180 Tan              (215,175,135)
        181 ClamShell        (215,175,175)
        182 Thistle          (215,175,215)
        183 Mauve            (215,175,255)
        184 Corn1            (215,215,0)
        185 Tacha1           (215,215,95)
        186 Deco             (215,215,135)
        187 GreenMist        (215,215,175)
        188 Alto             (215,215,215)
        189 Fog              (215,215,255)
        190 ChartreuseYellow (215,255,0)
        191 Canary           (215,255,95)
        192 Honeysuckle      (215,255,135)
        193 Reef             (215,255,175)
        194 SnowyMint        (215,255,215)
        195 OysterBay        (215,255,255)
        196 Red1             (255,0,0)
        197 Rose             (255,0,95)
        198 Rose1            (255,0,135)
        199 HollywoodCerise2 (255,0,175)
        200 PurplePizzazz1   (255,0,215)
        201 Fuchsia          (255,0,255)
        202 BlazeOrange      (255,95,0)
        203 Bittersweet      (255,95,95)
        204 WildWatermelon   (255,95,135)
        205 HotPink          (255,95,175)
        206 HotPink1         (255,95,215)
        207 PinkFlamingo     (255,95,255)
        208 FlushOrange      (255,135,0)
        209 Salmon           (255,135,95)
        210 VividTangerine   (255,135,135)
        211 PinkSalmon       (255,135,175)
        212 LavenderRose     (255,135,215)
        213 BlushPink        (255,135,255)
        214 YellowSea        (255,175,0)
        215 TexasRose        (255,175,95)
        216 HitPink          (255,175,135)
        217 Sundown          (255,175,175)
        218 CottonCandy      (255,175,215)
        219 LavenderRose1    (255,175,255)
        220 Gold             (255,215,0)
        221 Dandelion        (255,215,95)
        222 Grandis          (255,215,135)
        223 Caramel          (255,215,175)
        224 Cosmos           (255,215,215)
        225 PinkLace         (255,215,255)
        226 Yellow1          (255,255,0)
        227 LaserLemon       (255,255,95)
        228 Dolly            (255,255,135)
        229 Portafino        (255,255,175)
        230 Cumulus          (255,255,215)
        231 White1           (255,255,255)
        232 CodGray          (8,8,8)
        233 CodGray1         (18,18,18)
        234 CodGray2         (28,28,28)
        235 MineShaft        (38,38,38)
        236 MineShaft1       (48,48,48)
        237 MineShaft2       (58,58,58)
        238 Tundora          (68,68,68)
        239 Tundora1         (78,78,78)
        240 Scorpion1        (88,88,88)
        241 DoveGray         (98,98,98)
        242 DoveGray1        (108,108,108)
        243 Boulder          (118,118,118)
        244 Gray2            (128,128,128)
        245 Gray3            (138,138,138)
        246 DustyGray        (148,148,148)
        247 SilverChalice1   (158,158,158)
        248 SilverChalice2   (168,168,168)
        249 SilverChalice3   (178,178,178)
        250 Silver1          (188,188,188)
        251 Silver2          (198,198,198)
        252 Alto1            (208,208,208)
        253 Alto2            (218,218,218)
        254 Mercury          (228,228,228)
        255 Gallery          (238,238,238)
    }
}

#[cfg(feature = "custom")]
const U8_TO_STR: [[u8; 3]; 256] = [[48, 48, 48], [48, 48, 49], [48, 48, 50], [48, 48, 51], [48, 48, 52], [48, 48, 53], [48, 48, 54], [48, 48, 55], [48, 48, 56], [48, 48, 57], [48, 49, 48], [48, 49, 49], [48, 49, 50], [48, 49, 51], [48, 49, 52], [48, 49, 53], [48, 49, 54], [48, 49, 55], [48, 49, 56], [48, 49, 57], [48, 50, 48], [48, 50, 49], [48, 50, 50], [48, 50, 51], [48, 50, 52], [48, 50, 53], [48, 50, 54], [48, 50, 55], [48, 50, 56], [48, 50, 57], [48, 51, 48], [48, 51, 49], [48, 51, 50], [48, 51, 51], [48, 51, 52], [48, 51, 53], [48, 51, 54], [48, 51, 55], [48, 51, 56], [48, 51, 57], [48, 52, 48], [48, 52, 49], [48, 52, 50], [48, 52, 51], [48, 52, 52], [48, 52, 53], [48, 52, 54], [48, 52, 55], [48, 52, 56], [48, 52, 57], [48, 53, 48], [48, 53, 49], [48, 53, 50], [48, 53, 51], [48, 53, 52], [48, 53, 53], [48, 53, 54], [48, 53, 55], [48, 53, 56], [48, 53, 57], [48, 54, 48], [48, 54, 49], [48, 54, 50], [48, 54, 51], [48, 54, 52], [48, 54, 53], [48, 54, 54], [48, 54, 55], [48, 54, 56], [48, 54, 57], [48, 55, 48], [48, 55, 49], [48, 55, 50], [48, 55, 51], [48, 55, 52], [48, 55, 53], [48, 55, 54], [48, 55, 55], [48, 55, 56], [48, 55, 57], [48, 56, 48], [48, 56, 49], [48, 56, 50], [48, 56, 51], [48, 56, 52], [48, 56, 53], [48, 56, 54], [48, 56, 55], [48, 56, 56], [48, 56, 57], [48, 57, 48], [48, 57, 49], [48, 57, 50], [48, 57, 51], [48, 57, 52], [48, 57, 53], [48, 57, 54], [48, 57, 55], [48, 57, 56], [48, 57, 57], [49, 48, 48], [49, 48, 49], [49, 48, 50], [49, 48, 51], [49, 48, 52], [49, 48, 53], [49, 48, 54], [49, 48, 55], [49, 48, 56], [49, 48, 57], [49, 49, 48], [49, 49, 49], [49, 49, 50], [49, 49, 51], [49, 49, 52], [49, 49, 53], [49, 49, 54], [49, 49, 55], [49, 49, 56], [49, 49, 57], [49, 50, 48], [49, 50, 49], [49, 50, 50], [49, 50, 51], [49, 50, 52], [49, 50, 53], [49, 50, 54], [49, 50, 55], [49, 50, 56], [49, 50, 57], [49, 51, 48], [49, 51, 49], [49, 51, 50], [49, 51, 51], [49, 51, 52], [49, 51, 53], [49, 51, 54], [49, 51, 55], [49, 51, 56], [49, 51, 57], [49, 52, 48], [49, 52, 49], [49, 52, 50], [49, 52, 51], [49, 52, 52], [49, 52, 53], [49, 52, 54], [49, 52, 55], [49, 52, 56], [49, 52, 57], [49, 53, 48], [49, 53, 49], [49, 53, 50], [49, 53, 51], [49, 53, 52], [49, 53, 53], [49, 53, 54], [49, 53, 55], [49, 53, 56], [49, 53, 57], [49, 54, 48], [49, 54, 49], [49, 54, 50], [49, 54, 51], [49, 54, 52], [49, 54, 53], [49, 54, 54], [49, 54, 55], [49, 54, 56], [49, 54, 57], [49, 55, 48], [49, 55, 49], [49, 55, 50], [49, 55, 51], [49, 55, 52], [49, 55, 53], [49, 55, 54], [49, 55, 55], [49, 55, 56], [49, 55, 57], [49, 56, 48], [49, 56, 49], [49, 56, 50], [49, 56, 51], [49, 56, 52], [49, 56, 53], [49, 56, 54], [49, 56, 55], [49, 56, 56], [49, 56, 57], [49, 57, 48], [49, 57, 49], [49, 57, 50], [49, 57, 51], [49, 57, 52], [49, 57, 53], [49, 57, 54], [49, 57, 55], [49, 57, 56], [49, 57, 57], [50, 48, 48], [50, 48, 49], [50, 48, 50], [50, 48, 51], [50, 48, 52], [50, 48, 53], [50, 48, 54], [50, 48, 55], [50, 48, 56], [50, 48, 57], [50, 49, 48], [50, 49, 49], [50, 49, 50], [50, 49, 51], [50, 49, 52], [50, 49, 53], [50, 49, 54], [50, 49, 55], [50, 49, 56], [50, 49, 57], [50, 50, 48], [50, 50, 49], [50, 50, 50], [50, 50, 51], [50, 50, 52], [50, 50, 53], [50, 50, 54], [50, 50, 55], [50, 50, 56], [50, 50, 57], [50, 51, 48], [50, 51, 49], [50, 51, 50], [50, 51, 51], [50, 51, 52], [50, 51, 53], [50, 51, 54], [50, 51, 55], [50, 51, 56], [50, 51, 57], [50, 52, 48], [50, 52, 49], [50, 52, 50], [50, 52, 51], [50, 52, 52], [50, 52, 53], [50, 52, 54], [50, 52, 55], [50, 52, 56], [50, 52, 57], [50, 53, 48], [50, 53, 49], [50, 53, 50], [50, 53, 51], [50, 53, 52], [50, 53, 53]];

#[cfg(feature = "custom")]
const fn rgb_to_ansi(r: u8, g: u8, b: u8, is_fg: bool) -> [u8; 19] {
    let mut buf = if is_fg { *b"\x1b[38;2;rrr;ggg;bbbm" } else { *b"\x1b[48;2;rrr;ggg;bbbm" };
    
    let r = U8_TO_STR[r as usize];
    let g = U8_TO_STR[g as usize];
    let b = U8_TO_STR[b as usize];

    // r 7 
    buf[7] = r[0];
    buf[8] = r[1];
    buf[9] = r[2];

    // g 11
    buf[11] = g[0];
    buf[12] = g[1];
    buf[13] = g[2];

    // b 15
    buf[15] = b[0];
    buf[16] = b[1];
    buf[17] = b[2];
    
    buf
}

#[cfg(feature = "custom")]
pub struct CustomColor<const R: u8, const G: u8, const B: u8>;

#[cfg(feature = "custom")]
impl<const R: u8, const G: u8, const B: u8> Color for CustomColor<R, G, B> {
    const ANSI_FG: &'static str = unsafe { core::mem::transmute(&rgb_to_ansi(R, G, B, true) as &[u8]) };
    const ANSI_BG: &'static str = unsafe { core::mem::transmute(&rgb_to_ansi(R, G, B, false) as &[u8]) };
}

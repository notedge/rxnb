//! impl following files
//! https://github.com/KaTeX/KaTeX/blob/master/src/Options.js
//! https://github.com/KaTeX/KaTeX/blob/master/src/Style.js
//! https://github.com/KaTeX/KaTeX/blob/master/src/fontMetrics.js

/// Each element contains [textsize, scriptsize, scriptscriptsize].
/// The size mappings are taken from TeX with \normalsize=10pt.
pub const SIZE_STYLE_MAP: &[[i32; 3]; 11] = &[
    [1, 1, 1],   // size1: [5, 5, 5]              \tiny
    [2, 1, 1],   // size2: [6, 5, 5]
    [3, 1, 1],   // size3: [7, 5, 5]              \scriptsize
    [4, 2, 1],   // size4: [8, 6, 5]              \footnotesize
    [5, 2, 1],   // size5: [9, 6, 5]              \small
    [6, 3, 1],   // size6: [10, 7, 5]             \normalsize
    [7, 4, 2],   // size7: [12, 8, 6]             \large
    [8, 6, 3],   // size8: [14.4, 10, 7]          \Large
    [9, 7, 6],   // size9: [17.28, 12, 10]        \LARGE
    [10, 8, 7],  // size10: [20.74, 14.4, 12]     \huge
    [11, 10, 9], // size11: [24.88, 20.74, 17.28] \HUGE
];

/// fontMetrics.js:getGlobalMetrics also uses size indexes, so if
/// you change size indexes, change that function.
#[rustfmt::skip]
pub const SIZE_MULTIPLIERS: &[f64; 11] = &[
    0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.2, 1.44, 1.728, 2.074, 2.488,
];

fn test() {
    let a = [
        // fontMetrics.js:getGlobalMetrics also uses size indexes, so if
        // you change size indexes, change that function.
        0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.2, 1.44, 1.728, 2.074, 2.488,
    ];
}

/// type FontWeight = "textbf" | "textmd" | "";
pub enum FontWeight {
    TextBf,
    TextMd,
    NoChange,
}

/// type FontShape = "textit" | "textup" | "";
pub enum FontShape {
    TextIt,
    TextUp,
    NoChange,
}

pub enum Styles {
    D = 0,
    Dc = 1,
    T = 2,
    Tc = 3,
    S = 4,
    Sc = 5,
    SS = 6,
    SSc = 7,
}
// pub struct OptionsData {
// style: StyleInterface;
// color ?: string | void;
// size ?: number;
// textSize ?: number;
// phantom ?: boolean;
// font ?: string;
// fontFamily ?: string;
// fontWeight ?: FontWeight;
// fontShape ?: FontShape;
// sizeMultiplier ?: number;
// maxSize: number;
// minRuleThickness: number;
// };

//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "Effect Smart";
pub const APP_SLUG: &str = "effectcraft";
pub const DISCORD: &str = "https://instagram.com/disneymkt";
pub const WEBSITE: &str = "https://instagram.com/disneymkt";
pub const APP_PAGE: &str = "https://github.com/DisneyMjr/EFFECT-SMART/releases";
pub const GITHUB: &str = "https://github.com/DisneyMjr/EFFECT-SMART";
pub const ISSUES: &str = "https://github.com/storytold/effectcraft/issues";

/// Sibling apps of the ArtCraft family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("PhotoCraft", "photocraft"),
    ("VectorCraft", "vectorcraft"),
    ("FilmCraft", "filmcraft"),
    ("LightCraft", "lightcraft"),
    ("PdfCraft", "pdfcraft"),
    ("DesignCraft", "designcraft"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://getartcraft.com/apps/{slug}")
}

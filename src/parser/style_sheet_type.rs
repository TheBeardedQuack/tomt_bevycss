#[derive(Debug, Copy, Clone)]
#[derive(Default)]
pub enum StyleSheetType
{
    #[default]
    Css,

    #[cfg(feature = "sass")]
    Sass,
}

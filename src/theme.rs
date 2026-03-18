//! Syntax highlighting theme utilities.

use egui::Style;

/// Return the default [`CodeTheme`](egui_extras::syntax_highlighting::CodeTheme) for the given egui style.
#[cfg(feature = "syntax_highlighting")]
pub fn default_code_theme(style: &Style) -> egui_extras::syntax_highlighting::CodeTheme {
  egui_extras::syntax_highlighting::CodeTheme::from_style(style)
}

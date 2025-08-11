use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

#[cfg(feature = "accent-color")]
use bevy_color::Color;
use bevy_color::Srgba;

/// A collection of system preferences.
#[derive(Debug, Default, Clone, Copy, PartialEq, Resource, Reflect)]
#[reflect(Resource)]
#[non_exhaustive]
pub struct Preferences {
    /// The user's preference for either light or dark mode.
    #[cfg(feature = "color-scheme")]
    pub color_scheme: ColorScheme,
    /// The user's preferred contrast level.
    #[cfg(feature = "contrast")]
    pub contrast: Contrast,
    /// The user's reduced motion preference.
    #[cfg(feature = "reduced-motion")]
    pub reduced_motion: ReducedMotion,
    /// The user's reduced transparency preference.
    #[cfg(feature = "reduced-transparency")]
    pub reduced_transparency: ReducedTransparency,
    /// The user's current system wide accent color preference.
    #[cfg(feature = "accent-color")]
    pub accent_color: AccentColor,
    /// The maximum amount of time that may occur between the first and second click
    /// event for it to count as double click.
    #[cfg(feature = "double-click-interval")]
    pub double_click_interval: DoubleClickInterval,
}

impl From<mundy::Preferences> for Preferences {
    fn from(value: mundy::Preferences) -> Self {
        Preferences {
            color_scheme: value.color_scheme.into(),
            contrast: value.contrast.into(),
            reduced_motion: value.reduced_motion.into(),
            reduced_transparency: value.reduced_transparency.into(),
            accent_color: value.accent_color.into(),
            double_click_interval: value.double_click_interval.into(),
        }
    }
}

/// The user's preference for either light or dark mode.
///
/// See also <https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-color-scheme>.
///
/// ## Sources
/// * Linux: `org.freedesktop.appearance color-scheme` from the [XDG Settings portal][xdg].
/// * Windows: [`UISettings.GetColorValue(UIColorType::Foreground)`](https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/ui/apply-windows-themes#know-when-dark-mode-is-enabled)
/// * macOS: `NSApplication.effectiveAppearance`
/// * Web: `@media (prefers-color-scheme: ...)`
///
/// [xdg]: https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.Settings.html
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg(feature = "color-scheme")]
pub enum ColorScheme {
    /// Indicates that the user has not expressed an active preference,
    /// that the current platform doesn't support a color scheme preference
    /// or that an error occurred while trying to retrieve the preference.
    #[default]
    NoPreference,
    /// Indicates that the user prefers an interface with a light appearance.
    Light,
    /// Indicates that the user prefers an interface with a dark appearance.
    Dark,
}

impl From<mundy::ColorScheme> for ColorScheme {
    fn from(value: mundy::ColorScheme) -> Self {
        match value {
            mundy::ColorScheme::NoPreference => ColorScheme::NoPreference,
            mundy::ColorScheme::Light => ColorScheme::Light,
            mundy::ColorScheme::Dark => ColorScheme::Dark,
        }
    }
}

/// The user's preferred contrast level.
///
/// See also <https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-contrast>
///
/// ## Sources
/// * Linux: `org.freedesktop.appearance contrast` from the [XDG Settings portal][xdg].
/// * Windows: [`AccessibilitySettings.HighContrast`](https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.accessibilitysettings.highcontrast)
/// * macOS: [`accessibilityDisplayShouldIncreaseContrast`](https://developer.apple.com/documentation/appkit/nsworkspace/1526290-accessibilitydisplayshouldincrea)
/// * Web: `@media (prefers-contrast: ...)`
///
/// [xdg]: https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.Settings.html
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg(feature = "contrast")]
pub enum Contrast {
    /// Indicates that the user has not expressed an active preference,
    /// that the current platform doesn't support a contrast preference
    /// or that an error occurred while trying to retrieve the preference.
    #[default]
    NoPreference,
    /// Indicates that the user prefers an interface with a higher level of contrast.
    More,
    /// Indicates that the user prefers an interface with a lower level of contrast.
    Less,
    /// Indicates that the user has configured a specific set of colors (forced color mode)
    /// and the contrast from these colors neither matches [`Contrast::More`] or [`Contrast::Less`].
    Custom,
}

impl From<mundy::Contrast> for Contrast {
    fn from(value: mundy::Contrast) -> Self {
        match value {
            mundy::Contrast::NoPreference => Contrast::NoPreference,
            mundy::Contrast::More => Contrast::More,
            mundy::Contrast::Less => Contrast::Less,
            mundy::Contrast::Custom => Contrast::Custom,
        }
    }
}

/// The user prefers to have a minimal amount
/// of motion. Especially motion that simulates the third dimension.
///
/// Such motion can cause discomfort to people with [vestibular disorders](https://www.a11yproject.com/posts/understanding-vestibular-disorders/).
///
/// See also <https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion>.
///
/// ## Sources
/// * Linux (GNOME-only): `org.gnome.desktop.interface enable-animations` from the [XDG Settings portal][xdg].
/// * Windows: [`UISettings.AnimationsEnabled`](https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.animationsenabled)
/// * macOS: [`accessibilityDisplayShouldReduceMotion`](https://developer.apple.com/documentation/appkit/nsworkspace/1644069-accessibilitydisplayshouldreduce)
/// * Web: `@media (prefers-reduced-motion: ...)`
///
/// [xdg]: https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.Settings.html
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg(feature = "reduced-motion")]
pub enum ReducedMotion {
    /// Indicates that the user has not expressed an active preference,
    /// that the current platform doesn't support a reduced motion preference
    /// or that an error occurred while trying to retrieve the preference.
    #[default]
    NoPreference,
    /// Indicates that the user prefers a minimal amount of motion.
    Reduce,
}

impl From<mundy::ReducedMotion> for ReducedMotion {
    fn from(value: mundy::ReducedMotion) -> Self {
        match value {
            mundy::ReducedMotion::NoPreference => ReducedMotion::NoPreference,
            mundy::ReducedMotion::Reduce => ReducedMotion::Reduce,
        }
    }
}

/// Indicates that applications should not use transparent or semitransparent backgrounds.
///
/// See also <https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-transparency>.
///
/// ## Sources
/// * Windows: [`UISettings.AdvancedEffectsEnabled`](https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.advancedeffectsenabled)
/// * macOS: [`accessibilityDisplayShouldReduceTransparency`](https://developer.apple.com/documentation/appkit/nsworkspace/1533006-accessibilitydisplayshouldreduce)
/// * Web: `@media (prefers-reduced-transparency: ...)`
///
/// [xdg]: https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.Settings.html
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[cfg(feature = "reduced-transparency")]
pub enum ReducedTransparency {
    /// Indicates that the user has not expressed an active preference,
    /// that the current platform doesn't support a reduced transparency preference
    /// or that an error occurred while trying to retrieve the preference.
    #[default]
    NoPreference,
    /// Indicates that the user prefers an interface with no transparent
    /// or semitransparent backgrounds.
    Reduce,
}

impl From<mundy::ReducedTransparency> for ReducedTransparency {
    fn from(value: mundy::ReducedTransparency) -> Self {
        match value {
            mundy::ReducedTransparency::NoPreference => ReducedTransparency::NoPreference,
            mundy::ReducedTransparency::Reduce => ReducedTransparency::Reduce,
        }
    }
}

/// The user's current system wide accent color preference.
///
/// ## Sources
/// * Linux: `org.freedesktop.appearance accent-color` from the [XDG Settings portal][xdg].
/// * Windows: [`UISettings.GetColorValue(UIColorType::Accent)`](https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings)
/// * macOS: [`NSColor.controlAccentColor`](https://developer.apple.com/documentation/appkit/nscolor/3000782-controlaccentcolor)
/// * Web: The [`AccentColor`](https://developer.mozilla.org/en-US/docs/Web/CSS/system-color#accentcolor) system color.
///
/// [xdg]: https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.Settings.html
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg(feature = "accent-color")]
pub struct AccentColor(pub Option<Color>);

impl From<mundy::AccentColor> for AccentColor {
    fn from(value: mundy::AccentColor) -> Self {
        use bevy_color::ColorToComponents as _;
        AccentColor(
            value
                .0
                .map(|c| Srgba::from_f32_array(c.to_f64_array().map(|c| c as f32)).into()),
        )
    }
}

/// The maximum amount of time that may occur between the first and second click
/// event for it to count as double click.
///
/// A typical value for this setting is ~500 ms.
///
/// ## Sources
/// * Linux (GNOME-only): `org.gnome.desktop.peripherals.mouse double-click` from the [XDG Settings portal][xdg].
/// * Windows: [`GetDoubleClickTime`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdoubleclicktime)
/// * macOS: [`NSEvent.doubleClickInterval`](https://developer.apple.com/documentation/appkit/nsevent/1528384-doubleclickinterval)
/// * Web: Unsupported
///
/// [xdg]: https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.Settings.html
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[cfg(feature = "double-click-interval")]
pub struct DoubleClickInterval(pub Option<std::time::Duration>);

impl From<mundy::DoubleClickInterval> for DoubleClickInterval {
    fn from(value: mundy::DoubleClickInterval) -> Self {
        Self(value.0)
    }
}

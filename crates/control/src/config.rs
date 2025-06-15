use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use directories::ProjectDirs;
use ui::theme::{ThemeColor, ThemeMode};
use ui::Colorize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub working_dir: Option<PathBuf>,
    pub theme_mode: ThemeMode,
    pub ui_settings: UiSettings,
}
/// A trait for things which can access the global AppConfig.
///
/// Implemented for gpui::App so `cx.config()` yields `&AppConfig`
pub trait ActiveConfig {
    fn config(&self) -> &AppConfig;
}

impl ActiveConfig for gpui::App {
    #[inline(always)]
    fn config(&self) -> &AppConfig {
        AppConfig::global(self)
    }
}

impl gpui::Global for AppConfig {}

impl AppConfig {
    /// Returns the global AppConfig reference
    #[inline(always)]
    pub fn global(cx: &gpui::App) -> &AppConfig {
        cx.global::<AppConfig>()
    }
    /// Returns the global AppConfig mutable reference
    #[inline(always)]
    pub fn global_mut(cx: &mut gpui::App) -> &mut AppConfig {
        cx.global_mut::<AppConfig>()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiSettings {
    pub rounded_size: f32,
    pub light: ThemeColor,
    pub dark: ThemeColor,
}

impl Default for UiSettings {
    fn default() -> Self {
        use ui::theme::hsl;
        use ui::colors;

        Self {
            rounded_size: 8.0,
            light: ThemeColor {
                accent: hsl(240.0, 4.8, 93.9),
                accent_foreground: hsl(240.0, 5.9, 10.0),
                accordion: hsl(0.0, 0.0, 100.0),
                accordion_active: hsl(240.0, 5.9, 90.0),
                accordion_hover: hsl(240.0, 4.8, 95.9).opacity(0.7),
                background: hsl(0.0, 0.0, 100.),
                border: hsl(240.0, 5.9, 90.0),
                card: hsl(0.0, 0.0, 100.0),
                card_foreground: hsl(240.0, 10.0, 3.9),
                caret: hsl(240.0, 10., 3.9),
                danger: colors::red_500(),
                danger_active: colors::red_600(),
                danger_foreground: colors::red_50(),
                danger_hover: colors::red_500().opacity(0.9),
                description_list_label: hsl(240.0, 5.9, 96.9),
                description_list_label_foreground: hsl(240.0, 5.9, 10.0),
                drag_border: colors::blue_500(),
                drop_target: hsl(235.0, 30., 44.0).opacity(0.25),
                foreground: hsl(240.0, 10., 3.9),
                info: colors::sky_500(),
                info_active: colors::sky_600(),
                info_hover: colors::sky_500().opacity(0.9),
                info_foreground: colors::sky_50(),
                input: hsl(240.0, 5.9, 90.0),
                link: hsl(221.0, 83.0, 53.0),
                link_active: hsl(221.0, 83.0, 53.0).darken(0.2),
                link_hover: hsl(221.0, 83.0, 53.0).lighten(0.2),
                list: hsl(0.0, 0.0, 100.),
                list_active: hsl(211.0, 97.0, 85.0).opacity(0.2),
                list_active_border: hsl(211.0, 97.0, 85.0),
                list_even: hsl(240.0, 5.0, 96.0),
                list_head: hsl(0.0, 0.0, 100.),
                list_hover: hsl(240.0, 4.8, 95.0),
                muted: hsl(240.0, 4.8, 95.9),
                muted_foreground: hsl(240.0, 3.8, 46.1),
                popover: hsl(0.0, 0.0, 100.0),
                popover_foreground: hsl(240.0, 10.0, 3.9),
                primary: hsl(223.0, 5.9, 10.0),
                primary_active: hsl(223.0, 1.9, 25.0),
                primary_foreground: hsl(223.0, 0.0, 98.0),
                primary_hover: hsl(223.0, 5.9, 15.0),
                progress_bar: hsl(223.0, 5.9, 10.0),
                ring: hsl(223.81, 0., 6.),
                scrollbar: hsl(0., 0., 98.).opacity(0.95),
                scrollbar_thumb: hsl(0., 0., 69.).opacity(0.9),
                scrollbar_thumb_hover: hsl(0., 0., 59.),
                secondary: hsl(240.0, 5.9, 96.9),
                secondary_active: hsl(240.0, 5.9, 93.),
                secondary_foreground: hsl(240.0, 59.0, 10.),
                secondary_hover: hsl(240.0, 5.9, 98.),
                selection: hsl(211.0, 97.0, 85.0),
                sidebar: hsl(0.0, 0.0, 98.0),
                sidebar_accent: colors::zinc_200(),
                sidebar_accent_foreground: hsl(240.0, 5.9, 10.0),
                sidebar_border: hsl(220.0, 13.0, 91.0),
                sidebar_foreground: hsl(240.0, 5.3, 26.1),
                sidebar_primary: hsl(240.0, 5.9, 10.0),
                sidebar_primary_foreground: hsl(0.0, 0.0, 98.0),
                skeleton: hsl(223.0, 5.9, 10.0).opacity(0.1),
                slider_bar: hsl(223.0, 5.9, 10.0),
                slider_thumb: hsl(0.0, 0.0, 100.0),
                success: colors::green_500(),
                success_active: colors::green_600(),
                success_hover: colors::green_500().opacity(0.9),
                success_foreground: colors::gray_50(),
                switch: colors::zinc_300(),
                tab: gpui::transparent_black(),
                tab_active: hsl(0.0, 0.0, 100.0),
                tab_active_foreground: hsl(240.0, 10., 3.9),
                tab_bar: hsl(240.0, 14.3, 95.9),
                tab_bar_segmented: hsl(240.0, 14.3, 95.9),
                tab_foreground: hsl(240.0, 10., 33.9),
                table: hsl(0.0, 0.0, 100.),
                table_active: hsl(211.0, 97.0, 85.0).opacity(0.2),
                table_active_border: hsl(211.0, 97.0, 85.0),
                table_even: hsl(240.0, 5.0, 96.0),
                table_head: hsl(0.0, 0.0, 100.),
                table_head_foreground: hsl(240.0, 5.0, 34.),
                table_hover: hsl(240.0, 4.8, 95.0),
                table_row_border: hsl(240.0, 7.7, 94.5),
                tiles: hsl(0.0, 0.0, 95.),
                title_bar: hsl(0.0, 0.0, 100.),
                title_bar_border: hsl(240.0, 5.9, 90.0),
                warning: colors::yellow_500(),
                warning_active: colors::yellow_600(),
                warning_hover: colors::yellow_500().opacity(0.9),
                warning_foreground: colors::gray_50(),
                window_border: hsl(240.0, 5.9, 78.0),
            },
            dark: ThemeColor {
                accent: hsl(240.0, 3.7, 15.9),
                accent_foreground: hsl(0.0, 0.0, 78.0),
                accordion: hsl(299.0, 2., 11.),
                accordion_active: hsl(240.0, 3.7, 16.9),
                accordion_hover: hsl(240.0, 3.7, 15.9).opacity(0.7),
                background: hsl(0.0, 0.0, 8.0),
                border: hsl(240.0, 3.7, 16.9),
                card: hsl(0.0, 0.0, 8.0),
                card_foreground: hsl(0.0, 0.0, 78.0),
                caret: hsl(0., 0., 78.),
                danger: colors::red_800(),
                danger_active: colors::red_800().darken(0.2),
                danger_foreground: colors::red_50(),
                danger_hover: colors::red_800().opacity(0.9),
                description_list_label: hsl(240.0, 0., 13.0),
                description_list_label_foreground: hsl(0.0, 0.0, 78.0),
                drag_border: colors::blue_500(),
                drop_target: hsl(235.0, 30., 44.0).opacity(0.1),
                foreground: hsl(0., 0., 78.),
                info: colors::sky_900(),
                info_active: colors::sky_900().darken(0.2),
                info_foreground: colors::sky_50(),
                info_hover: colors::sky_900().opacity(0.8),
                input: hsl(240.0, 3.7, 15.9),
                link: hsl(221.0, 83.0, 53.0),
                link_active: hsl(221.0, 83.0, 53.0).darken(0.2),
                link_hover: hsl(221.0, 83.0, 53.0).lighten(0.2),
                list: hsl(0.0, 0.0, 8.0),
                list_active: hsl(240.0, 3.7, 15.0).opacity(0.2),
                list_active_border: hsl(240.0, 5.9, 35.5),
                list_even: hsl(240.0, 3.7, 10.0),
                list_head: hsl(0.0, 0.0, 8.0),
                list_hover: hsl(240.0, 3.7, 15.9),
                muted: hsl(240.0, 3.7, 15.9),
                muted_foreground: hsl(240.0, 5.0, 34.),
                popover: hsl(0.0, 0.0, 10.),
                popover_foreground: hsl(0.0, 0.0, 78.0),
                primary: hsl(223.0, 0.0, 98.0),
                primary_active: hsl(223.0, 0.0, 80.0),
                primary_foreground: hsl(223.0, 5.9, 10.0),
                primary_hover: hsl(223.0, 0.0, 90.0),
                progress_bar: hsl(223.0, 0.0, 98.0),
                ring: hsl(240.0, 4.9, 40.9),
                scrollbar: hsl(240.0, 0.0, 10.0).opacity(0.95),
                scrollbar_thumb: hsl(0., 0., 48.).opacity(0.9),
                scrollbar_thumb_hover: hsl(0., 0., 68.),
                secondary: hsl(240.0, 0., 13.0),
                secondary_active: hsl(240.0, 0., 13.),
                secondary_foreground: hsl(0.0, 0.0, 78.0),
                secondary_hover: hsl(240.0, 0., 15.),
                selection: hsl(211.0, 97.0, 22.0),
                sidebar: hsl(240.0, 0.0, 10.0),
                sidebar_accent: colors::zinc_800(),
                sidebar_accent_foreground: hsl(240.0, 4.8, 95.9),
                sidebar_border: hsl(240.0, 3.7, 15.9),
                sidebar_foreground: hsl(240.0, 4.8, 95.9),
                sidebar_primary: hsl(0.0, 0.0, 98.0),
                sidebar_primary_foreground: hsl(240.0, 5.9, 10.0),
                skeleton: hsl(223.0, 0.0, 98.0),
                slider_bar: hsl(223.0, 0.0, 98.0),
                slider_thumb: hsl(0.0, 0.0, 8.0),
                success: colors::green_800(),
                success_active: colors::green_800().darken(0.2),
                success_foreground: colors::green_50(),
                success_hover: colors::green_800().opacity(0.8),
                switch: colors::zinc_600(),
                tab: gpui::transparent_black(),
                tab_active: hsl(0.0, 0.0, 8.0),
                tab_active_foreground: hsl(0., 0., 78.),
                tab_bar: hsl(299.0, 0., 5.5),
                tab_bar_segmented: hsl(299.0, 0., 5.5),
                tab_foreground: hsl(0., 0., 78.),
                table: hsl(0.0, 0.0, 8.0),
                table_active: hsl(240.0, 3.7, 15.0).opacity(0.2),
                table_active_border: hsl(240.0, 5.9, 35.5),
                table_even: hsl(240.0, 3.7, 10.0),
                table_head: hsl(0.0, 0.0, 8.0),
                table_head_foreground: hsl(0., 0., 78.).opacity(0.7),
                table_hover: hsl(240.0, 3.7, 15.9).opacity(0.5),
                table_row_border: hsl(240.0, 3.7, 16.9).opacity(0.5),
                tiles: hsl(0.0, 0.0, 5.0),
                title_bar: hsl(0., 0., 9.7),
                title_bar_border: hsl(240.0, 3.7, 15.9),
                warning: colors::yellow_800(),
                warning_active: colors::yellow_800().darken(0.2),
                warning_foreground: colors::yellow_50(),
                warning_hover: colors::yellow_800().opacity(0.9),
                window_border: hsl(240.0, 3.7, 28.0),
            },
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            working_dir: None,
            theme_mode: ThemeMode::Dark,
            ui_settings: UiSettings::default(),
        }
    }
}

pub fn config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "", "control")
        .context("Could not determine config directory")?;
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir)?;                                     
    Ok(config_dir.join("config.toml"))
}

pub fn load_config() -> Result<AppConfig> {
    let path = config_path()?;
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        Ok(toml::from_str(&content)?)
    } else {
        let c = AppConfig::default();
        save_config(&c)?;
        Ok(c)
    }
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = config_path()?;
    let content = toml::to_string(config)?;
    fs::write(path, content)?;
    Ok(())
}

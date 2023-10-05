mod theme;
use crate::{components::*, icon::*, theme::*, utils::mount_style::mount_style};
use leptos::*;
use stylers::style_sheet_str;
pub use theme::ButtonTheme;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonType {
    #[default]
    PRIMARY,
    SOLID,
    TEXT,
    LINK,
}

#[derive(Default, Clone)]
pub enum ButtonColor {
    #[default]
    PRIMARY,
    SUCCESS,
    WARNING,
    ERROR,
}

impl ButtonColor {
    pub fn theme_color(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::PRIMARY => theme.common.color_primary.clone(),
            ButtonColor::SUCCESS => theme.common.color_success.clone(),
            ButtonColor::WARNING => theme.common.color_warning.clone(),
            ButtonColor::ERROR => theme.common.color_error.clone(),
        }
    }
    pub fn theme_color_hover(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::PRIMARY => theme.common.color_primary_hover.clone(),
            ButtonColor::SUCCESS => theme.common.color_success_hover.clone(),
            ButtonColor::WARNING => theme.common.color_warning_hover.clone(),
            ButtonColor::ERROR => theme.common.color_error_hover.clone(),
        }
    }

    pub fn theme_color_active(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::PRIMARY => theme.common.color_primary_active.clone(),
            ButtonColor::SUCCESS => theme.common.color_success_active.clone(),
            ButtonColor::WARNING => theme.common.color_warning_active.clone(),
            ButtonColor::ERROR => theme.common.color_error_active.clone(),
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional, into)] style: MaybeSignal<String>,
    #[prop(optional, into)] type_: MaybeSignal<ButtonType>,
    #[prop(optional, into)] color: MaybeSignal<ButtonColor>,
    #[prop(optional, into)] round: MaybeSignal<bool>,
    #[prop(optional, into)] icon: Option<Icon>,
    #[prop(optional, into)] loading: MaybeSignal<bool>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let bg_color = color.get().theme_color(&theme);
        let bg_color_hover = color.get().theme_color_hover(&theme);
        let bg_color_active = color.get().theme_color_active(&theme);
        if type_.get() == ButtonType::PRIMARY {
            css_vars.push_str(&format!("--background-color: {bg_color};"));
            css_vars.push_str(&format!("--background-color-hover: {bg_color_hover};"));
            css_vars.push_str(&format!("--background-color-active: {bg_color_active};"));
            css_vars.push_str(&format!("--font-color: #fff;"));
            css_vars.push_str(&format!("--border-color: {bg_color};"));
            css_vars.push_str(&format!("--border-color-hover: {bg_color};"));
        } else {
            css_vars.push_str(&format!("--font-color-hover: {bg_color};"));
            css_vars.push_str(&format!("--border-color: #555a;"));
            css_vars.push_str(&format!("--border-color-hover: #555;"));
        }

        css_vars
    });
    let class_name = mount_style("button", || style_sheet_str!("./src/button/button.css"));

    let icon_style = if children.is_some() {
        "margin-right: 6px"
    } else {
        ""
    };

    let disabled = create_memo(move |_| {
        if loading.get() {
            return true;
        }

        disabled.get()
    });

    let on_click = move |event| {
        if disabled.get() {
            return;
        }
        let Some(callback) = on_click.as_ref() else {
            return;
        };
        callback.call(event);
    };

    view! {class=class_name,
        <button
            class:melt-button=true
            class=("melt-button--text", move || type_.get() == ButtonType::TEXT)
            class=("melt-button--link", move || type_.get() == ButtonType::LINK)
            class=("melt-button--round", move || round.get())
            class=("melt-button--disabled", move || disabled.get())
            style=move || format!("{}{}", css_vars.get(), style.get())
            on:click=on_click
            >
                {
                    move || {
                        if loading.get() {
                            view! {
                                <Icon icon=Icon::from(AiIcon::AiLoadingOutlined) style=format!("animation: meltLoadingCircle 1s infinite linear;{icon_style}")/>
                            }.into()
                        } else if let Some(icon) = icon {
                            view! {
                                <Icon icon=icon style=icon_style/>
                            }.into()
                        } else {
                            None
                        }
                    }
                }
                <OptionComp value=children let:children>
                    { children() }
                </OptionComp>
        </button>
    }
}

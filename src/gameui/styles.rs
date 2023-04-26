pub mod button {
    use crate::gameui;
    use bevy::prelude::*;

    pub fn default() -> Style {
        Style {
            size: Size::new(Val::Px(120.0), Val::Px(50.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            ..Default::default()
        }
    }

    pub fn text(font: Handle<Font>) -> TextStyle {
        TextStyle {
            font,
            font_size: 40.0,
            color: gameui::colors::button::TEXT,
        }
    }
}

pub mod headers {
    use crate::gameui;
    use bevy::prelude::*;
    pub fn h1(font: Handle<Font>) -> TextStyle {
        TextStyle {
            font,
            font_size: 100.0,
            color: gameui::colors::headers::TITLE,
        }
    }
    pub fn h2(font: Handle<Font>) -> TextStyle {
        TextStyle {
            font,
            font_size: 30.0,
            color: gameui::colors::headers::SUBTITLE,
        }
    }
}

pub mod container {
    use bevy::prelude::*;

    pub fn frame() -> Style {
        Style {
            size: Size::width(Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        }
    }
}

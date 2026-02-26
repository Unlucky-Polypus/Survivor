use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

pub(crate) struct Window {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) skin: Skin,
    pub(crate) position: Vec2,
}

pub(crate) enum Action {
    Play,
    Options,
    Quit,
    Resume,
    QuitToMainMenu,
}

impl Window{
    pub(crate) async fn new(width: f32, height: f32, position: Vec2) -> Self {
        let font_result = load_ttf_font("assets/ui/MinimalPixel_v2.ttf").await;
        let font = match font_result {
            Ok(font) => font,
            Err(error) => panic!("{error}"),
        };
        let label_style = root_ui()
            .style_builder()
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(25)
            .build();
        
        let window_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("../../assets/ui/window_background_2.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
            .margin(RectOffset::new(-30.0, 0.0, -30.0, 0.0))
            .build();
        
        let button_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("../../assets/ui/button_background_2.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
            .background_hovered(
                Image::from_file_with_format(
                    include_bytes!("../../assets/ui/button_hovered_background_2.png"),
                    None,
                )
                .unwrap(),
            )
            .background_clicked(
                Image::from_file_with_format(
                    include_bytes!("../../assets/ui/button_clicked_background_2.png"),
                    None,
                )
                .unwrap(),
            )
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();
        
        let checkbox_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("../../assets/ui/checkbox_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_hovered(
                Image::from_file_with_format(
                    include_bytes!("../../assets/ui/checkbox_hovered_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_clicked(
                Image::from_file_with_format(
                    include_bytes!("../../assets/ui/checkbox_clicked_background.png"),
                    None,
                )
                .unwrap(),
            )
            .build();
        
        let editbox_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("../../assets/ui/editbox_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(2., 2., 2., 2.))
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(25)
            .build();
        
        let combobox_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("../../assets/ui/combobox_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(4., 25., 6., 6.))
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .color(Color::from_rgba(210, 210, 210, 255))
            .font_size(25)
            .build();
    
        let skin = Skin {
            window_style,
            button_style,
            label_style,
            checkbox_style,
            editbox_style,
            combobox_style,
            ..root_ui().default_skin()
        };
        
        Self {
            width,
            height,
            skin,
            position,
        }
    }
}
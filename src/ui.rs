mod window;

use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

pub(crate) fn draw_ui(skin: &Skin) {
    let window_skin = skin.clone();
    
    let mut checkbox = false;
    let mut text = String::new();
    let mut number = 0.0f32;
    let mut combobox = 0;
    
    root_ui().push_skin(&window_skin);
    
    clear_background(GRAY);
    
    root_ui().same_line(0.);
    
    
    root_ui().window(hash!(), vec2(20., 250.), vec2(300., 600.), |ui| {
        widgets::Button::new("Play")
        .position(vec2(65.0, 15.0))
        .ui(ui);
        widgets::Button::new("Options")
        .position(vec2(40.0, 75.0))
        .ui(ui);
        
        widgets::Button::new("Quit")
        .position(vec2(65.0, 195.0))
        .ui(ui);
    });
    // root_ui().window(hash!(), vec2(250., 20.), vec2(500., 250.), |ui| {
    //     ui.checkbox(hash!(), "Checkbox 1", &mut checkbox);
    //     ui.combo_box(
    //         hash!(),
    //         "Combobox",
    //         &["First option", "Second option"],
    //         &mut combobox,
    //     );
    //     ui.input_text(hash!(), "Text", &mut text);
    //     ui.drag(hash!(), "Drag", None, &mut number);
        
    //     widgets::Button::new("Apply")
    //     .position(vec2(80.0, 150.0))
    //     .ui(ui);
    //     widgets::Button::new("Cancel")
    //     .position(vec2(280.0, 150.0))
    //     .ui(ui);
    // });
    // root_ui().pop_skin();
    
    // next_frame().await;
}

pub(crate) async fn load_skin() -> Skin {
    
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
                include_bytes!("../assets/ui/window_background_2.png"),
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
                include_bytes!("../assets/ui/button_background_2.png"),
                None,
            )
            .unwrap(),
        )
        .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
        .background_hovered(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/button_hovered_background_2.png"),
                None,
            )
            .unwrap(),
        )
        .background_clicked(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/button_clicked_background_2.png"),
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
                include_bytes!("../assets/ui/checkbox_background.png"),
                None,
            )
            .unwrap(),
        )
        .background_hovered(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/checkbox_hovered_background.png"),
                None,
            )
            .unwrap(),
        )
        .background_clicked(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/checkbox_clicked_background.png"),
                None,
            )
            .unwrap(),
        )
        .build();
    
    let editbox_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/editbox_background.png"),
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
                include_bytes!("../assets/ui/combobox_background.png"),
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

    Skin {
        window_style,
        button_style,
        label_style,
        checkbox_style,
        editbox_style,
        combobox_style,
        ..root_ui().default_skin()
    }
}
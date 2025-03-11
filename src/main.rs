mod ui;

fn main() {
    let native_options = eframe::NativeOptions {

        viewport: egui::ViewportBuilder::default()
            // .with_resizable(false)
            .with_inner_size((600.0, 600.0)),
        // renderer: eframe::Renderer::default().egui_wgpu(),
        ..eframe::NativeOptions::default()
    };


    eframe::run_native(
        crate::ui::ui_main::Notatnik::name(),
        native_options.clone(),
        Box::new(|_| Ok(Box::<crate::ui::ui_main::Notatnik>::default())),
    )
        .unwrap();
}

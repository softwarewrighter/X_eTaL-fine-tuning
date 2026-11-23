fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<training_at_depth_web::app::App>::new().render();
}

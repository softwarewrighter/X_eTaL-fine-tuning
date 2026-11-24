fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<dynamic_mixing_web::app::App>::new().render();
}

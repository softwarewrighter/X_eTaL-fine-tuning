fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<gain_by_depth_web::app::App>::new().render();
}

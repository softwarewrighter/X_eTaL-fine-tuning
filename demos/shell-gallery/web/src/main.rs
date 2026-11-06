fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<shell_gallery_web::app::App>::new().render();
}

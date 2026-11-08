fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<rank_and_scaling_web::app::App>::new().render();
}

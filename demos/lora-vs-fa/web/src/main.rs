fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<lora_vs_fa_web::app::App>::new().render();
}

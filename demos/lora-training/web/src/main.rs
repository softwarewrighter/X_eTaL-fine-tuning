fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<lora_training_web::app::App>::new().render();
}

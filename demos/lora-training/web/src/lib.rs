//! LoRA training in the browser: the demo's own program
//! (lora-training.xtl) run at a preset's settings, its one step read
//! back as a trace, its record (the losses, the adapter at each
//! record) and its chart read back. `micro` is the model (tested
//! natively); `text` the prose; `app` the page.

pub mod app;
pub mod micro;
pub mod text;

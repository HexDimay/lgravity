use anyhow::Ok;
use lgravity::app::App;
use winit::event_loop::EventLoop;

#[pollster_macro::main]
async fn main() -> anyhow::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
        env_logger::init();
        log::info!("Start logger.");
    }

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

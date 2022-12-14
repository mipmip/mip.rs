
pub fn window(port: u16) -> wry::Result<()> {

  use wry::{
    application::{
      event::{Event, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder,
    },
    webview::WebViewBuilder,
  };

  let url = format!("http://localhost:{}/.temp.html", port);

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("MiP")
    .build(&event_loop)?;
  let _webview = WebViewBuilder::new(window)?
    .with_url(&url)?
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => (),
    }
  });
}

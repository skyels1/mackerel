use winit::event_loop::EventLoop;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::application::ApplicationHandler;
use winit::window::Window;
use winit::window::WindowId;
use winit::event::WindowEvent;

struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("mackerel");
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }
    fn window_event (&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping app");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main () {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App { window:None };
    event_loop.run_app(&mut app).unwrap();
}

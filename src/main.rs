use winit::event_loop::EventLoop;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::application::ApplicationHandler;
use winit::window::Window;
use winit::window::WindowId;
use winit::event::WindowEvent;
use egui::Context;
use std::sync::Arc;

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    last_key: String,
    command: String,
    history: Vec<String>,
    ctx: Context,
    egui_state: Option<egui_winit::State>,
    painter: Option<egui_wgpu::winit::Painter>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("mackerel");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.egui_state = Some(egui_winit::State::new(self.ctx.clone(), egui::ViewportId::ROOT, &window, None, None, None,));

        let mut painter = pollster::block_on(egui_wgpu::winit::Painter::new(self.ctx.clone(), egui_wgpu::WgpuConfiguration::default(), false, egui_wgpu::RendererOptions::default(),),);
        pollster::block_on(painter.set_window(egui::ViewportId::ROOT, Some(window.clone()))).unwrap();
        self.painter = Some(painter);
        self.window = Some(window);
    }
    fn window_event (&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping app");
                event_loop.exit();
            },
            WindowEvent::KeyboardInput {event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    self.window.as_ref().unwrap().request_redraw();
                    self.command.push_str(&self.last_key);

                    if event.logical_key == winit::keyboard::Key::Named (winit::keyboard::NamedKey::Enter) {
                        if !self.command.trim().is_empty() {
                            self.history.push(self.command.clone());
                            self.command.clear();
                        }
                    } else if let Some(text) = event.text.as_ref() {
                        self.command.push_str(text);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();

                let input = self.egui_state.as_mut().unwrap().take_egui_input(window);

                let output = self.ctx.run_ui(input, |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Hello, Mackerel!");

                        for command in &self.history {
                            ui.label(format!("> {}", command));
                        }

                        ui.text_edit_singleline(&mut self.command);
                    });
                });

                self.egui_state.as_mut().unwrap().handle_platform_output(window, output.platform_output);

                let clipped_primitives = self.ctx.tessellate(output.shapes, output.pixels_per_point,);
                self.painter.as_mut().unwrap().paint_and_update_textures(egui::ViewportId::ROOT, output.pixels_per_point, [0.05, 0.05, 0.05, 1.0], &clipped_primitives, &output.textures_delta, Vec::new(), window,);
            }
            _ => (),
        }
    }
}

fn main () {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

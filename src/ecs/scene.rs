use super::{ev::Control, Ev, SystemManager, World};
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
    },
    Display, Surface,
};

#[derive(Clone)]
pub struct Scene {
    pub display: Display,
    pub bg: [f32; 4],
}

impl Scene {
    pub fn new(display: Display, bg: [f32; 4]) -> Self {
        Self { display, bg }
    }

    pub fn init(
        mut self,
        event_loop: EventLoop<()>,
        mut world: World<'static>,
        mut system_manager: SystemManager<'static>,
    ) -> anyhow::Result<()> {
        system_manager.init(&mut self, &mut world)?;

        event_loop.run(move |event, _, control_flow| {
            if let Err(e) = self.update(
                Control::new(event),
                control_flow,
                &mut world,
                &mut system_manager,
            ) {
                eprintln!("{}", e);
            }
        })
    }

    pub fn update(
        &mut self,
        mut control: Control,
        flow: &mut ControlFlow,
        world: &mut World,
        system_manager: &mut SystemManager,
    ) -> anyhow::Result<()> {
        match &control.event {
            Event::RedrawRequested(window_id)
                if *window_id == self.display.gl_window().window().id() =>
            {
                let mut target = self.display.draw();

                target.clear_color_and_depth(
                    {
                        let [r, g, b, a] = self.bg;

                        (r, g, b, a)
                    },
                    1.0,
                );

                system_manager.update(&mut Ev::Draw((&mut control, &mut target)), self, world)?;

                target.finish()?;
            }
            _ => system_manager.update(&mut Ev::Event(&mut control), self, world)?,
        }

        *flow = match &control {
            Control {
                flow: Some(control_flow),
                event: _,
            } => *control_flow,
            Control { flow: None, event } => match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if *window_id == self.display.gl_window().window().id() => ControlFlow::Exit,
                _ => {
                    if let Event::MainEventsCleared = event {
                        self.display.gl_window().window().request_redraw();
                    }

                    ControlFlow::Poll
                }
            },
        };

        Ok(())
    }
}
use crate::core::window::SystemWindow;
use crate::core::window_builder::WindowConfig;

pub(crate) enum WindowEvent {
  Close,
  Resize { w: u32, h: u32 },
  RequestRedraw,
  FocusChange(FocusStatus),
  Input(InputEvent),
}

pub(crate) enum FocusStatus {
  Lost,
  Gained,
}

pub(crate) enum InputEvent {
  MouseEvent(MouseEvent),
  KeyEvent(KeyEvent)
}

pub(crate) enum MouseEvent {
  MouseMoved { x: i32, y: i32 },

  LeftButtonDown { x: i32, y: i32 },
  LeftButtonUp { x: i32, y: i32 },

  RightButtonDown { x: i32, y: i32 },
  RightButtonUp { x: i32, y: i32 },

  Scroll { delta_x: f32, delta_y: f32 },
}

pub(crate) enum KeyEvent {
  KeyDown { keycode: u32, modifiers: Modifiers },
  KeyUp { keycode: u32, modifiers: Modifiers  },
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct Modifiers(u8);

impl Modifiers {
  const SHIFT_MASK: u8 = 1 << 0;
  const CTRL_MASK: u8 = 1 << 1;
  const ALT_MASK: u8 = 1 << 2;
  const LOGO_MASK: u8 = 1 << 3;

  #[inline]
  pub fn shift(&self) -> bool {
    self.0 & Self::SHIFT_MASK != 0
  }

  #[inline]
  pub fn ctrl(&self) -> bool {
    self.0 & Self::CTRL_MASK != 0
  }

  #[inline]
  pub fn alt(&self) -> bool {
    self.0 & Self::ALT_MASK != 0
  }

  #[inline]
  pub fn logo(&self) -> bool {
    self.0 & Self::LOGO_MASK != 0
  }

}

/// Purely for developer experience 100% optional, provides the most essential events in an exposed manner
pub(crate) enum FlatEvent {
  Close,
  RedrawRequested,
  Resize { w: u32, h: u32 },
  LeftMouseButtonDown { x: f32, y: f32 },
  LeftMouseButtonUp { x: f32, y: f32 },
  RightMouseButtonDown { x: f32, y: f32 },
  RightMouseButtonUp { x: f32, y: f32 },
  MouseMoved { x: f32, y: f32 },
  MouseWheel { delta_x: f32, delta_y: f32 },
  KeyDown { code: u32, modifiers: Modifiers },
  KeyUp { code: u32, modifiers: Modifiers },
  Other(WindowEvent), // To handle events not in the flat list
}

impl From<WindowEvent> for FlatEvent {
  fn from(value: WindowEvent) -> Self {
    match value {
      WindowEvent::Close => FlatEvent::Close,
      WindowEvent::RequestRedraw => FlatEvent::RedrawRequested,
      WindowEvent::Resize { w, h } => FlatEvent::Resize { w, h },
      // We won't flatten FocusChange for a "most essential" list, put it in Other
      WindowEvent::FocusChange(_) => FlatEvent::Other(value),
      WindowEvent::Input(input_event) => {
        match input_event {
          InputEvent::MouseEvent(mouse_event) => {
            match mouse_event {
              MouseEvent::MouseMoved { x, y } => FlatEvent::MouseMoved { x: x as f32, y: y as f32 },
              MouseEvent::LeftButtonDown { x, y } => FlatEvent::LeftMouseButtonDown { x: x as f32, y: y as f32 },
              MouseEvent::LeftButtonUp { x, y } => FlatEvent::LeftMouseButtonUp { x: x as f32, y: y as f32 },
              MouseEvent::RightButtonDown { x, y } => FlatEvent::RightMouseButtonDown { x: x as f32, y: y as f32 },
              MouseEvent::RightButtonUp { x, y } => FlatEvent::RightMouseButtonUp { x: x as f32, y: y as f32 },
              // Note: Your MouseEvent for Scroll was simplified.
              // I've updated it in the struct definitions above to be more typical.
              MouseEvent::Scroll { delta_x, delta_y } => FlatEvent::MouseWheel { delta_x, delta_y },
            }
          }
          InputEvent::KeyEvent(key_event) => {
            match key_event {
              KeyEvent::KeyDown { keycode, modifiers } => FlatEvent::KeyDown { code: keycode, modifiers },
              KeyEvent::KeyUp { keycode, modifiers } => FlatEvent::KeyUp { code: keycode, modifiers },
            }
          }
        }
      }
    }
  }
}
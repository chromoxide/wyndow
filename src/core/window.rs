use crate::core::events::WindowEvent;
use crate::Result;



/// Contains behavior for all core functionality a window should have
pub(crate) trait SystemWindow: Drop {

  fn init() -> Result<Self>;

  fn set_title(&mut self, title: &str);

  fn get_title(&self) -> Result<&str>;

  fn set_size(&mut self, size: (u32, u32));

  fn get_size(&self) -> Result<(u32, u32)>;

  fn set_position(&mut self, position: (i32, i32));

  fn get_position(&self) -> Result<(i32, i32)>;

  fn next_event(&mut self) -> Option<WindowEvent>;

  fn cleanup(&mut self) -> Result<()>;

}

struct Window<T> where T: SystemWindow {
  system_window: T,
}


type WinRawSystemWindow = crate::win::window::Window;

#[cfg(target_os = "windows")]
impl<T> Window<T> where
  T: SystemWindow,
{
  fn new( window_info: WindowInfo ) -> Result<Self> {

  }

}

pub(crate) trait WindowIcon {
  todo!();
}

pub(crate) trait WindowCursor {
  todo!();
}
use crate::core::window_builder::WindowConfig;
use crate::Result;

type RawSystemWindow = crate::win::window::Window;

/// Contains behavior for all core functionality a window should have
pub(crate) trait SystemWindow: Drop {

  fn init() -> Result<Self>;

  fn cleanup(&mut self) -> Result<()>;

}

struct Window<T> where T: SystemWindow {
  system_window: T,
  title: String,
  size: (u32, u32),
}



#[cfg(target_os = "windows")]
impl<T> Window<T> where
  T: SystemWindow,
{
  fn new(window_config: WindowConfig) -> Self {

  }
  fn builder()
}


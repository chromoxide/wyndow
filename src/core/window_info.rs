use std::mem;
use std::ptr::{null, null_mut};
use libc::c_char;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{CS_HREDRAW, CS_OWNDC, CS_VREDRAW, WNDCLASSEXW, WNDCLASSW, WNDPROC};
use crate::win::window::{Window, WindowState};

/// Contains information that a window will use to initialize.
pub(crate) struct WindowInfo {
  pub(crate) title: String,
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) x: u32,
  pub(crate) y: u32,
}

impl WindowInfo {
  /// Returns the WindowInfoBuilder
  fn builder( title: &str, width: u32, height: u32 ) -> WindowInfoBuilder {
    WindowInfoBuilder::new(title, width, height)
  }

}

pub(crate) struct WindowInfoBuilder {
  title: String,
  width: u32,
  height: u32,
  x: Option<u32>,
  y: Option<u32>,
}

impl WindowInfoBuilder {
  pub fn new(title: &str, width: u32, height: u32) -> Self {
    Self {
      title: title.to_string(),
      width,
      height,
      x: None,
      y: None,
    }
  }

  pub fn pos(mut self, x: u32, y: u32) -> Self {
    self.x = Some(x);
    self.y = Some(y);
    self
  }

  pub fn build(mut self) -> WindowInfo {
    WindowInfo {
      title: self.title,
      width: self.width,
      height: self.height,
      x: 100,
      y: 100,
    }
  }

}


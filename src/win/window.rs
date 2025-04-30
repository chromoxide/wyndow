use crate::Result;
use crate::Error;
use crate::core::window::SystemWindow;

use libc::c_char;
use std::num::NonZeroU16;
use std::ptr::null_mut;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::windef::{HDC, HWND};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::SwapBuffers;
use winapi::um::winuser::{
  CreateWindowExA, DestroyWindow, GetDC, RegisterClassA, ReleaseDC, ShowWindow, CW_USEDEFAULT,
  SW_SHOW, WNDCLASSA, WS_OVERLAPPEDWINDOW,
};

pub struct Window {
  window: HWND,
  context: HDC,
  atom: NonZeroU16,
  title: &'static str,
}

impl Window {
  pub const CLASS_NAME: *const c_char = b"HexonWindow\0" as *const u8 as *const c_char;
  pub unsafe fn new(window_class: *const WNDCLASSA, title: &'static str) -> Result<Self> {
    if window_class.is_null() {
      return Err(Error::Error("window_class is null"));
    };

    // if (*window_class).lpszClassName != Self::CLASS_NAME {
    //   return Err(Error::Error("use the `SystemWindow::CLASS_NAME` as the class_name"))
    // }

    let h_instance = GetModuleHandleW(null_mut());

    let atom = Self::register_class(window_class)?;
    let window = Self::create_window(title, h_instance)?;
    // Can do unchecked since ^^^^^^^^^^^^^^^ will return err is window is null.
    let context = Self::create_context(window)?;

    Ok(Self {
      window,
      context,
      atom,
      title,
    })
  }

  pub unsafe fn register_class(window_class: *const WNDCLASSA) -> Result<NonZeroU16> {
    let atom = RegisterClassA(window_class);
    if atom == 0 {
      Err(Error::Error("Failed to register class"))
    } else {
      Ok(NonZeroU16::new_unchecked(atom))
    }
  }

  pub unsafe fn create_window(title: &'static str, h_instance: HMODULE) -> Result<HWND> {
    let window = CreateWindowExA(
      0,
      Self::CLASS_NAME,
      title.as_ptr() as *const c_char,
      WS_OVERLAPPEDWINDOW,
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      null_mut(),
      null_mut(),
      h_instance,
      null_mut(),
    );

    if window.is_null() {
      Err(Error::Error("Failed to create window"))
    } else {
      Ok(window)
    }
  }

  pub unsafe fn create_context(window: HWND) -> Result<HDC> {
    if window.is_null() {
      return Err(Error::Error("window is null"));
    };

    Ok(GetDC(window))
  }

  pub fn show_window(&self) {
    unsafe { ShowWindow(self.window, SW_SHOW) };
  }
  pub fn swap_buffers(&self) {
    unsafe { SwapBuffers(self.context) };
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    unsafe {
      DestroyWindow(self.window);
      ReleaseDC(self.window, self.context);
    }
  }
}
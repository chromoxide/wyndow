use std::collections::VecDeque;
use std::ffi::OsString;
use crate::Result;
use crate::Error;
use crate::core::window::SystemWindow;
use crate::core::events::WindowEvent;

use libc::c_char;
use std::num::NonZeroU16;
use std::os::windows::ffi::OsStrExt;
use std::pin::Pin;
use std::ptr::{null_mut, NonNull};
use std::sync::Mutex;
use winapi::shared::minwindef::{ATOM, HMODULE, LPARAM, UINT, WPARAM};
use winapi::shared::windef::{HDC, HWND};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::SwapBuffers;
use winapi::um::winuser::{CreateWindowExA, CreateWindowExW, DefWindowProcW, DestroyWindow, GetDC, GetMessageW, GetWindowLongPtrW, RegisterClassA, RegisterClassW, ReleaseDC, ShowWindow, CREATESTRUCTW, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, SW_SHOW, VK_EXECUTE, WM_CREATE, WM_NCCREATE, WNDCLASSA, WNDCLASSW, WNDPROC, WS_OVERLAPPEDWINDOW};
use crate::core::window_info::{WindowInfo, WindowInfoBuilder};

unsafe extern "system" fn window_proc(
  window: HWND,
  msg: u32,
  w_param: WPARAM,
  l_param: LPARAM,
) -> isize {
  let window_state_ptr = unsafe {
    if msg == WM_NCCREATE {
      let create_struct = &*(l_param as *const CREATESTRUCTW);
      create_struct.lpCreateParams as *mut WindowState
    } else {
      GetWindowLongPtrW(window, GWLP_USERDATA) as *mut WindowState
    }
  }

  if window_state_ptr.is_null() {
    return unsafe { DefWindowProcW(window, msg, w_param, l_param) };
  };

  {
    let window_state = unsafe { &mut *window_state_ptr };

    if let Ok(mut queue) = window_state.event_buffer.lock() {
      todo!("Implement event logic")
    }
  }
}

// Encodes a string into a null terminated wide string.
fn encode_wide(str: String) -> Vec<u16> {
  OsString::from(str).encode_wide().chain(Some(0)).collect()
}

pub(crate) fn wndclassw( window_info: WindowInfo, class_name: String, h_instance: HMODULE, event_proc: WNDPROC ) -> WNDCLASSW {
  WNDCLASSW {
    // To ensure each window has its own DC + redrawing for when the window changes size.
    style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,

    // Event procedure that will push events to the event_buffer in the WinWindow
    lpfnWndProc: event_proc,

    // Extra bytes to add for the class and window in memory.
    cbClsExtra: 0,
    // Needed to store the event queue mutex.
    cbWndExtra: size_of::<*mut WindowState>() as i32,

    hInstance: h_instance,

    // Custom icon and cursor support not implemented
    // TODO: implement custom icons and cursors.
    hIcon: null_mut(),
    hCursor: null_mut(),

    // Color, pattern or bitmap to use as the windows bg.
    // Legacy leftover feature not commonly used.
    hbrBackground: null_mut(),

    // Pointer to resource files for the top bar menu you can see in some applications.
    // Not commonly used either, so can be ignored.
    lpszMenuName: null_mut(),

    // Very important, used as the class_name for this specific window class.
    lpszClassName: encode_wide(class_name).as_ptr(),
  }
}

pub struct WindowState {
  window: HWND,
  event_buffer: Mutex<VecDeque<WindowEvent>>,
  title: String,
}

impl WindowState {
  pub fn new() -> Self {
    Self {
      window: null_mut(),
      event_buffer: Mutex::new(VecDeque::new()),
      title: String::from("Application Window"),
    }
  }
}

pub struct Window {
  window_state: Pin<Box<WindowState>>,
  context: HDC,
  atom: NonZeroU16,
}

impl Window {
  pub fn new(window_info: WindowInfo) -> Result<Self> {

    // Get the module for the current process.
    let h_instance =  unsafe { GetModuleHandleW(null_mut()) };

    let state = Box::new(WindowState::new());

    let mut pinned_state = Box::into_pin(state);
    let state_ptr: *mut WindowState = &mut *pinned_state;

    let window_class = wndclassw(window_info, "wyndow_class".into(), h_instance, window_proc as WNDPROC);

    // Register the window_class provided by a WindowConfig
    let atom = Self::register_class()?;
    let window = Self::create_window(state_ptr, h_instance)?;
    let context = Self::create_context(window)?;

    unsafe { (*state_ptr).window = window; };

    Ok(Self {
      window_state: pinned_state,
      context,
      atom,
    })

  }

  pub fn register_class(window_class: *const WNDCLASSW) -> Result<NonZeroU16> {

    // The atom is a unique identifier for the window.
    let atom: ATOM = unsafe { RegisterClassW(window_class) };

    // Atom cannot be zero since that means registering the class has failed.
    if atom == 0 {
      Err(Error::Error("Failed to register class"))
    } else {
      unsafe { Ok(NonZeroU16::new_unchecked(atom)) }
    }
  }

  pub fn create_window(state_ptr: *mut WindowState, h_instance: HMODULE) -> Result<HWND> {

    if h_instance.is_null() {
      return Err(Error::Error("Failed to create window"));
    }

    let window = unsafe { CreateWindowExW(
      0,
      encode_wide("wyndow_class".into()).as_ptr(),
      encode_wide("Application Window".into()).as_ptr(),
      WS_OVERLAPPEDWINDOW,
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      null_mut(),
      null_mut(),
      h_instance,
      state_ptr as *mut _,
    ) };

    if window.is_null() {
      Err(Error::Error("Failed to create window"))
    } else {
      Ok(window)
    }

  }

  pub fn create_context(window: HWND) -> Result<HDC> {
    if window.is_null() {
      return Err(Error::Error("window is null"));
    };

    unsafe { Ok(GetDC(window)) }
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
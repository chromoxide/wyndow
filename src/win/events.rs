use winapi::shared::windef::HWND;

unsafe extern "system" fn event_procedure(
  window: HWND,
  message: u32,
  wparam: usize,
  lparam: isize,
) -> isize {

}
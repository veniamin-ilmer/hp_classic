use arbitrary_int::u6;
use wasm_bindgen::{JsValue, JsCast};
use chips::hp_classic::cnt;

pub(super) struct Keyboard {
  pending_button_var: wasm_bindgen::JsValue,
}

impl Keyboard {
  pub(super) fn new() -> Self {
    let pending_button_var = js_sys::Reflect::get(
        &wasm_bindgen::JsValue::from(web_sys::window().unwrap()),
        &wasm_bindgen::JsValue::from("getPendingButton"),
    ).unwrap();
    
    Self {
      pending_button_var,
    }
  }
  
  pub(super) fn run_refresh_cycle(&self, cnt: &mut cnt::CnT) {
    let pending_click_func: &js_sys::Function = self.pending_button_var.dyn_ref().unwrap();
    let click_var = pending_click_func.apply(&JsValue::null(), &js_sys::Array::new()).unwrap();
    if let Some(click_float) = click_var.as_f64() {
      let code = click_float.round(); //Wish there were a way to get an integer directly without needing to go through a float...
      
      if code == -1.0 { //Depress event
        cnt.status.write_bit(0, false);
        cnt.current_keypress = None;
      } else if code == 255.0 { //Hack around javascript not able to handle 0.
        cnt.status.write_bit(0, true);  //The only status flag connected to hardware. (Set with key press).
        cnt.current_keypress = Some(u6::new(0));
      } else {
        cnt.status.write_bit(0, true);  //The only status flag connected to hardware. (Set with key press).
        cnt.current_keypress = Some(u6::new(code as u8));
      }
    }
  }

}

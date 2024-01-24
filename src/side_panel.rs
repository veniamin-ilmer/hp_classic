use chips::hp_classic;
use chips::shifter;
use chips::Indexer16;


pub struct SidePanel<const EXTRA_REGS: usize> {
  cnt: Option<web_sys::HtmlCollection>,
  anr: Option<web_sys::HtmlCollection>,
  ram: Option<web_sys::HtmlCollection>,
  status: Option<web_sys::HtmlCollection>,
  current_anr: [hp_classic::Register; 7],
  current_data: [hp_classic::Register; 10],
  current_status: Indexer16,
}

impl<const EXTRA_REGS: usize> SidePanel<EXTRA_REGS> {
  
  pub fn new() -> Self {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    Self {
      cnt: get_tr_list(&document, "cnt"),
      anr: get_tr_list(&document, "anr"),
      ram: get_tr_list(&document, "ram"),
      status: get_tr_list(&document, "status"),
      current_anr: Default::default(),
      current_data: Default::default(),
      current_status: Default::default(),
    }
  }
  
  pub fn run_refresh_cycle(&mut self, board: &hp_classic::Board<EXTRA_REGS>) {
    self.print_cnt(&board.cnt);
    self.print_status(&board.cnt);
    self.print_anr(&board.anr);
    self.print_datastorage(&board.ram);
  }
  
  fn print_anr(&mut self, anr: &hp_classic::anr::AnR) {
    if let Some(tr_list) = &self.anr {
      print_reg(&mut self.current_anr[0], tr_list, anr.a, 1);
      print_reg(&mut self.current_anr[1], tr_list, anr.b, 2);
      print_reg(&mut self.current_anr[2], tr_list, anr.c, 3);
      print_reg(&mut self.current_anr[3], tr_list, anr.d, 4);
      print_reg(&mut self.current_anr[4], tr_list, anr.e, 5);
      print_reg(&mut self.current_anr[5], tr_list, anr.f, 6);
      print_reg(&mut self.current_anr[6], tr_list, anr.m, 7);
    }
  }
  
  fn print_datastorage(&mut self, ram: &hp_classic::ram::RAM<EXTRA_REGS>) {
    if let Some(tr_list) = &self.ram {
      let mut row_index = 1;
      for i in 0..ram.regs.len() {
        print_reg(&mut self.current_data[i], tr_list, ram.regs[i], row_index);
        row_index += 1;
      }
    }
  }
  
  fn print_cnt(&self, cnt: &hp_classic::cnt::CnT) {
    if let Some(tr_list) = &self.cnt {
      let td_list = tr_list.item(1).expect("can't get tr").children();
      if let Some(td) = td_list.item(1) {
        td.set_text_content(Some(&format!("{:04o}", cnt.next_address)));
      }
      let td_list = tr_list.item(2).expect("can't get tr").children();
      if let Some(td) = td_list.item(1) {
        td.set_text_content(Some(&format!("{:04o}", cnt.saved_address)));
      }
      let td_list = tr_list.item(3).expect("can't get tr").children();
      if let Some(td) = td_list.item(1) {
        td.set_text_content(Some(&format!("{:X}", cnt.pointer)));
      }
    }
  }
  
  fn print_status(&mut self, cnt: &hp_classic::cnt::CnT) {
    if self.current_status != cnt.status {
      if let Some(status) = &self.status {
        let td_list = status.item(1).expect("can't get tr").children();
        for i in 0..12 {
          if let Some(td) = td_list.item(i as u32 + 1) {
            if cnt.status.read_bit(i) {
              td.set_text_content(Some("●"));
            } else {
              td.set_text_content(Some("○"));
            }
          }
        }
        self.current_status = cnt.status;
      }
    }
  }
}

fn print_reg(current_reg: &mut hp_classic::Register, tr_list: &web_sys::HtmlCollection, mut new_reg: hp_classic::Register, row_index: u32) {
  if new_reg != *current_reg {
    let td_list = tr_list.item(row_index).expect("can't get tr").children();
    for i in 0..14 {
      let col_index = i as u32 + 2;
      let nibble = new_reg.read_nibble(shifter::Direction::Left);
      if let Some(td) = td_list.item(col_index) {
        td.set_text_content(Some(&format!("{:X}", nibble)));
      }
      new_reg.shift_with_nibble(shifter::Direction::Left, nibble);
    }
    *current_reg = new_reg.clone();
  }
}

fn get_tr_list(document: &web_sys::Document, table_id: &str) -> Option<web_sys::HtmlCollection> {
  if let Some(table) = document.get_element_by_id(table_id) {
    if let Some(tbody) = table.children().item(0) {
      return Some(tbody.children())
    }
  }
  None
}
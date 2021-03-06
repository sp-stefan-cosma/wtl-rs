#![allow(dead_code)]
use wtl::*;
use ui::consts::*;

pub struct MainDialog<T> {
	pub this: Dialog<T>,
	
	pub lst_all_dlgs: CListBox,
    pub tree_selected_dlgs: CTreeViewCtrlEx,
    pub btn_select: CButton,
    pub btn_unselect: CButton,
    pub btn_parse: CButton,
    pub edt_rc_path: CEdit,
    pub btn_generate: CButton,
    pub edt_dlg_name: CEdit,
}
impl<T> MainDialog<T> {
    
	pub fn new()->MainDialog<T>{
		MainDialog{
			this: Dialog::new(IDD_MAIN_DIALOG),
			
            lst_all_dlgs: CListBox::new(),
            tree_selected_dlgs: CTreeViewCtrlEx::new(),
            btn_select: CButton::new(),
            btn_unselect: CButton::new(),
            btn_parse: CButton::new(),
            edt_rc_path: CEdit::new(),
            btn_generate: CButton::new(),
            edt_dlg_name: CEdit::new(),
		}
	}
    
    pub fn create(&mut self,r: *mut T){
        self.this.Create3(r);
        
    }
    
    pub fn this_msg(&mut self)->DlgMsg<T> {
        self.this.msg_handler()
    }
    // pub fn lst_all_dlgs_msg(&mut self)->LstBxMsg<T> {
    //     self.this.lstbx_handler(IDC_LST_ALL_DLGS)
    // }
    // pub fn tree_selected_dlgs_msg(&mut self)->TreeMsg<T> {
    //     self.this.tree_handler(IDC_TREE_SELECTED_DLGS)
    // }
    pub fn btn_select_msg(&mut self)->BtnMsg<T> {
        self.this.btn_handler(IDC_BTN_SELECT)
    }
    pub fn btn_unselect_msg(&mut self)->BtnMsg<T> {
        self.this.btn_handler(IDC_BTN_UNSELECT)
    }
    pub fn btn_parse_msg(&mut self)->BtnMsg<T> {
        self.this.btn_handler(IDC_BTN_PARSE)
    }
    // pub fn edt_rc_path_msg(&mut self)->EdtMsg<T> {
    //     self.this.edt_handler(IDC_EDT_RC_PATH)
    // }
    pub fn btn_generate_msg(&mut self)->BtnMsg<T> {
        self.this.btn_handler(IDC_BTN_GENERATE)
    }
    // pub fn edt_dlg_name_msg(&mut self)->EdtMsg<T> {
    //     self.this.edt_handler(IDC_EDT_DLG_NAME)
    // }
    
}
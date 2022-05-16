use windows::{
    core::*, 
    Win32::Foundation::*, 
    Win32::System::LibraryLoader::GetModuleHandleW, 
    Win32::UI::WindowsAndMessaging::*
};
use uuid::Uuid;
use std::rc::Rc;

pub struct Window {
    handle: Option<HWND>,
    class_str: String,
    atom: Option<u16>,
    instance: Option<HINSTANCE>,
    initialized: bool,
}

impl Window {
    unsafe fn on_destroy(&self, hwnd: HWND, wp: WPARAM, lp: LPARAM) -> LRESULT {
        DefWindowProcA(hwnd, WM_DESTROY, wp, lp)
    }

    unsafe fn on_nccreate(&self, hwnd: HWND, wp: WPARAM, lp: LPARAM) -> LRESULT {
        DefWindowProcA(hwnd, WM_DESTROY, wp, lp)
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.handle.clone().unwrap(), SW_NORMAL);
        }
    }

    unsafe extern "system" fn wndproc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        //let win = Box::from_raw(GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut Window); //std::mem::transmute(GetWindowLongPtrA(hwnd, GWLP_USERDATA));
        match message {
            WM_CLOSE => {
                DestroyWindow(hwnd);
                LRESULT(0)
            },
            WM_DESTROY => { 
                PostQuitMessage(0); 
                LRESULT(0) 
            },
            //WM_NCCREATE => (*win).on_nccreate(hwnd, wparam, lparam),
            _ => DefWindowProcA(hwnd, message, wparam, lparam)
        }
    }

    pub fn initialize(&mut self) -> core::result::Result<(), &str> {
        unsafe {
            if self.initialized {
                return Err("Window already initialized");
            }

            self.instance = match GetModuleHandleW(None) {
                Ok(i) => Some(i),
                Err(_) => None,
            };

            self.class_str = Uuid::new_v4().hyphenated().to_string();
            
            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: self.instance.clone().unwrap(),
                lpszClassName: PCSTR(self.class_str.clone().as_ptr() ),
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Window::wndproc),
                ..Default::default()            
            };

            self.atom = Some(RegisterClassA(&wc));

            self.handle = Some(CreateWindowExA(Default::default(),
                                                    PCSTR(self.class_str.clone().as_ptr()),
                                                    "Sample Window",
                                                    WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                                                    CW_USEDEFAULT,
                                                    CW_USEDEFAULT,
                                                    CW_USEDEFAULT,
                                                    CW_USEDEFAULT,
                                                    None,
                                                    None,
                                                    self.instance.clone().unwrap(),
                                                    std::ptr::null()
                                                ));
            
            let class_ptr: isize = std::mem::transmute(&self);
            println!("{:?}", class_ptr);
            SetWindowLongPtrA(self.handle.clone().unwrap(), GWLP_USERDATA, class_ptr);

            self.initialized = true;

            Ok(())
        }
    }

    pub fn new() -> Self {
        Self { handle: None, 
               class_str: String::new(), 
               atom: None, 
               instance: None, 
               initialized: false,
             }
    }

    pub fn process_messages(&self) {
        let mut msg = MSG::default();

        loop {
            unsafe {
                let gm_result = GetMessageA(&mut msg, HWND(0), 0, 0);
                if gm_result != BOOL(0i32) {
                    TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                }
                else
                {
                    //let a = GetLastError();
                    println!("WM_QUIT");
                    break;
                }
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if let Some(h) = self.handle {
            unsafe {
                DestroyWindow(h);
            }
        }

        if let Some(_) = self.atom {
            unsafe {
                UnregisterClassA(PCSTR(self.class_str.as_ptr()), self.instance.unwrap());
            }
        }
    }
}
#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
use std::os::raw::{c_void, c_int, c_uchar};
use std::mem::transmute;

#[macro_export]
macro_rules! connect_no_args {
    ($sender:expr, $signal:expr, $data:expr, $call_type:ident, $callback:path) => {
        {
            extern "C" fn temp_call(target: *mut c_void) {
                unsafe {
                    let app = target as *mut $call_type; 
                    $callback(&mut *app);
                }
            }

            unsafe {
                let object = (*(*$sender.widget).base).o;
                connect(object, $signal, $data, temp_call);
            }
        }
    }
}

struct MyTool {
    button: PushButton,
    some_value: i32,
}

impl MyTool {
    fn new() -> Box<MyTool> {
        Box::new(MyTool {
            button: PushButton::new(),
            some_value: 42,
        })
    }

    fn button_callback(&mut self) {
        self.some_value += 1;
        println!("button callback, {}", self.some_value);
    }

    fn some_init_code(&self) {
    }
}


fn main() {
    let app = Application::new();

    let mut tool = MyTool::new();

    connect_no_args!(&tool.button, push_button::RELEASED, &mut *tool, MyTool, MyTool::button_callback); 

    app.run();
}

pub mod push_button {
    pub const RELEASED: &'static [u8] = b"2released()\0";
}

pub struct Application {
    app: *const GUApplication,
}

pub struct PushButton {
    widget: *const GUPushButton,
}

impl PushButton {
    fn new() -> PushButton {
        unsafe {
            let ui = wrui_get();
            PushButton {
                widget: ((*ui).push_button_create)(b"Test\0" as *const u8),
            }
        }
    }
}

impl Application {
    fn new() -> Application {
        unsafe {
            let ui = wrui_get();
            Application {
                app: ((*ui).application_create)(),
            }
        }
    }

    fn run(&self) {
        unsafe {
            ((*self.app).run)((*self.app).p);
        }
    }
}



pub fn connect<D>(object: *const GUObject, signal: &[u8], data: &D, fun: extern fn(*mut c_void)) {
    unsafe {
        ((*object).connect)(object as *const c_void, signal.as_ptr(), transmute(data), fun);
    }
}


/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUObject {
    pub p: *mut c_void,
    //pub connect: extern "C" fn(sender: *const c_void, id: *const c_uchar, reciver: *mut c_void, func: *mut c_void) -> c_int,
    pub connect: extern "C" fn(sender: *const c_void, id: *const c_uchar, reciver: *mut c_void, 
                               func: extern fn (*mut c_void)) -> c_int,
}
impl ::std::default::Default for GUObject {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUWidget {
    pub o: *mut GUObject,
    pub set_size: ::std::option::Option<unsafe extern "C" fn(widget:
                                                                 *mut GUWidget,
                                                             width:
                                                                 c_int,
                                                             height:
                                                                 c_int)>,
    pub set_parent: ::std::option::Option<unsafe extern "C" fn(widget:
                                                                   *mut GUWidget,
                                                               width:
                                                                   c_int,
                                                               height:
                                                                   c_int)>,
}
impl ::std::default::Default for GUWidget {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUWindow {
    pub base: *mut GUWidget,
}
impl ::std::default::Default for GUWindow {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type GUIWindow = GUWindow;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUPushButton {
    pub base: *mut GUWidget,
    pub set_default: ::std::option::Option<unsafe extern "C" fn(button:
                                                                    *mut GUPushButton,
                                                                state:
                                                                    c_int)>,
}
impl ::std::default::Default for GUPushButton {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUApplication {
    pub p: *mut c_void,
    pub run: unsafe extern "C" fn(p: *mut c_void) -> c_int,
}
impl ::std::default::Default for GUApplication {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Wrui {
    pub application_create: extern "C" fn() -> *const GUApplication,
    pub window_create: extern "C" fn() -> *mut GUIWindow,
    pub push_button_create: extern "C" fn(label: *const c_uchar) -> *const GUPushButton,
}
impl ::std::default::Default for Wrui {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub fn wrui_get() -> *mut Wrui;
}

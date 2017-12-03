use std::mem;
use std::os::raw::c_void;

trait Draw {
    fn to_html(&self) -> String;
}

#[derive(Debug)]
pub struct Color {
    r: u32,
    g: u32,
    b: u32
}

pub struct Screen {
    bg_color: Color,
    children: Vec<Box<Draw>>
}
impl Draw for Screen {
    fn to_html(&self) -> String {
        let mut s = String::new();
        for child in self.children.iter() {
            s += child.to_html().as_str();
        }
        s
    }
}

#[derive(Debug)]
pub struct Header1 {
    text: String
}
impl Draw for Header1 {
    fn to_html(&self) -> String {
        let mut s = String::from("<h1>");
        s.push_str(self.text.as_str());
        s += "</h1>";
        s
    }
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

extern "C" {
    pub fn test_js(arg: i32);
    pub fn draw_html(html: String);
}

#[no_mangle]
pub fn browser() {
    let screen = Screen {
        bg_color: Color {
            r: 240,
            g: 240,
            b: 255
        },
        children: vec![
            Box::new(Header1 {
                text: "Header".to_string()
            })
        ]
    };
    unsafe {
        test_js(123);
        draw_html(screen.to_html());
    }
    println!("Html: {}", screen.to_html());
}

use std::mem;
use std::os::raw::c_void;

trait Draw {
    fn to_html(&self, s: String) -> String;
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
    fn to_html(&self, mut s: String) -> String {
        for child in self.children.iter() {
            s = child.to_html(s);
        }
        s
    }
}

#[derive(Debug)]
pub struct Header1 {
    text: String
}
impl Draw for Header1 {
    fn to_html(&self, mut s: String) -> String {
        s += "<h1>";
        s.push_str(self.text.as_str());
        s += "</h1>";
        s
    }
}

pub struct NavLink {
    text: String,
    id: String
}
impl Draw for NavLink {
    fn to_html(&self, mut s: String) -> String {
        s += "<a href=\"#";
        s.push_str(self.id.as_str());
        s.push_str("\">");
        s.push_str(self.text.as_str());
        s.push_str("</a>");
        s
    }
}

struct NavList {
    links: Vec<NavLink>
}
impl Draw for NavList {
    fn to_html(&self, mut s: String) -> String {
        s += "<ol>";
        for link in &self.links {
            s += "<li>";
            s = link.to_html(s);
            s += "</li>";
        }
        s += "</ol>";
        s
    }
}

struct Counter {
    count: u32
}
enum CounterEvent {
    Increment
}
trait HandleEvent {
    fn handle_event(&mut self, event: CounterEvent);
}
impl HandleEvent for Counter {
    fn handle_event(&mut self, event: CounterEvent) {
        match event {
            CounterEvent::Increment => {
                self.count += 1;
            }
        }
    }
}
impl Draw for Counter {
    fn to_html(&self, mut s: String) -> String {
        s += "Count is: ";
        s += &format!("{}", self.count);
        s += " <a href=\"#\" onclick=\"\">Increment</a>";
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

struct Data {
    val: i32
}
impl Data {
    pub fn test(&mut self) {
        self.val += 1;
        println!("val {}", self.val);
    }
}

#[no_mangle]
pub fn browser() {
    let mut d = Data { val: 2 };
    let mut comps = Vec::new();
    let test = &mut d as *mut _ as *mut c_void;
    comps.push(test);
    for ptr in &comps {
        let d2: &mut Data = unsafe { &mut *(*ptr as *mut Data) };
        println!("d2: {}", d2.val);
    }

    let screen = Screen {
        bg_color: Color {
            r: 240,
            g: 240,
            b: 255
        },
        children: vec![
            Box::new(Header1 {
                text: "Header".to_string()
            }),
            Box::new(NavList {
                links: vec![
                    NavLink { text: "Link1".to_string(), id: "link1".to_string() },
                    NavLink { text: "Link2".to_string(), id: "link2".to_string() },
                    NavLink { text: "Link3".to_string(), id: "link3".to_string() },
                ]
            }),
            Box::new(Counter {
                count: 1
            })
        ]
    };
    unsafe {
        test_js(123);
        let mut s = String::with_capacity(256);
        s = screen.to_html(s);
        draw_html(s);
    }
}

use x11::xlib;

mod layouts;

struct Window {
    xid: xlib::Window,
    tags: u8,
    full: bool,
    float: bool,
}
const MOD: u32 = xlib::Mod4Mask;
const MOD_SHIFT: u32 = MOD | xlib::ShiftMask;
const MOD_CTRL: u32 = MOD | xlib::ControlMask;
const MOD_CTRL_SHIFT: u32 = MOD | xlib::ShiftMask | xlib::ControlMask;
const KEYS_TO_GRAB: [(u32, u32); 39] = [
    (MOD, x11::keysym::XK_d),
    (MOD_SHIFT, x11::keysym::XK_q),
    (MOD_SHIFT, x11::keysym::XK_c),
    (MOD, x11::keysym::XK_0),
    (MOD, x11::keysym::XK_1),
    (MOD, x11::keysym::XK_2),
    (MOD, x11::keysym::XK_3),
    (MOD, x11::keysym::XK_4),
    (MOD, x11::keysym::XK_5),
    (MOD, x11::keysym::XK_6),
    (MOD, x11::keysym::XK_7),
    (MOD, x11::keysym::XK_8),
    (MOD_CTRL, x11::keysym::XK_0),
    (MOD_CTRL, x11::keysym::XK_1),
    (MOD_CTRL, x11::keysym::XK_2),
    (MOD_CTRL, x11::keysym::XK_3),
    (MOD_CTRL, x11::keysym::XK_4),
    (MOD_CTRL, x11::keysym::XK_5),
    (MOD_CTRL, x11::keysym::XK_6),
    (MOD_CTRL, x11::keysym::XK_7),
    (MOD_CTRL, x11::keysym::XK_8),
    (MOD_SHIFT, x11::keysym::XK_0),
    (MOD_SHIFT, x11::keysym::XK_1),
    (MOD_SHIFT, x11::keysym::XK_2),
    (MOD_SHIFT, x11::keysym::XK_3),
    (MOD_SHIFT, x11::keysym::XK_4),
    (MOD_SHIFT, x11::keysym::XK_5),
    (MOD_SHIFT, x11::keysym::XK_6),
    (MOD_SHIFT, x11::keysym::XK_7),
    (MOD_SHIFT, x11::keysym::XK_8),
    (MOD_CTRL_SHIFT, x11::keysym::XK_0),
    (MOD_CTRL_SHIFT, x11::keysym::XK_1),
    (MOD_CTRL_SHIFT, x11::keysym::XK_2),
    (MOD_CTRL_SHIFT, x11::keysym::XK_3),
    (MOD_CTRL_SHIFT, x11::keysym::XK_4),
    (MOD_CTRL_SHIFT, x11::keysym::XK_5),
    (MOD_CTRL_SHIFT, x11::keysym::XK_6),
    (MOD_CTRL_SHIFT, x11::keysym::XK_7),
    (MOD_CTRL_SHIFT, x11::keysym::XK_8),
];
struct WindowManager {
    display: *mut xlib::Display,
    root_window: xlib::Window,
    size: (u32, u32),
    windows: Vec<Window>,
    tags: u8,
}
impl WindowManager {
    fn grab_keys(&self) {
        unsafe {
            xlib::XUngrabKey(
                self.display,
                xlib::AnyKey,
                xlib::AnyModifier,
                self.root_window,
            );
        }
        for i in KEYS_TO_GRAB.iter() {
            unsafe {
                xlib::XGrabKey(
                    self.display,
                    xlib::XKeysymToKeycode(self.display, i.1 as u64) as i32,
                    i.0,
                    self.root_window,
                    0,
                    xlib::GrabModeAsync,
                    xlib::GrabModeAsync,
                );
            }
        }
    }

    fn reconfigure(&self) {
        let to_display: Vec<u64> = self
            .windows
            .iter()
            .filter(|w| w.tags & self.tags != 0)
            .map(|w| w.xid)
            .collect();
        let confs = layouts::get_layout(to_display.len(), self.size);
        for i in 0..to_display.len() {
            println!("window to display: {}", to_display[i]);
            unsafe {
                xlib::XMoveResizeWindow(
                    self.display,
                    to_display[i],
                    confs[i].x,
                    confs[i].y,
                    confs[i].w,
                    confs[i].h,
                );
            }
        }
        self.windows
            .iter()
            .filter(|w| w.tags & self.tags == 0)
            .for_each(|w| unsafe {
                xlib::XMoveWindow(self.display, w.xid, (self.size.0 * 2) as i32, 0);
            });
    }

    fn new() -> WindowManager {
        let display: *mut xlib::Display = unsafe { xlib::XOpenDisplay(std::ptr::null()) };
        let root_window: xlib::Window = unsafe { xlib::XDefaultRootWindow(display) };
        let screen = unsafe { xlib::XDefaultScreen(display) };
        let w = unsafe { xlib::XDisplayWidth(display, screen) } as u32;
        let h = unsafe { xlib::XDisplayHeight(display, screen) } as u32;
        WindowManager {
            display: display,
            root_window: root_window,
            size: (w, h),
            windows: Vec::<Window>::new(),
            tags: 1,
        }
    }

    fn run(&mut self) {
        unsafe {
            xlib::XSelectInput(
                self.display,
                self.root_window,
                xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask,
            );
        }

        self.grab_keys();

        let mut e: xlib::XEvent = xlib::XEvent { type_: 0 };
        loop {
            unsafe {
                xlib::XNextEvent(self.display, &mut e);
            }
            //println!("\n{:?}", e);
            match unsafe { e.type_ } {
                xlib::KeyPress => {
                    match unsafe {
                        (
                            e.key.state,
                            xlib::XKeycodeToKeysym(self.display, e.key.keycode as u8, 0) as u32,
                        )
                    } {
                        (MOD, x11::keysym::XK_d) => {
                            std::process::Command::new("dmenu_run")
                                .spawn()
                                .expect(&format!("could not spawn dmenu, line {}", line!()));
                        }
                        (MOD_SHIFT, x11::keysym::XK_q) => {
                            return;
                        }
                        (MOD_SHIFT, x11::keysym::XK_c) => {
                            println!("killing {}", unsafe { e.key.subwindow });
                            unsafe {
                                if e.key.subwindow != 0 {
                                    xlib::XKillClient(self.display, e.key.subwindow);
                                }
                            }
                        }
                        (MOD, num @ x11::keysym::XK_1..=x11::keysym::XK_8) => {
                            self.tags = 1 << (num - x11::keysym::XK_1);
                            self.reconfigure();
                        }
                        (MOD_CTRL, num @ x11::keysym::XK_1..=x11::keysym::XK_8) => {
                            self.tags |= 1 << (num - x11::keysym::XK_1);
                            self.reconfigure();
                        }
                        (MOD_SHIFT, num @ x11::keysym::XK_1..=x11::keysym::XK_8) => {
                            self.windows
                                .iter_mut()
                                .filter(|w| w.xid == unsafe { e.key.subwindow })
                                .for_each(|w| w.tags = 1 << (num - x11::keysym::XK_1));
                            self.reconfigure();
                        }
                        (MOD_CTRL_SHIFT, num @ x11::keysym::XK_1..=x11::keysym::XK_8) => {
                            self.windows
                                .iter_mut()
                                .filter(|w| w.xid == unsafe { e.key.subwindow })
                                .for_each(|w| w.tags |= 1 << (num - x11::keysym::XK_1));
                            self.reconfigure();
                        }
                        _ => {}
                    }
                }
                xlib::MapRequest => {
                    self.windows.push(Window {
                        xid: unsafe { e.map_request.window },
                        tags: 1,
                        full: false,
                        float: false,
                    });
                    println!("Managing window {}", unsafe { e.map_request.window });
                    unsafe {
                        xlib::XMapWindow(self.display, e.map_request.window);
                    }
                    self.reconfigure();
                }
                xlib::UnmapNotify => {
                    println!("Removing window {}", unsafe { e.unmap.window });
                    match self
                        .windows
                        .iter()
                        .position(|x| x.xid == unsafe { e.unmap.window })
                    {
                        Some(index) => {
                            self.windows.remove(index);
                        }
                        _ => {}
                    }
                    self.reconfigure();
                }
                xlib::MappingNotify => {
                    self.grab_keys();
                }
                xlib::CreateNotify => {}
                _ => {}
            }
        }
    }
}

fn main() {
    let mut wm = WindowManager::new();
    wm.run();
}

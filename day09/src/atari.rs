use crate::display::DisplayInterface;
use a800xl_utils::{consts, screen::clear_atract};

#[derive(Clone, Copy)]
struct DisplayListLine {
    pub mode: u8,
    pub addr: *const u8,
}

impl Default for DisplayListLine {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            addr: 0 as *const u8,
        }
    }
}
const SCREEN_MARGIN: usize = 8;

const SCREEN_MODE: u8 = 0x0d;
const SCREEN_HEIGHT: usize = 100;
const SCREEN_WIDTH: usize = 128;
const SCREEN_BYTE_WIDTH: usize = 4;
// const SCREEN_MODE: u8 = 0x0a;
// const SCREEN_HEIGHT: usize = 48;
// const SCREEN_WIDTH: usize = 64;
// const SCREEN_BYTE_WIDTH: usize = 4;

// #[repr(C, align(512))]
struct DisplayList {
    pub _header: [u8; 3],
    pub lines: [DisplayListLine; 100],
    pub jmp: DisplayListLine,
}

impl DisplayList {
    pub fn update(&mut self, memory_addr: *const u8, width: usize) {
        let mut addr = memory_addr;
        for line in self.lines.iter_mut() {
            line.mode = 0x40 + SCREEN_MODE;
            line.addr = addr;
            addr = unsafe { addr.add(width) };
        }
        self.jmp.addr = self as *const DisplayList as *const u8;
    }
}

impl Default for DisplayList {
    fn default() -> Self {
        Self {
            _header: [0x70, 0x70, 0x70],
            lines: [Default::default(); 100],
            jmp: DisplayListLine {
                mode: 0x41,
                addr: 0 as *const u8,
            },
        }
    }
}

pub struct Display {
    dlist: DisplayList,
    stored_dlist: *mut u8,
    stored_sdmctl: u8,
    stored_colors: [u8; 5],
    width: usize,
    height: usize,
    ptr: *const u8,
    dx: usize,
    dy: usize,
}

impl DisplayInterface for Display {
    fn init(width: usize, height: usize, ptr: *const u8) -> Self {
        let mut disp = Self {
            dlist: Default::default(),
            stored_dlist: unsafe { *consts::SDLST },
            stored_sdmctl: unsafe { *consts::SDMCTL },
            stored_colors: [0; 5],
            width,
            height,
            ptr,
            dx: Default::default(),
            dy: Default::default(),
        };
        disp.stored_colors
            .copy_from_slice(unsafe { core::slice::from_raw_parts(consts::COLOR0, 5) });
        disp.show();
        disp
    }

    fn show(&mut self) {
        self.dx = 0;
        self.dy = 0;
        self.dlist.update(self.ptr, self.width / SCREEN_BYTE_WIDTH);
        unsafe {
            *consts::SDLST = self.dlist.jmp.addr as *mut u8;
            *consts::SDMCTL = 0x21;
            *consts::COLOR0 = 0x04;
            *consts::COLOR1 = 0xb8;
            *consts::COLOR2 = 0xb6;
        };
    }

    fn scroll_to(&mut self, x: usize, y: usize) {
        let prev = (self.dx, self.dy);

        if x > SCREEN_WIDTH + self.dx - SCREEN_MARGIN {
            self.dx = (self.dx + 4).min(self.width - SCREEN_WIDTH);
        }
        if x < self.dx + SCREEN_MARGIN && self.dx > 0 {
            self.dx -= 4;
        }
        if y > SCREEN_HEIGHT + self.dy - SCREEN_MARGIN {
            self.dy = (self.dy + 4).max(0).min(self.height - SCREEN_HEIGHT);
        }
        if y < self.dy + SCREEN_MARGIN && self.dy > 0 {
            self.dy -= 4;
        }
        if (self.dx, self.dy) != prev {
            self.dlist.update(
                unsafe {
                    self.ptr
                        .add((self.dx + self.width * self.dy) / SCREEN_BYTE_WIDTH)
                },
                self.width / SCREEN_BYTE_WIDTH,
            );
            clear_atract();
        }
    }

    fn hide(&self) {
        unsafe {
            *consts::SDLST = self.stored_dlist;
            *consts::SDMCTL = self.stored_sdmctl;
            core::slice::from_raw_parts_mut(consts::COLOR0, 5).copy_from_slice(&self.stored_colors);
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        self.hide();
    }
}

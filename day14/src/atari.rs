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

#[repr(C, align(256))]
struct DisplayList {
    pub _header: [u8; 3],
    pub load_mem1: DisplayListLine,
    pub mode1: [u8; 99],
    pub load_mem2: DisplayListLine,
    pub mode2: [u8; 99],
    pub jmp: DisplayListLine,
}

struct VideoMemory {
    data: [u8; 8192 + 4096],
}

impl Default for VideoMemory {
    fn default() -> Self {
        Self {
            data: [0; 8192 + 4096],
        }
    }
}

impl DisplayList {
    pub fn update(&mut self, memory_addr: *const u8) {
        self.load_mem1.mode = 0x40 + 0xf;
        self.load_mem2.mode = 0x40 + 0xf;
        self.load_mem1.addr = memory_addr;
        self.load_mem2.addr = unsafe { memory_addr.add(40 * 100) };
        self.jmp.addr = self as *const DisplayList as *const u8;
    }
}

impl Default for DisplayList {
    fn default() -> Self {
        Self {
            _header: [0x70, 0x70, 0x70],
            load_mem1: Default::default(),
            load_mem2: Default::default(),
            mode1: [15; 99],
            mode2: [15; 99],
            jmp: DisplayListLine {
                mode: 0x41,
                addr: 0 as *const u8,
            },
        }
    }
}

pub struct Display {
    dlist: DisplayList,
    video_memory: VideoMemory,
    saved_dlist: *mut u8,
    saved_sdmctl: u8,
    saved_colors: [u8; 5],
    // width: usize,
    // height: usize,
    dx: usize,
    dy: usize,
}

impl DisplayInterface for Display {
    fn init() -> Self {
        let mut disp = Self {
            video_memory: VideoMemory::default(),
            dlist: Default::default(),
            saved_dlist: unsafe { *consts::SDLST },
            saved_sdmctl: unsafe { *consts::SDMCTL },
            saved_colors: [0; 5],
            // width,
            // height,
            dx: Default::default(),
            dy: Default::default(),
        };
        disp.saved_colors
            .copy_from_slice(unsafe { core::slice::from_raw_parts(consts::COLOR0, 5) });
        disp.show();
        disp
    }

    fn get_screen_memory(&mut self) -> *mut u8 {
        (((self.video_memory.data.as_ptr() as usize + 4095) & !4095) + 96) as *mut u8
    }

    fn show(&mut self) {
        self.dx = 0;
        self.dy = 0;
        let mem = self.get_screen_memory();
        self.dlist.update(mem);

        unsafe {
            *consts::SDLST = self.dlist.jmp.addr as *mut u8;
            *consts::SDMCTL = 0x22;
            *consts::COLOR0 = 0x04;
            *consts::COLOR1 = 0xb8;
            *consts::COLOR2 = 0xf0;
        };
    }
    fn hide(&self) {
        unsafe {
            *consts::SDLST = self.saved_dlist;
            *consts::SDMCTL = self.saved_sdmctl;
            core::slice::from_raw_parts_mut(consts::COLOR0, 5).copy_from_slice(&self.saved_colors);
        }
    }
    fn clear_atract(&self) {
        clear_atract();
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        self.hide();
    }
}

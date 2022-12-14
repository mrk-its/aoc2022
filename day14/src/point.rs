#[derive(PartialEq, Clone, Copy)]
pub struct Point {
    ptr: *mut u8,
    mask: u8,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.ptr.partial_cmp(&other.ptr) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        other.mask.partial_cmp(&self.mask)
    }
}

impl Point {
    pub fn from_coords(memory: *mut u8, &(x, y): &(i16, i16)) -> Self {
        let ptr = unsafe { memory.add(y as usize * 40 + x as usize / 8) };
        let mask = 0x80 >> (x as u8 & 7);
        Point { ptr, mask }
    }
    pub fn relative(&self, dx: isize, dy: isize) -> Point {
        let offs = if dy == 1 {
            40
        } else if dy == -1 {
            -40
        } else {
            0
        };
        let mut ptr = unsafe { self.ptr.offset(offs) };
        let mask = if dx == 1 {
            let mask = self.mask.rotate_right(1);
            if mask == 0x80 {
                ptr = unsafe { ptr.add(1) };
            }
            mask
        } else if dx == -1 {
            let mask = self.mask.rotate_left(1);
            if mask == 1 {
                ptr = unsafe { ptr.sub(1) };
            }
            mask
        } else {
            self.mask
        };
        Point { ptr, mask }
    }
    pub fn get(&self) -> bool {
        return unsafe { (*self.ptr & self.mask) > 0 };
    }
    pub fn clear(&mut self) {
        unsafe {
            *self.ptr &= !self.mask;
        }
    }
    pub fn set(&mut self) {
        //println!("set {:?}", self);
        unsafe {
            *self.ptr |= self.mask;
        }
    }
    pub fn line_to(&self, dst: &Point) {
        let mut dst = *dst;
        let mut start = *self;
        if start > dst {
            (start, dst) = (dst, start);
        }
        start.set();
        if start == dst {
            return;
        }
        let (dx, dy) = if (dst.ptr as usize) - (start.ptr as usize) < 40 {
            (1, 0)
        } else {
            (0, 1)
        };

        while start != dst {
            start = start.relative(dx, dy);
            start.set();
        }
    }
}

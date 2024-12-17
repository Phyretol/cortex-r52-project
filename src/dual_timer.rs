#![allow(unused)]

pub struct DualTimer<const ADDR: usize>();

impl DualTimer<0xe010_1000> {
    pub unsafe fn new_dual_timer() -> Self {
        DualTimer()
    }
}

impl<const ADDR: usize> DualTimer<ADDR> {
    const TIMER1_LOAD_OFFSET: usize = 0;
    const TIMER1_VALUE_OFFSET: usize = 1;
    const TIMER1_CONTROL_OFFSET: usize = 2;

    const ADDR_PTR: *mut u32 = ADDR as *mut u32;

    pub fn enable_timer1(&mut self) {
        unsafe {
            let ptr: *mut u32 = Self::ADDR_PTR.add(Self::TIMER1_CONTROL_OFFSET);
            let mut ctrl_value = ptr.read_volatile();
            ctrl_value |= 1 << 7; // enable
            ctrl_value |= 1 << 1; // 32 bit
            ctrl_value |= 1 << 0; // no wrap
            ptr.write_volatile(ctrl_value);
        }
    }

    pub fn set_timer1(&mut self, timer_value: u32) {
        unsafe {
            let ptr: *mut u32 = Self::ADDR_PTR.add(Self::TIMER1_LOAD_OFFSET);
            ptr.write_volatile(timer_value);
        }
    }

    pub fn get_timer1(&self) -> u32 {
        unsafe {
            let ptr: *mut u32 = Self::ADDR_PTR.add(Self::TIMER1_VALUE_OFFSET);
            ptr.read_volatile()
        }
    }
}
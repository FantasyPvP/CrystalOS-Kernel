#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(async_fn_in_trait)]
#![feature(global_asm)]

mod kernel;
pub mod std {
	pub use crate::std as std;
}

extern crate alloc;
use bootloader::{BootInfo};

pub fn init(boot_info: &'static BootInfo) {
	kernel::gdt::init();
	kernel::interrupts::init_idt();
	unsafe { kernel::interrupts::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();

    use x86_64::VirtAddr;

	let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { kernel::memory::init(physical_memory_offset) };
	let mut frame_allocator = unsafe {
		kernel::memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

    kernel::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialisation failed");
}
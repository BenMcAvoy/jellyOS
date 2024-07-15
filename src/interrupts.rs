use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::{println, vga_buffer::{WRITER, ColourCode, Colour}};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
) {
    {
        // Separate scope to avoid deadlocks where the WRITER lock is held and println! tries to acquire it
        let mut writer = WRITER.lock();
        writer.colour_code = ColourCode::new(Colour::White, Colour::Red);
        writer.column_position = 0;
    }

    println!("EXCEPTION: BREAKPOINT\n{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64
) -> ! {
    {
        // Separate scope to avoid deadlocks where the WRITER lock is held and println! tries to acquire it
        let mut writer = WRITER.lock();
        writer.colour_code = ColourCode::new(Colour::White, Colour::Red);
        writer.column_position = 0;
    }

    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

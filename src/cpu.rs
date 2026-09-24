// GENERAL CONSTANTS
const PC_INIT: u32 = 0x08000000; // default intialitation value of PC

// USER MODE CONSTANTS
const R0: usize = 0;
const R1: usize = 1;
const R2: usize = 2;
const R3: usize = 3;
const R4: usize = 4;
const R5: usize = 5;
const R6: usize = 6;
const R7: usize = 7;
const R8: usize = 8;
const R9: usize = 9;
const R10: usize = 10;
const R11: usize = 11;
const R12: usize = 12;
const SP: usize = 13;
const LR: usize = 14;
const PC: usize = 15;
const CPSR: usize = 16;

// FIQ MODE CONSTANTS
const R8_FIQ: usize = 17;
const R9_FIQ: usize = 18;
const R10_FIQ: usize = 19;
const R11_FIQ: usize = 20;
const R12_FIQ: usize = 21;
const SP_FIQ: usize = 22;
const LR_FIQ: usize = 23;
const SPSR_FIQ: usize = 24;

// SUPERVISOR MODE CONSTANTS
const SP_SVC: usize = 25;
const LR_SVC: usize = 26;
const SPSR_SVC: usize = 27;

// ABORT MODE CONSTANTS
const SP_ABT: usize = 28;
const LR_ABT: usize = 29;
const SPSR_ABT: usize = 30; 

// IRQ MODE CONSTANTS
const SP_IRQ: usize = 31;
const LR_IRQ: usize = 32;
const SPSR_IRQ: usize = 33;

// UNDEFINED MODE CONSTANTS
const SP_UND: usize = 34;
const LR_UND: usize = 35;
const SPSR_UND: usize = 36;

enum CPUMode {
    User,
    Fiq,
    Irq,
    Supervisor,
    Abort,
    Undefined,
    System,
}
struct CPU {
    internal_registers: [u32; 37],
    r0_index: usize,
    r1_index: usize,
    r2_index: usize,
    r3_index: usize,
    r4_index: usize,
    r5_index: usize,
    r6_index: usize,
    r7_index: usize,
    r8_index: usize,
    r9_index: usize,
    r10_index: usize,
    r11_index: usize,
    r12_index: usize,
    sp_index: usize,
    lr_index: usize,
    cpsr_index: usize,
    spsr_index: usize,
    mode: CPUMode,
}

impl CPU {
    fn new() -> Self {
        Self {
            internal_registers: {let mut internal_regs: [u32; 37] = [0b0; 37];
                                    internal_regs[PC] = PC_INIT;
                                    internal_regs},
            r0_index: R0,
            r1_index: R1,
            r2_index: R2,
            r3_index: R3,
            r4_index: R4,
            r5_index: R5,
            r6_index: R6,
            r7_index: R7,
            r8_index: R8,
            r9_index: R9,
            r10_index: R10,
            r11_index: R11,
            r12_index: R12,
            sp_index: SP,
            lr_index: LR,
            cpsr_index: CPSR,
            spsr_index: SPSR_FIQ,
            mode: CPUMode::User,
        }
    }

    fn switch_to_user(&mut self) {
        self.r8_index = R8;
        self.r9_index = R9;
        self.r10_index = R10;
        self.r11_index = R11;
        self.r12_index = R12;
        self.sp_index = SP;
        self.lr_index = LR;
        self.cpsr_index = CPSR;
        self.spsr_index = SPSR_FIQ;
        self.mode = CPUMode::User;
    }

    fn switch_to_fiq(&mut self) {
        self.r8_index = R8_FIQ;
        self.r9_index = R9_FIQ;
        self.r10_index = R10_FIQ;
        self.r11_index = R11_FIQ;
        self.r12_index = R12_FIQ;
        self.sp_index = SP_FIQ;
        self.lr_index = LR_FIQ;
        self.spsr_index = SPSR_FIQ;
        self.mode = CPUMode::Fiq;
    }

    fn switch_to_irq(&mut self) {
        self.sp_index = SP_IRQ;
        self.lr_index = LR_IRQ;
        self.spsr_index = SPSR_IRQ;
        self.mode = CPUMode::Irq;
    }

    fn switch_to_supervisor(&mut self) {
        self.sp_index = SP_SVC;
        self.lr_index = LR_SVC;
        self.spsr_index = SPSR_SVC;
        self.mode = CPUMode::Supervisor;
    }

    fn switch_to_abort(&mut self) {
        self.sp_index = SP_ABT;
        self.lr_index = LR_ABT;
        self.spsr_index = SPSR_ABT;
        self.mode = CPUMode::Abort;
    }

    fn switch_to_undefined(&mut self) {
        self.sp_index = SP_UND;
        self.lr_index = LR_UND;
        self.spsr_index = SPSR_UND;
        self.mode = CPUMode::Undefined;
    }

    fn switch_to_system(&mut self) {
        self.r8_index = R8;
        self.r9_index = R9;
        self.r10_index = R10;
        self.r11_index = R11;
        self.r12_index = R12;
        self.sp_index = SP;
        self.lr_index = LR;
        self.cpsr_index = CPSR;
        self.spsr_index = SPSR_FIQ;
        self.mode = CPUMode::System;
    }

    fn mode_switch(&mut self, mode: CPUMode) {
        match mode {
            CPUMode::User => self.switch_to_user(),
            CPUMode::Fiq => self.switch_to_fiq(),
            CPUMode::Irq => self.switch_to_irq(),
            CPUMode::Supervisor => self.switch_to_supervisor(),
            CPUMode::Abort => self.switch_to_abort(),
            CPUMode::Undefined => self.switch_to_undefined(),
            CPUMode::System => self.switch_to_system(),
        }
    }
}
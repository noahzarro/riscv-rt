#[allow(missing_docs)]
pub mod addr {

    pub struct MemoryMapper {
        base_address: *mut u8,
    }

    impl MemoryMapper {
        pub fn new(base_address: *mut u8) -> Self {
            Self {
                base_address: base_address,
            }
        }

        pub fn write(&self, reg_offset: isize, mask: u32, bitoffset: u32, value: u32) {
            unsafe {
                let reg_value =
                    core::ptr::read_volatile(self.base_address.offset(reg_offset) as *mut u32);
                let reg_value = (reg_value & !mask) | (value << bitoffset);
                core::ptr::write_volatile(
                    self.base_address.offset(reg_offset) as *mut u32,
                    reg_value,
                );
            }
        }

        pub fn write_byte(&self, reg_offset: isize, mask: u8, bitoffset: u8, value: u8) {
            unsafe {
                let reg_value = core::ptr::read_volatile((self.base_address).offset(reg_offset));
                let reg_value = (reg_value & !mask) | (value << bitoffset);
                core::ptr::write_volatile((self.base_address).offset(reg_offset), reg_value);
            }
        }

        pub fn read(&self, reg_offset: isize, mask: u32, bitoffset: u32) -> u32 {
            unsafe {
                let reg_value =
                    core::ptr::read_volatile(self.base_address.offset(reg_offset) as *mut u32);
                (reg_value & mask) >> bitoffset
            }
        }

        pub fn read_byte(&self, reg_offset: isize, mask: u8, bitoffset: u8) -> u8 {
            unsafe {
                let reg_value = core::ptr::read_volatile((self.base_address).offset(reg_offset));
                (reg_value & mask) >> bitoffset
            }
        }
    }

    /* CLIC Configuration */
    pub const CLICCFG_REG_OFFSET: isize = 0x0 as isize;
    pub const CLICCFG_NVBITS_MASK: u8 = 1;
    pub const CLICCFG_NVBITS_OFFSET: u8 = 0;
    pub const CLICCFG_NLBITS_MASK: u8 = 0x1E as u8;
    pub const CLICCFG_NLBITS_OFFSET: u8 = 1 as u8;
    pub const CLICCFG_NMBITS_MASK: u8 = 0x60 as u8;
    pub const CLICCFG_NMBITS_OFFSET: u8 = 5 as u8;

    /* CLIC Information */
    pub const CLICINFO_REG_OFFSET: isize = 0x4 as isize;
    pub const CLICINFO_NUM_INTERRUPT_MASK: u32 = 0x1FFF as u32;
    pub const CLICINFO_NUM_INTERRUPT_OFFSET: u32 = 0 as u32;

    pub const CLICINFO_VERSION_MASK: u32 = 0x1FE000 as u32;
    pub const CLICINFO_VERSION_OFFSET: u32 = 13 as u32;

    pub const CLICINFO_CLICINTCTLBITS_MASK: u32 = 0x1E00000 as u32;
    pub const CLICINFO_CLICINTCTLBITS_OFFSET: u32 = 21 as u32;

    pub const CLICINFO_NUM_TRIGGER_MASK: u32 = 0x7E000000 as u32;
    pub const CLICINFO_NUM_TRIGGER_OFFSET: u32 = 25 as u32;

    /* CLIC enable mnxti irq forwarding logic */
    /*
    pub const CLICXNXTICONF_REG_OFFSET: isize = 0x8 as isize;
    pub const CLICXNXTICONF_CLICXNXTICONF_BIT: u32 = 0 as  u32;
    */

    /* CLIC interrupt id pending */
    pub fn CLICINTIP_REG_OFFSET(id: u32) -> isize {
        (0x1000 + 0x10 * id) as isize
    }
    pub const CLICINTIP_CLICINTIP_BIT: u8 = 0;
    pub const CLICINTIP_CLICINTIP_MASK: u8 = 1;

    /* CLIC interrupt id enable */
    pub fn CLICINTIE_REG_OFFSET(id: u32) -> isize {
        (0x1004 + 0x10 * id) as isize
    }
    pub const CLICINTIE_CLICINTIE_BIT: u8 = 0;
    pub const CLICINTIE_CLICINTIE_MASK: u8 = 1;

    /* CLIC interrupt id attributes */
    pub fn CLICINTATTR_REG_OFFSET(id: u32) -> isize {
        (0x1008 + 0x10 * id) as isize
    }
    pub const CLICINTATTR_SHV_BIT: u8 = 0;
    pub const CLICINTATTR_SHV_MASK: u8 = 0x1;
    pub const CLICINTATTR_TRIG_MASK: u8 = 0x6;
    pub const CLICINTATTR_TRIG_OFFSET: u8 = 1;
    pub const CLICINTATTR_MODE_MASK: u8 = 0xC0;
    pub const CLICINTATTR_MODE_OFFSET: u8 = 6;

    pub const TRIG_LEVEL: u8 = 0;
    pub const TRIG_EDGE: u8 = 1;
    pub const TRIG_POSITIVE: u8 = 0 << 1;
    pub const TRIG_NEGATIVE: u8 = 1 << 1;

    /* CLIC interrupt id control */
    pub fn CLICINTCTL_REG_OFFSET(id: u32) -> isize {
        (0x100c + 0x10 * id) as isize
    }
    
    pub const CLICINTCTL_CLICINTCTL_MASK: u8 = 0xFF;
    pub const CLICINTCTL_CLICINTCTL_OFFSET: u8 = 0;


    /* Timers */
    
    pub const TIMER_CFG_LOW_REG_OFFSET: isize = 0x0;
    pub const TIMER_CFG_HIGH_REG_OFFSET: isize = 0x4;

    pub const TIMER_CNT_LOW_REG_OFFSET: isize = 0x8;
    pub const TIMER_CNT_HIGH_REG_OFFSET: isize = 0xc;

    pub const TIMER_CMP_LOW_REG_OFFSET: isize = 0x10;
    pub const TIMER_CMP_HIGH_REG_OFFSET: isize = 0x14;

    pub const TIMER_START_LOW_REG_OFFSET: isize = 0x18;
    pub const TIMER_START_HIGH_REG_OFFSET: isize = 0x1c;

    pub const TIMER_RESET_LOW_REG_OFFSET: isize = 0x20;
    pub const TIMER_RESET_HIGH_REG_OFFSET: isize = 0x24;

    /* Timer low enable configuration bitfield: - 1'b0: disabled - 1'b1: enabled
     * (access: R/W) */
    pub const TIMER_CFG_LO_ENABLE_BIT: u32 = 0;
    pub const TIMER_CFG_LO_ENABLE_WIDTH: u32 = 1;
    pub const TIMER_CFG_LO_ENABLE_MASK: u32 = 0x1;

    /* Timer low counter reset command bitfield. Cleared after Timer Low reset
     * execution. (access: R/W) */
    pub const TIMER_CFG_LO_RESET_BIT: u32 = 1;
    pub const TIMER_CFG_LO_RESET_WIDTH: u32 = 1;
    pub const TIMER_CFG_LO_RESET_MASK: u32 = 0x2;

    /* Timer low compare match interrupt enable configuration bitfield: - 1'b0:
     * disabled - 1'b1: enabled (access: R/W) */
    pub const TIMER_CFG_LO_IRQEN_BIT: u32 = 2;
    pub const TIMER_CFG_LO_IRQEN_WIDTH: u32 = 1;
    pub const TIMER_CFG_LO_IRQEN_MASK: u32 = 0x4;

    /* Timer low input event mask configuration bitfield: - 1'b0: disabled - 1'b1:
     * enabled (access: R/W) */
    pub const TIMER_CFG_LO_IEM_BIT: u32 = 3;
    pub const TIMER_CFG_LO_IEM_WIDTH: u32 = 1;
    pub const TIMER_CFG_LO_IEM_MASK: u32 = 0x8;

    /* Timer low continuous mode configuration bitfield: - 1'b0: Continue mode -
     * continue incrementing Timer low counter when compare match with CMP_LO
     * occurs. - 1'b1: Cycle mode - reset Timer low counter when compare match with
     * CMP_LO occurs. (access: R/W) */
    pub const TIMER_CFG_LO_MODE_BIT: u32 = 4;
    pub const TIMER_CFG_LO_MODE_WIDTH: u32 = 1;
    pub const TIMER_CFG_LO_MODE_MASK: u32 = 0x10;

    /* Timer low one shot configuration bitfield: - 1'b0: let Timer low enabled
     * counting when compare match with CMP_LO occurs. - 1'b1: disable Timer low
     * when compare match with CMP_LO occurs. (access: R/W) */
    pub const TIMER_CFG_LO_ONE_S_BIT: u32 = 5;
    pub const TIMER_CFG_LO_ONE_S_WIDTH: u32 = 1;
    pub const TIMER_CFG_LO_ONE_S_MASK: u32 = 0x20;

    /* Timer low prescaler enable configuration bitfield:- 1'b0: disabled - 1'b1:
     * enabled (access: R/W) */
    pub const TIMER_CFG_LO_PEN_BIT: u32 = 6;
    pub const TIMER_CFG_LO_PEN_WIDTH: u32 = 1;
    pub const TIMER_CFG_LO_PEN_MASK: u32 = 0x40;

    /* Timer low clock source configuration bitfield: - 1'b0: FLL or FLL+Prescaler -
     * 1'b1: Reference clock at 32kHz (access: R/W) */
    pub const TIMER_CFG_LO_CCFG_BIT: u32 = 7;
    pub const TIMER_CFG_LO_CCFG_WIDTH: u32 = 1;
    pub const TIMER_CFG_LO_CCFG_MASK: u32 = 0x80;

    /* Timer low prescaler value bitfield. Ftimer = Fclk / (1 + PRESC_VAL) (access:
     * R/W) */
    pub const TIMER_CFG_LO_PVAL_BIT: u32 = 8;
    pub const TIMER_CFG_LO_PVAL_WIDTH: u32 = 8;
    pub const TIMER_CFG_LO_PVAL_MASK: u32 = 0xff00;

    /* Timer low + Timer high 64bit cascaded mode configuration bitfield. (access:
     * R/W) */
    pub const TIMER_CFG_LO_CASC_BIT: u32 = 31;
    pub const TIMER_CFG_LO_CASC_WIDTH: u32 = 1;
    pub const TIMER_CFG_LO_CASC_MASK: u32 = 0x80000000;

    /* Timer high enable configuration bitfield: - 1'b0: disabled - 1'b1: enabled
     * (access: R/W) */
    pub const TIMER_CFG_HI_ENABLE_BIT: u32 = 0;
    pub const TIMER_CFG_HI_ENABLE_WIDTH: u32 = 1;
    pub const TIMER_CFG_HI_ENABLE_MASK: u32 = 0x1;

    /* Timer high counter reset command bitfield. Cleared after Timer high reset
     * execution. (access: W) */
    pub const TIMER_CFG_HI_RESET_BIT: u32 = 1;
    pub const TIMER_CFG_HI_RESET_WIDTH: u32 = 1;
    pub const TIMER_CFG_HI_RESET_MASK: u32 = 0x2;

    /* Timer high compare match interrupt enable configuration bitfield: - 1'b0:
     * disabled - 1'b1: enabled (access: R/W) */
    pub const TIMER_CFG_HI_IRQEN_BIT: u32 = 2;
    pub const TIMER_CFG_HI_IRQEN_WIDTH: u32 = 1;
    pub const TIMER_CFG_HI_IRQEN_MASK: u32 = 0x4;

    /* Timer high input event mask configuration bitfield: - 1'b0: disabled - 1'b1:
     * enabled (access: R/W) */
    pub const TIMER_CFG_HI_IEM_BIT: u32 = 3;
    pub const TIMER_CFG_HI_IEM_WIDTH: u32 = 1;
    pub const TIMER_CFG_HI_IEM_MASK: u32 = 0x8;

    /* Timer high continuous mode configuration bitfield: - 1'b0: Continue mode -
     * continue incrementing Timer high counter when compare match with CMP_LO
     * occurs. - 1'b1: Cycle mode - reset Timer high counter when compare match with
     * CMP_LO occurs. (access: R/W) */
    pub const TIMER_CFG_HI_MODE_BIT: u32 = 4;
    pub const TIMER_CFG_HI_MODE_WIDTH: u32 = 1;
    pub const TIMER_CFG_HI_MODE_MASK: u32 = 0x10;

    /* Timer high one shot configuration bitfield: - 1'b0: let Timer high enabled
     * counting when compare match with CMP_LO occurs. - 1'b1: disable Timer high
     * when compare match with CMP_LO occurs. (access: R/W) */
    pub const TIMER_CFG_HI_ONE_S_BIT: u32 = 5;
    pub const TIMER_CFG_HI_ONE_S_WIDTH: u32 = 1;
    pub const TIMER_CFG_HI_ONE_S_MASK: u32 = 0x20;

    /* Timer high prescaler enable configuration bitfield: - 1'b0: disabled - 1'b1:
     * enabled (access: R/W) */
    pub const TIMER_CFG_HI_PEN_BIT: u32 = 6;
    pub const TIMER_CFG_HI_PEN_WIDTH: u32 = 1;
    pub const TIMER_CFG_HI_PEN_MASK: u32 = 0x40;

    /* Timer high clock source configuration bitfield: - 1'b0: FLL or FLL+Prescaler
     * - 1'b1: Reference clock at 32kHz (access: R/W) */
    pub const TIMER_CFG_HI_CLKCFG_BIT: u32 = 7;
    pub const TIMER_CFG_HI_CLKCFG_WIDTH: u32 = 1;
    pub const TIMER_CFG_HI_CLKCFG_MASK: u32 = 0x80;

    /* Timer Low counter value bitfield. (access: R/W) */
    pub const TIMER_CNT_LO_CNT_LO_BIT: u32 = 0;
    pub const TIMER_CNT_LO_CNT_LO_WIDTH: u32 = 32;
    pub const TIMER_CNT_LO_CNT_LO_MASK: u32 = 0xffffffff;

    /* Timer High counter value bitfield. (access: R/W) */
    pub const TIMER_CNT_HI_CNT_HI_BIT: u32 = 0;
    pub const TIMER_CNT_HI_CNT_HI_WIDTH: u32 = 32;
    pub const TIMER_CNT_HI_CNT_HI_MASK: u32 = 0xffffffff;

    /* Timer Low comparator value bitfield. (access: R/W) */
    pub const TIMER_CMP_LO_CMP_LO_BIT: u32 = 0;
    pub const TIMER_CMP_LO_CMP_LO_WIDTH: u32 = 32;
    pub const TIMER_CMP_LO_CMP_LO_MASK: u32 = 0xffffffff;

    /* Timer High comparator value bitfield. (access: R/W) */
    pub const TIMER_CMP_HI_CMP_HI_BIT: u32 = 0;
    pub const TIMER_CMP_HI_CMP_HI_WIDTH: u32 = 32;
    pub const TIMER_CMP_HI_CMP_HI_MASK: u32 = 0xffffffff;

    /* Timer Low start command bitfield. When executed, CFG_LO.ENABLE is set.
     * (access: W) */
    pub const TIMER_START_LO_STRT_LO_BIT: u32 = 0;
    pub const TIMER_START_LO_STRT_LO_WIDTH: u32 = 1;
    pub const TIMER_START_LO_STRT_LO_MASK: u32 = 0x1;

    /* Timer High start command bitfield. When executed, CFG_HI.ENABLE is set.
     * (access: W) */
    pub const TIMER_START_HI_STRT_HI_BIT: u32 = 0;
    pub const TIMER_START_HI_STRT_HI_WIDTH: u32 = 1;
    pub const TIMER_START_HI_STRT_HI_MASK: u32 = 0x1;

    /* Timer Low counter reset command bitfield. When executed, CFG_LO.RESET is set.
     * (access: W) */
    pub const TIMER_RESET_LO_RST_LO_BIT: u32 = 0;
    pub const TIMER_RESET_LO_RST_LO_WIDTH: u32 = 1;
    pub const TIMER_RESET_LO_RST_LO_MASK: u32 = 0x1;

    /* Timer High counter reset command bitfield. When executed, CFG_HI.RESET is
     * set. (access: W) */
    pub const TIMER_RESET_HI_RST_HI_BIT: u32 = 0;
    pub const TIMER_RESET_HI_RST_HI_WIDTH: u32 = 1;
    pub const TIMER_RESET_HI_RST_HI_MASK: u32 = 0x1;

    pub const CSR_MXNTI_ID: u32 = 0x345 as u32;
    pub const MIE: u32 = 8 as u32;
}

#[allow(missing_docs)]
pub mod crs {
    use core::arch::asm;
    /*
    pub fn write_crs(crs_nr:u32, value:u32){
        unsafe {
            asm!(
                "csrw {0}, {1}",
                in(reg) crs_nr,
                in(reg) value,
            );
        }
    }

    pub fn read_crs(crs_nr:u32) -> u32{
        let mut value = 0;
        unsafe {
            asm!(
                "csrr {0}, {1}",
                out(reg) value,
                in(reg) crs_nr,
            );
        }
        value
    }
    */

    pub const CSR_MSTATUS: u32 = 0x300;
    pub const CSR_MISA: u32 = 0x301;
    pub const CSR_MIE: u32 = 0x304;
    pub const CSR_MTVEC: u32 = 0x305;
    pub const CSR_MTVT: u32 = 0x307;
    pub const CSR_MSCRATCH: u32 = 0x340;
    pub const CSR_MEPC: u32 = 0x341;
    pub const CSR_MCAUSE: u32 = 0x342;
    pub const CSR_MTVAL: u32 = 0x343;
    pub const CSR_MIP: u32 = 0x344;
    pub const CSR_MNXTI: u32 = 0x345;
    pub const CSR_PMPCFG0: u32 = 0x3a0;
    pub const CSR_PMPADDR0: u32 = 0x3b0;
    pub const CSR_MHARTID: u32 = 0xf14;
    pub const CSR_MINTSTATUS: u32 = 0x346;
    pub const CSR_MINTTHRESH: u32 = 0x347;
    pub const CSR_MCLICBASE: u32 = 0x350;
}

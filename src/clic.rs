#[allow(missing_docs)]
pub mod addr {

    pub struct MemoryMapper {
        base_address: *mut u8,
    }

    impl MemoryMapper {
        pub fn new(base_address: *mut u8) -> Self{
            Self {
                base_address: base_address,
            }
        }

        pub fn write(&self, reg_offset: isize, mask: u32, bitoffset: u32, value: u32) {
            unsafe {
                let reg_value = core::ptr::read_volatile(self.base_address.offset(reg_offset) as *mut u32);
                let reg_value = (reg_value | mask) & (value << bitoffset);
                core::ptr::write_volatile(self.base_address.offset(reg_offset) as *mut u32, reg_value);
            }
        }

        pub fn write_byte(&self, reg_offset: isize, mask: u8, bitoffset: u8, value: u8) {
            unsafe {
                let reg_value = core::ptr::read_volatile((self.base_address).offset(reg_offset));
                let reg_value = (reg_value | mask) & (value << bitoffset);
                core::ptr::write_volatile((self.base_address).offset(reg_offset), reg_value);
            }
        }

        pub fn read(&self, reg_offset: isize, mask: u32, bitoffset: u32) -> u32 {
            unsafe {
                let reg_value = core::ptr::read_volatile(self.base_address.offset(reg_offset) as *mut u32);
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
    pub const CLICCFG_NVBITS_MASK:  u8 = 1;
    pub const CLICCFG_NVBITS_OFFSET:  u8 = 0;
    pub const CLICCFG_NLBITS_MASK: u8 = 0x1E as  u8;
    pub const CLICCFG_NLBITS_OFFSET:  u8 = 1 as  u8;
    pub const CLICCFG_NMBITS_MASK:  u8 = 0x60 as  u8;
    pub const CLICCFG_NMBITS_OFFSET:  u8 = 5 as  u8;

    /* CLIC Information */
    pub const CLICINFO_REG_OFFSET: isize = 0x4 as isize;
    pub const CLICINFO_NUM_INTERRUPT_MASK:  u32 = 0x1FFF as  u32;
    pub const CLICINFO_NUM_INTERRUPT_OFFSET:  u32 = 0 as  u32;

    pub const CLICINFO_VERSION_MASK:  u32 = 0x1FE000 as  u32;
    pub const CLICINFO_VERSION_OFFSET:  u32 = 13 as  u32;

    pub const CLICINFO_CLICINTCTLBITS_MASK:  u32 = 0x1E00000 as  u32;
    pub const CLICINFO_CLICINTCTLBITS_OFFSET:  u32 = 21 as  u32;

    pub const CLICINFO_NUM_TRIGGER_MASK:  u32 = 0x7E000000 as  u32;
    pub const CLICINFO_NUM_TRIGGER_OFFSET:  u32 = 25 as  u32;

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
    pub const CLICINTATTR_SHV_BIT:  u8 = 0 ;
    pub const CLICINTATTR_SHV_MASK:  u8 = 0x1 ;
    pub const CLICINTATTR_TRIG_MASK:  u8 = 0x6 ;
    pub const CLICINTATTR_TRIG_OFFSET:  u8 = 1 ;
    pub const CLICINTATTR_MODE_MASK:  u8 = 0xC0 ;
    pub const CLICINTATTR_MODE_OFFSET:  u8 = 6 ;

    pub const TRIG_LEVEL:  u8 = 0 ;
    pub const TRIG_EDGE:  u8 = 1 ;
    pub const TRIG_POSITIVE:  u8 = 0 << 1 ;
    pub const TRIG_NEGATIVE:  u8 = 1 << 1 ;

    /* CLIC interrupt id control */
    pub fn CLICINTCTL_REG_OFFSET(id: u32) -> isize {
        (0x100c + 0x10 * id) as isize
    }
    pub const CLICINTCTL_CLICINTCTL_MASK:  u8 = 0xFF;
    pub const CLICINTCTL_CLICINTCTL_OFFSET:  u8 = 0;

    /* Timers */
    /*
    pub const TIMER_TIMER_LOW_REG_OFFSET: isize = 0x0;
    pub const TIMER_TIMER_HIGH_REG_OFFSET: isize = 0x10;
    
    pub const TIMER_CTRL_LOW_REG_OFFSET:isize = 0x4;
    pub const TIMER_CTRL_HIGH_REG_OFFSET:isize = 0x14;

    pub const TIMER_CTRL_EN_BIT:u32 = 0;
    pub const TIMER_CTRL_EN_MASK:u32 = 0xFFFFFFFE;
    
    pub const TIMER_CTRL_PRE_OFFSET:u32 = 3;
    pub const TIMER_CTRL_PRE_MASK:u32 = 0xFFFFFF38;
    
    pub const TIMER_CMP_LOW_REG_OFFSET: isize = 0x8;
    pub const TIMER_CMP_HIGH_REG_OFFSET: isize = 0x18;
    */

    
    pub const TIMER_CFG_LOW_REG_OFFSET:isize = 0x0;
    pub const TIMER_CFG_HIGH_REG_OFFSET:isize = 0x4;
    
    pub const TIMER_CNT_LOW_REG_OFFSET: isize = 0x8;
    pub const TIMER_CNT_HIGH_REG_OFFSET: isize = 0xc;

    pub const TIMER_CMP_LOW_REG_OFFSET: isize = 0x10;
    pub const TIMER_CMP_HIGH_REG_OFFSET: isize = 0x14;

    pub const TIMER_START_LOW_REG_OFFSET: isize = 0x18;
    pub const TIMER_START_HIGH_REG_OFFSET: isize = 0x1c;

    pub const TIMER_RESET_LOW_REG_OFFSET: isize = 0x20;
    pub const TIMER_RESET_HIGH_REG_OFFSET: isize = 0x24;

    pub const TIMER_CTRL_EN_BIT:u32 = 0;
    pub const TIMER_CTRL_EN_MASK:u32 = 0xFFFFFFFE;
    
    pub const TIMER_CTRL_PRE_OFFSET:u32 = 3;
    pub const TIMER_CTRL_PRE_MASK:u32 = 0xFFFFFF38;
    
    
    pub const CSR_MXNTI_ID:  u32 = 0x345 as  u32;
    pub const MIE:  u32 = 8 as  u32;
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
    pub const CSR_MTVT:u32 = 0x307;
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

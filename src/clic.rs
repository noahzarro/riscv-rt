pub mod addresses {

    /* CLIC Configuration */
    pub const CLIC_CLICCFG_REG_OFFSET: *mut u32 = 0x0 as *mut u32;
    pub const CLIC_CLICCFG_NVBITS_BIT:  u32 = 0 as u32;
    pub const CLIC_CLICCFG_NLBITS_MASK: u32 = 0xf as  u32;
    pub const CLIC_CLICCFG_NLBITS_OFFSET:  u32 = 1 as  u32;
    pub const CLIC_CLICCFG_NMBITS_MASK:  u32 = 0x3 as  u32;
    pub const CLIC_CLICCFG_NMBITS_OFFSET:  u32 = 5 as  u32;

    /* CLIC Information */
    pub const CLIC_CLICINFO_REG_OFFSET: *mut u32 = 0x4 as *mut u32;
    pub const CLIC_CLICINFO_NUM_INTERRUPT_MASK:  u32 = 0x1fff as  u32;
    pub const CLIC_CLICINFO_NUM_INTERRUPT_OFFSET:  u32 = 0 as  u32;

    pub const CLIC_CLICINFO_VERSION_MASK:  u32 = 0xff as  u32;
    pub const CLIC_CLICINFO_VERSION_OFFSET:  u32 = 13 as  u32;

    pub const CLIC_CLICINFO_CLICINTCTLBITS_MASK:  u32 = 0xf as  u32;
    pub const CLIC_CLICINFO_CLICINTCTLBITS_OFFSET:  u32 = 21 as  u32;

    pub const CLIC_CLICINFO_NUM_TRIGGER_MASK:  u32 = 0x3f as  u32;
    pub const CLIC_CLICINFO_NUM_TRIGGER_OFFSET:  u32 = 25 as  u32;

    /* CLIC enable mnxti irq forwarding logic */
    pub const CLIC_CLICXNXTICONF_REG_OFFSET: *mut u32 = 0x8 as *mut u32;
    pub const CLIC_CLICXNXTICONF_CLICXNXTICONF_BIT: u32 = 0 as  u32;

    /* CLIC interrupt id pending */
    pub fn CLIC_CLICINTIP_REG_OFFSET(id: u32) -> *mut u32 {
        (0x1000 + 0x10 * id) as *mut u32
    }
    pub const CLIC_CLICINTIP_CLICINTIP_BIT: u32 = 0 as u32;

    /* CLIC interrupt id enable */
    pub fn CLIC_CLICINTIE_REG_OFFSET(id: u32) -> *mut u32 {
        (0x1004 + 0x10 * id) as *mut u32
    }
    pub const CLIC_CLICINTIE_CLICINTIE_BIT: u32 = 0 as u32;

    /* CLIC interrupt id attributes */
    pub fn CLIC_CLICINTATTR_REG_OFFSET(id: u32) -> *mut u32 {
        (0x1008 + 0x10 * id) as *mut u32
    }
    pub const CLIC_CLICINTATTR_SHV_BIT:  u32 = 0 as  u32;
    pub const CLIC_CLICINTATTR_TRIG_MASK:  u32 = 0x3 as  u32;
    pub const CLIC_CLICINTATTR_TRIG_OFFSET:  u32 = 1 as  u32;
    pub const CLIC_CLICINTATTR_MODE_MASK:  u32 = 0x3 as  u32;
    pub const CLIC_CLICINTATTR_MODE_OFFSET:  u32 = 6 as  u32;

    pub const CLIC_TRIG_LEVEL:  u32 = 0 as  u32;
    pub const CLIC_TRIG_EDGE:  u32 = 1 as  u32;
    pub const CLIC_TRIG_POSITIVE:  u32 = 0 << 1 as  u32;
    pub const CLIC_TRIG_NEGATIVE:  u32 = 1 << 1 as  u32;

    /* CLIC interrupt id control */
    pub fn CLIC_CLICINTCTL_REG_OFFSET(id: u32) -> *mut u32 {
        (0x100c + 0x10 * id) as *mut u32
    }
    pub const CLIC_CLICINTCTL_CLICINTCTL_MASK:  u32 = 0xff as  u32;
    pub const CLIC_CLICINTCTL_CLICINTCTL_OFFSET:  u32 = 0 as  u32;

    pub const CSR_MXNTI_ID:  u32 = 0x345 as  u32;
    pub const MIE:  u32 = 8 as  u32;
}

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

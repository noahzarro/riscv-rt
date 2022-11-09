/* CLIC Configuration */
pub const CLIC_CLICCFG_REG_OFFSET: u32 = 0x0;
pub const CLIC_CLICCFG_NVBITS_BIT: u32 = 0;
pub const CLIC_CLICCFG_NLBITS_MASK: u32 = 0xf;
pub const CLIC_CLICCFG_NLBITS_OFFSET: u32 = 1;
pub const CLIC_CLICCFG_NMBITS_MASK: u32 = 0x3;
pub const CLIC_CLICCFG_NMBITS_OFFSET: u32 = 5;

/* CLIC Information */
pub const CLIC_CLICINFO_REG_OFFSET: u32 = 0x4;
pub const CLIC_CLICINFO_NUM_INTERRUPT_MASK: u32 = 0x1fff;
pub const CLIC_CLICINFO_NUM_INTERRUPT_OFFSET: u32 = 0;

pub const CLIC_CLICINFO_VERSION_MASK: u32 = 0xff;
pub const CLIC_CLICINFO_VERSION_OFFSET: u32 = 13;

pub const CLIC_CLICINFO_CLICINTCTLBITS_MASK: u32 = 0xf;
pub const CLIC_CLICINFO_CLICINTCTLBITS_OFFSET: u32 = 21;

pub const CLIC_CLICINFO_NUM_TRIGGER_MASK: u32 = 0x3f;
pub const CLIC_CLICINFO_NUM_TRIGGER_OFFSET: u32 = 25;

/* CLIC enable mnxti irq forwarding logic */
pub const CLIC_CLICXNXTICONF_REG_OFFSET: u32 = 0x8;
pub const CLIC_CLICXNXTICONF_CLICXNXTICONF_BIT: u32 = 0;

/* CLIC interrupt id pending */
pub fn CLIC_CLICINTIP_REG_OFFSET(id: u32) -> u32 {
    0x1000 + 0x10 * id
}
pub const CLIC_CLICINTIP_CLICINTIP_BIT: u32 = 0;

/* CLIC interrupt id enable */
pub fn CLIC_CLICINTIE_REG_OFFSET(id: u32) -> u32 {
    0x1004 + 0x10 * id
}
pub const CLIC_CLICINTIE_CLICINTIE_BIT: u32 = 0;

/* CLIC interrupt id attributes */
pub fn CLIC_CLICINTATTR_REG_OFFSET(id: u32) -> u32 {
    0x1008 + 0x10 * id
}
pub const CLIC_CLICINTATTR_SHV_BIT: u32 = 0;
pub const CLIC_CLICINTATTR_TRIG_MASK: u32 = 0x3;
pub const CLIC_CLICINTATTR_TRIG_OFFSET: u32 = 1;
pub const CLIC_CLICINTATTR_MODE_MASK: u32 = 0x3;
pub const CLIC_CLICINTATTR_MODE_OFFSET: u32 = 6;

pub const CLIC_TRIG_LEVEL: u32 = 0;
pub const CLIC_TRIG_EDGE: u32 = 1;
pub const CLIC_TRIG_POSITIVE: u32 = 0 << 1;
pub const CLIC_TRIG_NEGATIVE: u32 = 1 << 1;

/* CLIC interrupt id control */
pub fn CLIC_CLICINTCTL_REG_OFFSET(id: u32) -> u32 {
    0x100c + 0x10 * id
}
pub const CLIC_CLICINTCTL_CLICINTCTL_MASK: u32 = 0xff;
pub const CLIC_CLICINTCTL_CLICINTCTL_OFFSET: u32 = 0;

pub const CSR_MXNTI_ID: u32 = 0x345;
pub const MIE: u32 = 8;

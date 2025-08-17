#[doc = "Register `CCCR` reader"]
pub type R = crate::R<CCCR_SPEC>;
#[doc = "Register `CCCR` writer"]
pub type W = crate::W<CCCR_SPEC>;
#[doc = "Initialization\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT_A {
    #[doc = "0: Normal operation"]
    B_0x0 = 0,
    #[doc = "1: Initialization started"]
    B_0x1 = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization"]
pub type INIT_R = crate::BitReader<INIT_A>;
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::B_0x0,
            true => INIT_A::B_0x1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == INIT_A::B_0x0
    }
    #[doc = "Initialization started"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == INIT_A::B_0x1
    }
}
#[doc = "Field `INIT` writer - Initialization"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG, INIT_A>;
impl<'a, REG> INIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(INIT_A::B_0x0)
    }
    #[doc = "Initialization started"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(INIT_A::B_0x1)
    }
}
#[doc = "Configuration change enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCE_A {
    #[doc = "0: The CPU has no write access to the protected configuration registers."]
    B_0x0 = 0,
    #[doc = "1: The CPU has write access to the protected configuration registers (while CCCR.INIT = 1)."]
    B_0x1 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` reader - Configuration change enable"]
pub type CCE_R = crate::BitReader<CCE_A>;
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::B_0x0,
            true => CCE_A::B_0x1,
        }
    }
    #[doc = "The CPU has no write access to the protected configuration registers."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CCE_A::B_0x0
    }
    #[doc = "The CPU has write access to the protected configuration registers (while CCCR.INIT = 1)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CCE_A::B_0x1
    }
}
#[doc = "Field `CCE` writer - Configuration change enable"]
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG, CCE_A>;
impl<'a, REG> CCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CPU has no write access to the protected configuration registers."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCE_A::B_0x0)
    }
    #[doc = "The CPU has write access to the protected configuration registers (while CCCR.INIT = 1)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCE_A::B_0x1)
    }
}
#[doc = "ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASM_A {
    #[doc = "0: Normal CAN operation"]
    B_0x0 = 0,
    #[doc = "1: Restricted operation Mode active"]
    B_0x1 = 1,
}
impl From<ASM_A> for bool {
    #[inline(always)]
    fn from(variant: ASM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASM` reader - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
pub type ASM_R = crate::BitReader<ASM_A>;
impl ASM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASM_A {
        match self.bits {
            false => ASM_A::B_0x0,
            true => ASM_A::B_0x1,
        }
    }
    #[doc = "Normal CAN operation"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ASM_A::B_0x0
    }
    #[doc = "Restricted operation Mode active"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ASM_A::B_0x1
    }
}
#[doc = "Field `ASM` writer - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
pub type ASM_W<'a, REG> = crate::BitWriter<'a, REG, ASM_A>;
impl<'a, REG> ASM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal CAN operation"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ASM_A::B_0x0)
    }
    #[doc = "Restricted operation Mode active"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ASM_A::B_0x1)
    }
}
#[doc = "Clock stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSA_A {
    #[doc = "0: No clock stop acknowledged"]
    B_0x0 = 0,
    #[doc = "1: FDCAN may be set in power down by stopping APB clock and kernel clock."]
    B_0x1 = 1,
}
impl From<CSA_A> for bool {
    #[inline(always)]
    fn from(variant: CSA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSA` reader - Clock stop acknowledge"]
pub type CSA_R = crate::BitReader<CSA_A>;
impl CSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSA_A {
        match self.bits {
            false => CSA_A::B_0x0,
            true => CSA_A::B_0x1,
        }
    }
    #[doc = "No clock stop acknowledged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CSA_A::B_0x0
    }
    #[doc = "FDCAN may be set in power down by stopping APB clock and kernel clock."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CSA_A::B_0x1
    }
}
#[doc = "Clock stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSR_A {
    #[doc = "0: No clock stop requested"]
    B_0x0 = 0,
    #[doc = "1: Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle."]
    B_0x1 = 1,
}
impl From<CSR_A> for bool {
    #[inline(always)]
    fn from(variant: CSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSR` reader - Clock stop request"]
pub type CSR_R = crate::BitReader<CSR_A>;
impl CSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSR_A {
        match self.bits {
            false => CSR_A::B_0x0,
            true => CSR_A::B_0x1,
        }
    }
    #[doc = "No clock stop requested"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CSR_A::B_0x0
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CSR_A::B_0x1
    }
}
#[doc = "Field `CSR` writer - Clock stop request"]
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG, CSR_A>;
impl<'a, REG> CSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No clock stop requested"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSR_A::B_0x0)
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSR_A::B_0x1)
    }
}
#[doc = "Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MON_A {
    #[doc = "0: Bus monitoring mode disabled"]
    B_0x0 = 0,
    #[doc = "1: Bus monitoring mode enabled"]
    B_0x1 = 1,
}
impl From<MON_A> for bool {
    #[inline(always)]
    fn from(variant: MON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MON` reader - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
pub type MON_R = crate::BitReader<MON_A>;
impl MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MON_A {
        match self.bits {
            false => MON_A::B_0x0,
            true => MON_A::B_0x1,
        }
    }
    #[doc = "Bus monitoring mode disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MON_A::B_0x0
    }
    #[doc = "Bus monitoring mode enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MON_A::B_0x1
    }
}
#[doc = "Field `MON` writer - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
pub type MON_W<'a, REG> = crate::BitWriter<'a, REG, MON_A>;
impl<'a, REG> MON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus monitoring mode disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MON_A::B_0x0)
    }
    #[doc = "Bus monitoring mode enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MON_A::B_0x1)
    }
}
#[doc = "Disable automatic retransmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAR_A {
    #[doc = "0: Automatic retransmission of messages not transmitted successfully enabled"]
    B_0x0 = 0,
    #[doc = "1: Automatic retransmission disabled"]
    B_0x1 = 1,
}
impl From<DAR_A> for bool {
    #[inline(always)]
    fn from(variant: DAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAR` reader - Disable automatic retransmission"]
pub type DAR_R = crate::BitReader<DAR_A>;
impl DAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAR_A {
        match self.bits {
            false => DAR_A::B_0x0,
            true => DAR_A::B_0x1,
        }
    }
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DAR_A::B_0x0
    }
    #[doc = "Automatic retransmission disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DAR_A::B_0x1
    }
}
#[doc = "Field `DAR` writer - Disable automatic retransmission"]
pub type DAR_W<'a, REG> = crate::BitWriter<'a, REG, DAR_A>;
impl<'a, REG> DAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DAR_A::B_0x0)
    }
    #[doc = "Automatic retransmission disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DAR_A::B_0x1)
    }
}
#[doc = "Test mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEST_A {
    #[doc = "0: Normal operation, register TEST holds reset values"]
    B_0x0 = 0,
    #[doc = "1: Test Mode, write access to register TEST enabled"]
    B_0x1 = 1,
}
impl From<TEST_A> for bool {
    #[inline(always)]
    fn from(variant: TEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST` reader - Test mode enable"]
pub type TEST_R = crate::BitReader<TEST_A>;
impl TEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEST_A {
        match self.bits {
            false => TEST_A::B_0x0,
            true => TEST_A::B_0x1,
        }
    }
    #[doc = "Normal operation, register TEST holds reset values"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEST_A::B_0x0
    }
    #[doc = "Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEST_A::B_0x1
    }
}
#[doc = "Field `TEST` writer - Test mode enable"]
pub type TEST_W<'a, REG> = crate::BitWriter<'a, REG, TEST_A>;
impl<'a, REG> TEST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, register TEST holds reset values"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEST_A::B_0x0)
    }
    #[doc = "Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEST_A::B_0x1)
    }
}
#[doc = "FD operation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDOE_A {
    #[doc = "0: FD operation disabled"]
    B_0x0 = 0,
    #[doc = "1: FD operation enabled"]
    B_0x1 = 1,
}
impl From<FDOE_A> for bool {
    #[inline(always)]
    fn from(variant: FDOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDOE` reader - FD operation enable"]
pub type FDOE_R = crate::BitReader<FDOE_A>;
impl FDOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FDOE_A {
        match self.bits {
            false => FDOE_A::B_0x0,
            true => FDOE_A::B_0x1,
        }
    }
    #[doc = "FD operation disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FDOE_A::B_0x0
    }
    #[doc = "FD operation enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FDOE_A::B_0x1
    }
}
#[doc = "Field `FDOE` writer - FD operation enable"]
pub type FDOE_W<'a, REG> = crate::BitWriter<'a, REG, FDOE_A>;
impl<'a, REG> FDOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FD operation disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FDOE_A::B_0x0)
    }
    #[doc = "FD operation enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FDOE_A::B_0x1)
    }
}
#[doc = "FDCAN bit rate switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRSE_A {
    #[doc = "0: Bit rate switching for transmissions disabled"]
    B_0x0 = 0,
    #[doc = "1: Bit rate switching for transmissions enabled"]
    B_0x1 = 1,
}
impl From<BRSE_A> for bool {
    #[inline(always)]
    fn from(variant: BRSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRSE` reader - FDCAN bit rate switching"]
pub type BRSE_R = crate::BitReader<BRSE_A>;
impl BRSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRSE_A {
        match self.bits {
            false => BRSE_A::B_0x0,
            true => BRSE_A::B_0x1,
        }
    }
    #[doc = "Bit rate switching for transmissions disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BRSE_A::B_0x0
    }
    #[doc = "Bit rate switching for transmissions enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BRSE_A::B_0x1
    }
}
#[doc = "Field `BRSE` writer - FDCAN bit rate switching"]
pub type BRSE_W<'a, REG> = crate::BitWriter<'a, REG, BRSE_A>;
impl<'a, REG> BRSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit rate switching for transmissions disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BRSE_A::B_0x0)
    }
    #[doc = "Bit rate switching for transmissions enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BRSE_A::B_0x1)
    }
}
#[doc = "Protocol exception handling disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PXHD_A {
    #[doc = "0: Protocol exception handling enabled"]
    B_0x0 = 0,
    #[doc = "1: Protocol exception handling disabled"]
    B_0x1 = 1,
}
impl From<PXHD_A> for bool {
    #[inline(always)]
    fn from(variant: PXHD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PXHD` reader - Protocol exception handling disable"]
pub type PXHD_R = crate::BitReader<PXHD_A>;
impl PXHD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PXHD_A {
        match self.bits {
            false => PXHD_A::B_0x0,
            true => PXHD_A::B_0x1,
        }
    }
    #[doc = "Protocol exception handling enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PXHD_A::B_0x0
    }
    #[doc = "Protocol exception handling disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PXHD_A::B_0x1
    }
}
#[doc = "Field `PXHD` writer - Protocol exception handling disable"]
pub type PXHD_W<'a, REG> = crate::BitWriter<'a, REG, PXHD_A>;
impl<'a, REG> PXHD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protocol exception handling enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PXHD_A::B_0x0)
    }
    #[doc = "Protocol exception handling disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PXHD_A::B_0x1)
    }
}
#[doc = "Edge filtering during bus integration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EFBI_A {
    #[doc = "0: Edge filtering disabled"]
    B_0x0 = 0,
    #[doc = "1: Two consecutive dominant tq required to detect an edge for hard synchronization"]
    B_0x1 = 1,
}
impl From<EFBI_A> for bool {
    #[inline(always)]
    fn from(variant: EFBI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EFBI` reader - Edge filtering during bus integration"]
pub type EFBI_R = crate::BitReader<EFBI_A>;
impl EFBI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EFBI_A {
        match self.bits {
            false => EFBI_A::B_0x0,
            true => EFBI_A::B_0x1,
        }
    }
    #[doc = "Edge filtering disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EFBI_A::B_0x0
    }
    #[doc = "Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EFBI_A::B_0x1
    }
}
#[doc = "Field `EFBI` writer - Edge filtering during bus integration"]
pub type EFBI_W<'a, REG> = crate::BitWriter<'a, REG, EFBI_A>;
impl<'a, REG> EFBI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge filtering disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EFBI_A::B_0x0)
    }
    #[doc = "Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EFBI_A::B_0x1)
    }
}
#[doc = "If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP_A {
    #[doc = "0: disabled"]
    B_0x0 = 0,
    #[doc = "1: enabled"]
    B_0x1 = 1,
}
impl From<TXP_A> for bool {
    #[inline(always)]
    fn from(variant: TXP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXP` reader - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
pub type TXP_R = crate::BitReader<TXP_A>;
impl TXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXP_A {
        match self.bits {
            false => TXP_A::B_0x0,
            true => TXP_A::B_0x1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXP_A::B_0x0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXP_A::B_0x1
    }
}
#[doc = "Field `TXP` writer - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
pub type TXP_W<'a, REG> = crate::BitWriter<'a, REG, TXP_A>;
impl<'a, REG> TXP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXP_A::B_0x0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXP_A::B_0x1)
    }
}
#[doc = "Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NISO_A {
    #[doc = "0: CAN FD frame format according to ISO11898-1"]
    B_0x0 = 0,
    #[doc = "1: CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    B_0x1 = 1,
}
impl From<NISO_A> for bool {
    #[inline(always)]
    fn from(variant: NISO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NISO` reader - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
pub type NISO_R = crate::BitReader<NISO_A>;
impl NISO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NISO_A {
        match self.bits {
            false => NISO_A::B_0x0,
            true => NISO_A::B_0x1,
        }
    }
    #[doc = "CAN FD frame format according to ISO11898-1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NISO_A::B_0x0
    }
    #[doc = "CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NISO_A::B_0x1
    }
}
#[doc = "Field `NISO` writer - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
pub type NISO_W<'a, REG> = crate::BitWriter<'a, REG, NISO_A>;
impl<'a, REG> NISO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN FD frame format according to ISO11898-1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NISO_A::B_0x0)
    }
    #[doc = "CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NISO_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn INIT(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration change enable"]
    #[inline(always)]
    pub fn CCE(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
    #[inline(always)]
    pub fn ASM(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock stop acknowledge"]
    #[inline(always)]
    pub fn CSA(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock stop request"]
    #[inline(always)]
    pub fn CSR(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
    #[inline(always)]
    pub fn MON(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable automatic retransmission"]
    #[inline(always)]
    pub fn DAR(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test mode enable"]
    #[inline(always)]
    pub fn TEST(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FD operation enable"]
    #[inline(always)]
    pub fn FDOE(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN bit rate switching"]
    #[inline(always)]
    pub fn BRSE(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol exception handling disable"]
    #[inline(always)]
    pub fn PXHD(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge filtering during bus integration"]
    #[inline(always)]
    pub fn EFBI(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
    #[inline(always)]
    pub fn TXP(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
    #[inline(always)]
    pub fn NISO(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn INIT(&mut self) -> INIT_W<'_, CCCR_SPEC> {
        INIT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration change enable"]
    #[inline(always)]
    pub fn CCE(&mut self) -> CCE_W<'_, CCCR_SPEC> {
        CCE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
    #[inline(always)]
    pub fn ASM(&mut self) -> ASM_W<'_, CCCR_SPEC> {
        ASM_W::new(self, 2)
    }
    #[doc = "Bit 4 - Clock stop request"]
    #[inline(always)]
    pub fn CSR(&mut self) -> CSR_W<'_, CCCR_SPEC> {
        CSR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
    #[inline(always)]
    pub fn MON(&mut self) -> MON_W<'_, CCCR_SPEC> {
        MON_W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable automatic retransmission"]
    #[inline(always)]
    pub fn DAR(&mut self) -> DAR_W<'_, CCCR_SPEC> {
        DAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Test mode enable"]
    #[inline(always)]
    pub fn TEST(&mut self) -> TEST_W<'_, CCCR_SPEC> {
        TEST_W::new(self, 7)
    }
    #[doc = "Bit 8 - FD operation enable"]
    #[inline(always)]
    pub fn FDOE(&mut self) -> FDOE_W<'_, CCCR_SPEC> {
        FDOE_W::new(self, 8)
    }
    #[doc = "Bit 9 - FDCAN bit rate switching"]
    #[inline(always)]
    pub fn BRSE(&mut self) -> BRSE_W<'_, CCCR_SPEC> {
        BRSE_W::new(self, 9)
    }
    #[doc = "Bit 12 - Protocol exception handling disable"]
    #[inline(always)]
    pub fn PXHD(&mut self) -> PXHD_W<'_, CCCR_SPEC> {
        PXHD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Edge filtering during bus integration"]
    #[inline(always)]
    pub fn EFBI(&mut self) -> EFBI_W<'_, CCCR_SPEC> {
        EFBI_W::new(self, 13)
    }
    #[doc = "Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
    #[inline(always)]
    pub fn TXP(&mut self) -> TXP_W<'_, CCCR_SPEC> {
        TXP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
    #[inline(always)]
    pub fn NISO(&mut self) -> NISO_W<'_, CCCR_SPEC> {
        NISO_W::new(self, 15)
    }
}
#[doc = "FDCAN CC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccr::R`](R) reader structure"]
impl crate::Readable for CCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cccr::W`](W) writer structure"]
impl crate::Writable for CCCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCCR to value 0x01"]
impl crate::Resettable for CCCR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}

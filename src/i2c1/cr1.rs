#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: Peripheral disable"]
    B_0x0 = 0,
    #[doc = "1: Peripheral enable"]
    B_0x1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
pub type PE_R = crate::BitReader<PE_A>;
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::B_0x0,
            true => PE_A::B_0x1,
        }
    }
    #[doc = "Peripheral disable"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PE_A::B_0x0
    }
    #[doc = "Peripheral enable"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PE_A::B_0x1
    }
}
#[doc = "Field `PE` writer - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG, PE_A>;
impl<'a, REG> PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disable"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::B_0x0)
    }
    #[doc = "Peripheral enable"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::B_0x1)
    }
}
#[doc = "TX Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIE_A {
    #[doc = "0: Transmit (TXIS) interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Transmit (TXIS) interrupt enabled"]
    B_0x1 = 1,
}
impl From<TXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIE` reader - TX Interrupt enable"]
pub type TXIE_R = crate::BitReader<TXIE_A>;
impl TXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXIE_A {
        match self.bits {
            false => TXIE_A::B_0x0,
            true => TXIE_A::B_0x1,
        }
    }
    #[doc = "Transmit (TXIS) interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXIE_A::B_0x0
    }
    #[doc = "Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXIE_A::B_0x1
    }
}
#[doc = "Field `TXIE` writer - TX Interrupt enable"]
pub type TXIE_W<'a, REG> = crate::BitWriter<'a, REG, TXIE_A>;
impl<'a, REG> TXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit (TXIS) interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXIE_A::B_0x0)
    }
    #[doc = "Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXIE_A::B_0x1)
    }
}
#[doc = "RX Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXIE_A {
    #[doc = "0: Receive (RXNE) interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Receive (RXNE) interrupt enabled"]
    B_0x1 = 1,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIE` reader - RX Interrupt enable"]
pub type RXIE_R = crate::BitReader<RXIE_A>;
impl RXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::B_0x0,
            true => RXIE_A::B_0x1,
        }
    }
    #[doc = "Receive (RXNE) interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXIE_A::B_0x0
    }
    #[doc = "Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXIE_A::B_0x1
    }
}
#[doc = "Field `RXIE` writer - RX Interrupt enable"]
pub type RXIE_W<'a, REG> = crate::BitWriter<'a, REG, RXIE_A>;
impl<'a, REG> RXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive (RXNE) interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXIE_A::B_0x0)
    }
    #[doc = "Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXIE_A::B_0x1)
    }
}
#[doc = "Address match Interrupt enable (slave only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRIE_A {
    #[doc = "0: Address match (ADDR) interrupts disabled"]
    B_0x0 = 0,
    #[doc = "1: Address match (ADDR) interrupts enabled"]
    B_0x1 = 1,
}
impl From<ADDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRIE` reader - Address match Interrupt enable (slave only)"]
pub type ADDRIE_R = crate::BitReader<ADDRIE_A>;
impl ADDRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDRIE_A {
        match self.bits {
            false => ADDRIE_A::B_0x0,
            true => ADDRIE_A::B_0x1,
        }
    }
    #[doc = "Address match (ADDR) interrupts disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADDRIE_A::B_0x0
    }
    #[doc = "Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADDRIE_A::B_0x1
    }
}
#[doc = "Field `ADDRIE` writer - Address match Interrupt enable (slave only)"]
pub type ADDRIE_W<'a, REG> = crate::BitWriter<'a, REG, ADDRIE_A>;
impl<'a, REG> ADDRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address match (ADDR) interrupts disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRIE_A::B_0x0)
    }
    #[doc = "Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRIE_A::B_0x1)
    }
}
#[doc = "Not acknowledge received Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKIE_A {
    #[doc = "0: Not acknowledge (NACKF) received interrupts disabled"]
    B_0x0 = 0,
    #[doc = "1: Not acknowledge (NACKF) received interrupts enabled"]
    B_0x1 = 1,
}
impl From<NACKIE_A> for bool {
    #[inline(always)]
    fn from(variant: NACKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKIE` reader - Not acknowledge received Interrupt enable"]
pub type NACKIE_R = crate::BitReader<NACKIE_A>;
impl NACKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACKIE_A {
        match self.bits {
            false => NACKIE_A::B_0x0,
            true => NACKIE_A::B_0x1,
        }
    }
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NACKIE_A::B_0x0
    }
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NACKIE_A::B_0x1
    }
}
#[doc = "Field `NACKIE` writer - Not acknowledge received Interrupt enable"]
pub type NACKIE_W<'a, REG> = crate::BitWriter<'a, REG, NACKIE_A>;
impl<'a, REG> NACKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NACKIE_A::B_0x0)
    }
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NACKIE_A::B_0x1)
    }
}
#[doc = "Stop detection Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPIE_A {
    #[doc = "0: Stop detection (STOPF) interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Stop detection (STOPF) interrupt enabled"]
    B_0x1 = 1,
}
impl From<STOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: STOPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPIE` reader - Stop detection Interrupt enable"]
pub type STOPIE_R = crate::BitReader<STOPIE_A>;
impl STOPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPIE_A {
        match self.bits {
            false => STOPIE_A::B_0x0,
            true => STOPIE_A::B_0x1,
        }
    }
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STOPIE_A::B_0x0
    }
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STOPIE_A::B_0x1
    }
}
#[doc = "Field `STOPIE` writer - Stop detection Interrupt enable"]
pub type STOPIE_W<'a, REG> = crate::BitWriter<'a, REG, STOPIE_A>;
impl<'a, REG> STOPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(STOPIE_A::B_0x0)
    }
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(STOPIE_A::B_0x1)
    }
}
#[doc = "Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: Transfer Complete interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Transfer Complete interrupt enabled"]
    B_0x1 = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::B_0x0,
            true => TCIE_A::B_0x1,
        }
    }
    #[doc = "Transfer Complete interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TCIE_A::B_0x0
    }
    #[doc = "Transfer Complete interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TCIE_A::B_0x1
    }
}
#[doc = "Field `TCIE` writer - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE_A>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer Complete interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE_A::B_0x0)
    }
    #[doc = "Transfer Complete interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE_A::B_0x1)
    }
}
#[doc = "Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: Error detection interrupts disabled"]
    B_0x0 = 0,
    #[doc = "1: Error detection interrupts enabled"]
    B_0x1 = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::B_0x0,
            true => ERRIE_A::B_0x1,
        }
    }
    #[doc = "Error detection interrupts disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ERRIE_A::B_0x0
    }
    #[doc = "Error detection interrupts enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ERRIE_A::B_0x1
    }
}
#[doc = "Field `ERRIE` writer - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE_A>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error detection interrupts disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::B_0x0)
    }
    #[doc = "Error detection interrupts enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::B_0x1)
    }
}
#[doc = "Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\] * tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DNF_A {
    #[doc = "0: Digital filter disabled"]
    B_0x0 = 0,
    #[doc = "1: Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    B_0x1 = 1,
    #[doc = "15: digital filter enabled and filtering capability up to15 tI2CCLK"]
    B_0xF = 15,
}
impl From<DNF_A> for u8 {
    #[inline(always)]
    fn from(variant: DNF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DNF_A {
    type Ux = u8;
}
impl crate::IsEnum for DNF_A {}
#[doc = "Field `DNF` reader - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\] * tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
pub type DNF_R = crate::FieldReader<DNF_A>;
impl DNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DNF_A> {
        match self.bits {
            0 => Some(DNF_A::B_0x0),
            1 => Some(DNF_A::B_0x1),
            15 => Some(DNF_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DNF_A::B_0x0
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DNF_A::B_0x1
    }
    #[doc = "digital filter enabled and filtering capability up to15 tI2CCLK"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == DNF_A::B_0xF
    }
}
#[doc = "Field `DNF` writer - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\] * tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
pub type DNF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DNF_A>;
impl<'a, REG> DNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DNF_A::B_0x0)
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DNF_A::B_0x1)
    }
    #[doc = "digital filter enabled and filtering capability up to15 tI2CCLK"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(DNF_A::B_0xF)
    }
}
#[doc = "Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANFOFF_A {
    #[doc = "0: Analog noise filter enabled"]
    B_0x0 = 0,
    #[doc = "1: Analog noise filter disabled"]
    B_0x1 = 1,
}
impl From<ANFOFF_A> for bool {
    #[inline(always)]
    fn from(variant: ANFOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANFOFF` reader - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type ANFOFF_R = crate::BitReader<ANFOFF_A>;
impl ANFOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANFOFF_A {
        match self.bits {
            false => ANFOFF_A::B_0x0,
            true => ANFOFF_A::B_0x1,
        }
    }
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ANFOFF_A::B_0x0
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ANFOFF_A::B_0x1
    }
}
#[doc = "Field `ANFOFF` writer - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type ANFOFF_W<'a, REG> = crate::BitWriter<'a, REG, ANFOFF_A>;
impl<'a, REG> ANFOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ANFOFF_A::B_0x0)
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ANFOFF_A::B_0x1)
    }
}
#[doc = "DMA transmission requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN_A {
    #[doc = "0: DMA mode disabled for transmission"]
    B_0x0 = 0,
    #[doc = "1: DMA mode enabled for transmission"]
    B_0x1 = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - DMA transmission requests enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::B_0x0,
            true => TXDMAEN_A::B_0x1,
        }
    }
    #[doc = "DMA mode disabled for transmission"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXDMAEN_A::B_0x0
    }
    #[doc = "DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXDMAEN_A::B_0x1
    }
}
#[doc = "Field `TXDMAEN` writer - DMA transmission requests enable"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN_A>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled for transmission"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN_A::B_0x0)
    }
    #[doc = "DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN_A::B_0x1)
    }
}
#[doc = "DMA reception requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN_A {
    #[doc = "0: DMA mode disabled for reception"]
    B_0x0 = 0,
    #[doc = "1: DMA mode enabled for reception"]
    B_0x1 = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - DMA reception requests enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::B_0x0,
            true => RXDMAEN_A::B_0x1,
        }
    }
    #[doc = "DMA mode disabled for reception"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXDMAEN_A::B_0x0
    }
    #[doc = "DMA mode enabled for reception"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXDMAEN_A::B_0x1
    }
}
#[doc = "Field `RXDMAEN` writer - DMA reception requests enable"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN_A>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled for reception"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN_A::B_0x0)
    }
    #[doc = "DMA mode enabled for reception"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN_A::B_0x1)
    }
}
#[doc = "Slave byte control This bit is used to enable hardware byte control in slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBC_A {
    #[doc = "0: Slave byte control disabled"]
    B_0x0 = 0,
    #[doc = "1: Slave byte control enabled"]
    B_0x1 = 1,
}
impl From<SBC_A> for bool {
    #[inline(always)]
    fn from(variant: SBC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBC` reader - Slave byte control This bit is used to enable hardware byte control in slave mode."]
pub type SBC_R = crate::BitReader<SBC_A>;
impl SBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBC_A {
        match self.bits {
            false => SBC_A::B_0x0,
            true => SBC_A::B_0x1,
        }
    }
    #[doc = "Slave byte control disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SBC_A::B_0x0
    }
    #[doc = "Slave byte control enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SBC_A::B_0x1
    }
}
#[doc = "Field `SBC` writer - Slave byte control This bit is used to enable hardware byte control in slave mode."]
pub type SBC_W<'a, REG> = crate::BitWriter<'a, REG, SBC_A>;
impl<'a, REG> SBC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave byte control disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SBC_A::B_0x0)
    }
    #[doc = "Slave byte control enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SBC_A::B_0x1)
    }
}
#[doc = "Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOSTRETCH_A {
    #[doc = "0: Clock stretching enabled"]
    B_0x0 = 0,
    #[doc = "1: Clock stretching disabled"]
    B_0x1 = 1,
}
impl From<NOSTRETCH_A> for bool {
    #[inline(always)]
    fn from(variant: NOSTRETCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type NOSTRETCH_R = crate::BitReader<NOSTRETCH_A>;
impl NOSTRETCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NOSTRETCH_A {
        match self.bits {
            false => NOSTRETCH_A::B_0x0,
            true => NOSTRETCH_A::B_0x1,
        }
    }
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NOSTRETCH_A::B_0x0
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NOSTRETCH_A::B_0x1
    }
}
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type NOSTRETCH_W<'a, REG> = crate::BitWriter<'a, REG, NOSTRETCH_A>;
impl<'a, REG> NOSTRETCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NOSTRETCH_A::B_0x0)
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NOSTRETCH_A::B_0x1)
    }
}
#[doc = "Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to . Note: WUPEN can be set only when DNF = '0000'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN_A {
    #[doc = "0: Wakeup from Stop mode disable."]
    B_0x0 = 0,
    #[doc = "1: Wakeup from Stop mode enable."]
    B_0x1 = 1,
}
impl From<WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN` reader - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to . Note: WUPEN can be set only when DNF = '0000'"]
pub type WUPEN_R = crate::BitReader<WUPEN_A>;
impl WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN_A {
        match self.bits {
            false => WUPEN_A::B_0x0,
            true => WUPEN_A::B_0x1,
        }
    }
    #[doc = "Wakeup from Stop mode disable."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUPEN_A::B_0x0
    }
    #[doc = "Wakeup from Stop mode enable."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUPEN_A::B_0x1
    }
}
#[doc = "Field `WUPEN` writer - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to . Note: WUPEN can be set only when DNF = '0000'"]
pub type WUPEN_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN_A>;
impl<'a, REG> WUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup from Stop mode disable."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN_A::B_0x0)
    }
    #[doc = "Wakeup from Stop mode enable."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN_A::B_0x1)
    }
}
#[doc = "General call enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCEN_A {
    #[doc = "0: General call disabled. Address 0b00000000 is NACKed."]
    B_0x0 = 0,
    #[doc = "1: General call enabled. Address 0b00000000 is ACKed."]
    B_0x1 = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCEN` reader - General call enable"]
pub type GCEN_R = crate::BitReader<GCEN_A>;
impl GCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::B_0x0,
            true => GCEN_A::B_0x1,
        }
    }
    #[doc = "General call disabled. Address 0b00000000 is NACKed."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GCEN_A::B_0x0
    }
    #[doc = "General call enabled. Address 0b00000000 is ACKed."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GCEN_A::B_0x1
    }
}
#[doc = "Field `GCEN` writer - General call enable"]
pub type GCEN_W<'a, REG> = crate::BitWriter<'a, REG, GCEN_A>;
impl<'a, REG> GCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "General call disabled. Address 0b00000000 is NACKed."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GCEN_A::B_0x0)
    }
    #[doc = "General call enabled. Address 0b00000000 is ACKed."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GCEN_A::B_0x1)
    }
}
#[doc = "SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBHEN_A {
    #[doc = "0: Host address disabled. Address 0b0001000x is NACKed."]
    B_0x0 = 0,
    #[doc = "1: Host address enabled. Address 0b0001000x is ACKed."]
    B_0x1 = 1,
}
impl From<SMBHEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBHEN` reader - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type SMBHEN_R = crate::BitReader<SMBHEN_A>;
impl SMBHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMBHEN_A {
        match self.bits {
            false => SMBHEN_A::B_0x0,
            true => SMBHEN_A::B_0x1,
        }
    }
    #[doc = "Host address disabled. Address 0b0001000x is NACKed."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMBHEN_A::B_0x0
    }
    #[doc = "Host address enabled. Address 0b0001000x is ACKed."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMBHEN_A::B_0x1
    }
}
#[doc = "Field `SMBHEN` writer - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type SMBHEN_W<'a, REG> = crate::BitWriter<'a, REG, SMBHEN_A>;
impl<'a, REG> SMBHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host address disabled. Address 0b0001000x is NACKed."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMBHEN_A::B_0x0)
    }
    #[doc = "Host address enabled. Address 0b0001000x is ACKed."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMBHEN_A::B_0x1)
    }
}
#[doc = "SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBDEN_A {
    #[doc = "0: Device default address disabled. Address 0b1100001x is NACKed."]
    B_0x0 = 0,
    #[doc = "1: Device default address enabled. Address 0b1100001x is ACKed."]
    B_0x1 = 1,
}
impl From<SMBDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBDEN` reader - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type SMBDEN_R = crate::BitReader<SMBDEN_A>;
impl SMBDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMBDEN_A {
        match self.bits {
            false => SMBDEN_A::B_0x0,
            true => SMBDEN_A::B_0x1,
        }
    }
    #[doc = "Device default address disabled. Address 0b1100001x is NACKed."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMBDEN_A::B_0x0
    }
    #[doc = "Device default address enabled. Address 0b1100001x is ACKed."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMBDEN_A::B_0x1
    }
}
#[doc = "Field `SMBDEN` writer - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type SMBDEN_W<'a, REG> = crate::BitWriter<'a, REG, SMBDEN_A>;
impl<'a, REG> SMBDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device default address disabled. Address 0b1100001x is NACKed."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMBDEN_A::B_0x0)
    }
    #[doc = "Device default address enabled. Address 0b1100001x is ACKed."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMBDEN_A::B_0x1)
    }
}
#[doc = "SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTEN_A {
    #[doc = "0: The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK)."]
    B_0x0 = 0,
    #[doc = "1: The SMBus alert pin is supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK)."]
    B_0x1 = 1,
}
impl From<ALERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTEN` reader - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type ALERTEN_R = crate::BitReader<ALERTEN_A>;
impl ALERTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALERTEN_A {
        match self.bits {
            false => ALERTEN_A::B_0x0,
            true => ALERTEN_A::B_0x1,
        }
    }
    #[doc = "The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ALERTEN_A::B_0x0
    }
    #[doc = "The SMBus alert pin is supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ALERTEN_A::B_0x1
    }
}
#[doc = "Field `ALERTEN` writer - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type ALERTEN_W<'a, REG> = crate::BitWriter<'a, REG, ALERTEN_A>;
impl<'a, REG> ALERTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALERTEN_A::B_0x0)
    }
    #[doc = "The SMBus alert pin is supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALERTEN_A::B_0x1)
    }
}
#[doc = "PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECEN_A {
    #[doc = "0: PEC calculation disabled"]
    B_0x0 = 0,
    #[doc = "1: PEC calculation enabled"]
    B_0x1 = 1,
}
impl From<PECEN_A> for bool {
    #[inline(always)]
    fn from(variant: PECEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` reader - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type PECEN_R = crate::BitReader<PECEN_A>;
impl PECEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PECEN_A {
        match self.bits {
            false => PECEN_A::B_0x0,
            true => PECEN_A::B_0x1,
        }
    }
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PECEN_A::B_0x0
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PECEN_A::B_0x1
    }
}
#[doc = "Field `PECEN` writer - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type PECEN_W<'a, REG> = crate::BitWriter<'a, REG, PECEN_A>;
impl<'a, REG> PECEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PECEN_A::B_0x0)
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PECEN_A::B_0x1)
    }
}
#[doc = "Fast-mode Plus 20 mA drive enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMP_A {
    #[doc = "0: 20 mA I/O drive disabled"]
    B_0x0 = 0,
    #[doc = "1: 20 mA I/O drive enabled"]
    B_0x1 = 1,
}
impl From<FMP_A> for bool {
    #[inline(always)]
    fn from(variant: FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMP` reader - Fast-mode Plus 20 mA drive enable"]
pub type FMP_R = crate::BitReader<FMP_A>;
impl FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMP_A {
        match self.bits {
            false => FMP_A::B_0x0,
            true => FMP_A::B_0x1,
        }
    }
    #[doc = "20 mA I/O drive disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FMP_A::B_0x0
    }
    #[doc = "20 mA I/O drive enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FMP_A::B_0x1
    }
}
#[doc = "Field `FMP` writer - Fast-mode Plus 20 mA drive enable"]
pub type FMP_W<'a, REG> = crate::BitWriter<'a, REG, FMP_A>;
impl<'a, REG> FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "20 mA I/O drive disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FMP_A::B_0x0)
    }
    #[doc = "20 mA I/O drive enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FMP_A::B_0x1)
    }
}
#[doc = "Address match flag (ADDR) automatic clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRACLR_A {
    #[doc = "0: ADDR flag is set by hardware and cleared by software by setting ADDRCF bit."]
    B_0x0 = 0,
    #[doc = "1: ADDR flag remains cleared by hardware. This mode can be used in slave mode, to avoid the ADDR clock stretching if the I2C enables only one slave address. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    B_0x1 = 1,
}
impl From<ADDRACLR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRACLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRACLR` reader - Address match flag (ADDR) automatic clear"]
pub type ADDRACLR_R = crate::BitReader<ADDRACLR_A>;
impl ADDRACLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDRACLR_A {
        match self.bits {
            false => ADDRACLR_A::B_0x0,
            true => ADDRACLR_A::B_0x1,
        }
    }
    #[doc = "ADDR flag is set by hardware and cleared by software by setting ADDRCF bit."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADDRACLR_A::B_0x0
    }
    #[doc = "ADDR flag remains cleared by hardware. This mode can be used in slave mode, to avoid the ADDR clock stretching if the I2C enables only one slave address. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADDRACLR_A::B_0x1
    }
}
#[doc = "Field `ADDRACLR` writer - Address match flag (ADDR) automatic clear"]
pub type ADDRACLR_W<'a, REG> = crate::BitWriter<'a, REG, ADDRACLR_A>;
impl<'a, REG> ADDRACLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADDR flag is set by hardware and cleared by software by setting ADDRCF bit."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRACLR_A::B_0x0)
    }
    #[doc = "ADDR flag remains cleared by hardware. This mode can be used in slave mode, to avoid the ADDR clock stretching if the I2C enables only one slave address. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRACLR_A::B_0x1)
    }
}
#[doc = "STOP detection flag (STOPF) automatic clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPFACLR_A {
    #[doc = "0: STOPF flag is set by hardware and cleared by software by setting STOPCF bit."]
    B_0x0 = 0,
    #[doc = "1: STOPF flag remains cleared by hardware. This mode can be used in NOSTRETCH slave mode, to avoid the overrun error if the STOPF flag is not cleared before next data transmission. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    B_0x1 = 1,
}
impl From<STOPFACLR_A> for bool {
    #[inline(always)]
    fn from(variant: STOPFACLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPFACLR` reader - STOP detection flag (STOPF) automatic clear"]
pub type STOPFACLR_R = crate::BitReader<STOPFACLR_A>;
impl STOPFACLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPFACLR_A {
        match self.bits {
            false => STOPFACLR_A::B_0x0,
            true => STOPFACLR_A::B_0x1,
        }
    }
    #[doc = "STOPF flag is set by hardware and cleared by software by setting STOPCF bit."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STOPFACLR_A::B_0x0
    }
    #[doc = "STOPF flag remains cleared by hardware. This mode can be used in NOSTRETCH slave mode, to avoid the overrun error if the STOPF flag is not cleared before next data transmission. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STOPFACLR_A::B_0x1
    }
}
#[doc = "Field `STOPFACLR` writer - STOP detection flag (STOPF) automatic clear"]
pub type STOPFACLR_W<'a, REG> = crate::BitWriter<'a, REG, STOPFACLR_A>;
impl<'a, REG> STOPFACLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOPF flag is set by hardware and cleared by software by setting STOPCF bit."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(STOPFACLR_A::B_0x0)
    }
    #[doc = "STOPF flag remains cleared by hardware. This mode can be used in NOSTRETCH slave mode, to avoid the overrun error if the STOPF flag is not cleared before next data transmission. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(STOPFACLR_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
    #[inline(always)]
    pub fn PE(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    pub fn TXIE(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    pub fn RXIE(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match Interrupt enable (slave only)"]
    #[inline(always)]
    pub fn ADDRIE(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received Interrupt enable"]
    #[inline(always)]
    pub fn NACKIE(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection Interrupt enable"]
    #[inline(always)]
    pub fn STOPIE(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
    #[inline(always)]
    pub fn TCIE(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
    #[inline(always)]
    pub fn ERRIE(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\] * tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn DNF(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn ANFOFF(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    pub fn TXDMAEN(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    pub fn RXDMAEN(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode."]
    #[inline(always)]
    pub fn SBC(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn NOSTRETCH(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to . Note: WUPEN can be set only when DNF = '0000'"]
    #[inline(always)]
    pub fn WUPEN(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    pub fn GCEN(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn SMBHEN(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn SMBDEN(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn ALERTEN(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn PECEN(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Fast-mode Plus 20 mA drive enable"]
    #[inline(always)]
    pub fn FMP(&self) -> FMP_R {
        FMP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Address match flag (ADDR) automatic clear"]
    #[inline(always)]
    pub fn ADDRACLR(&self) -> ADDRACLR_R {
        ADDRACLR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - STOP detection flag (STOPF) automatic clear"]
    #[inline(always)]
    pub fn STOPFACLR(&self) -> STOPFACLR_R {
        STOPFACLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
    #[inline(always)]
    pub fn PE(&mut self) -> PE_W<'_, CR1_SPEC> {
        PE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    pub fn TXIE(&mut self) -> TXIE_W<'_, CR1_SPEC> {
        TXIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    pub fn RXIE(&mut self) -> RXIE_W<'_, CR1_SPEC> {
        RXIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Address match Interrupt enable (slave only)"]
    #[inline(always)]
    pub fn ADDRIE(&mut self) -> ADDRIE_W<'_, CR1_SPEC> {
        ADDRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Not acknowledge received Interrupt enable"]
    #[inline(always)]
    pub fn NACKIE(&mut self) -> NACKIE_W<'_, CR1_SPEC> {
        NACKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stop detection Interrupt enable"]
    #[inline(always)]
    pub fn STOPIE(&mut self) -> STOPIE_W<'_, CR1_SPEC> {
        STOPIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
    #[inline(always)]
    pub fn TCIE(&mut self) -> TCIE_W<'_, CR1_SPEC> {
        TCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
    #[inline(always)]
    pub fn ERRIE(&mut self) -> ERRIE_W<'_, CR1_SPEC> {
        ERRIE_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\] * tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn DNF(&mut self) -> DNF_W<'_, CR1_SPEC> {
        DNF_W::new(self, 8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn ANFOFF(&mut self) -> ANFOFF_W<'_, CR1_SPEC> {
        ANFOFF_W::new(self, 12)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    pub fn TXDMAEN(&mut self) -> TXDMAEN_W<'_, CR1_SPEC> {
        TXDMAEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    pub fn RXDMAEN(&mut self) -> RXDMAEN_W<'_, CR1_SPEC> {
        RXDMAEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode."]
    #[inline(always)]
    pub fn SBC(&mut self) -> SBC_W<'_, CR1_SPEC> {
        SBC_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn NOSTRETCH(&mut self) -> NOSTRETCH_W<'_, CR1_SPEC> {
        NOSTRETCH_W::new(self, 17)
    }
    #[doc = "Bit 18 - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to . Note: WUPEN can be set only when DNF = '0000'"]
    #[inline(always)]
    pub fn WUPEN(&mut self) -> WUPEN_W<'_, CR1_SPEC> {
        WUPEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    pub fn GCEN(&mut self) -> GCEN_W<'_, CR1_SPEC> {
        GCEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn SMBHEN(&mut self) -> SMBHEN_W<'_, CR1_SPEC> {
        SMBHEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn SMBDEN(&mut self) -> SMBDEN_W<'_, CR1_SPEC> {
        SMBDEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn ALERTEN(&mut self) -> ALERTEN_W<'_, CR1_SPEC> {
        ALERTEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn PECEN(&mut self) -> PECEN_W<'_, CR1_SPEC> {
        PECEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Fast-mode Plus 20 mA drive enable"]
    #[inline(always)]
    pub fn FMP(&mut self) -> FMP_W<'_, CR1_SPEC> {
        FMP_W::new(self, 24)
    }
    #[doc = "Bit 30 - Address match flag (ADDR) automatic clear"]
    #[inline(always)]
    pub fn ADDRACLR(&mut self) -> ADDRACLR_W<'_, CR1_SPEC> {
        ADDRACLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - STOP detection flag (STOPF) automatic clear"]
    #[inline(always)]
    pub fn STOPFACLR(&mut self) -> STOPFACLR_W<'_, CR1_SPEC> {
        STOPFACLR_W::new(self, 31)
    }
}
#[doc = "I2C control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {}

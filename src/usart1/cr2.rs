#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Synchronous Slave mode enable When the SLVEN bit is set, the Synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVEN_A {
    #[doc = "0: Slave mode disabled."]
    B_0x0 = 0,
    #[doc = "1: Slave mode enabled."]
    B_0x1 = 1,
}
impl From<SLVEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVEN` reader - Synchronous Slave mode enable When the SLVEN bit is set, the Synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SLVEN_R = crate::BitReader<SLVEN_A>;
impl SLVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLVEN_A {
        match self.bits {
            false => SLVEN_A::B_0x0,
            true => SLVEN_A::B_0x1,
        }
    }
    #[doc = "Slave mode disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SLVEN_A::B_0x0
    }
    #[doc = "Slave mode enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SLVEN_A::B_0x1
    }
}
#[doc = "Field `SLVEN` writer - Synchronous Slave mode enable When the SLVEN bit is set, the Synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SLVEN_W<'a, REG> = crate::BitWriter<'a, REG, SLVEN_A>;
impl<'a, REG> SLVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SLVEN_A::B_0x0)
    }
    #[doc = "Slave mode enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SLVEN_A::B_0x1)
    }
}
#[doc = "When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_NSS_A {
    #[doc = "0: SPI slave selection depends on NSS input pin."]
    B_0x0 = 0,
    #[doc = "1: SPI slave is always selected and NSS input pin is ignored."]
    B_0x1 = 1,
}
impl From<DIS_NSS_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_NSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_NSS` reader - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DIS_NSS_R = crate::BitReader<DIS_NSS_A>;
impl DIS_NSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIS_NSS_A {
        match self.bits {
            false => DIS_NSS_A::B_0x0,
            true => DIS_NSS_A::B_0x1,
        }
    }
    #[doc = "SPI slave selection depends on NSS input pin."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DIS_NSS_A::B_0x0
    }
    #[doc = "SPI slave is always selected and NSS input pin is ignored."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DIS_NSS_A::B_0x1
    }
}
#[doc = "Field `DIS_NSS` writer - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DIS_NSS_W<'a, REG> = crate::BitWriter<'a, REG, DIS_NSS_A>;
impl<'a, REG> DIS_NSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI slave selection depends on NSS input pin."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DIS_NSS_A::B_0x0)
    }
    #[doc = "SPI slave is always selected and NSS input pin is ignored."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DIS_NSS_A::B_0x1)
    }
}
#[doc = "7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\] and ADD\\[7:0\\]) respectively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDM7_A {
    #[doc = "0: 4-bit address detection"]
    B_0x0 = 0,
    #[doc = "1: 7-bit address detection (in 8-bit data mode)"]
    B_0x1 = 1,
}
impl From<ADDM7_A> for bool {
    #[inline(always)]
    fn from(variant: ADDM7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\] and ADD\\[7:0\\]) respectively."]
pub type ADDM7_R = crate::BitReader<ADDM7_A>;
impl ADDM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDM7_A {
        match self.bits {
            false => ADDM7_A::B_0x0,
            true => ADDM7_A::B_0x1,
        }
    }
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADDM7_A::B_0x0
    }
    #[doc = "7-bit address detection (in 8-bit data mode)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADDM7_A::B_0x1
    }
}
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\] and ADD\\[7:0\\]) respectively."]
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG, ADDM7_A>;
impl<'a, REG> ADDM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7_A::B_0x0)
    }
    #[doc = "7-bit address detection (in 8-bit data mode)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7_A::B_0x1)
    }
}
#[doc = "LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UE=0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDL_A {
    #[doc = "0: 10-bit break detection"]
    B_0x0 = 0,
    #[doc = "1: 11-bit break detection"]
    B_0x1 = 1,
}
impl From<LBDL_A> for bool {
    #[inline(always)]
    fn from(variant: LBDL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDL` reader - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UE=0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBDL_R = crate::BitReader<LBDL_A>;
impl LBDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBDL_A {
        match self.bits {
            false => LBDL_A::B_0x0,
            true => LBDL_A::B_0x1,
        }
    }
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LBDL_A::B_0x0
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LBDL_A::B_0x1
    }
}
#[doc = "Field `LBDL` writer - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UE=0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBDL_W<'a, REG> = crate::BitWriter<'a, REG, LBDL_A>;
impl<'a, REG> LBDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LBDL_A::B_0x0)
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LBDL_A::B_0x1)
    }
}
#[doc = "LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDIE_A {
    #[doc = "0: Interrupt is inhibited"]
    B_0x0 = 0,
    #[doc = "1: An interrupt is generated whenever LBDF=1 in the USART_ISR register"]
    B_0x1 = 1,
}
impl From<LBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBDIE_R = crate::BitReader<LBDIE_A>;
impl LBDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBDIE_A {
        match self.bits {
            false => LBDIE_A::B_0x0,
            true => LBDIE_A::B_0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LBDIE_A::B_0x0
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LBDIE_A::B_0x1
    }
}
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBDIE_W<'a, REG> = crate::BitWriter<'a, REG, LBDIE_A>;
impl<'a, REG> LBDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE_A::B_0x0)
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE_A::B_0x1)
    }
}
#[doc = "Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in Synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCL_A {
    #[doc = "0: The clock pulse of the last data bit is not output to the SCLK pin"]
    B_0x0 = 0,
    #[doc = "1: The clock pulse of the last data bit is output to the SCLK pin"]
    B_0x1 = 1,
}
impl From<LBCL_A> for bool {
    #[inline(always)]
    fn from(variant: LBCL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCL` reader - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in Synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBCL_R = crate::BitReader<LBCL_A>;
impl LBCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBCL_A {
        match self.bits {
            false => LBCL_A::B_0x0,
            true => LBCL_A::B_0x1,
        }
    }
    #[doc = "The clock pulse of the last data bit is not output to the SCLK pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LBCL_A::B_0x0
    }
    #[doc = "The clock pulse of the last data bit is output to the SCLK pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LBCL_A::B_0x1
    }
}
#[doc = "Field `LBCL` writer - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in Synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBCL_W<'a, REG> = crate::BitWriter<'a, REG, LBCL_A>;
impl<'a, REG> LBCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock pulse of the last data bit is not output to the SCLK pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LBCL_A::B_0x0)
    }
    #[doc = "The clock pulse of the last data bit is output to the SCLK pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LBCL_A::B_0x1)
    }
}
#[doc = "Clock phase This bit is used to select the phase of the clock output on the SCLK pin in Synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: The first clock transition is the first data capture edge"]
    B_0x0 = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    B_0x1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in Synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CPHA_R = crate::BitReader<CPHA_A>;
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::B_0x0,
            true => CPHA_A::B_0x1,
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CPHA_A::B_0x0
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CPHA_A::B_0x1
    }
}
#[doc = "Field `CPHA` writer - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in Synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA_A>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::B_0x0)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::B_0x1)
    }
}
#[doc = "Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in Synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: Steady low value on SCLK pin outside transmission window"]
    B_0x0 = 0,
    #[doc = "1: Steady high value on SCLK pin outside transmission window"]
    B_0x1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in Synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CPOL_R = crate::BitReader<CPOL_A>;
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::B_0x0,
            true => CPOL_A::B_0x1,
        }
    }
    #[doc = "Steady low value on SCLK pin outside transmission window"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CPOL_A::B_0x0
    }
    #[doc = "Steady high value on SCLK pin outside transmission window"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CPOL_A::B_0x1
    }
}
#[doc = "Field `CPOL` writer - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in Synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL_A>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Steady low value on SCLK pin outside transmission window"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::B_0x0)
    }
    #[doc = "Steady high value on SCLK pin outside transmission window"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::B_0x1)
    }
}
#[doc = "Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UE=0). Note: If neither Synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    #[doc = "0: SCLK pin disabled"]
    B_0x0 = 0,
    #[doc = "1: SCLK pin enabled"]
    B_0x1 = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UE=0). Note: If neither Synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::B_0x0,
            true => CLKEN_A::B_0x1,
        }
    }
    #[doc = "SCLK pin disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CLKEN_A::B_0x0
    }
    #[doc = "SCLK pin enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CLKEN_A::B_0x1
    }
}
#[doc = "Field `CLKEN` writer - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UE=0). Note: If neither Synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKEN_A>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCLK pin disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN_A::B_0x0)
    }
    #[doc = "SCLK pin enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN_A::B_0x1)
    }
}
#[doc = "stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP_A {
    #[doc = "0: 1 stop bit"]
    B_0x0 = 0,
    #[doc = "1: 0.5 stop bit."]
    B_0x1 = 1,
    #[doc = "2: 2 stop bits"]
    B_0x2 = 2,
    #[doc = "3: 1.5 stop bits"]
    B_0x3 = 3,
}
impl From<STOP_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOP_A {
    type Ux = u8;
}
impl crate::IsEnum for STOP_A {}
#[doc = "Field `STOP` reader - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type STOP_R = crate::FieldReader<STOP_A>;
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_A {
        match self.bits {
            0 => STOP_A::B_0x0,
            1 => STOP_A::B_0x1,
            2 => STOP_A::B_0x2,
            3 => STOP_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STOP_A::B_0x0
    }
    #[doc = "0.5 stop bit."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STOP_A::B_0x1
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == STOP_A::B_0x2
    }
    #[doc = "1.5 stop bits"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == STOP_A::B_0x3
    }
}
#[doc = "Field `STOP` writer - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STOP_A, crate::Safe>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::B_0x0)
    }
    #[doc = "0.5 stop bit."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::B_0x1)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::B_0x2)
    }
    #[doc = "1.5 stop bits"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::B_0x3)
    }
}
#[doc = "LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEN_A {
    #[doc = "0: LIN mode disabled"]
    B_0x0 = 0,
    #[doc = "1: LIN mode enabled"]
    B_0x1 = 1,
}
impl From<LINEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINEN` reader - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LINEN_R = crate::BitReader<LINEN_A>;
impl LINEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINEN_A {
        match self.bits {
            false => LINEN_A::B_0x0,
            true => LINEN_A::B_0x1,
        }
    }
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LINEN_A::B_0x0
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LINEN_A::B_0x1
    }
}
#[doc = "Field `LINEN` writer - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG, LINEN_A>;
impl<'a, REG> LINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LINEN_A::B_0x0)
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LINEN_A::B_0x1)
    }
}
#[doc = "Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP_A {
    #[doc = "0: TX/RX pins are used as defined in standard pinout"]
    B_0x0 = 0,
    #[doc = "1: The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    B_0x1 = 1,
}
impl From<SWAP_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP` reader - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type SWAP_R = crate::BitReader<SWAP_A>;
impl SWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWAP_A {
        match self.bits {
            false => SWAP_A::B_0x0,
            true => SWAP_A::B_0x1,
        }
    }
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWAP_A::B_0x0
    }
    #[doc = "The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWAP_A::B_0x1
    }
}
#[doc = "Field `SWAP` writer - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG, SWAP_A>;
impl<'a, REG> SWAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP_A::B_0x0)
    }
    #[doc = "The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP_A::B_0x1)
    }
}
#[doc = "RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV_A {
    #[doc = "0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    B_0x0 = 0,
    #[doc = "1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    B_0x1 = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type RXINV_R = crate::BitReader<RXINV_A>;
impl RXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::B_0x0,
            true => RXINV_A::B_0x1,
        }
    }
    #[doc = "RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXINV_A::B_0x0
    }
    #[doc = "RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXINV_A::B_0x1
    }
}
#[doc = "Field `RXINV` writer - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG, RXINV_A>;
impl<'a, REG> RXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV_A::B_0x0)
    }
    #[doc = "RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV_A::B_0x1)
    }
}
#[doc = "TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV_A {
    #[doc = "0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    B_0x0 = 0,
    #[doc = "1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    B_0x1 = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type TXINV_R = crate::BitReader<TXINV_A>;
impl TXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::B_0x0,
            true => TXINV_A::B_0x1,
        }
    }
    #[doc = "TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXINV_A::B_0x0
    }
    #[doc = "TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXINV_A::B_0x1
    }
}
#[doc = "Field `TXINV` writer - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG, TXINV_A>;
impl<'a, REG> TXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV_A::B_0x0)
    }
    #[doc = "TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV_A::B_0x1)
    }
}
#[doc = "Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAINV_A {
    #[doc = "0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L)"]
    B_0x0 = 0,
    #[doc = "1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted."]
    B_0x1 = 1,
}
impl From<DATAINV_A> for bool {
    #[inline(always)]
    fn from(variant: DATAINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAINV` reader - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type DATAINV_R = crate::BitReader<DATAINV_A>;
impl DATAINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATAINV_A {
        match self.bits {
            false => DATAINV_A::B_0x0,
            true => DATAINV_A::B_0x1,
        }
    }
    #[doc = "Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DATAINV_A::B_0x0
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DATAINV_A::B_0x1
    }
}
#[doc = "Field `DATAINV` writer - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type DATAINV_W<'a, REG> = crate::BitWriter<'a, REG, DATAINV_A>;
impl<'a, REG> DATAINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV_A::B_0x0)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV_A::B_0x1)
    }
}
#[doc = "Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBFIRST_A {
    #[doc = "0: data is transmitted/received with data bit 0 first, following the start bit."]
    B_0x0 = 0,
    #[doc = "1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    B_0x1 = 1,
}
impl From<MSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: MSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBFIRST` reader - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type MSBFIRST_R = crate::BitReader<MSBFIRST_A>;
impl MSBFIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSBFIRST_A {
        match self.bits {
            false => MSBFIRST_A::B_0x0,
            true => MSBFIRST_A::B_0x1,
        }
    }
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MSBFIRST_A::B_0x0
    }
    #[doc = "data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MSBFIRST_A::B_0x1
    }
}
#[doc = "Field `MSBFIRST` writer - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, MSBFIRST_A>;
impl<'a, REG> MSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST_A::B_0x0)
    }
    #[doc = "data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST_A::B_0x1)
    }
}
#[doc = "Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABREN_A {
    #[doc = "0: Auto baud rate detection is disabled."]
    B_0x0 = 0,
    #[doc = "1: Auto baud rate detection is enabled."]
    B_0x1 = 1,
}
impl From<ABREN_A> for bool {
    #[inline(always)]
    fn from(variant: ABREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABREN` reader - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type ABREN_R = crate::BitReader<ABREN_A>;
impl ABREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABREN_A {
        match self.bits {
            false => ABREN_A::B_0x0,
            true => ABREN_A::B_0x1,
        }
    }
    #[doc = "Auto baud rate detection is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ABREN_A::B_0x0
    }
    #[doc = "Auto baud rate detection is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ABREN_A::B_0x1
    }
}
#[doc = "Field `ABREN` writer - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type ABREN_W<'a, REG> = crate::BitWriter<'a, REG, ABREN_A>;
impl<'a, REG> ABREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto baud rate detection is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ABREN_A::B_0x0)
    }
    #[doc = "Auto baud rate detection is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ABREN_A::B_0x1)
    }
}
#[doc = "Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UE=0). Note: If DATAINV=1 and/or MSBFIRST=1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABRMOD_A {
    #[doc = "0: Measurement of the start bit is used to detect the baud rate."]
    B_0x0 = 0,
    #[doc = "1: Falling edge to falling edge measurement (the received frame must start with a single bit = 1 - Frame = Start10xxxxxx)"]
    B_0x1 = 1,
    #[doc = "2: 0x7F frame detection."]
    B_0x2 = 2,
    #[doc = "3: 0x55 frame detection"]
    B_0x3 = 3,
}
impl From<ABRMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: ABRMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ABRMOD_A {
    type Ux = u8;
}
impl crate::IsEnum for ABRMOD_A {}
#[doc = "Field `ABRMOD` reader - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UE=0). Note: If DATAINV=1 and/or MSBFIRST=1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type ABRMOD_R = crate::FieldReader<ABRMOD_A>;
impl ABRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABRMOD_A {
        match self.bits {
            0 => ABRMOD_A::B_0x0,
            1 => ABRMOD_A::B_0x1,
            2 => ABRMOD_A::B_0x2,
            3 => ABRMOD_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Measurement of the start bit is used to detect the baud rate."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ABRMOD_A::B_0x0
    }
    #[doc = "Falling edge to falling edge measurement (the received frame must start with a single bit = 1 - Frame = Start10xxxxxx)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ABRMOD_A::B_0x1
    }
    #[doc = "0x7F frame detection."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ABRMOD_A::B_0x2
    }
    #[doc = "0x55 frame detection"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ABRMOD_A::B_0x3
    }
}
#[doc = "Field `ABRMOD` writer - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UE=0). Note: If DATAINV=1 and/or MSBFIRST=1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type ABRMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ABRMOD_A, crate::Safe>;
impl<'a, REG> ABRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Measurement of the start bit is used to detect the baud rate."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD_A::B_0x0)
    }
    #[doc = "Falling edge to falling edge measurement (the received frame must start with a single bit = 1 - Frame = Start10xxxxxx)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD_A::B_0x1)
    }
    #[doc = "0x7F frame detection."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD_A::B_0x2)
    }
    #[doc = "0x55 frame detection"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD_A::B_0x3)
    }
}
#[doc = "Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOEN_A {
    #[doc = "0: Receiver timeout feature disabled."]
    B_0x0 = 0,
    #[doc = "1: Receiver timeout feature enabled."]
    B_0x1 = 1,
}
impl From<RTOEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOEN` reader - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type RTOEN_R = crate::BitReader<RTOEN_A>;
impl RTOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTOEN_A {
        match self.bits {
            false => RTOEN_A::B_0x0,
            true => RTOEN_A::B_0x1,
        }
    }
    #[doc = "Receiver timeout feature disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RTOEN_A::B_0x0
    }
    #[doc = "Receiver timeout feature enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RTOEN_A::B_0x1
    }
}
#[doc = "Field `RTOEN` writer - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type RTOEN_W<'a, REG> = crate::BitWriter<'a, REG, RTOEN_A>;
impl<'a, REG> RTOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver timeout feature disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTOEN_A::B_0x0)
    }
    #[doc = "Receiver timeout feature enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTOEN_A::B_0x1)
    }
}
#[doc = "Field `ADD` reader - Address of the USART node These bits give the address of the USART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wakeup from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\] bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\] is programmed to 0b00 (WUF active on address match), the wakeup from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\] or ADD\\[3:0\\] bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\] value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE = 0) or when the USART is disabled (UE = 0)."]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - Address of the USART node These bits give the address of the USART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wakeup from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\] bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\] is programmed to 0b00 (WUF active on address match), the wakeup from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\] or ADD\\[3:0\\] bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\] value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE = 0) or when the USART is disabled (UE = 0)."]
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Synchronous Slave mode enable When the SLVEN bit is set, the Synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn SLVEN(&self) -> SLVEN_R {
        SLVEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn DIS_NSS(&self) -> DIS_NSS_R {
        DIS_NSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\] and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    pub fn ADDM7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UE=0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn LBDL(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn LBDIE(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in Synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn LBCL(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in Synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn CPHA(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in Synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn CPOL(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UE=0). Note: If neither Synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
    #[inline(always)]
    pub fn CLKEN(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn STOP(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn LINEN(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn SWAP(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn RXINV(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn TXINV(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn DATAINV(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn MSBFIRST(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ABREN(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UE=0). Note: If DATAINV=1 and/or MSBFIRST=1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ABRMOD(&self) -> ABRMOD_R {
        ABRMOD_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn RTOEN(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the USART node These bits give the address of the USART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wakeup from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\] bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\] is programmed to 0b00 (WUF active on address match), the wakeup from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\] or ADD\\[3:0\\] bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\] value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE = 0) or when the USART is disabled (UE = 0)."]
    #[inline(always)]
    pub fn ADD(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Slave mode enable When the SLVEN bit is set, the Synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn SLVEN(&mut self) -> SLVEN_W<'_, CR2_SPEC> {
        SLVEN_W::new(self, 0)
    }
    #[doc = "Bit 3 - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn DIS_NSS(&mut self) -> DIS_NSS_W<'_, CR2_SPEC> {
        DIS_NSS_W::new(self, 3)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\] and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    pub fn ADDM7(&mut self) -> ADDM7_W<'_, CR2_SPEC> {
        ADDM7_W::new(self, 4)
    }
    #[doc = "Bit 5 - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UE=0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn LBDL(&mut self) -> LBDL_W<'_, CR2_SPEC> {
        LBDL_W::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn LBDIE(&mut self) -> LBDIE_W<'_, CR2_SPEC> {
        LBDIE_W::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in Synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn LBCL(&mut self) -> LBCL_W<'_, CR2_SPEC> {
        LBCL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in Synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn CPHA(&mut self) -> CPHA_W<'_, CR2_SPEC> {
        CPHA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in Synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UE=0). Note: If Synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn CPOL(&mut self) -> CPOL_W<'_, CR2_SPEC> {
        CPOL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UE=0). Note: If neither Synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
    #[inline(always)]
    pub fn CLKEN(&mut self) -> CLKEN_W<'_, CR2_SPEC> {
        CLKEN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn STOP(&mut self) -> STOP_W<'_, CR2_SPEC> {
        STOP_W::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn LINEN(&mut self) -> LINEN_W<'_, CR2_SPEC> {
        LINEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn SWAP(&mut self) -> SWAP_W<'_, CR2_SPEC> {
        SWAP_W::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn RXINV(&mut self) -> RXINV_W<'_, CR2_SPEC> {
        RXINV_W::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn TXINV(&mut self) -> TXINV_W<'_, CR2_SPEC> {
        TXINV_W::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn DATAINV(&mut self) -> DATAINV_W<'_, CR2_SPEC> {
        DATAINV_W::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn MSBFIRST(&mut self) -> MSBFIRST_W<'_, CR2_SPEC> {
        MSBFIRST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ABREN(&mut self) -> ABREN_W<'_, CR2_SPEC> {
        ABREN_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UE=0). Note: If DATAINV=1 and/or MSBFIRST=1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ABRMOD(&mut self) -> ABRMOD_W<'_, CR2_SPEC> {
        ABRMOD_W::new(self, 21)
    }
    #[doc = "Bit 23 - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn RTOEN(&mut self) -> RTOEN_W<'_, CR2_SPEC> {
        RTOEN_W::new(self, 23)
    }
    #[doc = "Bits 24:31 - Address of the USART node These bits give the address of the USART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wakeup from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\] bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\] is programmed to 0b00 (WUF active on address match), the wakeup from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\] or ADD\\[3:0\\] bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\] value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE = 0) or when the USART is disabled (UE = 0)."]
    #[inline(always)]
    pub fn ADD(&mut self) -> ADD_W<'_, CR2_SPEC> {
        ADD_W::new(self, 24)
    }
}
#[doc = "USART control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {}

#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3_SPEC>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3_SPEC>;
#[doc = "Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1or UDR = 1 in the USART_ISR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0x0 = 0,
    #[doc = "1: interrupt generated when FE=1 or ORE=1 or NE=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
    B_0x1 = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIE` reader - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1or UDR = 1 in the USART_ISR register)."]
pub type EIE_R = crate::BitReader<EIE_A>;
impl EIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::B_0x0,
            true => EIE_A::B_0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EIE_A::B_0x0
    }
    #[doc = "interrupt generated when FE=1 or ORE=1 or NE=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EIE_A::B_0x1
    }
}
#[doc = "Field `EIE` writer - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1or UDR = 1 in the USART_ISR register)."]
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG, EIE_A>;
impl<'a, REG> EIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EIE_A::B_0x0)
    }
    #[doc = "interrupt generated when FE=1 or ORE=1 or NE=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EIE_A::B_0x1)
    }
}
#[doc = "IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN_A {
    #[doc = "0: IrDA disabled"]
    B_0x0 = 0,
    #[doc = "1: IrDA enabled"]
    B_0x1 = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type IREN_R = crate::BitReader<IREN_A>;
impl IREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::B_0x0,
            true => IREN_A::B_0x1,
        }
    }
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IREN_A::B_0x0
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IREN_A::B_0x1
    }
}
#[doc = "Field `IREN` writer - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG, IREN_A>;
impl<'a, REG> IREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IREN_A::B_0x0)
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IREN_A::B_0x1)
    }
}
#[doc = "IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRLP_A {
    #[doc = "0: Normal mode"]
    B_0x0 = 0,
    #[doc = "1: Low-power mode"]
    B_0x1 = 1,
}
impl From<IRLP_A> for bool {
    #[inline(always)]
    fn from(variant: IRLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRLP` reader - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type IRLP_R = crate::BitReader<IRLP_A>;
impl IRLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IRLP_A {
        match self.bits {
            false => IRLP_A::B_0x0,
            true => IRLP_A::B_0x1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IRLP_A::B_0x0
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IRLP_A::B_0x1
    }
}
#[doc = "Field `IRLP` writer - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type IRLP_W<'a, REG> = crate::BitWriter<'a, REG, IRLP_A>;
impl<'a, REG> IRLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IRLP_A::B_0x0)
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IRLP_A::B_0x1)
    }
}
#[doc = "Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSEL_A {
    #[doc = "0: Half-duplex mode is not selected"]
    B_0x0 = 0,
    #[doc = "1: Half-duplex mode is selected"]
    B_0x1 = 1,
}
impl From<HDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSEL` reader - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0)."]
pub type HDSEL_R = crate::BitReader<HDSEL_A>;
impl HDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDSEL_A {
        match self.bits {
            false => HDSEL_A::B_0x0,
            true => HDSEL_A::B_0x1,
        }
    }
    #[doc = "Half-duplex mode is not selected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HDSEL_A::B_0x0
    }
    #[doc = "Half-duplex mode is selected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HDSEL_A::B_0x1
    }
}
#[doc = "Field `HDSEL` writer - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0)."]
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG, HDSEL_A>;
impl<'a, REG> HDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half-duplex mode is not selected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL_A::B_0x0)
    }
    #[doc = "Half-duplex mode is selected"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL_A::B_0x1)
    }
}
#[doc = "Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACK_A {
    #[doc = "0: NACK transmission in case of parity error is disabled"]
    B_0x0 = 0,
    #[doc = "1: NACK transmission during parity error is enabled"]
    B_0x1 = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type NACK_R = crate::BitReader<NACK_A>;
impl NACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::B_0x0,
            true => NACK_A::B_0x1,
        }
    }
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NACK_A::B_0x0
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NACK_A::B_0x1
    }
}
#[doc = "Field `NACK` writer - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG, NACK_A>;
impl<'a, REG> NACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NACK_A::B_0x0)
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NACK_A::B_0x1)
    }
}
#[doc = "Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCEN_A {
    #[doc = "0: Smartcard mode disabled"]
    B_0x0 = 0,
    #[doc = "1: Smartcard mode enabled"]
    B_0x1 = 1,
}
impl From<SCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCEN` reader - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SCEN_R = crate::BitReader<SCEN_A>;
impl SCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCEN_A {
        match self.bits {
            false => SCEN_A::B_0x0,
            true => SCEN_A::B_0x1,
        }
    }
    #[doc = "Smartcard mode disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SCEN_A::B_0x0
    }
    #[doc = "Smartcard mode enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SCEN_A::B_0x1
    }
}
#[doc = "Field `SCEN` writer - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG, SCEN_A>;
impl<'a, REG> SCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Smartcard mode disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SCEN_A::B_0x0)
    }
    #[doc = "Smartcard mode enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SCEN_A::B_0x1)
    }
}
#[doc = "DMA enable receiver This bit is set/reset by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAR_A {
    #[doc = "1: DMA mode is enabled for reception"]
    B_0x1 = 1,
    #[doc = "0: DMA mode is disabled for reception"]
    B_0x0 = 0,
}
impl From<DMAR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAR` reader - DMA enable receiver This bit is set/reset by software"]
pub type DMAR_R = crate::BitReader<DMAR_A>;
impl DMAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAR_A {
        match self.bits {
            true => DMAR_A::B_0x1,
            false => DMAR_A::B_0x0,
        }
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAR_A::B_0x1
    }
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAR_A::B_0x0
    }
}
#[doc = "Field `DMAR` writer - DMA enable receiver This bit is set/reset by software"]
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG, DMAR_A>;
impl<'a, REG> DMAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR_A::B_0x1)
    }
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR_A::B_0x0)
    }
}
#[doc = "DMA enable transmitter This bit is set/reset by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAT_A {
    #[doc = "1: DMA mode is enabled for transmission"]
    B_0x1 = 1,
    #[doc = "0: DMA mode is disabled for transmission"]
    B_0x0 = 0,
}
impl From<DMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAT` reader - DMA enable transmitter This bit is set/reset by software"]
pub type DMAT_R = crate::BitReader<DMAT_A>;
impl DMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAT_A {
        match self.bits {
            true => DMAT_A::B_0x1,
            false => DMAT_A::B_0x0,
        }
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAT_A::B_0x1
    }
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAT_A::B_0x0
    }
}
#[doc = "Field `DMAT` writer - DMA enable transmitter This bit is set/reset by software"]
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG, DMAT_A>;
impl<'a, REG> DMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT_A::B_0x1)
    }
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT_A::B_0x0)
    }
}
#[doc = "RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSE_A {
    #[doc = "0: RTS hardware flow control disabled"]
    B_0x0 = 0,
    #[doc = "1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received."]
    B_0x1 = 1,
}
impl From<RTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RTSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSE` reader - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type RTSE_R = crate::BitReader<RTSE_A>;
impl RTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTSE_A {
        match self.bits {
            false => RTSE_A::B_0x0,
            true => RTSE_A::B_0x1,
        }
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RTSE_A::B_0x0
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RTSE_A::B_0x1
    }
}
#[doc = "Field `RTSE` writer - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG, RTSE_A>;
impl<'a, REG> RTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE_A::B_0x0)
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE_A::B_0x1)
    }
}
#[doc = "CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE_A {
    #[doc = "0: CTS hardware flow control disabled"]
    B_0x0 = 0,
    #[doc = "1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted."]
    B_0x1 = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSE` reader - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CTSE_R = crate::BitReader<CTSE_A>;
impl CTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::B_0x0,
            true => CTSE_A::B_0x1,
        }
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CTSE_A::B_0x0
    }
    #[doc = "CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CTSE_A::B_0x1
    }
}
#[doc = "Field `CTSE` writer - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG, CTSE_A>;
impl<'a, REG> CTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE_A::B_0x0)
    }
    #[doc = "CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE_A::B_0x1)
    }
}
#[doc = "CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIE_A {
    #[doc = "0: Interrupt is inhibited"]
    B_0x0 = 0,
    #[doc = "1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register"]
    B_0x1 = 1,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIE` reader - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CTSIE_R = crate::BitReader<CTSIE_A>;
impl CTSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::B_0x0,
            true => CTSIE_A::B_0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CTSIE_A::B_0x0
    }
    #[doc = "An interrupt is generated whenever CTSIF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CTSIE_A::B_0x1
    }
}
#[doc = "Field `CTSIE` writer - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG, CTSIE_A>;
impl<'a, REG> CTSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE_A::B_0x0)
    }
    #[doc = "An interrupt is generated whenever CTSIF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE_A::B_0x1)
    }
}
#[doc = "One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONEBIT_A {
    #[doc = "0: Three sample bit method"]
    B_0x0 = 0,
    #[doc = "1: One sample bit method"]
    B_0x1 = 1,
}
impl From<ONEBIT_A> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONEBIT` reader - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0)."]
pub type ONEBIT_R = crate::BitReader<ONEBIT_A>;
impl ONEBIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ONEBIT_A {
        match self.bits {
            false => ONEBIT_A::B_0x0,
            true => ONEBIT_A::B_0x1,
        }
    }
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ONEBIT_A::B_0x0
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ONEBIT_A::B_0x1
    }
}
#[doc = "Field `ONEBIT` writer - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0)."]
pub type ONEBIT_W<'a, REG> = crate::BitWriter<'a, REG, ONEBIT_A>;
impl<'a, REG> ONEBIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ONEBIT_A::B_0x0)
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ONEBIT_A::B_0x1)
    }
}
#[doc = "Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRDIS_A {
    #[doc = "0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data."]
    B_0x0 = 0,
    #[doc = "1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set"]
    B_0x1 = 1,
}
impl From<OVRDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRDIS` reader - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data"]
pub type OVRDIS_R = crate::BitReader<OVRDIS_A>;
impl OVRDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRDIS_A {
        match self.bits {
            false => OVRDIS_A::B_0x0,
            true => OVRDIS_A::B_0x1,
        }
    }
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OVRDIS_A::B_0x0
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OVRDIS_A::B_0x1
    }
}
#[doc = "Field `OVRDIS` writer - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data"]
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG, OVRDIS_A>;
impl<'a, REG> OVRDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRDIS_A::B_0x0)
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRDIS_A::B_0x1)
    }
}
#[doc = "DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDRE_A {
    #[doc = "0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred. (used for Smartcard mode)"]
    B_0x0 = 0,
    #[doc = "1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag."]
    B_0x1 = 1,
}
impl From<DDRE_A> for bool {
    #[inline(always)]
    fn from(variant: DDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
pub type DDRE_R = crate::BitReader<DDRE_A>;
impl DDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDRE_A {
        match self.bits {
            false => DDRE_A::B_0x0,
            true => DDRE_A::B_0x1,
        }
    }
    #[doc = "DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred. (used for Smartcard mode)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DDRE_A::B_0x0
    }
    #[doc = "DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DDRE_A::B_0x1
    }
}
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG, DDRE_A>;
impl<'a, REG> DDRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred. (used for Smartcard mode)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DDRE_A::B_0x0)
    }
    #[doc = "DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DDRE_A::B_0x1)
    }
}
#[doc = "Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEM_A {
    #[doc = "0: DE function is disabled."]
    B_0x0 = 0,
    #[doc = "1: DE function is enabled. The DE signal is output on the RTS pin."]
    B_0x1 = 1,
}
impl From<DEM_A> for bool {
    #[inline(always)]
    fn from(variant: DEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEM` reader - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
pub type DEM_R = crate::BitReader<DEM_A>;
impl DEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEM_A {
        match self.bits {
            false => DEM_A::B_0x0,
            true => DEM_A::B_0x1,
        }
    }
    #[doc = "DE function is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DEM_A::B_0x0
    }
    #[doc = "DE function is enabled. The DE signal is output on the RTS pin."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DEM_A::B_0x1
    }
}
#[doc = "Field `DEM` writer - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG, DEM_A>;
impl<'a, REG> DEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE function is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DEM_A::B_0x0)
    }
    #[doc = "DE function is enabled. The DE signal is output on the RTS pin."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DEM_A::B_0x1)
    }
}
#[doc = "Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEP_A {
    #[doc = "0: DE signal is active high."]
    B_0x0 = 0,
    #[doc = "1: DE signal is active low."]
    B_0x1 = 1,
}
impl From<DEP_A> for bool {
    #[inline(always)]
    fn from(variant: DEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEP` reader - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DEP_R = crate::BitReader<DEP_A>;
impl DEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEP_A {
        match self.bits {
            false => DEP_A::B_0x0,
            true => DEP_A::B_0x1,
        }
    }
    #[doc = "DE signal is active high."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DEP_A::B_0x0
    }
    #[doc = "DE signal is active low."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DEP_A::B_0x1
    }
}
#[doc = "Field `DEP` writer - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG, DEP_A>;
impl<'a, REG> DEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE signal is active high."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DEP_A::B_0x0)
    }
    #[doc = "DE signal is active low."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DEP_A::B_0x1)
    }
}
#[doc = "Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In Transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In Reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCARCNT_A {
    #[doc = "0: retransmission disabled - No automatic retransmission in Transmission mode."]
    B_0x0 = 0,
    #[doc = "1: number of automatic retransmission attempts (before signaling error)"]
    B_0x1 = 1,
    #[doc = "2: number of automatic retransmission attempts (before signaling error)"]
    B_0x2 = 2,
    #[doc = "3: number of automatic retransmission attempts (before signaling error)"]
    B_0x3 = 3,
    #[doc = "4: number of automatic retransmission attempts (before signaling error)"]
    B_0x4 = 4,
    #[doc = "5: number of automatic retransmission attempts (before signaling error)"]
    B_0x5 = 5,
    #[doc = "6: number of automatic retransmission attempts (before signaling error)"]
    B_0x6 = 6,
    #[doc = "7: number of automatic retransmission attempts (before signaling error)"]
    B_0x7 = 7,
}
impl From<SCARCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SCARCNT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCARCNT_A {
    type Ux = u8;
}
impl crate::IsEnum for SCARCNT_A {}
#[doc = "Field `SCARCNT` reader - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In Transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In Reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SCARCNT_R = crate::FieldReader<SCARCNT_A>;
impl SCARCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCARCNT_A {
        match self.bits {
            0 => SCARCNT_A::B_0x0,
            1 => SCARCNT_A::B_0x1,
            2 => SCARCNT_A::B_0x2,
            3 => SCARCNT_A::B_0x3,
            4 => SCARCNT_A::B_0x4,
            5 => SCARCNT_A::B_0x5,
            6 => SCARCNT_A::B_0x6,
            7 => SCARCNT_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "retransmission disabled - No automatic retransmission in Transmission mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SCARCNT_A::B_0x0
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SCARCNT_A::B_0x1
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SCARCNT_A::B_0x2
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SCARCNT_A::B_0x3
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SCARCNT_A::B_0x4
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SCARCNT_A::B_0x5
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SCARCNT_A::B_0x6
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SCARCNT_A::B_0x7
    }
}
#[doc = "Field `SCARCNT` writer - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In Transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In Reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SCARCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SCARCNT_A, crate::Safe>;
impl<'a, REG> SCARCNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "retransmission disabled - No automatic retransmission in Transmission mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SCARCNT_A::B_0x0)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SCARCNT_A::B_0x1)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SCARCNT_A::B_0x2)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SCARCNT_A::B_0x3)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SCARCNT_A::B_0x4)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SCARCNT_A::B_0x5)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SCARCNT_A::B_0x6)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SCARCNT_A::B_0x7)
    }
}
#[doc = "Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUS0_A {
    #[doc = "0: WUF active on address match (as defined by ADD\\[7:0\\] and ADDM7)"]
    B_0x0 = 0,
}
impl From<WUS0_A> for bool {
    #[inline(always)]
    fn from(variant: WUS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUS0` reader - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
pub type WUS0_R = crate::BitReader<WUS0_A>;
impl WUS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUS0_A> {
        match self.bits {
            false => Some(WUS0_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "WUF active on address match (as defined by ADD\\[7:0\\] and ADDM7)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUS0_A::B_0x0
    }
}
#[doc = "Field `WUS0` writer - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
pub type WUS0_W<'a, REG> = crate::BitWriter<'a, REG, WUS0_A>;
impl<'a, REG> WUS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WUF active on address match (as defined by ADD\\[7:0\\] and ADDM7)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUS0_A::B_0x0)
    }
}
#[doc = "Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUS1_A {
    #[doc = "0: WUF active on address match (as defined by ADD\\[7:0\\] and ADDM7)"]
    B_0x0 = 0,
}
impl From<WUS1_A> for bool {
    #[inline(always)]
    fn from(variant: WUS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUS1` reader - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
pub type WUS1_R = crate::BitReader<WUS1_A>;
impl WUS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUS1_A> {
        match self.bits {
            false => Some(WUS1_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "WUF active on address match (as defined by ADD\\[7:0\\] and ADDM7)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUS1_A::B_0x0
    }
}
#[doc = "Field `WUS1` writer - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
pub type WUS1_W<'a, REG> = crate::BitWriter<'a, REG, WUS1_A>;
impl<'a, REG> WUS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WUF active on address match (as defined by ADD\\[7:0\\] and ADDM7)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUS1_A::B_0x0)
    }
}
#[doc = "Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0x0 = 0,
    #[doc = "1: USART interrupt generated whenever WUF=1 in the USART_ISR register"]
    B_0x1 = 1,
}
impl From<WUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUFIE` reader - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
pub type WUFIE_R = crate::BitReader<WUFIE_A>;
impl WUFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUFIE_A {
        match self.bits {
            false => WUFIE_A::B_0x0,
            true => WUFIE_A::B_0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUFIE_A::B_0x0
    }
    #[doc = "USART interrupt generated whenever WUF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUFIE_A::B_0x1
    }
}
#[doc = "Field `WUFIE` writer - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
pub type WUFIE_W<'a, REG> = crate::BitWriter<'a, REG, WUFIE_A>;
impl<'a, REG> WUFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUFIE_A::B_0x0)
    }
    #[doc = "USART interrupt generated whenever WUF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUFIE_A::B_0x1)
    }
}
#[doc = "TXFIFO threshold interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFTIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0x0 = 0,
    #[doc = "1: USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
    B_0x1 = 1,
}
impl From<TXFTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFTIE` reader - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type TXFTIE_R = crate::BitReader<TXFTIE_A>;
impl TXFTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXFTIE_A {
        match self.bits {
            false => TXFTIE_A::B_0x0,
            true => TXFTIE_A::B_0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXFTIE_A::B_0x0
    }
    #[doc = "USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXFTIE_A::B_0x1
    }
}
#[doc = "Field `TXFTIE` writer - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type TXFTIE_W<'a, REG> = crate::BitWriter<'a, REG, TXFTIE_A>;
impl<'a, REG> TXFTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTIE_A::B_0x0)
    }
    #[doc = "USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTIE_A::B_0x1)
    }
}
#[doc = "Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGTIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0x0 = 0,
    #[doc = "1: USART interrupt generated whenever TCBGT=1 in the USART_ISR register"]
    B_0x1 = 1,
}
impl From<TCBGTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCBGTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCBGTIE` reader - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type TCBGTIE_R = crate::BitReader<TCBGTIE_A>;
impl TCBGTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCBGTIE_A {
        match self.bits {
            false => TCBGTIE_A::B_0x0,
            true => TCBGTIE_A::B_0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TCBGTIE_A::B_0x0
    }
    #[doc = "USART interrupt generated whenever TCBGT=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TCBGTIE_A::B_0x1
    }
}
#[doc = "Field `TCBGTIE` writer - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type TCBGTIE_W<'a, REG> = crate::BitWriter<'a, REG, TCBGTIE_A>;
impl<'a, REG> TCBGTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCBGTIE_A::B_0x0)
    }
    #[doc = "USART interrupt generated whenever TCBGT=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCBGTIE_A::B_0x1)
    }
}
#[doc = "Receive FIFO threshold configuration Remaining combinations: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXFTCFG_A {
    #[doc = "0: Receive FIFO reaches 1/8 of its depth"]
    B_0x0 = 0,
    #[doc = "1: Receive FIFO reaches 1/4 of its depth"]
    B_0x1 = 1,
    #[doc = "2: Receive FIFO reaches 1/2 of its depth"]
    B_0x2 = 2,
    #[doc = "3: Receive FIFO reaches 3/4 of its depth"]
    B_0x3 = 3,
    #[doc = "4: Receive FIFO reaches 7/8 of its depth"]
    B_0x4 = 4,
    #[doc = "5: Receive FIFO becomes full"]
    B_0x5 = 5,
}
impl From<RXFTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFTCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXFTCFG_A {
    type Ux = u8;
}
impl crate::IsEnum for RXFTCFG_A {}
#[doc = "Field `RXFTCFG` reader - Receive FIFO threshold configuration Remaining combinations: Reserved"]
pub type RXFTCFG_R = crate::FieldReader<RXFTCFG_A>;
impl RXFTCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXFTCFG_A> {
        match self.bits {
            0 => Some(RXFTCFG_A::B_0x0),
            1 => Some(RXFTCFG_A::B_0x1),
            2 => Some(RXFTCFG_A::B_0x2),
            3 => Some(RXFTCFG_A::B_0x3),
            4 => Some(RXFTCFG_A::B_0x4),
            5 => Some(RXFTCFG_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "Receive FIFO reaches 1/8 of its depth"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXFTCFG_A::B_0x0
    }
    #[doc = "Receive FIFO reaches 1/4 of its depth"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXFTCFG_A::B_0x1
    }
    #[doc = "Receive FIFO reaches 1/2 of its depth"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == RXFTCFG_A::B_0x2
    }
    #[doc = "Receive FIFO reaches 3/4 of its depth"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == RXFTCFG_A::B_0x3
    }
    #[doc = "Receive FIFO reaches 7/8 of its depth"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == RXFTCFG_A::B_0x4
    }
    #[doc = "Receive FIFO becomes full"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == RXFTCFG_A::B_0x5
    }
}
#[doc = "Field `RXFTCFG` writer - Receive FIFO threshold configuration Remaining combinations: Reserved"]
pub type RXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RXFTCFG_A>;
impl<'a, REG> RXFTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive FIFO reaches 1/8 of its depth"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG_A::B_0x0)
    }
    #[doc = "Receive FIFO reaches 1/4 of its depth"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG_A::B_0x1)
    }
    #[doc = "Receive FIFO reaches 1/2 of its depth"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG_A::B_0x2)
    }
    #[doc = "Receive FIFO reaches 3/4 of its depth"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG_A::B_0x3)
    }
    #[doc = "Receive FIFO reaches 7/8 of its depth"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG_A::B_0x4)
    }
    #[doc = "Receive FIFO becomes full"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG_A::B_0x5)
    }
}
#[doc = "RXFIFO threshold interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFTIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0x0 = 0,
    #[doc = "1: USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
    B_0x1 = 1,
}
impl From<RXFTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFTIE` reader - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type RXFTIE_R = crate::BitReader<RXFTIE_A>;
impl RXFTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXFTIE_A {
        match self.bits {
            false => RXFTIE_A::B_0x0,
            true => RXFTIE_A::B_0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXFTIE_A::B_0x0
    }
    #[doc = "USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXFTIE_A::B_0x1
    }
}
#[doc = "Field `RXFTIE` writer - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type RXFTIE_W<'a, REG> = crate::BitWriter<'a, REG, RXFTIE_A>;
impl<'a, REG> RXFTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTIE_A::B_0x0)
    }
    #[doc = "USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTIE_A::B_0x1)
    }
}
#[doc = "TXFIFO threshold configuration Remaining combinations: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXFTCFG_A {
    #[doc = "0: TXFIFO reaches 1/8 of its depth"]
    B_0x0 = 0,
    #[doc = "1: TXFIFO reaches 1/4 of its depth"]
    B_0x1 = 1,
    #[doc = "2: TXFIFO reaches 1/2 of its depth"]
    B_0x2 = 2,
    #[doc = "3: TXFIFO reaches 3/4 of its depth"]
    B_0x3 = 3,
    #[doc = "4: TXFIFO reaches 7/8 of its depth"]
    B_0x4 = 4,
    #[doc = "5: TXFIFO becomes empty"]
    B_0x5 = 5,
}
impl From<TXFTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFTCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXFTCFG_A {
    type Ux = u8;
}
impl crate::IsEnum for TXFTCFG_A {}
#[doc = "Field `TXFTCFG` reader - TXFIFO threshold configuration Remaining combinations: Reserved"]
pub type TXFTCFG_R = crate::FieldReader<TXFTCFG_A>;
impl TXFTCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXFTCFG_A> {
        match self.bits {
            0 => Some(TXFTCFG_A::B_0x0),
            1 => Some(TXFTCFG_A::B_0x1),
            2 => Some(TXFTCFG_A::B_0x2),
            3 => Some(TXFTCFG_A::B_0x3),
            4 => Some(TXFTCFG_A::B_0x4),
            5 => Some(TXFTCFG_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "TXFIFO reaches 1/8 of its depth"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXFTCFG_A::B_0x0
    }
    #[doc = "TXFIFO reaches 1/4 of its depth"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXFTCFG_A::B_0x1
    }
    #[doc = "TXFIFO reaches 1/2 of its depth"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TXFTCFG_A::B_0x2
    }
    #[doc = "TXFIFO reaches 3/4 of its depth"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TXFTCFG_A::B_0x3
    }
    #[doc = "TXFIFO reaches 7/8 of its depth"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == TXFTCFG_A::B_0x4
    }
    #[doc = "TXFIFO becomes empty"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == TXFTCFG_A::B_0x5
    }
}
#[doc = "Field `TXFTCFG` writer - TXFIFO threshold configuration Remaining combinations: Reserved"]
pub type TXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TXFTCFG_A>;
impl<'a, REG> TXFTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TXFIFO reaches 1/8 of its depth"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG_A::B_0x0)
    }
    #[doc = "TXFIFO reaches 1/4 of its depth"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG_A::B_0x1)
    }
    #[doc = "TXFIFO reaches 1/2 of its depth"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG_A::B_0x2)
    }
    #[doc = "TXFIFO reaches 3/4 of its depth"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG_A::B_0x3)
    }
    #[doc = "TXFIFO reaches 7/8 of its depth"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG_A::B_0x4)
    }
    #[doc = "TXFIFO becomes empty"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG_A::B_0x5)
    }
}
impl R {
    #[doc = "Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1or UDR = 1 in the USART_ISR register)."]
    #[inline(always)]
    pub fn EIE(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn IREN(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn IRLP(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn HDSEL(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn NACK(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn SCEN(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver This bit is set/reset by software"]
    #[inline(always)]
    pub fn DMAR(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter This bit is set/reset by software"]
    #[inline(always)]
    pub fn DMAT(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn RTSE(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn CTSE(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn CTSIE(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ONEBIT(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data"]
    #[inline(always)]
    pub fn OVRDIS(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
    #[inline(always)]
    pub fn DDRE(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
    #[inline(always)]
    pub fn DEM(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn DEP(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In Transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In Reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn SCARCNT(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
    #[inline(always)]
    pub fn WUS0(&self) -> WUS0_R {
        WUS0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
    #[inline(always)]
    pub fn WUS1(&self) -> WUS1_R {
        WUS1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
    #[inline(always)]
    pub fn WUFIE(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn TXFTIE(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn TCBGTIE(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn RXFTCFG(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn RXFTIE(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn TXFTCFG(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1or UDR = 1 in the USART_ISR register)."]
    #[inline(always)]
    pub fn EIE(&mut self) -> EIE_W<'_, CR3_SPEC> {
        EIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn IREN(&mut self) -> IREN_W<'_, CR3_SPEC> {
        IREN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn IRLP(&mut self) -> IRLP_W<'_, CR3_SPEC> {
        IRLP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn HDSEL(&mut self) -> HDSEL_W<'_, CR3_SPEC> {
        HDSEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn NACK(&mut self) -> NACK_W<'_, CR3_SPEC> {
        NACK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn SCEN(&mut self) -> SCEN_W<'_, CR3_SPEC> {
        SCEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA enable receiver This bit is set/reset by software"]
    #[inline(always)]
    pub fn DMAR(&mut self) -> DMAR_W<'_, CR3_SPEC> {
        DMAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter This bit is set/reset by software"]
    #[inline(always)]
    pub fn DMAT(&mut self) -> DMAT_W<'_, CR3_SPEC> {
        DMAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn RTSE(&mut self) -> RTSE_W<'_, CR3_SPEC> {
        RTSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn CTSE(&mut self) -> CTSE_W<'_, CR3_SPEC> {
        CTSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn CTSIE(&mut self) -> CTSIE_W<'_, CR3_SPEC> {
        CTSIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ONEBIT(&mut self) -> ONEBIT_W<'_, CR3_SPEC> {
        ONEBIT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data"]
    #[inline(always)]
    pub fn OVRDIS(&mut self) -> OVRDIS_W<'_, CR3_SPEC> {
        OVRDIS_W::new(self, 12)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
    #[inline(always)]
    pub fn DDRE(&mut self) -> DDRE_W<'_, CR3_SPEC> {
        DDRE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
    #[inline(always)]
    pub fn DEM(&mut self) -> DEM_W<'_, CR3_SPEC> {
        DEM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn DEP(&mut self) -> DEP_W<'_, CR3_SPEC> {
        DEP_W::new(self, 15)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In Transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In Reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn SCARCNT(&mut self) -> SCARCNT_W<'_, CR3_SPEC> {
        SCARCNT_W::new(self, 17)
    }
    #[doc = "Bit 20 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
    #[inline(always)]
    pub fn WUS0(&mut self) -> WUS0_W<'_, CR3_SPEC> {
        WUS0_W::new(self, 20)
    }
    #[doc = "Bit 21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
    #[inline(always)]
    pub fn WUS1(&mut self) -> WUS1_W<'_, CR3_SPEC> {
        WUS1_W::new(self, 21)
    }
    #[doc = "Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
    #[inline(always)]
    pub fn WUFIE(&mut self) -> WUFIE_W<'_, CR3_SPEC> {
        WUFIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn TXFTIE(&mut self) -> TXFTIE_W<'_, CR3_SPEC> {
        TXFTIE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn TCBGTIE(&mut self) -> TCBGTIE_W<'_, CR3_SPEC> {
        TCBGTIE_W::new(self, 24)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn RXFTCFG(&mut self) -> RXFTCFG_W<'_, CR3_SPEC> {
        RXFTCFG_W::new(self, 25)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn RXFTIE(&mut self) -> RXFTIE_W<'_, CR3_SPEC> {
        RXFTIE_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn TXFTCFG(&mut self) -> TXFTCFG_W<'_, CR3_SPEC> {
        TXFTCFG_W::new(self, 29)
    }
}
#[doc = "USART control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {}

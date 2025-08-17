#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "RXP interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPIE_A {
    #[doc = "0: RXP interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: RXP interrupt enabled"]
    B_0x1 = 1,
}
impl From<RXPIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPIE` reader - RXP interrupt enable"]
pub type RXPIE_R = crate::BitReader<RXPIE_A>;
impl RXPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXPIE_A {
        match self.bits {
            false => RXPIE_A::B_0x0,
            true => RXPIE_A::B_0x1,
        }
    }
    #[doc = "RXP interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXPIE_A::B_0x0
    }
    #[doc = "RXP interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXPIE_A::B_0x1
    }
}
#[doc = "Field `RXPIE` writer - RXP interrupt enable"]
pub type RXPIE_W<'a, REG> = crate::BitWriter<'a, REG, RXPIE_A>;
impl<'a, REG> RXPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RXP interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXPIE_A::B_0x0)
    }
    #[doc = "RXP interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXPIE_A::B_0x1)
    }
}
#[doc = "TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPIE_A {
    #[doc = "0: TXP interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: TXP interrupt enabled"]
    B_0x1 = 1,
}
impl From<TXPIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPIE` reader - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
pub type TXPIE_R = crate::BitReader<TXPIE_A>;
impl TXPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXPIE_A {
        match self.bits {
            false => TXPIE_A::B_0x0,
            true => TXPIE_A::B_0x1,
        }
    }
    #[doc = "TXP interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXPIE_A::B_0x0
    }
    #[doc = "TXP interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXPIE_A::B_0x1
    }
}
#[doc = "Field `TXPIE` writer - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
pub type TXPIE_W<'a, REG> = crate::BitWriter<'a, REG, TXPIE_A>;
impl<'a, REG> TXPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXP interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXPIE_A::B_0x0)
    }
    #[doc = "TXP interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXPIE_A::B_0x1)
    }
}
#[doc = "DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXPIE_A {
    #[doc = "0: DXP interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: DXP interrupt enabled"]
    B_0x1 = 1,
}
impl From<DXPIE_A> for bool {
    #[inline(always)]
    fn from(variant: DXPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DXPIE` reader - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
pub type DXPIE_R = crate::BitReader<DXPIE_A>;
impl DXPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DXPIE_A {
        match self.bits {
            false => DXPIE_A::B_0x0,
            true => DXPIE_A::B_0x1,
        }
    }
    #[doc = "DXP interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DXPIE_A::B_0x0
    }
    #[doc = "DXP interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DXPIE_A::B_0x1
    }
}
#[doc = "Field `DXPIE` writer - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
pub type DXPIE_W<'a, REG> = crate::BitWriter<'a, REG, DXPIE_A>;
impl<'a, REG> DXPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DXP interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DXPIE_A::B_0x0)
    }
    #[doc = "DXP interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DXPIE_A::B_0x1)
    }
}
#[doc = "EOT, SUSP and TXC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOTIE_A {
    #[doc = "0: EOT/SUSP/TXC interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: EOT/SUSP/TXC interrupt enabled"]
    B_0x1 = 1,
}
impl From<EOTIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOTIE` reader - EOT, SUSP and TXC interrupt enable"]
pub type EOTIE_R = crate::BitReader<EOTIE_A>;
impl EOTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOTIE_A {
        match self.bits {
            false => EOTIE_A::B_0x0,
            true => EOTIE_A::B_0x1,
        }
    }
    #[doc = "EOT/SUSP/TXC interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EOTIE_A::B_0x0
    }
    #[doc = "EOT/SUSP/TXC interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EOTIE_A::B_0x1
    }
}
#[doc = "Field `EOTIE` writer - EOT, SUSP and TXC interrupt enable"]
pub type EOTIE_W<'a, REG> = crate::BitWriter<'a, REG, EOTIE_A>;
impl<'a, REG> EOTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOT/SUSP/TXC interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOTIE_A::B_0x0)
    }
    #[doc = "EOT/SUSP/TXC interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOTIE_A::B_0x1)
    }
}
#[doc = "TXTFIE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTFIE_A {
    #[doc = "0: TXTF interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: TXTF interrupt enabled"]
    B_0x1 = 1,
}
impl From<TXTFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXTFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXTFIE` reader - TXTFIE interrupt enable"]
pub type TXTFIE_R = crate::BitReader<TXTFIE_A>;
impl TXTFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXTFIE_A {
        match self.bits {
            false => TXTFIE_A::B_0x0,
            true => TXTFIE_A::B_0x1,
        }
    }
    #[doc = "TXTF interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXTFIE_A::B_0x0
    }
    #[doc = "TXTF interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXTFIE_A::B_0x1
    }
}
#[doc = "Field `TXTFIE` writer - TXTFIE interrupt enable"]
pub type TXTFIE_W<'a, REG> = crate::BitWriter<'a, REG, TXTFIE_A>;
impl<'a, REG> TXTFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXTF interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXTFIE_A::B_0x0)
    }
    #[doc = "TXTF interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXTFIE_A::B_0x1)
    }
}
#[doc = "UDR interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRIE_A {
    #[doc = "0: UDR interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: UDR interrupt enabled"]
    B_0x1 = 1,
}
impl From<UDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: UDRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDRIE` reader - UDR interrupt enable"]
pub type UDRIE_R = crate::BitReader<UDRIE_A>;
impl UDRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDRIE_A {
        match self.bits {
            false => UDRIE_A::B_0x0,
            true => UDRIE_A::B_0x1,
        }
    }
    #[doc = "UDR interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UDRIE_A::B_0x0
    }
    #[doc = "UDR interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UDRIE_A::B_0x1
    }
}
#[doc = "Field `UDRIE` writer - UDR interrupt enable"]
pub type UDRIE_W<'a, REG> = crate::BitWriter<'a, REG, UDRIE_A>;
impl<'a, REG> UDRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UDR interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UDRIE_A::B_0x0)
    }
    #[doc = "UDR interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UDRIE_A::B_0x1)
    }
}
#[doc = "OVR interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE_A {
    #[doc = "0: OVR interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: OVR interrupt enabled"]
    B_0x1 = 1,
}
impl From<OVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRIE` reader - OVR interrupt enable"]
pub type OVRIE_R = crate::BitReader<OVRIE_A>;
impl OVRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRIE_A {
        match self.bits {
            false => OVRIE_A::B_0x0,
            true => OVRIE_A::B_0x1,
        }
    }
    #[doc = "OVR interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OVRIE_A::B_0x0
    }
    #[doc = "OVR interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OVRIE_A::B_0x1
    }
}
#[doc = "Field `OVRIE` writer - OVR interrupt enable"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRIE_A>;
impl<'a, REG> OVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OVR interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE_A::B_0x0)
    }
    #[doc = "OVR interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE_A::B_0x1)
    }
}
#[doc = "CRC error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEIE_A {
    #[doc = "0: CRC interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: CRC interrupt enabled"]
    B_0x1 = 1,
}
impl From<CRCEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEIE` reader - CRC error interrupt enable"]
pub type CRCEIE_R = crate::BitReader<CRCEIE_A>;
impl CRCEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCEIE_A {
        match self.bits {
            false => CRCEIE_A::B_0x0,
            true => CRCEIE_A::B_0x1,
        }
    }
    #[doc = "CRC interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRCEIE_A::B_0x0
    }
    #[doc = "CRC interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRCEIE_A::B_0x1
    }
}
#[doc = "Field `CRCEIE` writer - CRC error interrupt enable"]
pub type CRCEIE_W<'a, REG> = crate::BitWriter<'a, REG, CRCEIE_A>;
impl<'a, REG> CRCEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEIE_A::B_0x0)
    }
    #[doc = "CRC interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEIE_A::B_0x1)
    }
}
#[doc = "TIFRE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFREIE_A {
    #[doc = "0: TIFRE interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: TIFRE interrupt enabled"]
    B_0x1 = 1,
}
impl From<TIFREIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIFREIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIFREIE` reader - TIFRE interrupt enable"]
pub type TIFREIE_R = crate::BitReader<TIFREIE_A>;
impl TIFREIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIFREIE_A {
        match self.bits {
            false => TIFREIE_A::B_0x0,
            true => TIFREIE_A::B_0x1,
        }
    }
    #[doc = "TIFRE interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIFREIE_A::B_0x0
    }
    #[doc = "TIFRE interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIFREIE_A::B_0x1
    }
}
#[doc = "Field `TIFREIE` writer - TIFRE interrupt enable"]
pub type TIFREIE_W<'a, REG> = crate::BitWriter<'a, REG, TIFREIE_A>;
impl<'a, REG> TIFREIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIFRE interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIFREIE_A::B_0x0)
    }
    #[doc = "TIFRE interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIFREIE_A::B_0x1)
    }
}
#[doc = "mode Fault interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFIE_A {
    #[doc = "0: MODF interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: MODF interrupt enabled"]
    B_0x1 = 1,
}
impl From<MODFIE_A> for bool {
    #[inline(always)]
    fn from(variant: MODFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODFIE` reader - mode Fault interrupt enable"]
pub type MODFIE_R = crate::BitReader<MODFIE_A>;
impl MODFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODFIE_A {
        match self.bits {
            false => MODFIE_A::B_0x0,
            true => MODFIE_A::B_0x1,
        }
    }
    #[doc = "MODF interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODFIE_A::B_0x0
    }
    #[doc = "MODF interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODFIE_A::B_0x1
    }
}
#[doc = "Field `MODFIE` writer - mode Fault interrupt enable"]
pub type MODFIE_W<'a, REG> = crate::BitWriter<'a, REG, MODFIE_A>;
impl<'a, REG> MODFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MODF interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODFIE_A::B_0x0)
    }
    #[doc = "MODF interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODFIE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - RXP interrupt enable"]
    #[inline(always)]
    pub fn RXPIE(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    pub fn TXPIE(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    pub fn DXPIE(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    pub fn EOTIE(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    pub fn TXTFIE(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    pub fn UDRIE(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    pub fn OVRIE(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC error interrupt enable"]
    #[inline(always)]
    pub fn CRCEIE(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    pub fn TIFREIE(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - mode Fault interrupt enable"]
    #[inline(always)]
    pub fn MODFIE(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXP interrupt enable"]
    #[inline(always)]
    pub fn RXPIE(&mut self) -> RXPIE_W<'_, IER_SPEC> {
        RXPIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    pub fn TXPIE(&mut self) -> TXPIE_W<'_, IER_SPEC> {
        TXPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    pub fn DXPIE(&mut self) -> DXPIE_W<'_, IER_SPEC> {
        DXPIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    pub fn EOTIE(&mut self) -> EOTIE_W<'_, IER_SPEC> {
        EOTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    pub fn TXTFIE(&mut self) -> TXTFIE_W<'_, IER_SPEC> {
        TXTFIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    pub fn UDRIE(&mut self) -> UDRIE_W<'_, IER_SPEC> {
        UDRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    pub fn OVRIE(&mut self) -> OVRIE_W<'_, IER_SPEC> {
        OVRIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC error interrupt enable"]
    #[inline(always)]
    pub fn CRCEIE(&mut self) -> CRCEIE_W<'_, IER_SPEC> {
        CRCEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    pub fn TIFREIE(&mut self) -> TIFREIE_W<'_, IER_SPEC> {
        TIFREIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - mode Fault interrupt enable"]
    #[inline(always)]
    pub fn MODFIE(&mut self) -> MODFIE_W<'_, IER_SPEC> {
        MODFIE_W::new(self, 9)
    }
}
#[doc = "SPI/I2S interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {}

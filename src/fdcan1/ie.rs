#[doc = "Register `IE` reader"]
pub type R = crate::R<IE_SPEC>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IE_SPEC>;
#[doc = "Rx FIFO 0 new message interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0NE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<RF0NE_A> for bool {
    #[inline(always)]
    fn from(variant: RF0NE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0NE` reader - Rx FIFO 0 new message interrupt enable"]
pub type RF0NE_R = crate::BitReader<RF0NE_A>;
impl RF0NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF0NE_A {
        match self.bits {
            false => RF0NE_A::B_0x0,
            true => RF0NE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF0NE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF0NE_A::B_0x1
    }
}
#[doc = "Field `RF0NE` writer - Rx FIFO 0 new message interrupt enable"]
pub type RF0NE_W<'a, REG> = crate::BitWriter<'a, REG, RF0NE_A>;
impl<'a, REG> RF0NE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0NE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0NE_A::B_0x1)
    }
}
#[doc = "Rx FIFO 0 full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0FE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<RF0FE_A> for bool {
    #[inline(always)]
    fn from(variant: RF0FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0FE` reader - Rx FIFO 0 full interrupt enable"]
pub type RF0FE_R = crate::BitReader<RF0FE_A>;
impl RF0FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF0FE_A {
        match self.bits {
            false => RF0FE_A::B_0x0,
            true => RF0FE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF0FE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF0FE_A::B_0x1
    }
}
#[doc = "Field `RF0FE` writer - Rx FIFO 0 full interrupt enable"]
pub type RF0FE_W<'a, REG> = crate::BitWriter<'a, REG, RF0FE_A>;
impl<'a, REG> RF0FE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0FE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0FE_A::B_0x1)
    }
}
#[doc = "Rx FIFO 0 message lost interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0LE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<RF0LE_A> for bool {
    #[inline(always)]
    fn from(variant: RF0LE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0LE` reader - Rx FIFO 0 message lost interrupt enable"]
pub type RF0LE_R = crate::BitReader<RF0LE_A>;
impl RF0LE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF0LE_A {
        match self.bits {
            false => RF0LE_A::B_0x0,
            true => RF0LE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF0LE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF0LE_A::B_0x1
    }
}
#[doc = "Field `RF0LE` writer - Rx FIFO 0 message lost interrupt enable"]
pub type RF0LE_W<'a, REG> = crate::BitWriter<'a, REG, RF0LE_A>;
impl<'a, REG> RF0LE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0LE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0LE_A::B_0x1)
    }
}
#[doc = "Rx FIFO 1 new message interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1NE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<RF1NE_A> for bool {
    #[inline(always)]
    fn from(variant: RF1NE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1NE` reader - Rx FIFO 1 new message interrupt enable"]
pub type RF1NE_R = crate::BitReader<RF1NE_A>;
impl RF1NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF1NE_A {
        match self.bits {
            false => RF1NE_A::B_0x0,
            true => RF1NE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF1NE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF1NE_A::B_0x1
    }
}
#[doc = "Field `RF1NE` writer - Rx FIFO 1 new message interrupt enable"]
pub type RF1NE_W<'a, REG> = crate::BitWriter<'a, REG, RF1NE_A>;
impl<'a, REG> RF1NE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1NE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1NE_A::B_0x1)
    }
}
#[doc = "Rx FIFO 1 full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1FE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<RF1FE_A> for bool {
    #[inline(always)]
    fn from(variant: RF1FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1FE` reader - Rx FIFO 1 full interrupt enable"]
pub type RF1FE_R = crate::BitReader<RF1FE_A>;
impl RF1FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF1FE_A {
        match self.bits {
            false => RF1FE_A::B_0x0,
            true => RF1FE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF1FE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF1FE_A::B_0x1
    }
}
#[doc = "Field `RF1FE` writer - Rx FIFO 1 full interrupt enable"]
pub type RF1FE_W<'a, REG> = crate::BitWriter<'a, REG, RF1FE_A>;
impl<'a, REG> RF1FE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1FE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1FE_A::B_0x1)
    }
}
#[doc = "Rx FIFO 1 message lost interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1LE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<RF1LE_A> for bool {
    #[inline(always)]
    fn from(variant: RF1LE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1LE` reader - Rx FIFO 1 message lost interrupt enable"]
pub type RF1LE_R = crate::BitReader<RF1LE_A>;
impl RF1LE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF1LE_A {
        match self.bits {
            false => RF1LE_A::B_0x0,
            true => RF1LE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF1LE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF1LE_A::B_0x1
    }
}
#[doc = "Field `RF1LE` writer - Rx FIFO 1 message lost interrupt enable"]
pub type RF1LE_W<'a, REG> = crate::BitWriter<'a, REG, RF1LE_A>;
impl<'a, REG> RF1LE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1LE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1LE_A::B_0x1)
    }
}
#[doc = "High-priority message interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPME_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<HPME_A> for bool {
    #[inline(always)]
    fn from(variant: HPME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPME` reader - High-priority message interrupt enable"]
pub type HPME_R = crate::BitReader<HPME_A>;
impl HPME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPME_A {
        match self.bits {
            false => HPME_A::B_0x0,
            true => HPME_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HPME_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HPME_A::B_0x1
    }
}
#[doc = "Field `HPME` writer - High-priority message interrupt enable"]
pub type HPME_W<'a, REG> = crate::BitWriter<'a, REG, HPME_A>;
impl<'a, REG> HPME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HPME_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HPME_A::B_0x1)
    }
}
#[doc = "Transmission completed interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<TCE_A> for bool {
    #[inline(always)]
    fn from(variant: TCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCE` reader - Transmission completed interrupt enable"]
pub type TCE_R = crate::BitReader<TCE_A>;
impl TCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCE_A {
        match self.bits {
            false => TCE_A::B_0x0,
            true => TCE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TCE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TCE_A::B_0x1
    }
}
#[doc = "Field `TCE` writer - Transmission completed interrupt enable"]
pub type TCE_W<'a, REG> = crate::BitWriter<'a, REG, TCE_A>;
impl<'a, REG> TCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCE_A::B_0x1)
    }
}
#[doc = "Transmission cancellation finished interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<TCFE_A> for bool {
    #[inline(always)]
    fn from(variant: TCFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCFE` reader - Transmission cancellation finished interrupt enable"]
pub type TCFE_R = crate::BitReader<TCFE_A>;
impl TCFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCFE_A {
        match self.bits {
            false => TCFE_A::B_0x0,
            true => TCFE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TCFE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TCFE_A::B_0x1
    }
}
#[doc = "Field `TCFE` writer - Transmission cancellation finished interrupt enable"]
pub type TCFE_W<'a, REG> = crate::BitWriter<'a, REG, TCFE_A>;
impl<'a, REG> TCFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFE_A::B_0x1)
    }
}
#[doc = "Tx FIFO empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFEE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<TFEE_A> for bool {
    #[inline(always)]
    fn from(variant: TFEE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFEE` reader - Tx FIFO empty interrupt enable"]
pub type TFEE_R = crate::BitReader<TFEE_A>;
impl TFEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFEE_A {
        match self.bits {
            false => TFEE_A::B_0x0,
            true => TFEE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TFEE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TFEE_A::B_0x1
    }
}
#[doc = "Field `TFEE` writer - Tx FIFO empty interrupt enable"]
pub type TFEE_W<'a, REG> = crate::BitWriter<'a, REG, TFEE_A>;
impl<'a, REG> TFEE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TFEE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TFEE_A::B_0x1)
    }
}
#[doc = "Tx event FIFO new entry interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFNE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<TEFNE_A> for bool {
    #[inline(always)]
    fn from(variant: TEFNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFNE` reader - Tx event FIFO new entry interrupt enable"]
pub type TEFNE_R = crate::BitReader<TEFNE_A>;
impl TEFNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEFNE_A {
        match self.bits {
            false => TEFNE_A::B_0x0,
            true => TEFNE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEFNE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEFNE_A::B_0x1
    }
}
#[doc = "Field `TEFNE` writer - Tx event FIFO new entry interrupt enable"]
pub type TEFNE_W<'a, REG> = crate::BitWriter<'a, REG, TEFNE_A>;
impl<'a, REG> TEFNE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFNE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFNE_A::B_0x1)
    }
}
#[doc = "Tx event FIFO full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFFE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<TEFFE_A> for bool {
    #[inline(always)]
    fn from(variant: TEFFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFFE` reader - Tx event FIFO full interrupt enable"]
pub type TEFFE_R = crate::BitReader<TEFFE_A>;
impl TEFFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEFFE_A {
        match self.bits {
            false => TEFFE_A::B_0x0,
            true => TEFFE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEFFE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEFFE_A::B_0x1
    }
}
#[doc = "Field `TEFFE` writer - Tx event FIFO full interrupt enable"]
pub type TEFFE_W<'a, REG> = crate::BitWriter<'a, REG, TEFFE_A>;
impl<'a, REG> TEFFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFFE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFFE_A::B_0x1)
    }
}
#[doc = "Tx event FIFO element lost interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFLE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<TEFLE_A> for bool {
    #[inline(always)]
    fn from(variant: TEFLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFLE` reader - Tx event FIFO element lost interrupt enable"]
pub type TEFLE_R = crate::BitReader<TEFLE_A>;
impl TEFLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEFLE_A {
        match self.bits {
            false => TEFLE_A::B_0x0,
            true => TEFLE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEFLE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEFLE_A::B_0x1
    }
}
#[doc = "Field `TEFLE` writer - Tx event FIFO element lost interrupt enable"]
pub type TEFLE_W<'a, REG> = crate::BitWriter<'a, REG, TEFLE_A>;
impl<'a, REG> TEFLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFLE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFLE_A::B_0x1)
    }
}
#[doc = "Timestamp wraparound interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSWE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<TSWE_A> for bool {
    #[inline(always)]
    fn from(variant: TSWE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSWE` reader - Timestamp wraparound interrupt enable"]
pub type TSWE_R = crate::BitReader<TSWE_A>;
impl TSWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSWE_A {
        match self.bits {
            false => TSWE_A::B_0x0,
            true => TSWE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TSWE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TSWE_A::B_0x1
    }
}
#[doc = "Field `TSWE` writer - Timestamp wraparound interrupt enable"]
pub type TSWE_W<'a, REG> = crate::BitWriter<'a, REG, TSWE_A>;
impl<'a, REG> TSWE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSWE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSWE_A::B_0x1)
    }
}
#[doc = "Message RAM access failure interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRAFE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<MRAFE_A> for bool {
    #[inline(always)]
    fn from(variant: MRAFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRAFE` reader - Message RAM access failure interrupt enable"]
pub type MRAFE_R = crate::BitReader<MRAFE_A>;
impl MRAFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MRAFE_A {
        match self.bits {
            false => MRAFE_A::B_0x0,
            true => MRAFE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MRAFE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MRAFE_A::B_0x1
    }
}
#[doc = "Field `MRAFE` writer - Message RAM access failure interrupt enable"]
pub type MRAFE_W<'a, REG> = crate::BitWriter<'a, REG, MRAFE_A>;
impl<'a, REG> MRAFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MRAFE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MRAFE_A::B_0x1)
    }
}
#[doc = "Timeout occurred interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOOE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<TOOE_A> for bool {
    #[inline(always)]
    fn from(variant: TOOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOOE` reader - Timeout occurred interrupt enable"]
pub type TOOE_R = crate::BitReader<TOOE_A>;
impl TOOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOOE_A {
        match self.bits {
            false => TOOE_A::B_0x0,
            true => TOOE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TOOE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TOOE_A::B_0x1
    }
}
#[doc = "Field `TOOE` writer - Timeout occurred interrupt enable"]
pub type TOOE_W<'a, REG> = crate::BitWriter<'a, REG, TOOE_A>;
impl<'a, REG> TOOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOOE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOOE_A::B_0x1)
    }
}
#[doc = "Error logging overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELOE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<ELOE_A> for bool {
    #[inline(always)]
    fn from(variant: ELOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELOE` reader - Error logging overflow interrupt enable"]
pub type ELOE_R = crate::BitReader<ELOE_A>;
impl ELOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ELOE_A {
        match self.bits {
            false => ELOE_A::B_0x0,
            true => ELOE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ELOE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ELOE_A::B_0x1
    }
}
#[doc = "Field `ELOE` writer - Error logging overflow interrupt enable"]
pub type ELOE_W<'a, REG> = crate::BitWriter<'a, REG, ELOE_A>;
impl<'a, REG> ELOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ELOE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ELOE_A::B_0x1)
    }
}
#[doc = "Error passive interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<EPE_A> for bool {
    #[inline(always)]
    fn from(variant: EPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPE` reader - Error passive interrupt enable"]
pub type EPE_R = crate::BitReader<EPE_A>;
impl EPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPE_A {
        match self.bits {
            false => EPE_A::B_0x0,
            true => EPE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EPE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EPE_A::B_0x1
    }
}
#[doc = "Field `EPE` writer - Error passive interrupt enable"]
pub type EPE_W<'a, REG> = crate::BitWriter<'a, REG, EPE_A>;
impl<'a, REG> EPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EPE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EPE_A::B_0x1)
    }
}
#[doc = "Warning status interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<EWE_A> for bool {
    #[inline(always)]
    fn from(variant: EWE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWE` reader - Warning status interrupt enable"]
pub type EWE_R = crate::BitReader<EWE_A>;
impl EWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWE_A {
        match self.bits {
            false => EWE_A::B_0x0,
            true => EWE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EWE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EWE_A::B_0x1
    }
}
#[doc = "Field `EWE` writer - Warning status interrupt enable"]
pub type EWE_W<'a, REG> = crate::BitWriter<'a, REG, EWE_A>;
impl<'a, REG> EWE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EWE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EWE_A::B_0x1)
    }
}
#[doc = "Bus_Off status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<BOE_A> for bool {
    #[inline(always)]
    fn from(variant: BOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOE` reader - Bus_Off status"]
pub type BOE_R = crate::BitReader<BOE_A>;
impl BOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOE_A {
        match self.bits {
            false => BOE_A::B_0x0,
            true => BOE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BOE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BOE_A::B_0x1
    }
}
#[doc = "Field `BOE` writer - Bus_Off status"]
pub type BOE_W<'a, REG> = crate::BitWriter<'a, REG, BOE_A>;
impl<'a, REG> BOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BOE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BOE_A::B_0x1)
    }
}
#[doc = "Watchdog interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDIE_A {
    #[doc = "0: Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B_0x1 = 1,
}
impl From<WDIE_A> for bool {
    #[inline(always)]
    fn from(variant: WDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDIE` reader - Watchdog interrupt enable"]
pub type WDIE_R = crate::BitReader<WDIE_A>;
impl WDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDIE_A {
        match self.bits {
            false => WDIE_A::B_0x0,
            true => WDIE_A::B_0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WDIE_A::B_0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WDIE_A::B_0x1
    }
}
#[doc = "Field `WDIE` writer - Watchdog interrupt enable"]
pub type WDIE_W<'a, REG> = crate::BitWriter<'a, REG, WDIE_A>;
impl<'a, REG> WDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDIE_A::B_0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDIE_A::B_0x1)
    }
}
#[doc = "Field `PEAE` reader - Protocol error in arbitration phase enable"]
pub type PEAE_R = crate::BitReader;
#[doc = "Field `PEAE` writer - Protocol error in arbitration phase enable"]
pub type PEAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDE` reader - Protocol error in data phase enable"]
pub type PEDE_R = crate::BitReader;
#[doc = "Field `PEDE` writer - Protocol error in data phase enable"]
pub type PEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARAE` reader - Access to reserved address enable"]
pub type ARAE_R = crate::BitReader;
#[doc = "Field `ARAE` writer - Access to reserved address enable"]
pub type ARAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable"]
    #[inline(always)]
    pub fn RF0NE(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn RF0FE(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost interrupt enable"]
    #[inline(always)]
    pub fn RF0LE(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message interrupt enable"]
    #[inline(always)]
    pub fn RF1NE(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn RF1FE(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost interrupt enable"]
    #[inline(always)]
    pub fn RF1LE(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High-priority message interrupt enable"]
    #[inline(always)]
    pub fn HPME(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission completed interrupt enable"]
    #[inline(always)]
    pub fn TCE(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission cancellation finished interrupt enable"]
    #[inline(always)]
    pub fn TCFE(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn TFEE(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx event FIFO new entry interrupt enable"]
    #[inline(always)]
    pub fn TEFNE(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx event FIFO full interrupt enable"]
    #[inline(always)]
    pub fn TEFFE(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost interrupt enable"]
    #[inline(always)]
    pub fn TEFLE(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timestamp wraparound interrupt enable"]
    #[inline(always)]
    pub fn TSWE(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message RAM access failure interrupt enable"]
    #[inline(always)]
    pub fn MRAFE(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout occurred interrupt enable"]
    #[inline(always)]
    pub fn TOOE(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Error logging overflow interrupt enable"]
    #[inline(always)]
    pub fn ELOE(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn EPE(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Warning status interrupt enable"]
    #[inline(always)]
    pub fn EWE(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn BOE(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Watchdog interrupt enable"]
    #[inline(always)]
    pub fn WDIE(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase enable"]
    #[inline(always)]
    pub fn PEAE(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protocol error in data phase enable"]
    #[inline(always)]
    pub fn PEDE(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Access to reserved address enable"]
    #[inline(always)]
    pub fn ARAE(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable"]
    #[inline(always)]
    pub fn RF0NE(&mut self) -> RF0NE_W<'_, IE_SPEC> {
        RF0NE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn RF0FE(&mut self) -> RF0FE_W<'_, IE_SPEC> {
        RF0FE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost interrupt enable"]
    #[inline(always)]
    pub fn RF0LE(&mut self) -> RF0LE_W<'_, IE_SPEC> {
        RF0LE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message interrupt enable"]
    #[inline(always)]
    pub fn RF1NE(&mut self) -> RF1NE_W<'_, IE_SPEC> {
        RF1NE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn RF1FE(&mut self) -> RF1FE_W<'_, IE_SPEC> {
        RF1FE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost interrupt enable"]
    #[inline(always)]
    pub fn RF1LE(&mut self) -> RF1LE_W<'_, IE_SPEC> {
        RF1LE_W::new(self, 5)
    }
    #[doc = "Bit 6 - High-priority message interrupt enable"]
    #[inline(always)]
    pub fn HPME(&mut self) -> HPME_W<'_, IE_SPEC> {
        HPME_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmission completed interrupt enable"]
    #[inline(always)]
    pub fn TCE(&mut self) -> TCE_W<'_, IE_SPEC> {
        TCE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Transmission cancellation finished interrupt enable"]
    #[inline(always)]
    pub fn TCFE(&mut self) -> TCFE_W<'_, IE_SPEC> {
        TCFE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn TFEE(&mut self) -> TFEE_W<'_, IE_SPEC> {
        TFEE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Tx event FIFO new entry interrupt enable"]
    #[inline(always)]
    pub fn TEFNE(&mut self) -> TEFNE_W<'_, IE_SPEC> {
        TEFNE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Tx event FIFO full interrupt enable"]
    #[inline(always)]
    pub fn TEFFE(&mut self) -> TEFFE_W<'_, IE_SPEC> {
        TEFFE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost interrupt enable"]
    #[inline(always)]
    pub fn TEFLE(&mut self) -> TEFLE_W<'_, IE_SPEC> {
        TEFLE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Timestamp wraparound interrupt enable"]
    #[inline(always)]
    pub fn TSWE(&mut self) -> TSWE_W<'_, IE_SPEC> {
        TSWE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Message RAM access failure interrupt enable"]
    #[inline(always)]
    pub fn MRAFE(&mut self) -> MRAFE_W<'_, IE_SPEC> {
        MRAFE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Timeout occurred interrupt enable"]
    #[inline(always)]
    pub fn TOOE(&mut self) -> TOOE_W<'_, IE_SPEC> {
        TOOE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Error logging overflow interrupt enable"]
    #[inline(always)]
    pub fn ELOE(&mut self) -> ELOE_W<'_, IE_SPEC> {
        ELOE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn EPE(&mut self) -> EPE_W<'_, IE_SPEC> {
        EPE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Warning status interrupt enable"]
    #[inline(always)]
    pub fn EWE(&mut self) -> EWE_W<'_, IE_SPEC> {
        EWE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn BOE(&mut self) -> BOE_W<'_, IE_SPEC> {
        BOE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Watchdog interrupt enable"]
    #[inline(always)]
    pub fn WDIE(&mut self) -> WDIE_W<'_, IE_SPEC> {
        WDIE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase enable"]
    #[inline(always)]
    pub fn PEAE(&mut self) -> PEAE_W<'_, IE_SPEC> {
        PEAE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Protocol error in data phase enable"]
    #[inline(always)]
    pub fn PEDE(&mut self) -> PEDE_W<'_, IE_SPEC> {
        PEDE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Access to reserved address enable"]
    #[inline(always)]
    pub fn ARAE(&mut self) -> ARAE_W<'_, IE_SPEC> {
        ARAE_W::new(self, 23)
    }
}
#[doc = "FDCAN interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {}

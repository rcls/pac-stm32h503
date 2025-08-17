#[doc = "Register `TZSC_PRIVCFGR2` reader"]
pub type R = crate::R<TZSC_PRIVCFGR2_SPEC>;
#[doc = "Register `TZSC_PRIVCFGR2` writer"]
pub type W = crate::W<TZSC_PRIVCFGR2_SPEC>;
#[doc = "privileged access mode for FDCAN1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDCAN1PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<FDCAN1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: FDCAN1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDCAN1PRIV` reader - privileged access mode for FDCAN1"]
pub type FDCAN1PRIV_R = crate::BitReader<FDCAN1PRIV_A>;
impl FDCAN1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FDCAN1PRIV_A {
        match self.bits {
            false => FDCAN1PRIV_A::B_0x0,
            true => FDCAN1PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FDCAN1PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FDCAN1PRIV_A::B_0x1
    }
}
#[doc = "Field `FDCAN1PRIV` writer - privileged access mode for FDCAN1"]
pub type FDCAN1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, FDCAN1PRIV_A>;
impl<'a, REG> FDCAN1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for OPAMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMPPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<OPAMPPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: OPAMPPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAMPPRIV` reader - privileged access mode for OPAMP"]
pub type OPAMPPRIV_R = crate::BitReader<OPAMPPRIV_A>;
impl OPAMPPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAMPPRIV_A {
        match self.bits {
            false => OPAMPPRIV_A::B_0x0,
            true => OPAMPPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OPAMPPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OPAMPPRIV_A::B_0x1
    }
}
#[doc = "Field `OPAMPPRIV` writer - privileged access mode for OPAMP"]
pub type OPAMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG, OPAMPPRIV_A>;
impl<'a, REG> OPAMPPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for COMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<COMPPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: COMPPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPPRIV` reader - privileged access mode for COMP"]
pub type COMPPRIV_R = crate::BitReader<COMPPRIV_A>;
impl COMPPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMPPRIV_A {
        match self.bits {
            false => COMPPRIV_A::B_0x0,
            true => COMPPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == COMPPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == COMPPRIV_A::B_0x1
    }
}
#[doc = "Field `COMPPRIV` writer - privileged access mode for COMP"]
pub type COMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG, COMPPRIV_A>;
impl<'a, REG> COMPPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMPPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMPPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for TIM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<TIM1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1PRIV` reader - privileged access mode for TIM1"]
pub type TIM1PRIV_R = crate::BitReader<TIM1PRIV_A>;
impl TIM1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1PRIV_A {
        match self.bits {
            false => TIM1PRIV_A::B_0x0,
            true => TIM1PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM1PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM1PRIV_A::B_0x1
    }
}
#[doc = "Field `TIM1PRIV` writer - privileged access mode for TIM1"]
pub type TIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, TIM1PRIV_A>;
impl<'a, REG> TIM1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for SPI1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<SPI1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1PRIV` reader - privileged access mode for SPI1"]
pub type SPI1PRIV_R = crate::BitReader<SPI1PRIV_A>;
impl SPI1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI1PRIV_A {
        match self.bits {
            false => SPI1PRIV_A::B_0x0,
            true => SPI1PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI1PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI1PRIV_A::B_0x1
    }
}
#[doc = "Field `SPI1PRIV` writer - privileged access mode for SPI1"]
pub type SPI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, SPI1PRIV_A>;
impl<'a, REG> SPI1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for USART1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<USART1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: USART1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1PRIV` reader - privileged access mode for USART1"]
pub type USART1PRIV_R = crate::BitReader<USART1PRIV_A>;
impl USART1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART1PRIV_A {
        match self.bits {
            false => USART1PRIV_A::B_0x0,
            true => USART1PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART1PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART1PRIV_A::B_0x1
    }
}
#[doc = "Field `USART1PRIV` writer - privileged access mode for USART1"]
pub type USART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, USART1PRIV_A>;
impl<'a, REG> USART1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for USBSF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBFSPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<USBFSPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: USBFSPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBFSPRIV` reader - privileged access mode for USBSF"]
pub type USBFSPRIV_R = crate::BitReader<USBFSPRIV_A>;
impl USBFSPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBFSPRIV_A {
        match self.bits {
            false => USBFSPRIV_A::B_0x0,
            true => USBFSPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USBFSPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USBFSPRIV_A::B_0x1
    }
}
#[doc = "Field `USBFSPRIV` writer - privileged access mode for USBSF"]
pub type USBFSPRIV_W<'a, REG> = crate::BitWriter<'a, REG, USBFSPRIV_A>;
impl<'a, REG> USBFSPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for LPUART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<LPUART1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1PRIV` reader - privileged access mode for LPUART"]
pub type LPUART1PRIV_R = crate::BitReader<LPUART1PRIV_A>;
impl LPUART1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1PRIV_A {
        match self.bits {
            false => LPUART1PRIV_A::B_0x0,
            true => LPUART1PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPUART1PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPUART1PRIV_A::B_0x1
    }
}
#[doc = "Field `LPUART1PRIV` writer - privileged access mode for LPUART"]
pub type LPUART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1PRIV_A>;
impl<'a, REG> LPUART1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for LPTIM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<LPTIM1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1PRIV` reader - privileged access mode for LPTIM1"]
pub type LPTIM1PRIV_R = crate::BitReader<LPTIM1PRIV_A>;
impl LPTIM1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1PRIV_A {
        match self.bits {
            false => LPTIM1PRIV_A::B_0x0,
            true => LPTIM1PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPTIM1PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPTIM1PRIV_A::B_0x1
    }
}
#[doc = "Field `LPTIM1PRIV` writer - privileged access mode for LPTIM1"]
pub type LPTIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM1PRIV_A>;
impl<'a, REG> LPTIM1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1PRIV_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - privileged access mode for FDCAN1"]
    #[inline(always)]
    pub fn FDCAN1PRIV(&self) -> FDCAN1PRIV_R {
        FDCAN1PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - privileged access mode for OPAMP"]
    #[inline(always)]
    pub fn OPAMPPRIV(&self) -> OPAMPPRIV_R {
        OPAMPPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - privileged access mode for COMP"]
    #[inline(always)]
    pub fn COMPPRIV(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for TIM1"]
    #[inline(always)]
    pub fn TIM1PRIV(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - privileged access mode for SPI1"]
    #[inline(always)]
    pub fn SPI1PRIV(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for USART1"]
    #[inline(always)]
    pub fn USART1PRIV(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 19 - privileged access mode for USBSF"]
    #[inline(always)]
    pub fn USBFSPRIV(&self) -> USBFSPRIV_R {
        USBFSPRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - privileged access mode for LPUART"]
    #[inline(always)]
    pub fn LPUART1PRIV(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - privileged access mode for LPTIM1"]
    #[inline(always)]
    pub fn LPTIM1PRIV(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for FDCAN1"]
    #[inline(always)]
    pub fn FDCAN1PRIV(&mut self) -> FDCAN1PRIV_W<'_, TZSC_PRIVCFGR2_SPEC> {
        FDCAN1PRIV_W::new(self, 0)
    }
    #[doc = "Bit 3 - privileged access mode for OPAMP"]
    #[inline(always)]
    pub fn OPAMPPRIV(&mut self) -> OPAMPPRIV_W<'_, TZSC_PRIVCFGR2_SPEC> {
        OPAMPPRIV_W::new(self, 3)
    }
    #[doc = "Bit 4 - privileged access mode for COMP"]
    #[inline(always)]
    pub fn COMPPRIV(&mut self) -> COMPPRIV_W<'_, TZSC_PRIVCFGR2_SPEC> {
        COMPPRIV_W::new(self, 4)
    }
    #[doc = "Bit 8 - privileged access mode for TIM1"]
    #[inline(always)]
    pub fn TIM1PRIV(&mut self) -> TIM1PRIV_W<'_, TZSC_PRIVCFGR2_SPEC> {
        TIM1PRIV_W::new(self, 8)
    }
    #[doc = "Bit 9 - privileged access mode for SPI1"]
    #[inline(always)]
    pub fn SPI1PRIV(&mut self) -> SPI1PRIV_W<'_, TZSC_PRIVCFGR2_SPEC> {
        SPI1PRIV_W::new(self, 9)
    }
    #[doc = "Bit 11 - privileged access mode for USART1"]
    #[inline(always)]
    pub fn USART1PRIV(&mut self) -> USART1PRIV_W<'_, TZSC_PRIVCFGR2_SPEC> {
        USART1PRIV_W::new(self, 11)
    }
    #[doc = "Bit 19 - privileged access mode for USBSF"]
    #[inline(always)]
    pub fn USBFSPRIV(&mut self) -> USBFSPRIV_W<'_, TZSC_PRIVCFGR2_SPEC> {
        USBFSPRIV_W::new(self, 19)
    }
    #[doc = "Bit 25 - privileged access mode for LPUART"]
    #[inline(always)]
    pub fn LPUART1PRIV(&mut self) -> LPUART1PRIV_W<'_, TZSC_PRIVCFGR2_SPEC> {
        LPUART1PRIV_W::new(self, 25)
    }
    #[doc = "Bit 28 - privileged access mode for LPTIM1"]
    #[inline(always)]
    pub fn LPTIM1PRIV(&mut self) -> LPTIM1PRIV_W<'_, TZSC_PRIVCFGR2_SPEC> {
        LPTIM1PRIV_W::new(self, 28)
    }
}
#[doc = "GTZC1 TZSC privilege configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_PRIVCFGR2_SPEC;
impl crate::RegisterSpec for TZSC_PRIVCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_privcfgr2::R`](R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzsc_privcfgr2::W`](W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TZSC_PRIVCFGR2 to value 0"]
impl crate::Resettable for TZSC_PRIVCFGR2_SPEC {}

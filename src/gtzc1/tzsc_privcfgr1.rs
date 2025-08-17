#[doc = "Register `TZSC_PRIVCFGR1` reader"]
pub type R = crate::R<TZSC_PRIVCFGR1_SPEC>;
#[doc = "Register `TZSC_PRIVCFGR1` writer"]
pub type W = crate::W<TZSC_PRIVCFGR1_SPEC>;
#[doc = "privileged access mode for TIM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<TIM2PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2PRIV` reader - privileged access mode for TIM2"]
pub type TIM2PRIV_R = crate::BitReader<TIM2PRIV_A>;
impl TIM2PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2PRIV_A {
        match self.bits {
            false => TIM2PRIV_A::B_0x0,
            true => TIM2PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM2PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM2PRIV_A::B_0x1
    }
}
#[doc = "Field `TIM2PRIV` writer - privileged access mode for TIM2"]
pub type TIM2PRIV_W<'a, REG> = crate::BitWriter<'a, REG, TIM2PRIV_A>;
impl<'a, REG> TIM2PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for TIM3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<TIM3PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: TIM3PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3PRIV` reader - privileged access mode for TIM3"]
pub type TIM3PRIV_R = crate::BitReader<TIM3PRIV_A>;
impl TIM3PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM3PRIV_A {
        match self.bits {
            false => TIM3PRIV_A::B_0x0,
            true => TIM3PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM3PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM3PRIV_A::B_0x1
    }
}
#[doc = "Field `TIM3PRIV` writer - privileged access mode for TIM3"]
pub type TIM3PRIV_W<'a, REG> = crate::BitWriter<'a, REG, TIM3PRIV_A>;
impl<'a, REG> TIM3PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for TIM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM6PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<TIM6PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: TIM6PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6PRIV` reader - privileged access mode for TIM6"]
pub type TIM6PRIV_R = crate::BitReader<TIM6PRIV_A>;
impl TIM6PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM6PRIV_A {
        match self.bits {
            false => TIM6PRIV_A::B_0x0,
            true => TIM6PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM6PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM6PRIV_A::B_0x1
    }
}
#[doc = "Field `TIM6PRIV` writer - privileged access mode for TIM6"]
pub type TIM6PRIV_W<'a, REG> = crate::BitWriter<'a, REG, TIM6PRIV_A>;
impl<'a, REG> TIM6PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for TIM7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM7PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<TIM7PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: TIM7PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7PRIV` reader - privileged access mode for TIM7"]
pub type TIM7PRIV_R = crate::BitReader<TIM7PRIV_A>;
impl TIM7PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM7PRIV_A {
        match self.bits {
            false => TIM7PRIV_A::B_0x0,
            true => TIM7PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM7PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM7PRIV_A::B_0x1
    }
}
#[doc = "Field `TIM7PRIV` writer - privileged access mode for TIM7"]
pub type TIM7PRIV_W<'a, REG> = crate::BitWriter<'a, REG, TIM7PRIV_A>;
impl<'a, REG> TIM7PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for WWDG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<WWDGPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: WWDGPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGPRIV` reader - privileged access mode for WWDG"]
pub type WWDGPRIV_R = crate::BitReader<WWDGPRIV_A>;
impl WWDGPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDGPRIV_A {
        match self.bits {
            false => WWDGPRIV_A::B_0x0,
            true => WWDGPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WWDGPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WWDGPRIV_A::B_0x1
    }
}
#[doc = "Field `WWDGPRIV` writer - privileged access mode for WWDG"]
pub type WWDGPRIV_W<'a, REG> = crate::BitWriter<'a, REG, WWDGPRIV_A>;
impl<'a, REG> WWDGPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for IWDG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDGPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<IWDGPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: IWDGPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDGPRIV` reader - privileged access mode for IWDG"]
pub type IWDGPRIV_R = crate::BitReader<IWDGPRIV_A>;
impl IWDGPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDGPRIV_A {
        match self.bits {
            false => IWDGPRIV_A::B_0x0,
            true => IWDGPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IWDGPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IWDGPRIV_A::B_0x1
    }
}
#[doc = "Field `IWDGPRIV` writer - privileged access mode for IWDG"]
pub type IWDGPRIV_W<'a, REG> = crate::BitWriter<'a, REG, IWDGPRIV_A>;
impl<'a, REG> IWDGPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IWDGPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IWDGPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for SPI2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<SPI2PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2PRIV` reader - privileged access mode for SPI2"]
pub type SPI2PRIV_R = crate::BitReader<SPI2PRIV_A>;
impl SPI2PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI2PRIV_A {
        match self.bits {
            false => SPI2PRIV_A::B_0x0,
            true => SPI2PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI2PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI2PRIV_A::B_0x1
    }
}
#[doc = "Field `SPI2PRIV` writer - privileged access mode for SPI2"]
pub type SPI2PRIV_W<'a, REG> = crate::BitWriter<'a, REG, SPI2PRIV_A>;
impl<'a, REG> SPI2PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for SPI3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI3PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<SPI3PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: SPI3PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI3PRIV` reader - privileged access mode for SPI3"]
pub type SPI3PRIV_R = crate::BitReader<SPI3PRIV_A>;
impl SPI3PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI3PRIV_A {
        match self.bits {
            false => SPI3PRIV_A::B_0x0,
            true => SPI3PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI3PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI3PRIV_A::B_0x1
    }
}
#[doc = "Field `SPI3PRIV` writer - privileged access mode for SPI3"]
pub type SPI3PRIV_W<'a, REG> = crate::BitWriter<'a, REG, SPI3PRIV_A>;
impl<'a, REG> SPI3PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for USART2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<USART2PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: USART2PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2PRIV` reader - privileged access mode for USART2"]
pub type USART2PRIV_R = crate::BitReader<USART2PRIV_A>;
impl USART2PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART2PRIV_A {
        match self.bits {
            false => USART2PRIV_A::B_0x0,
            true => USART2PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART2PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART2PRIV_A::B_0x1
    }
}
#[doc = "Field `USART2PRIV` writer - privileged access mode for USART2"]
pub type USART2PRIV_W<'a, REG> = crate::BitWriter<'a, REG, USART2PRIV_A>;
impl<'a, REG> USART2PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART2PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for USART3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART3PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<USART3PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: USART3PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART3PRIV` reader - privileged access mode for USART3"]
pub type USART3PRIV_R = crate::BitReader<USART3PRIV_A>;
impl USART3PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART3PRIV_A {
        match self.bits {
            false => USART3PRIV_A::B_0x0,
            true => USART3PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART3PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART3PRIV_A::B_0x1
    }
}
#[doc = "Field `USART3PRIV` writer - privileged access mode for USART3"]
pub type USART3PRIV_W<'a, REG> = crate::BitWriter<'a, REG, USART3PRIV_A>;
impl<'a, REG> USART3PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART3PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART3PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for I2C1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<I2C1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1PRIV` reader - privileged access mode for I2C1"]
pub type I2C1PRIV_R = crate::BitReader<I2C1PRIV_A>;
impl I2C1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1PRIV_A {
        match self.bits {
            false => I2C1PRIV_A::B_0x0,
            true => I2C1PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2C1PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2C1PRIV_A::B_0x1
    }
}
#[doc = "Field `I2C1PRIV` writer - privileged access mode for I2C1"]
pub type I2C1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, I2C1PRIV_A>;
impl<'a, REG> I2C1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for I2C2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<I2C2PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2PRIV` reader - privileged access mode for I2C2"]
pub type I2C2PRIV_R = crate::BitReader<I2C2PRIV_A>;
impl I2C2PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C2PRIV_A {
        match self.bits {
            false => I2C2PRIV_A::B_0x0,
            true => I2C2PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2C2PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2C2PRIV_A::B_0x1
    }
}
#[doc = "Field `I2C2PRIV` writer - privileged access mode for I2C2"]
pub type I2C2PRIV_W<'a, REG> = crate::BitWriter<'a, REG, I2C2PRIV_A>;
impl<'a, REG> I2C2PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for I3C1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C1PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<I3C1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: I3C1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C1PRIV` reader - privileged access mode for I3C1"]
pub type I3C1PRIV_R = crate::BitReader<I3C1PRIV_A>;
impl I3C1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3C1PRIV_A {
        match self.bits {
            false => I3C1PRIV_A::B_0x0,
            true => I3C1PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I3C1PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I3C1PRIV_A::B_0x1
    }
}
#[doc = "Field `I3C1PRIV` writer - privileged access mode for I3C1"]
pub type I3C1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, I3C1PRIV_A>;
impl<'a, REG> I3C1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for CRS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<CRSPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: CRSPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSPRIV` reader - privileged access mode for CRS"]
pub type CRSPRIV_R = crate::BitReader<CRSPRIV_A>;
impl CRSPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRSPRIV_A {
        match self.bits {
            false => CRSPRIV_A::B_0x0,
            true => CRSPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRSPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRSPRIV_A::B_0x1
    }
}
#[doc = "Field `CRSPRIV` writer - privileged access mode for CRS"]
pub type CRSPRIV_W<'a, REG> = crate::BitWriter<'a, REG, CRSPRIV_A>;
impl<'a, REG> CRSPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRSPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRSPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for DAC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<DAC1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1PRIV` reader - privileged access mode for DAC1"]
pub type DAC1PRIV_R = crate::BitReader<DAC1PRIV_A>;
impl DAC1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC1PRIV_A {
        match self.bits {
            false => DAC1PRIV_A::B_0x0,
            true => DAC1PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DAC1PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DAC1PRIV_A::B_0x1
    }
}
#[doc = "Field `DAC1PRIV` writer - privileged access mode for DAC1"]
pub type DAC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, DAC1PRIV_A>;
impl<'a, REG> DAC1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1PRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for DTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSPRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<DTSPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: DTSPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTSPRIV` reader - privileged access mode for DTS"]
pub type DTSPRIV_R = crate::BitReader<DTSPRIV_A>;
impl DTSPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTSPRIV_A {
        match self.bits {
            false => DTSPRIV_A::B_0x0,
            true => DTSPRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DTSPRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DTSPRIV_A::B_0x1
    }
}
#[doc = "Field `DTSPRIV` writer - privileged access mode for DTS"]
pub type DTSPRIV_W<'a, REG> = crate::BitWriter<'a, REG, DTSPRIV_A>;
impl<'a, REG> DTSPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DTSPRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DTSPRIV_A::B_0x1)
    }
}
#[doc = "privileged access mode for LPTIM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2PRIV_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<LPTIM2PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2PRIV` reader - privileged access mode for LPTIM2"]
pub type LPTIM2PRIV_R = crate::BitReader<LPTIM2PRIV_A>;
impl LPTIM2PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2PRIV_A {
        match self.bits {
            false => LPTIM2PRIV_A::B_0x0,
            true => LPTIM2PRIV_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPTIM2PRIV_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPTIM2PRIV_A::B_0x1
    }
}
#[doc = "Field `LPTIM2PRIV` writer - privileged access mode for LPTIM2"]
pub type LPTIM2PRIV_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM2PRIV_A>;
impl<'a, REG> LPTIM2PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2PRIV_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2PRIV_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - privileged access mode for TIM2"]
    #[inline(always)]
    pub fn TIM2PRIV(&self) -> TIM2PRIV_R {
        TIM2PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - privileged access mode for TIM3"]
    #[inline(always)]
    pub fn TIM3PRIV(&self) -> TIM3PRIV_R {
        TIM3PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - privileged access mode for TIM6"]
    #[inline(always)]
    pub fn TIM6PRIV(&self) -> TIM6PRIV_R {
        TIM6PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - privileged access mode for TIM7"]
    #[inline(always)]
    pub fn TIM7PRIV(&self) -> TIM7PRIV_R {
        TIM7PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - privileged access mode for WWDG"]
    #[inline(always)]
    pub fn WWDGPRIV(&self) -> WWDGPRIV_R {
        WWDGPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - privileged access mode for IWDG"]
    #[inline(always)]
    pub fn IWDGPRIV(&self) -> IWDGPRIV_R {
        IWDGPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for SPI2"]
    #[inline(always)]
    pub fn SPI2PRIV(&self) -> SPI2PRIV_R {
        SPI2PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for SPI3"]
    #[inline(always)]
    pub fn SPI3PRIV(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - privileged access mode for USART2"]
    #[inline(always)]
    pub fn USART2PRIV(&self) -> USART2PRIV_R {
        USART2PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for USART3"]
    #[inline(always)]
    pub fn USART3PRIV(&self) -> USART3PRIV_R {
        USART3PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - privileged access mode for I2C1"]
    #[inline(always)]
    pub fn I2C1PRIV(&self) -> I2C1PRIV_R {
        I2C1PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - privileged access mode for I2C2"]
    #[inline(always)]
    pub fn I2C2PRIV(&self) -> I2C2PRIV_R {
        I2C2PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - privileged access mode for I3C1"]
    #[inline(always)]
    pub fn I3C1PRIV(&self) -> I3C1PRIV_R {
        I3C1PRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - privileged access mode for CRS"]
    #[inline(always)]
    pub fn CRSPRIV(&self) -> CRSPRIV_R {
        CRSPRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - privileged access mode for DAC1"]
    #[inline(always)]
    pub fn DAC1PRIV(&self) -> DAC1PRIV_R {
        DAC1PRIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - privileged access mode for DTS"]
    #[inline(always)]
    pub fn DTSPRIV(&self) -> DTSPRIV_R {
        DTSPRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - privileged access mode for LPTIM2"]
    #[inline(always)]
    pub fn LPTIM2PRIV(&self) -> LPTIM2PRIV_R {
        LPTIM2PRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for TIM2"]
    #[inline(always)]
    pub fn TIM2PRIV(&mut self) -> TIM2PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        TIM2PRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - privileged access mode for TIM3"]
    #[inline(always)]
    pub fn TIM3PRIV(&mut self) -> TIM3PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        TIM3PRIV_W::new(self, 1)
    }
    #[doc = "Bit 4 - privileged access mode for TIM6"]
    #[inline(always)]
    pub fn TIM6PRIV(&mut self) -> TIM6PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        TIM6PRIV_W::new(self, 4)
    }
    #[doc = "Bit 5 - privileged access mode for TIM7"]
    #[inline(always)]
    pub fn TIM7PRIV(&mut self) -> TIM7PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        TIM7PRIV_W::new(self, 5)
    }
    #[doc = "Bit 9 - privileged access mode for WWDG"]
    #[inline(always)]
    pub fn WWDGPRIV(&mut self) -> WWDGPRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        WWDGPRIV_W::new(self, 9)
    }
    #[doc = "Bit 10 - privileged access mode for IWDG"]
    #[inline(always)]
    pub fn IWDGPRIV(&mut self) -> IWDGPRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        IWDGPRIV_W::new(self, 10)
    }
    #[doc = "Bit 11 - privileged access mode for SPI2"]
    #[inline(always)]
    pub fn SPI2PRIV(&mut self) -> SPI2PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        SPI2PRIV_W::new(self, 11)
    }
    #[doc = "Bit 12 - privileged access mode for SPI3"]
    #[inline(always)]
    pub fn SPI3PRIV(&mut self) -> SPI3PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        SPI3PRIV_W::new(self, 12)
    }
    #[doc = "Bit 13 - privileged access mode for USART2"]
    #[inline(always)]
    pub fn USART2PRIV(&mut self) -> USART2PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        USART2PRIV_W::new(self, 13)
    }
    #[doc = "Bit 14 - privileged access mode for USART3"]
    #[inline(always)]
    pub fn USART3PRIV(&mut self) -> USART3PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        USART3PRIV_W::new(self, 14)
    }
    #[doc = "Bit 17 - privileged access mode for I2C1"]
    #[inline(always)]
    pub fn I2C1PRIV(&mut self) -> I2C1PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        I2C1PRIV_W::new(self, 17)
    }
    #[doc = "Bit 18 - privileged access mode for I2C2"]
    #[inline(always)]
    pub fn I2C2PRIV(&mut self) -> I2C2PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        I2C2PRIV_W::new(self, 18)
    }
    #[doc = "Bit 19 - privileged access mode for I3C1"]
    #[inline(always)]
    pub fn I3C1PRIV(&mut self) -> I3C1PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        I3C1PRIV_W::new(self, 19)
    }
    #[doc = "Bit 20 - privileged access mode for CRS"]
    #[inline(always)]
    pub fn CRSPRIV(&mut self) -> CRSPRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        CRSPRIV_W::new(self, 20)
    }
    #[doc = "Bit 25 - privileged access mode for DAC1"]
    #[inline(always)]
    pub fn DAC1PRIV(&mut self) -> DAC1PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        DAC1PRIV_W::new(self, 25)
    }
    #[doc = "Bit 30 - privileged access mode for DTS"]
    #[inline(always)]
    pub fn DTSPRIV(&mut self) -> DTSPRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        DTSPRIV_W::new(self, 30)
    }
    #[doc = "Bit 31 - privileged access mode for LPTIM2"]
    #[inline(always)]
    pub fn LPTIM2PRIV(&mut self) -> LPTIM2PRIV_W<'_, TZSC_PRIVCFGR1_SPEC> {
        LPTIM2PRIV_W::new(self, 31)
    }
}
#[doc = "GTZC1 TZSC privilege configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_PRIVCFGR1_SPEC;
impl crate::RegisterSpec for TZSC_PRIVCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_privcfgr1::R`](R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzsc_privcfgr1::W`](W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TZSC_PRIVCFGR1 to value 0"]
impl crate::Resettable for TZSC_PRIVCFGR1_SPEC {}

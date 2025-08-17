#[doc = "Register `APB1LENR` reader"]
pub type R = crate::R<APB1LENR_SPEC>;
#[doc = "Register `APB1LENR` writer"]
pub type W = crate::W<APB1LENR_SPEC>;
#[doc = "TIM2 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN_A {
    #[doc = "0: TIM2 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: TIM2 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<TIM2EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2EN` reader - TIM2 clock enable Set and reset by software."]
pub type TIM2EN_R = crate::BitReader<TIM2EN_A>;
impl TIM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2EN_A {
        match self.bits {
            false => TIM2EN_A::B_0x0,
            true => TIM2EN_A::B_0x1,
        }
    }
    #[doc = "TIM2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM2EN_A::B_0x0
    }
    #[doc = "TIM2 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM2EN_A::B_0x1
    }
}
#[doc = "Field `TIM2EN` writer - TIM2 clock enable Set and reset by software."]
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN_A>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN_A::B_0x0)
    }
    #[doc = "TIM2 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN_A::B_0x1)
    }
}
#[doc = "TIM3 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3EN_A {
    #[doc = "0: TIM3 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: TIM3 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<TIM3EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3EN` reader - TIM3 clock enable Set and reset by software."]
pub type TIM3EN_R = crate::BitReader<TIM3EN_A>;
impl TIM3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM3EN_A {
        match self.bits {
            false => TIM3EN_A::B_0x0,
            true => TIM3EN_A::B_0x1,
        }
    }
    #[doc = "TIM3 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM3EN_A::B_0x0
    }
    #[doc = "TIM3 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM3EN_A::B_0x1
    }
}
#[doc = "Field `TIM3EN` writer - TIM3 clock enable Set and reset by software."]
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM3EN_A>;
impl<'a, REG> TIM3EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM3 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3EN_A::B_0x0)
    }
    #[doc = "TIM3 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3EN_A::B_0x1)
    }
}
#[doc = "TIM6 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM6EN_A {
    #[doc = "0: TIM6 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: TIM6 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<TIM6EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM6EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6EN` reader - TIM6 clock enable Set and reset by software."]
pub type TIM6EN_R = crate::BitReader<TIM6EN_A>;
impl TIM6EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM6EN_A {
        match self.bits {
            false => TIM6EN_A::B_0x0,
            true => TIM6EN_A::B_0x1,
        }
    }
    #[doc = "TIM6 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM6EN_A::B_0x0
    }
    #[doc = "TIM6 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM6EN_A::B_0x1
    }
}
#[doc = "Field `TIM6EN` writer - TIM6 clock enable Set and reset by software."]
pub type TIM6EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM6EN_A>;
impl<'a, REG> TIM6EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM6 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6EN_A::B_0x0)
    }
    #[doc = "TIM6 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6EN_A::B_0x1)
    }
}
#[doc = "TIM7 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM7EN_A {
    #[doc = "0: TIM7 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: TIM7 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<TIM7EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM7EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7EN` reader - TIM7 clock enable Set and reset by software."]
pub type TIM7EN_R = crate::BitReader<TIM7EN_A>;
impl TIM7EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM7EN_A {
        match self.bits {
            false => TIM7EN_A::B_0x0,
            true => TIM7EN_A::B_0x1,
        }
    }
    #[doc = "TIM7 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM7EN_A::B_0x0
    }
    #[doc = "TIM7 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM7EN_A::B_0x1
    }
}
#[doc = "Field `TIM7EN` writer - TIM7 clock enable Set and reset by software."]
pub type TIM7EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM7EN_A>;
impl<'a, REG> TIM7EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM7 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7EN_A::B_0x0)
    }
    #[doc = "TIM7 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7EN_A::B_0x1)
    }
}
#[doc = "WWDG clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGEN_A {
    #[doc = "0: WWDG peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: WWDG peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<WWDGEN_A> for bool {
    #[inline(always)]
    fn from(variant: WWDGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGEN` reader - WWDG clock enable Set and reset by software."]
pub type WWDGEN_R = crate::BitReader<WWDGEN_A>;
impl WWDGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDGEN_A {
        match self.bits {
            false => WWDGEN_A::B_0x0,
            true => WWDGEN_A::B_0x1,
        }
    }
    #[doc = "WWDG peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WWDGEN_A::B_0x0
    }
    #[doc = "WWDG peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WWDGEN_A::B_0x1
    }
}
#[doc = "Field `WWDGEN` writer - WWDG clock enable Set and reset by software."]
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG, WWDGEN_A>;
impl<'a, REG> WWDGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WWDG peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGEN_A::B_0x0)
    }
    #[doc = "WWDG peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGEN_A::B_0x1)
    }
}
#[doc = "OPAMP clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMPEN_A {
    #[doc = "0: OPAMP peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: OPAMP peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<OPAMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAMPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAMPEN` reader - OPAMP clock enable Set and reset by software."]
pub type OPAMPEN_R = crate::BitReader<OPAMPEN_A>;
impl OPAMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAMPEN_A {
        match self.bits {
            false => OPAMPEN_A::B_0x0,
            true => OPAMPEN_A::B_0x1,
        }
    }
    #[doc = "OPAMP peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OPAMPEN_A::B_0x0
    }
    #[doc = "OPAMP peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OPAMPEN_A::B_0x1
    }
}
#[doc = "Field `OPAMPEN` writer - OPAMP clock enable Set and reset by software."]
pub type OPAMPEN_W<'a, REG> = crate::BitWriter<'a, REG, OPAMPEN_A>;
impl<'a, REG> OPAMPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OPAMP peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPEN_A::B_0x0)
    }
    #[doc = "OPAMP peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPEN_A::B_0x1)
    }
}
#[doc = "SPI2 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2EN_A {
    #[doc = "0: SPI2 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: SPI2 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<SPI2EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2EN` reader - SPI2 clock enable Set and reset by software."]
pub type SPI2EN_R = crate::BitReader<SPI2EN_A>;
impl SPI2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI2EN_A {
        match self.bits {
            false => SPI2EN_A::B_0x0,
            true => SPI2EN_A::B_0x1,
        }
    }
    #[doc = "SPI2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI2EN_A::B_0x0
    }
    #[doc = "SPI2 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI2EN_A::B_0x1
    }
}
#[doc = "Field `SPI2EN` writer - SPI2 clock enable Set and reset by software."]
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG, SPI2EN_A>;
impl<'a, REG> SPI2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2EN_A::B_0x0)
    }
    #[doc = "SPI2 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2EN_A::B_0x1)
    }
}
#[doc = "SPI3 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI3EN_A {
    #[doc = "0: SPI3 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: SPI3 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<SPI3EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI3EN` reader - SPI3 clock enable Set and reset by software."]
pub type SPI3EN_R = crate::BitReader<SPI3EN_A>;
impl SPI3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI3EN_A {
        match self.bits {
            false => SPI3EN_A::B_0x0,
            true => SPI3EN_A::B_0x1,
        }
    }
    #[doc = "SPI3 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI3EN_A::B_0x0
    }
    #[doc = "SPI3 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI3EN_A::B_0x1
    }
}
#[doc = "Field `SPI3EN` writer - SPI3 clock enable Set and reset by software."]
pub type SPI3EN_W<'a, REG> = crate::BitWriter<'a, REG, SPI3EN_A>;
impl<'a, REG> SPI3EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI3 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3EN_A::B_0x0)
    }
    #[doc = "SPI3 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3EN_A::B_0x1)
    }
}
#[doc = "COMP clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPEN_A {
    #[doc = "0: COMP peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: COMP peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<COMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPEN` reader - COMP clock enable Set and reset by software."]
pub type COMPEN_R = crate::BitReader<COMPEN_A>;
impl COMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMPEN_A {
        match self.bits {
            false => COMPEN_A::B_0x0,
            true => COMPEN_A::B_0x1,
        }
    }
    #[doc = "COMP peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == COMPEN_A::B_0x0
    }
    #[doc = "COMP peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == COMPEN_A::B_0x1
    }
}
#[doc = "Field `COMPEN` writer - COMP clock enable Set and reset by software."]
pub type COMPEN_W<'a, REG> = crate::BitWriter<'a, REG, COMPEN_A>;
impl<'a, REG> COMPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMPEN_A::B_0x0)
    }
    #[doc = "COMP peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMPEN_A::B_0x1)
    }
}
#[doc = "USART2 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2EN_A {
    #[doc = "0: USART2 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: USART2 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<USART2EN_A> for bool {
    #[inline(always)]
    fn from(variant: USART2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2EN` reader - USART2 clock enable Set and reset by software."]
pub type USART2EN_R = crate::BitReader<USART2EN_A>;
impl USART2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART2EN_A {
        match self.bits {
            false => USART2EN_A::B_0x0,
            true => USART2EN_A::B_0x1,
        }
    }
    #[doc = "USART2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART2EN_A::B_0x0
    }
    #[doc = "USART2 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART2EN_A::B_0x1
    }
}
#[doc = "Field `USART2EN` writer - USART2 clock enable Set and reset by software."]
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG, USART2EN_A>;
impl<'a, REG> USART2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART2EN_A::B_0x0)
    }
    #[doc = "USART2 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2EN_A::B_0x1)
    }
}
#[doc = "USART3 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART3EN_A {
    #[doc = "0: USART3 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: USART3 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<USART3EN_A> for bool {
    #[inline(always)]
    fn from(variant: USART3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART3EN` reader - USART3 clock enable Set and reset by software."]
pub type USART3EN_R = crate::BitReader<USART3EN_A>;
impl USART3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART3EN_A {
        match self.bits {
            false => USART3EN_A::B_0x0,
            true => USART3EN_A::B_0x1,
        }
    }
    #[doc = "USART3 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART3EN_A::B_0x0
    }
    #[doc = "USART3 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART3EN_A::B_0x1
    }
}
#[doc = "Field `USART3EN` writer - USART3 clock enable Set and reset by software."]
pub type USART3EN_W<'a, REG> = crate::BitWriter<'a, REG, USART3EN_A>;
impl<'a, REG> USART3EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART3 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART3EN_A::B_0x0)
    }
    #[doc = "USART3 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART3EN_A::B_0x1)
    }
}
#[doc = "I2C1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1EN_A {
    #[doc = "0: I2C1 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: I2C1 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<I2C1EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1EN` reader - I2C1 clock enable Set and reset by software."]
pub type I2C1EN_R = crate::BitReader<I2C1EN_A>;
impl I2C1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1EN_A {
        match self.bits {
            false => I2C1EN_A::B_0x0,
            true => I2C1EN_A::B_0x1,
        }
    }
    #[doc = "I2C1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2C1EN_A::B_0x0
    }
    #[doc = "I2C1 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2C1EN_A::B_0x1
    }
}
#[doc = "Field `I2C1EN` writer - I2C1 clock enable Set and reset by software."]
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG, I2C1EN_A>;
impl<'a, REG> I2C1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1EN_A::B_0x0)
    }
    #[doc = "I2C1 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1EN_A::B_0x1)
    }
}
#[doc = "I2C2 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2EN_A {
    #[doc = "0: I2C2 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: I2C2 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<I2C2EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2EN` reader - I2C2 clock enable Set and reset by software."]
pub type I2C2EN_R = crate::BitReader<I2C2EN_A>;
impl I2C2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C2EN_A {
        match self.bits {
            false => I2C2EN_A::B_0x0,
            true => I2C2EN_A::B_0x1,
        }
    }
    #[doc = "I2C2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2C2EN_A::B_0x0
    }
    #[doc = "I2C2 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2C2EN_A::B_0x1
    }
}
#[doc = "Field `I2C2EN` writer - I2C2 clock enable Set and reset by software."]
pub type I2C2EN_W<'a, REG> = crate::BitWriter<'a, REG, I2C2EN_A>;
impl<'a, REG> I2C2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2EN_A::B_0x0)
    }
    #[doc = "I2C2 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2EN_A::B_0x1)
    }
}
#[doc = "I3C1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C1EN_A {
    #[doc = "0: I3C1 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: I3C1 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<I3C1EN_A> for bool {
    #[inline(always)]
    fn from(variant: I3C1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C1EN` reader - I3C1 clock enable Set and reset by software."]
pub type I3C1EN_R = crate::BitReader<I3C1EN_A>;
impl I3C1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3C1EN_A {
        match self.bits {
            false => I3C1EN_A::B_0x0,
            true => I3C1EN_A::B_0x1,
        }
    }
    #[doc = "I3C1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I3C1EN_A::B_0x0
    }
    #[doc = "I3C1 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I3C1EN_A::B_0x1
    }
}
#[doc = "Field `I3C1EN` writer - I3C1 clock enable Set and reset by software."]
pub type I3C1EN_W<'a, REG> = crate::BitWriter<'a, REG, I3C1EN_A>;
impl<'a, REG> I3C1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I3C1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1EN_A::B_0x0)
    }
    #[doc = "I3C1 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1EN_A::B_0x1)
    }
}
#[doc = "CRS clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSEN_A {
    #[doc = "0: CRS peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: CRS peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<CRSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSEN` reader - CRS clock enable Set and reset by software."]
pub type CRSEN_R = crate::BitReader<CRSEN_A>;
impl CRSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRSEN_A {
        match self.bits {
            false => CRSEN_A::B_0x0,
            true => CRSEN_A::B_0x1,
        }
    }
    #[doc = "CRS peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRSEN_A::B_0x0
    }
    #[doc = "CRS peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRSEN_A::B_0x1
    }
}
#[doc = "Field `CRSEN` writer - CRS clock enable Set and reset by software."]
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSEN_A>;
impl<'a, REG> CRSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRS peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRSEN_A::B_0x0)
    }
    #[doc = "CRS peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRSEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn TIM2EN(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn TIM3EN(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn TIM6EN(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn TIM7EN(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn WWDGEN(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - OPAMP clock enable Set and reset by software."]
    #[inline(always)]
    pub fn OPAMPEN(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SPI2EN(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SPI3EN(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - COMP clock enable Set and reset by software."]
    #[inline(always)]
    pub fn COMPEN(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn USART2EN(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn USART3EN(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn I2C1EN(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn I2C2EN(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn I3C1EN(&self) -> I3C1EN_R {
        I3C1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn CRSEN(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn TIM2EN(&mut self) -> TIM2EN_W<'_, APB1LENR_SPEC> {
        TIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn TIM3EN(&mut self) -> TIM3EN_W<'_, APB1LENR_SPEC> {
        TIM3EN_W::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn TIM6EN(&mut self) -> TIM6EN_W<'_, APB1LENR_SPEC> {
        TIM6EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn TIM7EN(&mut self) -> TIM7EN_W<'_, APB1LENR_SPEC> {
        TIM7EN_W::new(self, 5)
    }
    #[doc = "Bit 11 - WWDG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn WWDGEN(&mut self) -> WWDGEN_W<'_, APB1LENR_SPEC> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 13 - OPAMP clock enable Set and reset by software."]
    #[inline(always)]
    pub fn OPAMPEN(&mut self) -> OPAMPEN_W<'_, APB1LENR_SPEC> {
        OPAMPEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SPI2EN(&mut self) -> SPI2EN_W<'_, APB1LENR_SPEC> {
        SPI2EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SPI3EN(&mut self) -> SPI3EN_W<'_, APB1LENR_SPEC> {
        SPI3EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - COMP clock enable Set and reset by software."]
    #[inline(always)]
    pub fn COMPEN(&mut self) -> COMPEN_W<'_, APB1LENR_SPEC> {
        COMPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn USART2EN(&mut self) -> USART2EN_W<'_, APB1LENR_SPEC> {
        USART2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn USART3EN(&mut self) -> USART3EN_W<'_, APB1LENR_SPEC> {
        USART3EN_W::new(self, 18)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn I2C1EN(&mut self) -> I2C1EN_W<'_, APB1LENR_SPEC> {
        I2C1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn I2C2EN(&mut self) -> I2C2EN_W<'_, APB1LENR_SPEC> {
        I2C2EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I3C1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn I3C1EN(&mut self) -> I3C1EN_W<'_, APB1LENR_SPEC> {
        I3C1EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn CRSEN(&mut self) -> CRSEN_W<'_, APB1LENR_SPEC> {
        CRSEN_W::new(self, 24)
    }
}
#[doc = "RCC APB1 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LENR_SPEC;
impl crate::RegisterSpec for APB1LENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lenr::R`](R) reader structure"]
impl crate::Readable for APB1LENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1lenr::W`](W) writer structure"]
impl crate::Writable for APB1LENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB1LENR to value 0"]
impl crate::Resettable for APB1LENR_SPEC {}

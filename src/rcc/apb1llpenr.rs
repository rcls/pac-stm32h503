#[doc = "Register `APB1LLPENR` reader"]
pub type R = crate::R<APB1LLPENR_SPEC>;
#[doc = "Register `APB1LLPENR` writer"]
pub type W = crate::W<APB1LLPENR_SPEC>;
#[doc = "TIM2 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2LPEN_A {
    #[doc = "0: TIM2 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: TIM2 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<TIM2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2LPEN` reader - TIM2 clock enable during sleep mode Set and reset by software."]
pub type TIM2LPEN_R = crate::BitReader<TIM2LPEN_A>;
impl TIM2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2LPEN_A {
        match self.bits {
            false => TIM2LPEN_A::B_0x0,
            true => TIM2LPEN_A::B_0x1,
        }
    }
    #[doc = "TIM2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM2LPEN_A::B_0x0
    }
    #[doc = "TIM2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM2LPEN_A::B_0x1
    }
}
#[doc = "Field `TIM2LPEN` writer - TIM2 clock enable during sleep mode Set and reset by software."]
pub type TIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2LPEN_A>;
impl<'a, REG> TIM2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN_A::B_0x0)
    }
    #[doc = "TIM2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN_A::B_0x1)
    }
}
#[doc = "TIM3 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3LPEN_A {
    #[doc = "0: TIM3 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: TIM3 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<TIM3LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM3LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3LPEN` reader - TIM3 clock enable during sleep mode Set and reset by software."]
pub type TIM3LPEN_R = crate::BitReader<TIM3LPEN_A>;
impl TIM3LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM3LPEN_A {
        match self.bits {
            false => TIM3LPEN_A::B_0x0,
            true => TIM3LPEN_A::B_0x1,
        }
    }
    #[doc = "TIM3 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM3LPEN_A::B_0x0
    }
    #[doc = "TIM3 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM3LPEN_A::B_0x1
    }
}
#[doc = "Field `TIM3LPEN` writer - TIM3 clock enable during sleep mode Set and reset by software."]
pub type TIM3LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM3LPEN_A>;
impl<'a, REG> TIM3LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM3 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3LPEN_A::B_0x0)
    }
    #[doc = "TIM3 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3LPEN_A::B_0x1)
    }
}
#[doc = "TIM6 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM6LPEN_A {
    #[doc = "0: TIM6 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: TIM6 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<TIM6LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM6LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6LPEN` reader - TIM6 clock enable during sleep mode Set and reset by software."]
pub type TIM6LPEN_R = crate::BitReader<TIM6LPEN_A>;
impl TIM6LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM6LPEN_A {
        match self.bits {
            false => TIM6LPEN_A::B_0x0,
            true => TIM6LPEN_A::B_0x1,
        }
    }
    #[doc = "TIM6 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM6LPEN_A::B_0x0
    }
    #[doc = "TIM6 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM6LPEN_A::B_0x1
    }
}
#[doc = "Field `TIM6LPEN` writer - TIM6 clock enable during sleep mode Set and reset by software."]
pub type TIM6LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM6LPEN_A>;
impl<'a, REG> TIM6LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM6 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6LPEN_A::B_0x0)
    }
    #[doc = "TIM6 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6LPEN_A::B_0x1)
    }
}
#[doc = "TIM7 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM7LPEN_A {
    #[doc = "0: TIM7 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: TIM7 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<TIM7LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM7LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7LPEN` reader - TIM7 clock enable during sleep mode Set and reset by software."]
pub type TIM7LPEN_R = crate::BitReader<TIM7LPEN_A>;
impl TIM7LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM7LPEN_A {
        match self.bits {
            false => TIM7LPEN_A::B_0x0,
            true => TIM7LPEN_A::B_0x1,
        }
    }
    #[doc = "TIM7 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM7LPEN_A::B_0x0
    }
    #[doc = "TIM7 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM7LPEN_A::B_0x1
    }
}
#[doc = "Field `TIM7LPEN` writer - TIM7 clock enable during sleep mode Set and reset by software."]
pub type TIM7LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM7LPEN_A>;
impl<'a, REG> TIM7LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM7 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7LPEN_A::B_0x0)
    }
    #[doc = "TIM7 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7LPEN_A::B_0x1)
    }
}
#[doc = "WWDG clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGLPEN_A {
    #[doc = "0: WWDG peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: WWDG peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<WWDGLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WWDGLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGLPEN` reader - WWDG clock enable during sleep mode Set and reset by software."]
pub type WWDGLPEN_R = crate::BitReader<WWDGLPEN_A>;
impl WWDGLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDGLPEN_A {
        match self.bits {
            false => WWDGLPEN_A::B_0x0,
            true => WWDGLPEN_A::B_0x1,
        }
    }
    #[doc = "WWDG peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WWDGLPEN_A::B_0x0
    }
    #[doc = "WWDG peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WWDGLPEN_A::B_0x1
    }
}
#[doc = "Field `WWDGLPEN` writer - WWDG clock enable during sleep mode Set and reset by software."]
pub type WWDGLPEN_W<'a, REG> = crate::BitWriter<'a, REG, WWDGLPEN_A>;
impl<'a, REG> WWDGLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WWDG peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGLPEN_A::B_0x0)
    }
    #[doc = "WWDG peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGLPEN_A::B_0x1)
    }
}
#[doc = "OPAMP clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMPLPEN_A {
    #[doc = "0: OPAMP peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: OPAMP peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<OPAMPLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAMPLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAMPLPEN` reader - OPAMP clock enable during sleep mode Set and reset by software."]
pub type OPAMPLPEN_R = crate::BitReader<OPAMPLPEN_A>;
impl OPAMPLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAMPLPEN_A {
        match self.bits {
            false => OPAMPLPEN_A::B_0x0,
            true => OPAMPLPEN_A::B_0x1,
        }
    }
    #[doc = "OPAMP peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OPAMPLPEN_A::B_0x0
    }
    #[doc = "OPAMP peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OPAMPLPEN_A::B_0x1
    }
}
#[doc = "Field `OPAMPLPEN` writer - OPAMP clock enable during sleep mode Set and reset by software."]
pub type OPAMPLPEN_W<'a, REG> = crate::BitWriter<'a, REG, OPAMPLPEN_A>;
impl<'a, REG> OPAMPLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OPAMP peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPLPEN_A::B_0x0)
    }
    #[doc = "OPAMP peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPLPEN_A::B_0x1)
    }
}
#[doc = "SPI2 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2LPEN_A {
    #[doc = "0: SPI2 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: SPI2 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<SPI2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during sleep mode Set and reset by software."]
pub type SPI2LPEN_R = crate::BitReader<SPI2LPEN_A>;
impl SPI2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI2LPEN_A {
        match self.bits {
            false => SPI2LPEN_A::B_0x0,
            true => SPI2LPEN_A::B_0x1,
        }
    }
    #[doc = "SPI2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI2LPEN_A::B_0x0
    }
    #[doc = "SPI2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI2LPEN_A::B_0x1
    }
}
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during sleep mode Set and reset by software."]
pub type SPI2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, SPI2LPEN_A>;
impl<'a, REG> SPI2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2LPEN_A::B_0x0)
    }
    #[doc = "SPI2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2LPEN_A::B_0x1)
    }
}
#[doc = "SPI3 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI3LPEN_A {
    #[doc = "0: SPI3 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: SPI3 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<SPI3LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI3LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during sleep mode Set and reset by software."]
pub type SPI3LPEN_R = crate::BitReader<SPI3LPEN_A>;
impl SPI3LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI3LPEN_A {
        match self.bits {
            false => SPI3LPEN_A::B_0x0,
            true => SPI3LPEN_A::B_0x1,
        }
    }
    #[doc = "SPI3 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI3LPEN_A::B_0x0
    }
    #[doc = "SPI3 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI3LPEN_A::B_0x1
    }
}
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during sleep mode Set and reset by software."]
pub type SPI3LPEN_W<'a, REG> = crate::BitWriter<'a, REG, SPI3LPEN_A>;
impl<'a, REG> SPI3LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI3 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3LPEN_A::B_0x0)
    }
    #[doc = "SPI3 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3LPEN_A::B_0x1)
    }
}
#[doc = "COMP clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPLPEN_A {
    #[doc = "0: COMP peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: COMP peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<COMPLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPLPEN` reader - COMP clock enable during sleep mode Set and reset by software."]
pub type COMPLPEN_R = crate::BitReader<COMPLPEN_A>;
impl COMPLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMPLPEN_A {
        match self.bits {
            false => COMPLPEN_A::B_0x0,
            true => COMPLPEN_A::B_0x1,
        }
    }
    #[doc = "COMP peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == COMPLPEN_A::B_0x0
    }
    #[doc = "COMP peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == COMPLPEN_A::B_0x1
    }
}
#[doc = "Field `COMPLPEN` writer - COMP clock enable during sleep mode Set and reset by software."]
pub type COMPLPEN_W<'a, REG> = crate::BitWriter<'a, REG, COMPLPEN_A>;
impl<'a, REG> COMPLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMPLPEN_A::B_0x0)
    }
    #[doc = "COMP peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMPLPEN_A::B_0x1)
    }
}
#[doc = "USART2 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2LPEN_A {
    #[doc = "0: USART2 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: USART2 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<USART2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USART2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during sleep mode Set and reset by software."]
pub type USART2LPEN_R = crate::BitReader<USART2LPEN_A>;
impl USART2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART2LPEN_A {
        match self.bits {
            false => USART2LPEN_A::B_0x0,
            true => USART2LPEN_A::B_0x1,
        }
    }
    #[doc = "USART2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART2LPEN_A::B_0x0
    }
    #[doc = "USART2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART2LPEN_A::B_0x1
    }
}
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during sleep mode Set and reset by software."]
pub type USART2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, USART2LPEN_A>;
impl<'a, REG> USART2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART2LPEN_A::B_0x0)
    }
    #[doc = "USART2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2LPEN_A::B_0x1)
    }
}
#[doc = "USART3 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART3LPEN_A {
    #[doc = "0: USART3 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: USART3 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<USART3LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USART3LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART3LPEN` reader - USART3 clock enable during sleep mode Set and reset by software."]
pub type USART3LPEN_R = crate::BitReader<USART3LPEN_A>;
impl USART3LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART3LPEN_A {
        match self.bits {
            false => USART3LPEN_A::B_0x0,
            true => USART3LPEN_A::B_0x1,
        }
    }
    #[doc = "USART3 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART3LPEN_A::B_0x0
    }
    #[doc = "USART3 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART3LPEN_A::B_0x1
    }
}
#[doc = "Field `USART3LPEN` writer - USART3 clock enable during sleep mode Set and reset by software."]
pub type USART3LPEN_W<'a, REG> = crate::BitWriter<'a, REG, USART3LPEN_A>;
impl<'a, REG> USART3LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART3 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART3LPEN_A::B_0x0)
    }
    #[doc = "USART3 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART3LPEN_A::B_0x1)
    }
}
#[doc = "I2C1 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1LPEN_A {
    #[doc = "0: I2C1 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: I2C1 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<I2C1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1LPEN` reader - I2C1 clock enable during sleep mode Set and reset by software."]
pub type I2C1LPEN_R = crate::BitReader<I2C1LPEN_A>;
impl I2C1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1LPEN_A {
        match self.bits {
            false => I2C1LPEN_A::B_0x0,
            true => I2C1LPEN_A::B_0x1,
        }
    }
    #[doc = "I2C1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2C1LPEN_A::B_0x0
    }
    #[doc = "I2C1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2C1LPEN_A::B_0x1
    }
}
#[doc = "Field `I2C1LPEN` writer - I2C1 clock enable during sleep mode Set and reset by software."]
pub type I2C1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, I2C1LPEN_A>;
impl<'a, REG> I2C1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1LPEN_A::B_0x0)
    }
    #[doc = "I2C1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1LPEN_A::B_0x1)
    }
}
#[doc = "I2C2 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2LPEN_A {
    #[doc = "0: I2C2 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: I2C2 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<I2C2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2LPEN` reader - I2C2 clock enable during sleep mode Set and reset by software."]
pub type I2C2LPEN_R = crate::BitReader<I2C2LPEN_A>;
impl I2C2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C2LPEN_A {
        match self.bits {
            false => I2C2LPEN_A::B_0x0,
            true => I2C2LPEN_A::B_0x1,
        }
    }
    #[doc = "I2C2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2C2LPEN_A::B_0x0
    }
    #[doc = "I2C2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2C2LPEN_A::B_0x1
    }
}
#[doc = "Field `I2C2LPEN` writer - I2C2 clock enable during sleep mode Set and reset by software."]
pub type I2C2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, I2C2LPEN_A>;
impl<'a, REG> I2C2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2LPEN_A::B_0x0)
    }
    #[doc = "I2C2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2LPEN_A::B_0x1)
    }
}
#[doc = "I3C1 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C1LPEN_A {
    #[doc = "0: I3C1 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: I3C1 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<I3C1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: I3C1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C1LPEN` reader - I3C1 clock enable during sleep mode Set and reset by software."]
pub type I3C1LPEN_R = crate::BitReader<I3C1LPEN_A>;
impl I3C1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3C1LPEN_A {
        match self.bits {
            false => I3C1LPEN_A::B_0x0,
            true => I3C1LPEN_A::B_0x1,
        }
    }
    #[doc = "I3C1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I3C1LPEN_A::B_0x0
    }
    #[doc = "I3C1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I3C1LPEN_A::B_0x1
    }
}
#[doc = "Field `I3C1LPEN` writer - I3C1 clock enable during sleep mode Set and reset by software."]
pub type I3C1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, I3C1LPEN_A>;
impl<'a, REG> I3C1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I3C1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1LPEN_A::B_0x0)
    }
    #[doc = "I3C1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1LPEN_A::B_0x1)
    }
}
#[doc = "CRS clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSLPEN_A {
    #[doc = "0: CRS peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: CRS peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<CRSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRSLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSLPEN` reader - CRS clock enable during sleep mode Set and reset by software."]
pub type CRSLPEN_R = crate::BitReader<CRSLPEN_A>;
impl CRSLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRSLPEN_A {
        match self.bits {
            false => CRSLPEN_A::B_0x0,
            true => CRSLPEN_A::B_0x1,
        }
    }
    #[doc = "CRS peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRSLPEN_A::B_0x0
    }
    #[doc = "CRS peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRSLPEN_A::B_0x1
    }
}
#[doc = "Field `CRSLPEN` writer - CRS clock enable during sleep mode Set and reset by software."]
pub type CRSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSLPEN_A>;
impl<'a, REG> CRSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRS peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRSLPEN_A::B_0x0)
    }
    #[doc = "CRS peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRSLPEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn TIM2LPEN(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn TIM3LPEN(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn TIM6LPEN(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn TIM7LPEN(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn WWDGLPEN(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - OPAMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn OPAMPLPEN(&self) -> OPAMPLPEN_R {
        OPAMPLPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn SPI2LPEN(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn SPI3LPEN(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - COMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn COMPLPEN(&self) -> COMPLPEN_R {
        COMPLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn USART2LPEN(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn USART3LPEN(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn I2C1LPEN(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn I2C2LPEN(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn I3C1LPEN(&self) -> I3C1LPEN_R {
        I3C1LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn CRSLPEN(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn TIM2LPEN(&mut self) -> TIM2LPEN_W<'_, APB1LLPENR_SPEC> {
        TIM2LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn TIM3LPEN(&mut self) -> TIM3LPEN_W<'_, APB1LLPENR_SPEC> {
        TIM3LPEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn TIM6LPEN(&mut self) -> TIM6LPEN_W<'_, APB1LLPENR_SPEC> {
        TIM6LPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn TIM7LPEN(&mut self) -> TIM7LPEN_W<'_, APB1LLPENR_SPEC> {
        TIM7LPEN_W::new(self, 5)
    }
    #[doc = "Bit 11 - WWDG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn WWDGLPEN(&mut self) -> WWDGLPEN_W<'_, APB1LLPENR_SPEC> {
        WWDGLPEN_W::new(self, 11)
    }
    #[doc = "Bit 13 - OPAMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn OPAMPLPEN(&mut self) -> OPAMPLPEN_W<'_, APB1LLPENR_SPEC> {
        OPAMPLPEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn SPI2LPEN(&mut self) -> SPI2LPEN_W<'_, APB1LLPENR_SPEC> {
        SPI2LPEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn SPI3LPEN(&mut self) -> SPI3LPEN_W<'_, APB1LLPENR_SPEC> {
        SPI3LPEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - COMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn COMPLPEN(&mut self) -> COMPLPEN_W<'_, APB1LLPENR_SPEC> {
        COMPLPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn USART2LPEN(&mut self) -> USART2LPEN_W<'_, APB1LLPENR_SPEC> {
        USART2LPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn USART3LPEN(&mut self) -> USART3LPEN_W<'_, APB1LLPENR_SPEC> {
        USART3LPEN_W::new(self, 18)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn I2C1LPEN(&mut self) -> I2C1LPEN_W<'_, APB1LLPENR_SPEC> {
        I2C1LPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn I2C2LPEN(&mut self) -> I2C2LPEN_W<'_, APB1LLPENR_SPEC> {
        I2C2LPEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I3C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn I3C1LPEN(&mut self) -> I3C1LPEN_W<'_, APB1LLPENR_SPEC> {
        I3C1LPEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn CRSLPEN(&mut self) -> CRSLPEN_W<'_, APB1LLPENR_SPEC> {
        CRSLPEN_W::new(self, 24)
    }
}
#[doc = "RCC APB1 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1llpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LLPENR_SPEC;
impl crate::RegisterSpec for APB1LLPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1llpenr::R`](R) reader structure"]
impl crate::Readable for APB1LLPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1llpenr::W`](W) writer structure"]
impl crate::Writable for APB1LLPENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB1LLPENR to value 0xffff_ffff"]
impl crate::Resettable for APB1LLPENR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

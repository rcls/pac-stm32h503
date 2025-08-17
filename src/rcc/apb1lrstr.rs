#[doc = "Register `APB1LRSTR` reader"]
pub type R = crate::R<APB1LRSTR_SPEC>;
#[doc = "Register `APB1LRSTR` writer"]
pub type W = crate::W<APB1LRSTR_SPEC>;
#[doc = "TIM2 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST_A {
    #[doc = "0: does not reset the TIM2 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the TIM2 block"]
    B_0x1 = 1,
}
impl From<TIM2RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2RST` reader - TIM2 block reset Set and reset by software."]
pub type TIM2RST_R = crate::BitReader<TIM2RST_A>;
impl TIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2RST_A {
        match self.bits {
            false => TIM2RST_A::B_0x0,
            true => TIM2RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the TIM2 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM2RST_A::B_0x0
    }
    #[doc = "resets the TIM2 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM2RST_A::B_0x1
    }
}
#[doc = "Field `TIM2RST` writer - TIM2 block reset Set and reset by software."]
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM2RST_A>;
impl<'a, REG> TIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the TIM2 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST_A::B_0x0)
    }
    #[doc = "resets the TIM2 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST_A::B_0x1)
    }
}
#[doc = "TIM3 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3RST_A {
    #[doc = "0: does not reset the TIM3 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the TIM3 block"]
    B_0x1 = 1,
}
impl From<TIM3RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3RST` reader - TIM3 block reset Set and reset by software."]
pub type TIM3RST_R = crate::BitReader<TIM3RST_A>;
impl TIM3RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM3RST_A {
        match self.bits {
            false => TIM3RST_A::B_0x0,
            true => TIM3RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the TIM3 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM3RST_A::B_0x0
    }
    #[doc = "resets the TIM3 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM3RST_A::B_0x1
    }
}
#[doc = "Field `TIM3RST` writer - TIM3 block reset Set and reset by software."]
pub type TIM3RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM3RST_A>;
impl<'a, REG> TIM3RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the TIM3 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3RST_A::B_0x0)
    }
    #[doc = "resets the TIM3 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3RST_A::B_0x1)
    }
}
#[doc = "TIM6 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM6RST_A {
    #[doc = "0: does not reset the TIM6 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the TIM6 block"]
    B_0x1 = 1,
}
impl From<TIM6RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM6RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6RST` reader - TIM6 block reset Set and reset by software."]
pub type TIM6RST_R = crate::BitReader<TIM6RST_A>;
impl TIM6RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM6RST_A {
        match self.bits {
            false => TIM6RST_A::B_0x0,
            true => TIM6RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the TIM6 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM6RST_A::B_0x0
    }
    #[doc = "resets the TIM6 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM6RST_A::B_0x1
    }
}
#[doc = "Field `TIM6RST` writer - TIM6 block reset Set and reset by software."]
pub type TIM6RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM6RST_A>;
impl<'a, REG> TIM6RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the TIM6 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6RST_A::B_0x0)
    }
    #[doc = "resets the TIM6 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM6RST_A::B_0x1)
    }
}
#[doc = "TIM7 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM7RST_A {
    #[doc = "0: does not reset the TIM7 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the TIM7 block"]
    B_0x1 = 1,
}
impl From<TIM7RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM7RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7RST` reader - TIM7 block reset Set and reset by software."]
pub type TIM7RST_R = crate::BitReader<TIM7RST_A>;
impl TIM7RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM7RST_A {
        match self.bits {
            false => TIM7RST_A::B_0x0,
            true => TIM7RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the TIM7 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM7RST_A::B_0x0
    }
    #[doc = "resets the TIM7 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM7RST_A::B_0x1
    }
}
#[doc = "Field `TIM7RST` writer - TIM7 block reset Set and reset by software."]
pub type TIM7RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM7RST_A>;
impl<'a, REG> TIM7RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the TIM7 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7RST_A::B_0x0)
    }
    #[doc = "resets the TIM7 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM7RST_A::B_0x1)
    }
}
#[doc = "OPAMP block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMPRST_A {
    #[doc = "0: does not reset the OPAMP block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the OPAMP block"]
    B_0x1 = 1,
}
impl From<OPAMPRST_A> for bool {
    #[inline(always)]
    fn from(variant: OPAMPRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAMPRST` reader - OPAMP block reset Set and reset by software."]
pub type OPAMPRST_R = crate::BitReader<OPAMPRST_A>;
impl OPAMPRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAMPRST_A {
        match self.bits {
            false => OPAMPRST_A::B_0x0,
            true => OPAMPRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the OPAMP block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OPAMPRST_A::B_0x0
    }
    #[doc = "resets the OPAMP block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OPAMPRST_A::B_0x1
    }
}
#[doc = "Field `OPAMPRST` writer - OPAMP block reset Set and reset by software."]
pub type OPAMPRST_W<'a, REG> = crate::BitWriter<'a, REG, OPAMPRST_A>;
impl<'a, REG> OPAMPRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the OPAMP block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPRST_A::B_0x0)
    }
    #[doc = "resets the OPAMP block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPRST_A::B_0x1)
    }
}
#[doc = "SPI2 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2RST_A {
    #[doc = "0: does not reset the SPI2 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the SPI2 block"]
    B_0x1 = 1,
}
impl From<SPI2RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2RST` reader - SPI2 block reset Set and reset by software."]
pub type SPI2RST_R = crate::BitReader<SPI2RST_A>;
impl SPI2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI2RST_A {
        match self.bits {
            false => SPI2RST_A::B_0x0,
            true => SPI2RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the SPI2 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI2RST_A::B_0x0
    }
    #[doc = "resets the SPI2 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI2RST_A::B_0x1
    }
}
#[doc = "Field `SPI2RST` writer - SPI2 block reset Set and reset by software."]
pub type SPI2RST_W<'a, REG> = crate::BitWriter<'a, REG, SPI2RST_A>;
impl<'a, REG> SPI2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the SPI2 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2RST_A::B_0x0)
    }
    #[doc = "resets the SPI2 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2RST_A::B_0x1)
    }
}
#[doc = "SPI3 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI3RST_A {
    #[doc = "0: does not reset the SPI3 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the SPI3 block"]
    B_0x1 = 1,
}
impl From<SPI3RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI3RST` reader - SPI3 block reset Set and reset by software."]
pub type SPI3RST_R = crate::BitReader<SPI3RST_A>;
impl SPI3RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI3RST_A {
        match self.bits {
            false => SPI3RST_A::B_0x0,
            true => SPI3RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the SPI3 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI3RST_A::B_0x0
    }
    #[doc = "resets the SPI3 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI3RST_A::B_0x1
    }
}
#[doc = "Field `SPI3RST` writer - SPI3 block reset Set and reset by software."]
pub type SPI3RST_W<'a, REG> = crate::BitWriter<'a, REG, SPI3RST_A>;
impl<'a, REG> SPI3RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the SPI3 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3RST_A::B_0x0)
    }
    #[doc = "resets the SPI3 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3RST_A::B_0x1)
    }
}
#[doc = "COMP block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPRST_A {
    #[doc = "0: does not reset the COMP block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the COMP block"]
    B_0x1 = 1,
}
impl From<COMPRST_A> for bool {
    #[inline(always)]
    fn from(variant: COMPRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPRST` reader - COMP block reset Set and reset by software."]
pub type COMPRST_R = crate::BitReader<COMPRST_A>;
impl COMPRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMPRST_A {
        match self.bits {
            false => COMPRST_A::B_0x0,
            true => COMPRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the COMP block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == COMPRST_A::B_0x0
    }
    #[doc = "resets the COMP block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == COMPRST_A::B_0x1
    }
}
#[doc = "Field `COMPRST` writer - COMP block reset Set and reset by software."]
pub type COMPRST_W<'a, REG> = crate::BitWriter<'a, REG, COMPRST_A>;
impl<'a, REG> COMPRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the COMP block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMPRST_A::B_0x0)
    }
    #[doc = "resets the COMP block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMPRST_A::B_0x1)
    }
}
#[doc = "USART2 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2RST_A {
    #[doc = "0: does not reset the USART2 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the USART2 block"]
    B_0x1 = 1,
}
impl From<USART2RST_A> for bool {
    #[inline(always)]
    fn from(variant: USART2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2RST` reader - USART2 block reset Set and reset by software."]
pub type USART2RST_R = crate::BitReader<USART2RST_A>;
impl USART2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART2RST_A {
        match self.bits {
            false => USART2RST_A::B_0x0,
            true => USART2RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the USART2 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART2RST_A::B_0x0
    }
    #[doc = "resets the USART2 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART2RST_A::B_0x1
    }
}
#[doc = "Field `USART2RST` writer - USART2 block reset Set and reset by software."]
pub type USART2RST_W<'a, REG> = crate::BitWriter<'a, REG, USART2RST_A>;
impl<'a, REG> USART2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the USART2 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART2RST_A::B_0x0)
    }
    #[doc = "resets the USART2 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2RST_A::B_0x1)
    }
}
#[doc = "USART3 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART3RST_A {
    #[doc = "0: does not reset the USART3 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the USART3 block"]
    B_0x1 = 1,
}
impl From<USART3RST_A> for bool {
    #[inline(always)]
    fn from(variant: USART3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART3RST` reader - USART3 block reset Set and reset by software."]
pub type USART3RST_R = crate::BitReader<USART3RST_A>;
impl USART3RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART3RST_A {
        match self.bits {
            false => USART3RST_A::B_0x0,
            true => USART3RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the USART3 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART3RST_A::B_0x0
    }
    #[doc = "resets the USART3 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART3RST_A::B_0x1
    }
}
#[doc = "Field `USART3RST` writer - USART3 block reset Set and reset by software."]
pub type USART3RST_W<'a, REG> = crate::BitWriter<'a, REG, USART3RST_A>;
impl<'a, REG> USART3RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the USART3 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART3RST_A::B_0x0)
    }
    #[doc = "resets the USART3 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART3RST_A::B_0x1)
    }
}
#[doc = "I2C1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1RST_A {
    #[doc = "0: does not reset the I2C1 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the I2C1 block"]
    B_0x1 = 1,
}
impl From<I2C1RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1RST` reader - I2C1 block reset Set and reset by software."]
pub type I2C1RST_R = crate::BitReader<I2C1RST_A>;
impl I2C1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1RST_A {
        match self.bits {
            false => I2C1RST_A::B_0x0,
            true => I2C1RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the I2C1 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2C1RST_A::B_0x0
    }
    #[doc = "resets the I2C1 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2C1RST_A::B_0x1
    }
}
#[doc = "Field `I2C1RST` writer - I2C1 block reset Set and reset by software."]
pub type I2C1RST_W<'a, REG> = crate::BitWriter<'a, REG, I2C1RST_A>;
impl<'a, REG> I2C1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the I2C1 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1RST_A::B_0x0)
    }
    #[doc = "resets the I2C1 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1RST_A::B_0x1)
    }
}
#[doc = "I2C2 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2RST_A {
    #[doc = "0: does not reset the I2C2 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the I2C2 block"]
    B_0x1 = 1,
}
impl From<I2C2RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2RST` reader - I2C2 block reset Set and reset by software."]
pub type I2C2RST_R = crate::BitReader<I2C2RST_A>;
impl I2C2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C2RST_A {
        match self.bits {
            false => I2C2RST_A::B_0x0,
            true => I2C2RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the I2C2 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2C2RST_A::B_0x0
    }
    #[doc = "resets the I2C2 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2C2RST_A::B_0x1
    }
}
#[doc = "Field `I2C2RST` writer - I2C2 block reset Set and reset by software."]
pub type I2C2RST_W<'a, REG> = crate::BitWriter<'a, REG, I2C2RST_A>;
impl<'a, REG> I2C2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the I2C2 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2RST_A::B_0x0)
    }
    #[doc = "resets the I2C2 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2RST_A::B_0x1)
    }
}
#[doc = "I3C1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C1RST_A {
    #[doc = "0: does not reset the I3C1 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the I3C1 block"]
    B_0x1 = 1,
}
impl From<I3C1RST_A> for bool {
    #[inline(always)]
    fn from(variant: I3C1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C1RST` reader - I3C1 block reset Set and reset by software."]
pub type I3C1RST_R = crate::BitReader<I3C1RST_A>;
impl I3C1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3C1RST_A {
        match self.bits {
            false => I3C1RST_A::B_0x0,
            true => I3C1RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the I3C1 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I3C1RST_A::B_0x0
    }
    #[doc = "resets the I3C1 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I3C1RST_A::B_0x1
    }
}
#[doc = "Field `I3C1RST` writer - I3C1 block reset Set and reset by software."]
pub type I3C1RST_W<'a, REG> = crate::BitWriter<'a, REG, I3C1RST_A>;
impl<'a, REG> I3C1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the I3C1 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1RST_A::B_0x0)
    }
    #[doc = "resets the I3C1 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1RST_A::B_0x1)
    }
}
#[doc = "CRS block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSRST_A {
    #[doc = "0: does not reset the CRS block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the CRS block"]
    B_0x1 = 1,
}
impl From<CRSRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRSRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSRST` reader - CRS block reset Set and reset by software."]
pub type CRSRST_R = crate::BitReader<CRSRST_A>;
impl CRSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRSRST_A {
        match self.bits {
            false => CRSRST_A::B_0x0,
            true => CRSRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the CRS block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRSRST_A::B_0x0
    }
    #[doc = "resets the CRS block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRSRST_A::B_0x1
    }
}
#[doc = "Field `CRSRST` writer - CRS block reset Set and reset by software."]
pub type CRSRST_W<'a, REG> = crate::BitWriter<'a, REG, CRSRST_A>;
impl<'a, REG> CRSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the CRS block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRSRST_A::B_0x0)
    }
    #[doc = "resets the CRS block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRSRST_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn TIM2RST(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn TIM3RST(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 block reset Set and reset by software."]
    #[inline(always)]
    pub fn TIM6RST(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 block reset Set and reset by software."]
    #[inline(always)]
    pub fn TIM7RST(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 13 - OPAMP block reset Set and reset by software."]
    #[inline(always)]
    pub fn OPAMPRST(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn SPI2RST(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn SPI3RST(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - COMP block reset Set and reset by software."]
    #[inline(always)]
    pub fn COMPRST(&self) -> COMPRST_R {
        COMPRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn USART2RST(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn USART3RST(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn I2C1RST(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn I2C2RST(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn I3C1RST(&self) -> I3C1RST_R {
        I3C1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS block reset Set and reset by software."]
    #[inline(always)]
    pub fn CRSRST(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn TIM2RST(&mut self) -> TIM2RST_W<'_, APB1LRSTR_SPEC> {
        TIM2RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn TIM3RST(&mut self) -> TIM3RST_W<'_, APB1LRSTR_SPEC> {
        TIM3RST_W::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 block reset Set and reset by software."]
    #[inline(always)]
    pub fn TIM6RST(&mut self) -> TIM6RST_W<'_, APB1LRSTR_SPEC> {
        TIM6RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 block reset Set and reset by software."]
    #[inline(always)]
    pub fn TIM7RST(&mut self) -> TIM7RST_W<'_, APB1LRSTR_SPEC> {
        TIM7RST_W::new(self, 5)
    }
    #[doc = "Bit 13 - OPAMP block reset Set and reset by software."]
    #[inline(always)]
    pub fn OPAMPRST(&mut self) -> OPAMPRST_W<'_, APB1LRSTR_SPEC> {
        OPAMPRST_W::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn SPI2RST(&mut self) -> SPI2RST_W<'_, APB1LRSTR_SPEC> {
        SPI2RST_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn SPI3RST(&mut self) -> SPI3RST_W<'_, APB1LRSTR_SPEC> {
        SPI3RST_W::new(self, 15)
    }
    #[doc = "Bit 16 - COMP block reset Set and reset by software."]
    #[inline(always)]
    pub fn COMPRST(&mut self) -> COMPRST_W<'_, APB1LRSTR_SPEC> {
        COMPRST_W::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn USART2RST(&mut self) -> USART2RST_W<'_, APB1LRSTR_SPEC> {
        USART2RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn USART3RST(&mut self) -> USART3RST_W<'_, APB1LRSTR_SPEC> {
        USART3RST_W::new(self, 18)
    }
    #[doc = "Bit 21 - I2C1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn I2C1RST(&mut self) -> I2C1RST_W<'_, APB1LRSTR_SPEC> {
        I2C1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn I2C2RST(&mut self) -> I2C2RST_W<'_, APB1LRSTR_SPEC> {
        I2C2RST_W::new(self, 22)
    }
    #[doc = "Bit 23 - I3C1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn I3C1RST(&mut self) -> I3C1RST_W<'_, APB1LRSTR_SPEC> {
        I3C1RST_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRS block reset Set and reset by software."]
    #[inline(always)]
    pub fn CRSRST(&mut self) -> CRSRST_W<'_, APB1LRSTR_SPEC> {
        CRSRST_W::new(self, 24)
    }
}
#[doc = "RCC APB1 peripheral low reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LRSTR_SPEC;
impl crate::RegisterSpec for APB1LRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lrstr::R`](R) reader structure"]
impl crate::Readable for APB1LRSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1lrstr::W`](W) writer structure"]
impl crate::Writable for APB1LRSTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB1LRSTR to value 0"]
impl crate::Resettable for APB1LRSTR_SPEC {}

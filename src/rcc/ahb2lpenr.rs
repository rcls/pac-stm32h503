#[doc = "Register `AHB2LPENR` reader"]
pub type R = crate::R<AHB2LPENR_SPEC>;
#[doc = "Register `AHB2LPENR` writer"]
pub type W = crate::W<AHB2LPENR_SPEC>;
#[doc = "GPIOA clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOALPEN_A {
    #[doc = "0: GPIOA peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: GPIOA peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<GPIOALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOALPEN` reader - GPIOA clock enable during sleep mode Set and reset by software."]
pub type GPIOALPEN_R = crate::BitReader<GPIOALPEN_A>;
impl GPIOALPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOALPEN_A {
        match self.bits {
            false => GPIOALPEN_A::B_0x0,
            true => GPIOALPEN_A::B_0x1,
        }
    }
    #[doc = "GPIOA peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOALPEN_A::B_0x0
    }
    #[doc = "GPIOA peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOALPEN_A::B_0x1
    }
}
#[doc = "Field `GPIOALPEN` writer - GPIOA clock enable during sleep mode Set and reset by software."]
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOALPEN_A>;
impl<'a, REG> GPIOALPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIOA peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN_A::B_0x0)
    }
    #[doc = "GPIOA peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN_A::B_0x1)
    }
}
#[doc = "GPIOB clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOBLPEN_A {
    #[doc = "0: GPIOB peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: GPIOB peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<GPIOBLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOBLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOBLPEN` reader - GPIOB clock enable during sleep mode Set and reset by software."]
pub type GPIOBLPEN_R = crate::BitReader<GPIOBLPEN_A>;
impl GPIOBLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOBLPEN_A {
        match self.bits {
            false => GPIOBLPEN_A::B_0x0,
            true => GPIOBLPEN_A::B_0x1,
        }
    }
    #[doc = "GPIOB peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOBLPEN_A::B_0x0
    }
    #[doc = "GPIOB peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOBLPEN_A::B_0x1
    }
}
#[doc = "Field `GPIOBLPEN` writer - GPIOB clock enable during sleep mode Set and reset by software."]
pub type GPIOBLPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOBLPEN_A>;
impl<'a, REG> GPIOBLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIOB peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBLPEN_A::B_0x0)
    }
    #[doc = "GPIOB peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBLPEN_A::B_0x1)
    }
}
#[doc = "GPIOC clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOCLPEN_A {
    #[doc = "0: GPIOC peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: GPIOC peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<GPIOCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOCLPEN` reader - GPIOC clock enable during sleep mode Set and reset by software."]
pub type GPIOCLPEN_R = crate::BitReader<GPIOCLPEN_A>;
impl GPIOCLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOCLPEN_A {
        match self.bits {
            false => GPIOCLPEN_A::B_0x0,
            true => GPIOCLPEN_A::B_0x1,
        }
    }
    #[doc = "GPIOC peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOCLPEN_A::B_0x0
    }
    #[doc = "GPIOC peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOCLPEN_A::B_0x1
    }
}
#[doc = "Field `GPIOCLPEN` writer - GPIOC clock enable during sleep mode Set and reset by software."]
pub type GPIOCLPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOCLPEN_A>;
impl<'a, REG> GPIOCLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIOC peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCLPEN_A::B_0x0)
    }
    #[doc = "GPIOC peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCLPEN_A::B_0x1)
    }
}
#[doc = "GPIOD clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIODLPEN_A {
    #[doc = "0: GPIOD peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: GPIOD peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<GPIODLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIODLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIODLPEN` reader - GPIOD clock enable during sleep mode Set and reset by software."]
pub type GPIODLPEN_R = crate::BitReader<GPIODLPEN_A>;
impl GPIODLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIODLPEN_A {
        match self.bits {
            false => GPIODLPEN_A::B_0x0,
            true => GPIODLPEN_A::B_0x1,
        }
    }
    #[doc = "GPIOD peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIODLPEN_A::B_0x0
    }
    #[doc = "GPIOD peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIODLPEN_A::B_0x1
    }
}
#[doc = "Field `GPIODLPEN` writer - GPIOD clock enable during sleep mode Set and reset by software."]
pub type GPIODLPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIODLPEN_A>;
impl<'a, REG> GPIODLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIOD peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODLPEN_A::B_0x0)
    }
    #[doc = "GPIOD peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODLPEN_A::B_0x1)
    }
}
#[doc = "GPIOH clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOHLPEN_A {
    #[doc = "0: GPIOH peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: GPIOH peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<GPIOHLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOHLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOHLPEN` reader - GPIOH clock enable during sleep mode Set and reset by software."]
pub type GPIOHLPEN_R = crate::BitReader<GPIOHLPEN_A>;
impl GPIOHLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOHLPEN_A {
        match self.bits {
            false => GPIOHLPEN_A::B_0x0,
            true => GPIOHLPEN_A::B_0x1,
        }
    }
    #[doc = "GPIOH peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOHLPEN_A::B_0x0
    }
    #[doc = "GPIOH peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOHLPEN_A::B_0x1
    }
}
#[doc = "Field `GPIOHLPEN` writer - GPIOH clock enable during sleep mode Set and reset by software."]
pub type GPIOHLPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOHLPEN_A>;
impl<'a, REG> GPIOHLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIOH peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOHLPEN_A::B_0x0)
    }
    #[doc = "GPIOH peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOHLPEN_A::B_0x1)
    }
}
#[doc = "ADC1 peripherals clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1LPEN_A {
    #[doc = "0: ADC1 peripherals clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: ADC1 peripherals clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<ADC1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC1LPEN` reader - ADC1 peripherals clock enable during sleep mode Set and reset by software."]
pub type ADC1LPEN_R = crate::BitReader<ADC1LPEN_A>;
impl ADC1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC1LPEN_A {
        match self.bits {
            false => ADC1LPEN_A::B_0x0,
            true => ADC1LPEN_A::B_0x1,
        }
    }
    #[doc = "ADC1 peripherals clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADC1LPEN_A::B_0x0
    }
    #[doc = "ADC1 peripherals clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADC1LPEN_A::B_0x1
    }
}
#[doc = "Field `ADC1LPEN` writer - ADC1 peripherals clock enable during sleep mode Set and reset by software."]
pub type ADC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, ADC1LPEN_A>;
impl<'a, REG> ADC1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC1 peripherals clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1LPEN_A::B_0x0)
    }
    #[doc = "ADC1 peripherals clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1LPEN_A::B_0x1)
    }
}
#[doc = "DAC clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC12LPEN_A {
    #[doc = "0: DAC peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: DAC peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<DAC12LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC12LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC12LPEN` reader - DAC clock enable during sleep mode Set and reset by software."]
pub type DAC12LPEN_R = crate::BitReader<DAC12LPEN_A>;
impl DAC12LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC12LPEN_A {
        match self.bits {
            false => DAC12LPEN_A::B_0x0,
            true => DAC12LPEN_A::B_0x1,
        }
    }
    #[doc = "DAC peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DAC12LPEN_A::B_0x0
    }
    #[doc = "DAC peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DAC12LPEN_A::B_0x1
    }
}
#[doc = "Field `DAC12LPEN` writer - DAC clock enable during sleep mode Set and reset by software."]
pub type DAC12LPEN_W<'a, REG> = crate::BitWriter<'a, REG, DAC12LPEN_A>;
impl<'a, REG> DAC12LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DAC12LPEN_A::B_0x0)
    }
    #[doc = "DAC peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DAC12LPEN_A::B_0x1)
    }
}
#[doc = "HASH clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHLPEN_A {
    #[doc = "0: HASH peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: HASH peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<HASHLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: HASHLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHLPEN` reader - HASH clock enable during sleep mode Set and reset by software."]
pub type HASHLPEN_R = crate::BitReader<HASHLPEN_A>;
impl HASHLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HASHLPEN_A {
        match self.bits {
            false => HASHLPEN_A::B_0x0,
            true => HASHLPEN_A::B_0x1,
        }
    }
    #[doc = "HASH peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HASHLPEN_A::B_0x0
    }
    #[doc = "HASH peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HASHLPEN_A::B_0x1
    }
}
#[doc = "Field `HASHLPEN` writer - HASH clock enable during sleep mode Set and reset by software."]
pub type HASHLPEN_W<'a, REG> = crate::BitWriter<'a, REG, HASHLPEN_A>;
impl<'a, REG> HASHLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HASH peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HASHLPEN_A::B_0x0)
    }
    #[doc = "HASH peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HASHLPEN_A::B_0x1)
    }
}
#[doc = "RNG clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGLPEN_A {
    #[doc = "0: RNG peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: RNG peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<RNGLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGLPEN` reader - RNG clock enable during sleep mode Set and reset by software."]
pub type RNGLPEN_R = crate::BitReader<RNGLPEN_A>;
impl RNGLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGLPEN_A {
        match self.bits {
            false => RNGLPEN_A::B_0x0,
            true => RNGLPEN_A::B_0x1,
        }
    }
    #[doc = "RNG peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RNGLPEN_A::B_0x0
    }
    #[doc = "RNG peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RNGLPEN_A::B_0x1
    }
}
#[doc = "Field `RNGLPEN` writer - RNG clock enable during sleep mode Set and reset by software."]
pub type RNGLPEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGLPEN_A>;
impl<'a, REG> RNGLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RNG peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RNGLPEN_A::B_0x0)
    }
    #[doc = "RNG peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RNGLPEN_A::B_0x1)
    }
}
#[doc = "SRAM2 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2LPEN_A {
    #[doc = "0: SRAM2 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: SRAM2 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<SRAM2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2LPEN` reader - SRAM2 clock enable during sleep mode Set and reset by software."]
pub type SRAM2LPEN_R = crate::BitReader<SRAM2LPEN_A>;
impl SRAM2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2LPEN_A {
        match self.bits {
            false => SRAM2LPEN_A::B_0x0,
            true => SRAM2LPEN_A::B_0x1,
        }
    }
    #[doc = "SRAM2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM2LPEN_A::B_0x0
    }
    #[doc = "SRAM2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM2LPEN_A::B_0x1
    }
}
#[doc = "Field `SRAM2LPEN` writer - SRAM2 clock enable during sleep mode Set and reset by software."]
pub type SRAM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2LPEN_A>;
impl<'a, REG> SRAM2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2LPEN_A::B_0x0)
    }
    #[doc = "SRAM2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2LPEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - GPIOA clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPIOALPEN(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPIOBLPEN(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPIOCLPEN(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPIODLPEN(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPIOHLPEN(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 peripherals clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ADC1LPEN(&self) -> ADC1LPEN_R {
        ADC1LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn DAC12LPEN(&self) -> DAC12LPEN_R {
        DAC12LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn HASHLPEN(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn RNGLPEN(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn SRAM2LPEN(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPIOALPEN(&mut self) -> GPIOALPEN_W<'_, AHB2LPENR_SPEC> {
        GPIOALPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPIOBLPEN(&mut self) -> GPIOBLPEN_W<'_, AHB2LPENR_SPEC> {
        GPIOBLPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPIOCLPEN(&mut self) -> GPIOCLPEN_W<'_, AHB2LPENR_SPEC> {
        GPIOCLPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOD clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPIODLPEN(&mut self) -> GPIODLPEN_W<'_, AHB2LPENR_SPEC> {
        GPIODLPEN_W::new(self, 3)
    }
    #[doc = "Bit 7 - GPIOH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn GPIOHLPEN(&mut self) -> GPIOHLPEN_W<'_, AHB2LPENR_SPEC> {
        GPIOHLPEN_W::new(self, 7)
    }
    #[doc = "Bit 10 - ADC1 peripherals clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ADC1LPEN(&mut self) -> ADC1LPEN_W<'_, AHB2LPENR_SPEC> {
        ADC1LPEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - DAC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn DAC12LPEN(&mut self) -> DAC12LPEN_W<'_, AHB2LPENR_SPEC> {
        DAC12LPEN_W::new(self, 11)
    }
    #[doc = "Bit 17 - HASH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn HASHLPEN(&mut self) -> HASHLPEN_W<'_, AHB2LPENR_SPEC> {
        HASHLPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn RNGLPEN(&mut self) -> RNGLPEN_W<'_, AHB2LPENR_SPEC> {
        RNGLPEN_W::new(self, 18)
    }
    #[doc = "Bit 30 - SRAM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn SRAM2LPEN(&mut self) -> SRAM2LPEN_W<'_, AHB2LPENR_SPEC> {
        SRAM2LPEN_W::new(self, 30)
    }
}
#[doc = "RCC AHB2 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2LPENR_SPEC;
impl crate::RegisterSpec for AHB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2lpenr::R`](R) reader structure"]
impl crate::Readable for AHB2LPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure"]
impl crate::Writable for AHB2LPENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AHB2LPENR to value 0xffff_ffff"]
impl crate::Resettable for AHB2LPENR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

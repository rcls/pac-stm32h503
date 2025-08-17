#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<AHB2ENR_SPEC>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<AHB2ENR_SPEC>;
#[doc = "GPIOA clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN_A {
    #[doc = "0: GPIOA peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: GPIOA peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOAEN` reader - GPIOA clock enable Set and reset by software."]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN_A>;
impl GPIOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::B_0x0,
            true => GPIOAEN_A::B_0x1,
        }
    }
    #[doc = "GPIOA peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOAEN_A::B_0x0
    }
    #[doc = "GPIOA peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOAEN_A::B_0x1
    }
}
#[doc = "Field `GPIOAEN` writer - GPIOA clock enable Set and reset by software."]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN_A>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIOA peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN_A::B_0x0)
    }
    #[doc = "GPIOA peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN_A::B_0x1)
    }
}
#[doc = "GPIOB clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOBEN_A {
    #[doc = "0: GPIOB peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: GPIOB peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<GPIOBEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOBEN` reader - GPIOB clock enable Set and reset by software."]
pub type GPIOBEN_R = crate::BitReader<GPIOBEN_A>;
impl GPIOBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOBEN_A {
        match self.bits {
            false => GPIOBEN_A::B_0x0,
            true => GPIOBEN_A::B_0x1,
        }
    }
    #[doc = "GPIOB peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOBEN_A::B_0x0
    }
    #[doc = "GPIOB peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOBEN_A::B_0x1
    }
}
#[doc = "Field `GPIOBEN` writer - GPIOB clock enable Set and reset by software."]
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOBEN_A>;
impl<'a, REG> GPIOBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIOB peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBEN_A::B_0x0)
    }
    #[doc = "GPIOB peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBEN_A::B_0x1)
    }
}
#[doc = "GPIOC clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOCEN_A {
    #[doc = "0: GPIOC peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: GPIOC peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<GPIOCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOCEN` reader - GPIOC clock enable Set and reset by software."]
pub type GPIOCEN_R = crate::BitReader<GPIOCEN_A>;
impl GPIOCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOCEN_A {
        match self.bits {
            false => GPIOCEN_A::B_0x0,
            true => GPIOCEN_A::B_0x1,
        }
    }
    #[doc = "GPIOC peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOCEN_A::B_0x0
    }
    #[doc = "GPIOC peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOCEN_A::B_0x1
    }
}
#[doc = "Field `GPIOCEN` writer - GPIOC clock enable Set and reset by software."]
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOCEN_A>;
impl<'a, REG> GPIOCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIOC peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCEN_A::B_0x0)
    }
    #[doc = "GPIOC peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCEN_A::B_0x1)
    }
}
#[doc = "GPIOD clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIODEN_A {
    #[doc = "0: GPIOD peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: GPIOD peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<GPIODEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIODEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIODEN` reader - GPIOD clock enable Set and reset by software."]
pub type GPIODEN_R = crate::BitReader<GPIODEN_A>;
impl GPIODEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIODEN_A {
        match self.bits {
            false => GPIODEN_A::B_0x0,
            true => GPIODEN_A::B_0x1,
        }
    }
    #[doc = "GPIOD peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIODEN_A::B_0x0
    }
    #[doc = "GPIOD peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIODEN_A::B_0x1
    }
}
#[doc = "Field `GPIODEN` writer - GPIOD clock enable Set and reset by software."]
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIODEN_A>;
impl<'a, REG> GPIODEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIOD peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODEN_A::B_0x0)
    }
    #[doc = "GPIOD peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODEN_A::B_0x1)
    }
}
#[doc = "GPIOH clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOHEN_A {
    #[doc = "0: GPIOH peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: GPIOH peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<GPIOHEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOHEN` reader - GPIOH clock enable Set and reset by software."]
pub type GPIOHEN_R = crate::BitReader<GPIOHEN_A>;
impl GPIOHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOHEN_A {
        match self.bits {
            false => GPIOHEN_A::B_0x0,
            true => GPIOHEN_A::B_0x1,
        }
    }
    #[doc = "GPIOH peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOHEN_A::B_0x0
    }
    #[doc = "GPIOH peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOHEN_A::B_0x1
    }
}
#[doc = "Field `GPIOHEN` writer - GPIOH clock enable Set and reset by software."]
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOHEN_A>;
impl<'a, REG> GPIOHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIOH peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOHEN_A::B_0x0)
    }
    #[doc = "GPIOH peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOHEN_A::B_0x1)
    }
}
#[doc = "ADC1 peripherals clock enabled Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1EN_A {
    #[doc = "0: ADC1 peripherals clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: ADC1 peripherals clock enabled"]
    B_0x1 = 1,
}
impl From<ADC1EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC1EN` reader - ADC1 peripherals clock enabled Set and reset by software."]
pub type ADC1EN_R = crate::BitReader<ADC1EN_A>;
impl ADC1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC1EN_A {
        match self.bits {
            false => ADC1EN_A::B_0x0,
            true => ADC1EN_A::B_0x1,
        }
    }
    #[doc = "ADC1 peripherals clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADC1EN_A::B_0x0
    }
    #[doc = "ADC1 peripherals clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADC1EN_A::B_0x1
    }
}
#[doc = "Field `ADC1EN` writer - ADC1 peripherals clock enabled Set and reset by software."]
pub type ADC1EN_W<'a, REG> = crate::BitWriter<'a, REG, ADC1EN_A>;
impl<'a, REG> ADC1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC1 peripherals clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1EN_A::B_0x0)
    }
    #[doc = "ADC1 peripherals clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1EN_A::B_0x1)
    }
}
#[doc = "DAC clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC12EN_A {
    #[doc = "0: DAC peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: DAC peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<DAC12EN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC12EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC12EN` reader - DAC clock enable Set and reset by software."]
pub type DAC12EN_R = crate::BitReader<DAC12EN_A>;
impl DAC12EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC12EN_A {
        match self.bits {
            false => DAC12EN_A::B_0x0,
            true => DAC12EN_A::B_0x1,
        }
    }
    #[doc = "DAC peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DAC12EN_A::B_0x0
    }
    #[doc = "DAC peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DAC12EN_A::B_0x1
    }
}
#[doc = "Field `DAC12EN` writer - DAC clock enable Set and reset by software."]
pub type DAC12EN_W<'a, REG> = crate::BitWriter<'a, REG, DAC12EN_A>;
impl<'a, REG> DAC12EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DAC12EN_A::B_0x0)
    }
    #[doc = "DAC peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DAC12EN_A::B_0x1)
    }
}
#[doc = "HASH clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHEN_A {
    #[doc = "0: HASH peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HASH peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<HASHEN_A> for bool {
    #[inline(always)]
    fn from(variant: HASHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHEN` reader - HASH clock enable Set and reset by software."]
pub type HASHEN_R = crate::BitReader<HASHEN_A>;
impl HASHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HASHEN_A {
        match self.bits {
            false => HASHEN_A::B_0x0,
            true => HASHEN_A::B_0x1,
        }
    }
    #[doc = "HASH peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HASHEN_A::B_0x0
    }
    #[doc = "HASH peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HASHEN_A::B_0x1
    }
}
#[doc = "Field `HASHEN` writer - HASH clock enable Set and reset by software."]
pub type HASHEN_W<'a, REG> = crate::BitWriter<'a, REG, HASHEN_A>;
impl<'a, REG> HASHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HASH peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HASHEN_A::B_0x0)
    }
    #[doc = "HASH peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HASHEN_A::B_0x1)
    }
}
#[doc = "RNG clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN_A {
    #[doc = "0: RNG peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: RNG peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGEN` reader - RNG clock enable Set and reset by software."]
pub type RNGEN_R = crate::BitReader<RNGEN_A>;
impl RNGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::B_0x0,
            true => RNGEN_A::B_0x1,
        }
    }
    #[doc = "RNG peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RNGEN_A::B_0x0
    }
    #[doc = "RNG peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RNGEN_A::B_0x1
    }
}
#[doc = "Field `RNGEN` writer - RNG clock enable Set and reset by software."]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGEN_A>;
impl<'a, REG> RNGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RNG peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN_A::B_0x0)
    }
    #[doc = "RNG peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN_A::B_0x1)
    }
}
#[doc = "SRAM2 clock enable Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2EN_A {
    #[doc = "0: SRAM2 clock disabled"]
    B_0x0 = 0,
    #[doc = "1: SRAM2 clock enabled (default after reset)"]
    B_0x1 = 1,
}
impl From<SRAM2EN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2EN` reader - SRAM2 clock enable Set and reset by software."]
pub type SRAM2EN_R = crate::BitReader<SRAM2EN_A>;
impl SRAM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2EN_A {
        match self.bits {
            false => SRAM2EN_A::B_0x0,
            true => SRAM2EN_A::B_0x1,
        }
    }
    #[doc = "SRAM2 clock disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAM2EN_A::B_0x0
    }
    #[doc = "SRAM2 clock enabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAM2EN_A::B_0x1
    }
}
#[doc = "Field `SRAM2EN` writer - SRAM2 clock enable Set and reset by software."]
pub type SRAM2EN_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2EN_A>;
impl<'a, REG> SRAM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 clock disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2EN_A::B_0x0)
    }
    #[doc = "SRAM2 clock enabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2EN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - GPIOA clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPIOAEN(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPIOBEN(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPIOCEN(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPIODEN(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPIOHEN(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 peripherals clock enabled Set and reset by software."]
    #[inline(always)]
    pub fn ADC1EN(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn DAC12EN(&self) -> DAC12EN_R {
        DAC12EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable Set and reset by software."]
    #[inline(always)]
    pub fn HASHEN(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn RNGEN(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SRAM2EN(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPIOAEN(&mut self) -> GPIOAEN_W<'_, AHB2ENR_SPEC> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPIOBEN(&mut self) -> GPIOBEN_W<'_, AHB2ENR_SPEC> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPIOCEN(&mut self) -> GPIOCEN_W<'_, AHB2ENR_SPEC> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOD clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPIODEN(&mut self) -> GPIODEN_W<'_, AHB2ENR_SPEC> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 7 - GPIOH clock enable Set and reset by software."]
    #[inline(always)]
    pub fn GPIOHEN(&mut self) -> GPIOHEN_W<'_, AHB2ENR_SPEC> {
        GPIOHEN_W::new(self, 7)
    }
    #[doc = "Bit 10 - ADC1 peripherals clock enabled Set and reset by software."]
    #[inline(always)]
    pub fn ADC1EN(&mut self) -> ADC1EN_W<'_, AHB2ENR_SPEC> {
        ADC1EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - DAC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn DAC12EN(&mut self) -> DAC12EN_W<'_, AHB2ENR_SPEC> {
        DAC12EN_W::new(self, 11)
    }
    #[doc = "Bit 17 - HASH clock enable Set and reset by software."]
    #[inline(always)]
    pub fn HASHEN(&mut self) -> HASHEN_W<'_, AHB2ENR_SPEC> {
        HASHEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn RNGEN(&mut self) -> RNGEN_W<'_, AHB2ENR_SPEC> {
        RNGEN_W::new(self, 18)
    }
    #[doc = "Bit 30 - SRAM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SRAM2EN(&mut self) -> SRAM2EN_W<'_, AHB2ENR_SPEC> {
        SRAM2EN_W::new(self, 30)
    }
}
#[doc = "RCC AHB2 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AHB2ENR to value 0x4000_0000"]
impl crate::Resettable for AHB2ENR_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}

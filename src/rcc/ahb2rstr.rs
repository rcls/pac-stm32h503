#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<AHB2RSTR_SPEC>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<AHB2RSTR_SPEC>;
#[doc = "GPIOA block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST_A {
    #[doc = "0: does not reset the GPIOA block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the GPIOA block"]
    B_0x1 = 1,
}
impl From<GPIOARST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOARST` reader - GPIOA block reset Set and reset by software."]
pub type GPIOARST_R = crate::BitReader<GPIOARST_A>;
impl GPIOARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOARST_A {
        match self.bits {
            false => GPIOARST_A::B_0x0,
            true => GPIOARST_A::B_0x1,
        }
    }
    #[doc = "does not reset the GPIOA block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOARST_A::B_0x0
    }
    #[doc = "resets the GPIOA block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOARST_A::B_0x1
    }
}
#[doc = "Field `GPIOARST` writer - GPIOA block reset Set and reset by software."]
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOARST_A>;
impl<'a, REG> GPIOARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the GPIOA block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST_A::B_0x0)
    }
    #[doc = "resets the GPIOA block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST_A::B_0x1)
    }
}
#[doc = "GPIOB block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOBRST_A {
    #[doc = "0: does not reset the GPIOB block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the GPIOB block"]
    B_0x1 = 1,
}
impl From<GPIOBRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOBRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOBRST` reader - GPIOB block reset Set and reset by software."]
pub type GPIOBRST_R = crate::BitReader<GPIOBRST_A>;
impl GPIOBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOBRST_A {
        match self.bits {
            false => GPIOBRST_A::B_0x0,
            true => GPIOBRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the GPIOB block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOBRST_A::B_0x0
    }
    #[doc = "resets the GPIOB block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOBRST_A::B_0x1
    }
}
#[doc = "Field `GPIOBRST` writer - GPIOB block reset Set and reset by software."]
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOBRST_A>;
impl<'a, REG> GPIOBRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the GPIOB block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBRST_A::B_0x0)
    }
    #[doc = "resets the GPIOB block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBRST_A::B_0x1)
    }
}
#[doc = "GPIOC block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOCRST_A {
    #[doc = "0: does not reset the GPIOC block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the GPIOC block"]
    B_0x1 = 1,
}
impl From<GPIOCRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOCRST` reader - GPIOC block reset Set and reset by software."]
pub type GPIOCRST_R = crate::BitReader<GPIOCRST_A>;
impl GPIOCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOCRST_A {
        match self.bits {
            false => GPIOCRST_A::B_0x0,
            true => GPIOCRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the GPIOC block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOCRST_A::B_0x0
    }
    #[doc = "resets the GPIOC block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOCRST_A::B_0x1
    }
}
#[doc = "Field `GPIOCRST` writer - GPIOC block reset Set and reset by software."]
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOCRST_A>;
impl<'a, REG> GPIOCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the GPIOC block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCRST_A::B_0x0)
    }
    #[doc = "resets the GPIOC block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCRST_A::B_0x1)
    }
}
#[doc = "GPIOD block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIODRST_A {
    #[doc = "0: does not reset the GPIOD block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the GPIOD block"]
    B_0x1 = 1,
}
impl From<GPIODRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIODRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIODRST` reader - GPIOD block reset Set and reset by software."]
pub type GPIODRST_R = crate::BitReader<GPIODRST_A>;
impl GPIODRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIODRST_A {
        match self.bits {
            false => GPIODRST_A::B_0x0,
            true => GPIODRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the GPIOD block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIODRST_A::B_0x0
    }
    #[doc = "resets the GPIOD block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIODRST_A::B_0x1
    }
}
#[doc = "Field `GPIODRST` writer - GPIOD block reset Set and reset by software."]
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG, GPIODRST_A>;
impl<'a, REG> GPIODRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the GPIOD block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODRST_A::B_0x0)
    }
    #[doc = "resets the GPIOD block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODRST_A::B_0x1)
    }
}
#[doc = "GPIOH block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOHRST_A {
    #[doc = "0: does not reset the GPIOH block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the GPIOH block"]
    B_0x1 = 1,
}
impl From<GPIOHRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOHRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOHRST` reader - GPIOH block reset Set and reset by software."]
pub type GPIOHRST_R = crate::BitReader<GPIOHRST_A>;
impl GPIOHRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOHRST_A {
        match self.bits {
            false => GPIOHRST_A::B_0x0,
            true => GPIOHRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the GPIOH block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GPIOHRST_A::B_0x0
    }
    #[doc = "resets the GPIOH block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GPIOHRST_A::B_0x1
    }
}
#[doc = "Field `GPIOHRST` writer - GPIOH block reset Set and reset by software."]
pub type GPIOHRST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOHRST_A>;
impl<'a, REG> GPIOHRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the GPIOH block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOHRST_A::B_0x0)
    }
    #[doc = "resets the GPIOH block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOHRST_A::B_0x1)
    }
}
#[doc = "ADC1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1RST_A {
    #[doc = "0: does not reset ADC1 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets ADC1 block"]
    B_0x1 = 1,
}
impl From<ADC1RST_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC1RST` reader - ADC1 block reset Set and reset by software."]
pub type ADC1RST_R = crate::BitReader<ADC1RST_A>;
impl ADC1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC1RST_A {
        match self.bits {
            false => ADC1RST_A::B_0x0,
            true => ADC1RST_A::B_0x1,
        }
    }
    #[doc = "does not reset ADC1 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADC1RST_A::B_0x0
    }
    #[doc = "resets ADC1 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADC1RST_A::B_0x1
    }
}
#[doc = "Field `ADC1RST` writer - ADC1 block reset Set and reset by software."]
pub type ADC1RST_W<'a, REG> = crate::BitWriter<'a, REG, ADC1RST_A>;
impl<'a, REG> ADC1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset ADC1 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1RST_A::B_0x0)
    }
    #[doc = "resets ADC1 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1RST_A::B_0x1)
    }
}
#[doc = "DAC block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC12RST_A {
    #[doc = "0: does not reset DAC block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets DAC block"]
    B_0x1 = 1,
}
impl From<DAC12RST_A> for bool {
    #[inline(always)]
    fn from(variant: DAC12RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC12RST` reader - DAC block reset Set and reset by software."]
pub type DAC12RST_R = crate::BitReader<DAC12RST_A>;
impl DAC12RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC12RST_A {
        match self.bits {
            false => DAC12RST_A::B_0x0,
            true => DAC12RST_A::B_0x1,
        }
    }
    #[doc = "does not reset DAC block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DAC12RST_A::B_0x0
    }
    #[doc = "resets DAC block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DAC12RST_A::B_0x1
    }
}
#[doc = "Field `DAC12RST` writer - DAC block reset Set and reset by software."]
pub type DAC12RST_W<'a, REG> = crate::BitWriter<'a, REG, DAC12RST_A>;
impl<'a, REG> DAC12RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset DAC block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DAC12RST_A::B_0x0)
    }
    #[doc = "resets DAC block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DAC12RST_A::B_0x1)
    }
}
#[doc = "HASH block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHRST_A {
    #[doc = "0: does not reset HASH block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets HASH block"]
    B_0x1 = 1,
}
impl From<HASHRST_A> for bool {
    #[inline(always)]
    fn from(variant: HASHRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHRST` reader - HASH block reset Set and reset by software."]
pub type HASHRST_R = crate::BitReader<HASHRST_A>;
impl HASHRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HASHRST_A {
        match self.bits {
            false => HASHRST_A::B_0x0,
            true => HASHRST_A::B_0x1,
        }
    }
    #[doc = "does not reset HASH block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HASHRST_A::B_0x0
    }
    #[doc = "resets HASH block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HASHRST_A::B_0x1
    }
}
#[doc = "Field `HASHRST` writer - HASH block reset Set and reset by software."]
pub type HASHRST_W<'a, REG> = crate::BitWriter<'a, REG, HASHRST_A>;
impl<'a, REG> HASHRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset HASH block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HASHRST_A::B_0x0)
    }
    #[doc = "resets HASH block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HASHRST_A::B_0x1)
    }
}
#[doc = "RNG block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGRST_A {
    #[doc = "0: does not reset RNG block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets RNG block"]
    B_0x1 = 1,
}
impl From<RNGRST_A> for bool {
    #[inline(always)]
    fn from(variant: RNGRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGRST` reader - RNG block reset Set and reset by software."]
pub type RNGRST_R = crate::BitReader<RNGRST_A>;
impl RNGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGRST_A {
        match self.bits {
            false => RNGRST_A::B_0x0,
            true => RNGRST_A::B_0x1,
        }
    }
    #[doc = "does not reset RNG block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RNGRST_A::B_0x0
    }
    #[doc = "resets RNG block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RNGRST_A::B_0x1
    }
}
#[doc = "Field `RNGRST` writer - RNG block reset Set and reset by software."]
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG, RNGRST_A>;
impl<'a, REG> RNGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset RNG block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RNGRST_A::B_0x0)
    }
    #[doc = "resets RNG block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RNGRST_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - GPIOA block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPIOARST(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPIOBRST(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPIOCRST(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPIODRST(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPIOHRST(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn ADC1RST(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC block reset Set and reset by software."]
    #[inline(always)]
    pub fn DAC12RST(&self) -> DAC12RST_R {
        DAC12RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH block reset Set and reset by software."]
    #[inline(always)]
    pub fn HASHRST(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG block reset Set and reset by software."]
    #[inline(always)]
    pub fn RNGRST(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPIOARST(&mut self) -> GPIOARST_W<'_, AHB2RSTR_SPEC> {
        GPIOARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPIOBRST(&mut self) -> GPIOBRST_W<'_, AHB2RSTR_SPEC> {
        GPIOBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOC block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPIOCRST(&mut self) -> GPIOCRST_W<'_, AHB2RSTR_SPEC> {
        GPIOCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOD block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPIODRST(&mut self) -> GPIODRST_W<'_, AHB2RSTR_SPEC> {
        GPIODRST_W::new(self, 3)
    }
    #[doc = "Bit 7 - GPIOH block reset Set and reset by software."]
    #[inline(always)]
    pub fn GPIOHRST(&mut self) -> GPIOHRST_W<'_, AHB2RSTR_SPEC> {
        GPIOHRST_W::new(self, 7)
    }
    #[doc = "Bit 10 - ADC1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn ADC1RST(&mut self) -> ADC1RST_W<'_, AHB2RSTR_SPEC> {
        ADC1RST_W::new(self, 10)
    }
    #[doc = "Bit 11 - DAC block reset Set and reset by software."]
    #[inline(always)]
    pub fn DAC12RST(&mut self) -> DAC12RST_W<'_, AHB2RSTR_SPEC> {
        DAC12RST_W::new(self, 11)
    }
    #[doc = "Bit 17 - HASH block reset Set and reset by software."]
    #[inline(always)]
    pub fn HASHRST(&mut self) -> HASHRST_W<'_, AHB2RSTR_SPEC> {
        HASHRST_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNG block reset Set and reset by software."]
    #[inline(always)]
    pub fn RNGRST(&mut self) -> RNGRST_W<'_, AHB2RSTR_SPEC> {
        RNGRST_W::new(self, 18)
    }
}
#[doc = "RCC AHB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for AHB2RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for AHB2RSTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTR_SPEC {}

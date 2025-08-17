#[doc = "Register `ITENR` reader"]
pub type R = crate::R<ITENR_SPEC>;
#[doc = "Register `ITENR` writer"]
pub type W = crate::W<ITENR_SPEC>;
#[doc = "Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_ITEEN_A {
    #[doc = "0: Synchronous interrupt for end of measurement disabled on temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: Synchronous interrupt for end of measurement enabled on temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_ITEEN_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_ITEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_ITEEN` reader - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement."]
pub type TS1_ITEEN_R = crate::BitReader<TS1_ITEEN_A>;
impl TS1_ITEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_ITEEN_A {
        match self.bits {
            false => TS1_ITEEN_A::B_0x0,
            true => TS1_ITEEN_A::B_0x1,
        }
    }
    #[doc = "Synchronous interrupt for end of measurement disabled on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_ITEEN_A::B_0x0
    }
    #[doc = "Synchronous interrupt for end of measurement enabled on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_ITEEN_A::B_0x1
    }
}
#[doc = "Field `TS1_ITEEN` writer - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement."]
pub type TS1_ITEEN_W<'a, REG> = crate::BitWriter<'a, REG, TS1_ITEEN_A>;
impl<'a, REG> TS1_ITEEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronous interrupt for end of measurement disabled on temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_ITEEN_A::B_0x0)
    }
    #[doc = "Synchronous interrupt for end of measurement enabled on temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_ITEEN_A::B_0x1)
    }
}
#[doc = "Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_ITLEN_A {
    #[doc = "0: Synchronous interrupt for low threshold disabled on temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: Synchronous interrupt for low threshold enabled on temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_ITLEN_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_ITLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_ITLEN` reader - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold."]
pub type TS1_ITLEN_R = crate::BitReader<TS1_ITLEN_A>;
impl TS1_ITLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_ITLEN_A {
        match self.bits {
            false => TS1_ITLEN_A::B_0x0,
            true => TS1_ITLEN_A::B_0x1,
        }
    }
    #[doc = "Synchronous interrupt for low threshold disabled on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_ITLEN_A::B_0x0
    }
    #[doc = "Synchronous interrupt for low threshold enabled on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_ITLEN_A::B_0x1
    }
}
#[doc = "Field `TS1_ITLEN` writer - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold."]
pub type TS1_ITLEN_W<'a, REG> = crate::BitWriter<'a, REG, TS1_ITLEN_A>;
impl<'a, REG> TS1_ITLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronous interrupt for low threshold disabled on temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_ITLEN_A::B_0x0)
    }
    #[doc = "Synchronous interrupt for low threshold enabled on temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_ITLEN_A::B_0x1)
    }
}
#[doc = "Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_ITHEN_A {
    #[doc = "0: Synchronous interrupt for high threshold disabled on temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: Synchronous interrupt for high threshold enabled on temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_ITHEN_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_ITHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_ITHEN` reader - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold."]
pub type TS1_ITHEN_R = crate::BitReader<TS1_ITHEN_A>;
impl TS1_ITHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_ITHEN_A {
        match self.bits {
            false => TS1_ITHEN_A::B_0x0,
            true => TS1_ITHEN_A::B_0x1,
        }
    }
    #[doc = "Synchronous interrupt for high threshold disabled on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_ITHEN_A::B_0x0
    }
    #[doc = "Synchronous interrupt for high threshold enabled on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_ITHEN_A::B_0x1
    }
}
#[doc = "Field `TS1_ITHEN` writer - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold."]
pub type TS1_ITHEN_W<'a, REG> = crate::BitWriter<'a, REG, TS1_ITHEN_A>;
impl<'a, REG> TS1_ITHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronous interrupt for high threshold disabled on temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_ITHEN_A::B_0x0)
    }
    #[doc = "Synchronous interrupt for high threshold enabled on temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_ITHEN_A::B_0x1)
    }
}
#[doc = "Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_AITEEN_A {
    #[doc = "0: Asynchronous interrupt for end of measurement disabled on temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: Asynchronous interrupt for end of measurement enabled on temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_AITEEN_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_AITEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_AITEEN` reader - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1)."]
pub type TS1_AITEEN_R = crate::BitReader<TS1_AITEEN_A>;
impl TS1_AITEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_AITEEN_A {
        match self.bits {
            false => TS1_AITEEN_A::B_0x0,
            true => TS1_AITEEN_A::B_0x1,
        }
    }
    #[doc = "Asynchronous interrupt for end of measurement disabled on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_AITEEN_A::B_0x0
    }
    #[doc = "Asynchronous interrupt for end of measurement enabled on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_AITEEN_A::B_0x1
    }
}
#[doc = "Field `TS1_AITEEN` writer - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1)."]
pub type TS1_AITEEN_W<'a, REG> = crate::BitWriter<'a, REG, TS1_AITEEN_A>;
impl<'a, REG> TS1_AITEEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous interrupt for end of measurement disabled on temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_AITEEN_A::B_0x0)
    }
    #[doc = "Asynchronous interrupt for end of measurement enabled on temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_AITEEN_A::B_0x1)
    }
}
#[doc = "Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_AITLEN_A {
    #[doc = "0: Asynchronous interrupt on low threshold disabled for temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: Asynchronous interrupt on low threshold enabled for temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_AITLEN_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_AITLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_AITLEN` reader - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)"]
pub type TS1_AITLEN_R = crate::BitReader<TS1_AITLEN_A>;
impl TS1_AITLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_AITLEN_A {
        match self.bits {
            false => TS1_AITLEN_A::B_0x0,
            true => TS1_AITLEN_A::B_0x1,
        }
    }
    #[doc = "Asynchronous interrupt on low threshold disabled for temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_AITLEN_A::B_0x0
    }
    #[doc = "Asynchronous interrupt on low threshold enabled for temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_AITLEN_A::B_0x1
    }
}
#[doc = "Field `TS1_AITLEN` writer - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)"]
pub type TS1_AITLEN_W<'a, REG> = crate::BitWriter<'a, REG, TS1_AITLEN_A>;
impl<'a, REG> TS1_AITLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous interrupt on low threshold disabled for temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_AITLEN_A::B_0x0)
    }
    #[doc = "Asynchronous interrupt on low threshold enabled for temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_AITLEN_A::B_0x1)
    }
}
#[doc = "Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1'')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_AITHEN_A {
    #[doc = "0: Asynchronous interrupt on high threshold disabled for temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: Asynchronous interrupt on high threshold enabled for temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_AITHEN_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_AITHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_AITHEN` reader - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1'')"]
pub type TS1_AITHEN_R = crate::BitReader<TS1_AITHEN_A>;
impl TS1_AITHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_AITHEN_A {
        match self.bits {
            false => TS1_AITHEN_A::B_0x0,
            true => TS1_AITHEN_A::B_0x1,
        }
    }
    #[doc = "Asynchronous interrupt on high threshold disabled for temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_AITHEN_A::B_0x0
    }
    #[doc = "Asynchronous interrupt on high threshold enabled for temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_AITHEN_A::B_0x1
    }
}
#[doc = "Field `TS1_AITHEN` writer - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1'')"]
pub type TS1_AITHEN_W<'a, REG> = crate::BitWriter<'a, REG, TS1_AITHEN_A>;
impl<'a, REG> TS1_AITHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous interrupt on high threshold disabled for temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_AITHEN_A::B_0x0)
    }
    #[doc = "Asynchronous interrupt on high threshold enabled for temperature sensor 1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_AITHEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement."]
    #[inline(always)]
    pub fn TS1_ITEEN(&self) -> TS1_ITEEN_R {
        TS1_ITEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold."]
    #[inline(always)]
    pub fn TS1_ITLEN(&self) -> TS1_ITLEN_R {
        TS1_ITLEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold."]
    #[inline(always)]
    pub fn TS1_ITHEN(&self) -> TS1_ITHEN_R {
        TS1_ITHEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1)."]
    #[inline(always)]
    pub fn TS1_AITEEN(&self) -> TS1_AITEEN_R {
        TS1_AITEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)"]
    #[inline(always)]
    pub fn TS1_AITLEN(&self) -> TS1_AITLEN_R {
        TS1_AITLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1'')"]
    #[inline(always)]
    pub fn TS1_AITHEN(&self) -> TS1_AITHEN_R {
        TS1_AITHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement."]
    #[inline(always)]
    pub fn TS1_ITEEN(&mut self) -> TS1_ITEEN_W<'_, ITENR_SPEC> {
        TS1_ITEEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold."]
    #[inline(always)]
    pub fn TS1_ITLEN(&mut self) -> TS1_ITLEN_W<'_, ITENR_SPEC> {
        TS1_ITLEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold."]
    #[inline(always)]
    pub fn TS1_ITHEN(&mut self) -> TS1_ITHEN_W<'_, ITENR_SPEC> {
        TS1_ITHEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1)."]
    #[inline(always)]
    pub fn TS1_AITEEN(&mut self) -> TS1_AITEEN_W<'_, ITENR_SPEC> {
        TS1_AITEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)"]
    #[inline(always)]
    pub fn TS1_AITLEN(&mut self) -> TS1_AITLEN_W<'_, ITENR_SPEC> {
        TS1_AITLEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1'')"]
    #[inline(always)]
    pub fn TS1_AITHEN(&mut self) -> TS1_AITHEN_W<'_, ITENR_SPEC> {
        TS1_AITHEN_W::new(self, 6)
    }
}
#[doc = "Temperature sensor interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`itenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITENR_SPEC;
impl crate::RegisterSpec for ITENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itenr::R`](R) reader structure"]
impl crate::Readable for ITENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itenr::W`](W) writer structure"]
impl crate::Writable for ITENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ITENR to value 0"]
impl crate::Resettable for ITENR_SPEC {}

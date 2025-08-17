#[doc = "Register `APB3LPENR` reader"]
pub type R = crate::R<APB3LPENR_SPEC>;
#[doc = "Register `APB3LPENR` writer"]
pub type W = crate::W<APB3LPENR_SPEC>;
#[doc = "SBS clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSLPEN_A {
    #[doc = "0: SBS peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: SBS peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<SBSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SBSLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBSLPEN` reader - SBS clock enable during sleep mode Set and reset by software."]
pub type SBSLPEN_R = crate::BitReader<SBSLPEN_A>;
impl SBSLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBSLPEN_A {
        match self.bits {
            false => SBSLPEN_A::B_0x0,
            true => SBSLPEN_A::B_0x1,
        }
    }
    #[doc = "SBS peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SBSLPEN_A::B_0x0
    }
    #[doc = "SBS peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SBSLPEN_A::B_0x1
    }
}
#[doc = "Field `SBSLPEN` writer - SBS clock enable during sleep mode Set and reset by software."]
pub type SBSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, SBSLPEN_A>;
impl<'a, REG> SBSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SBS peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SBSLPEN_A::B_0x0)
    }
    #[doc = "SBS peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SBSLPEN_A::B_0x1)
    }
}
#[doc = "LPUART1 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1LPEN_A {
    #[doc = "0: LPUART1 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: LPUART1 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<LPUART1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1LPEN` reader - LPUART1 clock enable during sleep mode Set and reset by software."]
pub type LPUART1LPEN_R = crate::BitReader<LPUART1LPEN_A>;
impl LPUART1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1LPEN_A {
        match self.bits {
            false => LPUART1LPEN_A::B_0x0,
            true => LPUART1LPEN_A::B_0x1,
        }
    }
    #[doc = "LPUART1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPUART1LPEN_A::B_0x0
    }
    #[doc = "LPUART1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPUART1LPEN_A::B_0x1
    }
}
#[doc = "Field `LPUART1LPEN` writer - LPUART1 clock enable during sleep mode Set and reset by software."]
pub type LPUART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1LPEN_A>;
impl<'a, REG> LPUART1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1LPEN_A::B_0x0)
    }
    #[doc = "LPUART1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1LPEN_A::B_0x1)
    }
}
#[doc = "I3C2 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C2LPEN_A {
    #[doc = "0: I3C2 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: I3C2 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<I3C2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: I3C2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C2LPEN` reader - I3C2 clock enable during sleep mode Set and reset by software."]
pub type I3C2LPEN_R = crate::BitReader<I3C2LPEN_A>;
impl I3C2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3C2LPEN_A {
        match self.bits {
            false => I3C2LPEN_A::B_0x0,
            true => I3C2LPEN_A::B_0x1,
        }
    }
    #[doc = "I3C2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I3C2LPEN_A::B_0x0
    }
    #[doc = "I3C2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I3C2LPEN_A::B_0x1
    }
}
#[doc = "Field `I3C2LPEN` writer - I3C2 clock enable during sleep mode Set and reset by software."]
pub type I3C2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, I3C2LPEN_A>;
impl<'a, REG> I3C2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I3C2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2LPEN_A::B_0x0)
    }
    #[doc = "I3C2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2LPEN_A::B_0x1)
    }
}
#[doc = "LPTIM1 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1LPEN_A {
    #[doc = "0: LPTIM1 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: LPTIM1 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<LPTIM1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1LPEN` reader - LPTIM1 clock enable during sleep mode Set and reset by software."]
pub type LPTIM1LPEN_R = crate::BitReader<LPTIM1LPEN_A>;
impl LPTIM1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1LPEN_A {
        match self.bits {
            false => LPTIM1LPEN_A::B_0x0,
            true => LPTIM1LPEN_A::B_0x1,
        }
    }
    #[doc = "LPTIM1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPTIM1LPEN_A::B_0x0
    }
    #[doc = "LPTIM1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPTIM1LPEN_A::B_0x1
    }
}
#[doc = "Field `LPTIM1LPEN` writer - LPTIM1 clock enable during sleep mode Set and reset by software."]
pub type LPTIM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM1LPEN_A>;
impl<'a, REG> LPTIM1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1LPEN_A::B_0x0)
    }
    #[doc = "LPTIM1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1LPEN_A::B_0x1)
    }
}
#[doc = "VREF clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFLPEN_A {
    #[doc = "0: VREF peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: VREF peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<VREFLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFLPEN` reader - VREF clock enable during sleep mode Set and reset by software."]
pub type VREFLPEN_R = crate::BitReader<VREFLPEN_A>;
impl VREFLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFLPEN_A {
        match self.bits {
            false => VREFLPEN_A::B_0x0,
            true => VREFLPEN_A::B_0x1,
        }
    }
    #[doc = "VREF peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VREFLPEN_A::B_0x0
    }
    #[doc = "VREF peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VREFLPEN_A::B_0x1
    }
}
#[doc = "Field `VREFLPEN` writer - VREF clock enable during sleep mode Set and reset by software."]
pub type VREFLPEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFLPEN_A>;
impl<'a, REG> VREFLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREF peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VREFLPEN_A::B_0x0)
    }
    #[doc = "VREF peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VREFLPEN_A::B_0x1)
    }
}
#[doc = "RTC APB interface clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAPBLPEN_A {
    #[doc = "0: RTC APB interface clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: RTC APB interface clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<RTCAPBLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCAPBLPEN` reader - RTC APB interface clock enable during sleep mode Set and reset by software."]
pub type RTCAPBLPEN_R = crate::BitReader<RTCAPBLPEN_A>;
impl RTCAPBLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCAPBLPEN_A {
        match self.bits {
            false => RTCAPBLPEN_A::B_0x0,
            true => RTCAPBLPEN_A::B_0x1,
        }
    }
    #[doc = "RTC APB interface clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RTCAPBLPEN_A::B_0x0
    }
    #[doc = "RTC APB interface clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RTCAPBLPEN_A::B_0x1
    }
}
#[doc = "Field `RTCAPBLPEN` writer - RTC APB interface clock enable during sleep mode Set and reset by software."]
pub type RTCAPBLPEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCAPBLPEN_A>;
impl<'a, REG> RTCAPBLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC APB interface clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBLPEN_A::B_0x0)
    }
    #[doc = "RTC APB interface clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBLPEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 1 - SBS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn SBSLPEN(&self) -> SBSLPEN_R {
        SBSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn LPUART1LPEN(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - I3C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn I3C2LPEN(&self) -> I3C2LPEN_R {
        I3C2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM1LPEN(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn VREFLPEN(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn RTCAPBLPEN(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn SBSLPEN(&mut self) -> SBSLPEN_W<'_, APB3LPENR_SPEC> {
        SBSLPEN_W::new(self, 1)
    }
    #[doc = "Bit 6 - LPUART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn LPUART1LPEN(&mut self) -> LPUART1LPEN_W<'_, APB3LPENR_SPEC> {
        LPUART1LPEN_W::new(self, 6)
    }
    #[doc = "Bit 9 - I3C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn I3C2LPEN(&mut self) -> I3C2LPEN_W<'_, APB3LPENR_SPEC> {
        I3C2LPEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM1LPEN(&mut self) -> LPTIM1LPEN_W<'_, APB3LPENR_SPEC> {
        LPTIM1LPEN_W::new(self, 11)
    }
    #[doc = "Bit 20 - VREF clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn VREFLPEN(&mut self) -> VREFLPEN_W<'_, APB3LPENR_SPEC> {
        VREFLPEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn RTCAPBLPEN(&mut self) -> RTCAPBLPEN_W<'_, APB3LPENR_SPEC> {
        RTCAPBLPEN_W::new(self, 21)
    }
}
#[doc = "RCC APB3 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3LPENR_SPEC;
impl crate::RegisterSpec for APB3LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3lpenr::R`](R) reader structure"]
impl crate::Readable for APB3LPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb3lpenr::W`](W) writer structure"]
impl crate::Writable for APB3LPENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB3LPENR to value 0xffff_ffff"]
impl crate::Resettable for APB3LPENR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

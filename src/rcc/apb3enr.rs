#[doc = "Register `APB3ENR` reader"]
pub type R = crate::R<APB3ENR_SPEC>;
#[doc = "Register `APB3ENR` writer"]
pub type W = crate::W<APB3ENR_SPEC>;
#[doc = "SBS clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSEN_A {
    #[doc = "0: SBS peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: SBS peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<SBSEN_A> for bool {
    #[inline(always)]
    fn from(variant: SBSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBSEN` reader - SBS clock enable Set and reset by software."]
pub type SBSEN_R = crate::BitReader<SBSEN_A>;
impl SBSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBSEN_A {
        match self.bits {
            false => SBSEN_A::B_0x0,
            true => SBSEN_A::B_0x1,
        }
    }
    #[doc = "SBS peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SBSEN_A::B_0x0
    }
    #[doc = "SBS peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SBSEN_A::B_0x1
    }
}
#[doc = "Field `SBSEN` writer - SBS clock enable Set and reset by software."]
pub type SBSEN_W<'a, REG> = crate::BitWriter<'a, REG, SBSEN_A>;
impl<'a, REG> SBSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SBS peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SBSEN_A::B_0x0)
    }
    #[doc = "SBS peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SBSEN_A::B_0x1)
    }
}
#[doc = "LPUART1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1EN_A {
    #[doc = "0: LPUART1 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: LPUART1 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<LPUART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable Set and reset by software."]
pub type LPUART1EN_R = crate::BitReader<LPUART1EN_A>;
impl LPUART1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1EN_A {
        match self.bits {
            false => LPUART1EN_A::B_0x0,
            true => LPUART1EN_A::B_0x1,
        }
    }
    #[doc = "LPUART1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPUART1EN_A::B_0x0
    }
    #[doc = "LPUART1 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPUART1EN_A::B_0x1
    }
}
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable Set and reset by software."]
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1EN_A>;
impl<'a, REG> LPUART1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN_A::B_0x0)
    }
    #[doc = "LPUART1 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN_A::B_0x1)
    }
}
#[doc = "I3C2EN clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C2EN_A {
    #[doc = "0: I3C2EN peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: I3C2EN peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<I3C2EN_A> for bool {
    #[inline(always)]
    fn from(variant: I3C2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C2EN` reader - I3C2EN clock enable Set and reset by software."]
pub type I3C2EN_R = crate::BitReader<I3C2EN_A>;
impl I3C2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3C2EN_A {
        match self.bits {
            false => I3C2EN_A::B_0x0,
            true => I3C2EN_A::B_0x1,
        }
    }
    #[doc = "I3C2EN peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I3C2EN_A::B_0x0
    }
    #[doc = "I3C2EN peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I3C2EN_A::B_0x1
    }
}
#[doc = "Field `I3C2EN` writer - I3C2EN clock enable Set and reset by software."]
pub type I3C2EN_W<'a, REG> = crate::BitWriter<'a, REG, I3C2EN_A>;
impl<'a, REG> I3C2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I3C2EN peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2EN_A::B_0x0)
    }
    #[doc = "I3C2EN peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2EN_A::B_0x1)
    }
}
#[doc = "LPTIM1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1EN_A {
    #[doc = "0: LPTIM1 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: LPTIM1 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<LPTIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable Set and reset by software."]
pub type LPTIM1EN_R = crate::BitReader<LPTIM1EN_A>;
impl LPTIM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1EN_A {
        match self.bits {
            false => LPTIM1EN_A::B_0x0,
            true => LPTIM1EN_A::B_0x1,
        }
    }
    #[doc = "LPTIM1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPTIM1EN_A::B_0x0
    }
    #[doc = "LPTIM1 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPTIM1EN_A::B_0x1
    }
}
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable Set and reset by software."]
pub type LPTIM1EN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM1EN_A>;
impl<'a, REG> LPTIM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1EN_A::B_0x0)
    }
    #[doc = "LPTIM1 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1EN_A::B_0x1)
    }
}
#[doc = "VREF clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN_A {
    #[doc = "0: VREF peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: VREF peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFEN` reader - VREF clock enable Set and reset by software."]
pub type VREFEN_R = crate::BitReader<VREFEN_A>;
impl VREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::B_0x0,
            true => VREFEN_A::B_0x1,
        }
    }
    #[doc = "VREF peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VREFEN_A::B_0x0
    }
    #[doc = "VREF peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VREFEN_A::B_0x1
    }
}
#[doc = "Field `VREFEN` writer - VREF clock enable Set and reset by software."]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN_A>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREF peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN_A::B_0x0)
    }
    #[doc = "VREF peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN_A::B_0x1)
    }
}
#[doc = "RTC APB interface clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAPBEN_A {
    #[doc = "0: RTC APB interface clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: RTC APB interface clock enabled"]
    B_0x1 = 1,
}
impl From<RTCAPBEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCAPBEN` reader - RTC APB interface clock enable Set and reset by software."]
pub type RTCAPBEN_R = crate::BitReader<RTCAPBEN_A>;
impl RTCAPBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCAPBEN_A {
        match self.bits {
            false => RTCAPBEN_A::B_0x0,
            true => RTCAPBEN_A::B_0x1,
        }
    }
    #[doc = "RTC APB interface clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RTCAPBEN_A::B_0x0
    }
    #[doc = "RTC APB interface clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RTCAPBEN_A::B_0x1
    }
}
#[doc = "Field `RTCAPBEN` writer - RTC APB interface clock enable Set and reset by software."]
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCAPBEN_A>;
impl<'a, REG> RTCAPBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC APB interface clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBEN_A::B_0x0)
    }
    #[doc = "RTC APB interface clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 1 - SBS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SBSEN(&self) -> SBSEN_R {
        SBSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn LPUART1EN(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - I3C2EN clock enable Set and reset by software."]
    #[inline(always)]
    pub fn I3C2EN(&self) -> I3C2EN_R {
        I3C2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM1EN(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF clock enable Set and reset by software."]
    #[inline(always)]
    pub fn VREFEN(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable Set and reset by software."]
    #[inline(always)]
    pub fn RTCAPBEN(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SBSEN(&mut self) -> SBSEN_W<'_, APB3ENR_SPEC> {
        SBSEN_W::new(self, 1)
    }
    #[doc = "Bit 6 - LPUART1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn LPUART1EN(&mut self) -> LPUART1EN_W<'_, APB3ENR_SPEC> {
        LPUART1EN_W::new(self, 6)
    }
    #[doc = "Bit 9 - I3C2EN clock enable Set and reset by software."]
    #[inline(always)]
    pub fn I3C2EN(&mut self) -> I3C2EN_W<'_, APB3ENR_SPEC> {
        I3C2EN_W::new(self, 9)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM1EN(&mut self) -> LPTIM1EN_W<'_, APB3ENR_SPEC> {
        LPTIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 20 - VREF clock enable Set and reset by software."]
    #[inline(always)]
    pub fn VREFEN(&mut self) -> VREFEN_W<'_, APB3ENR_SPEC> {
        VREFEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable Set and reset by software."]
    #[inline(always)]
    pub fn RTCAPBEN(&mut self) -> RTCAPBEN_W<'_, APB3ENR_SPEC> {
        RTCAPBEN_W::new(self, 21)
    }
}
#[doc = "RCC APB3 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3ENR_SPEC;
impl crate::RegisterSpec for APB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3enr::R`](R) reader structure"]
impl crate::Readable for APB3ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb3enr::W`](W) writer structure"]
impl crate::Writable for APB3ENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB3ENR to value 0"]
impl crate::Resettable for APB3ENR_SPEC {}

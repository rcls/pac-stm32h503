#[doc = "Register `RWD` reader"]
pub type R = crate::R<RWD_SPEC>;
#[doc = "Register `RWD` writer"]
pub type W = crate::W<RWD_SPEC>;
#[doc = "Field `WDC` reader - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of FDCAN_CCCR register are set to 1."]
pub type WDC_R = crate::FieldReader;
#[doc = "Field `WDC` writer - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of FDCAN_CCCR register are set to 1."]
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDV` reader - Watchdog value Actual message RAM watchdog counter value."]
pub type WDV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of FDCAN_CCCR register are set to 1."]
    #[inline(always)]
    pub fn WDC(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog value Actual message RAM watchdog counter value."]
    #[inline(always)]
    pub fn WDV(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of FDCAN_CCCR register are set to 1."]
    #[inline(always)]
    pub fn WDC(&mut self) -> WDC_W<'_, RWD_SPEC> {
        WDC_W::new(self, 0)
    }
}
#[doc = "FDCAN RAM watchdog register\n\nYou can [`read`](crate::Reg::read) this register and get [`rwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RWD_SPEC;
impl crate::RegisterSpec for RWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rwd::R`](R) reader structure"]
impl crate::Readable for RWD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rwd::W`](W) writer structure"]
impl crate::Writable for RWD_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RWD to value 0"]
impl crate::Resettable for RWD_SPEC {}

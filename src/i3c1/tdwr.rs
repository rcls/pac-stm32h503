#[doc = "Register `TDWR` writer"]
pub type W = crate::W<TDWR_SPEC>;
#[doc = "Field `TDB0` writer - 8-bit transmit data (earliest byte on I3C bus)"]
pub type TDB0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDB1` writer - 8-bit transmit data (next byte after TDB0\\[7:0\\] on I3C bus)."]
pub type TDB1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDB2` writer - 8-bit transmit data (next byte after TDB1\\[7:0\\] on I3C bus)."]
pub type TDB2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDB3` writer - 8-bit transmit data (latest byte on I3C bus)."]
pub type TDB3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - 8-bit transmit data (earliest byte on I3C bus)"]
    #[inline(always)]
    pub fn TDB0(&mut self) -> TDB0_W<'_, TDWR_SPEC> {
        TDB0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit transmit data (next byte after TDB0\\[7:0\\] on I3C bus)."]
    #[inline(always)]
    pub fn TDB1(&mut self) -> TDB1_W<'_, TDWR_SPEC> {
        TDB1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit transmit data (next byte after TDB1\\[7:0\\] on I3C bus)."]
    #[inline(always)]
    pub fn TDB2(&mut self) -> TDB2_W<'_, TDWR_SPEC> {
        TDB2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit transmit data (latest byte on I3C bus)."]
    #[inline(always)]
    pub fn TDB3(&mut self) -> TDB3_W<'_, TDWR_SPEC> {
        TDB3_W::new(self, 24)
    }
}
#[doc = "I3C transmit data word register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDWR_SPEC;
impl crate::RegisterSpec for TDWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdwr::W`](W) writer structure"]
impl crate::Writable for TDWR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TDWR to value 0"]
impl crate::Resettable for TDWR_SPEC {}

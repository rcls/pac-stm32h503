#[doc = "Register `TOCV` reader"]
pub type R = crate::R<TOCV_SPEC>;
#[doc = "Register `TOCV` writer"]
pub type W = crate::W<TOCV_SPEC>;
#[doc = "Field `TOC` reader - Timeout counter The timeout counter is decremented in multiples of CAN bit times depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the timeout counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS."]
pub type TOC_R = crate::FieldReader<u16>;
#[doc = "Field `TOC` writer - Timeout counter The timeout counter is decremented in multiples of CAN bit times depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the timeout counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS."]
pub type TOC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout counter The timeout counter is decremented in multiples of CAN bit times depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the timeout counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS."]
    #[inline(always)]
    pub fn TOC(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout counter The timeout counter is decremented in multiples of CAN bit times depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the timeout counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS."]
    #[inline(always)]
    pub fn TOC(&mut self) -> TOC_W<'_, TOCV_SPEC> {
        TOC_W::new(self, 0)
    }
}
#[doc = "FDCAN timeout counter value register\n\nYou can [`read`](crate::Reg::read) this register and get [`tocv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCV_SPEC;
impl crate::RegisterSpec for TOCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocv::R`](R) reader structure"]
impl crate::Readable for TOCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tocv::W`](W) writer structure"]
impl crate::Writable for TOCV_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TOCV to value 0xffff"]
impl crate::Resettable for TOCV_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}

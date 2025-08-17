#[doc = "Register `XIDAM` reader"]
pub type R = crate::R<XIDAM_SPEC>;
#[doc = "Register `XIDAM` writer"]
pub type W = crate::W<XIDAM_SPEC>;
#[doc = "Field `EIDM` reader - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type EIDM_R = crate::FieldReader<u32>;
#[doc = "Field `EIDM` writer - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type EIDM_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn EIDM(&self) -> EIDM_R {
        EIDM_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn EIDM(&mut self) -> EIDM_W<'_, XIDAM_SPEC> {
        EIDM_W::new(self, 0)
    }
}
#[doc = "FDCAN extended ID and mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`xidam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XIDAM_SPEC;
impl crate::RegisterSpec for XIDAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xidam::R`](R) reader structure"]
impl crate::Readable for XIDAM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xidam::W`](W) writer structure"]
impl crate::Writable for XIDAM_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets XIDAM to value 0x1fff_ffff"]
impl crate::Resettable for XIDAM_SPEC {
    const RESET_VALUE: u32 = 0x1fff_ffff;
}

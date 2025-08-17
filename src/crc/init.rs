#[doc = "Register `INIT` reader"]
pub type R = crate::R<INIT_SPEC>;
#[doc = "Register `INIT` writer"]
pub type W = crate::W<INIT_SPEC>;
#[doc = "Field `CRC_INIT` reader - Programmable initial CRC value This register is used to write the CRC initial value."]
pub type CRC_INIT_R = crate::FieldReader<u32>;
#[doc = "Field `CRC_INIT` writer - Programmable initial CRC value This register is used to write the CRC initial value."]
pub type CRC_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Programmable initial CRC value This register is used to write the CRC initial value."]
    #[inline(always)]
    pub fn CRC_INIT(&self) -> CRC_INIT_R {
        CRC_INIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable initial CRC value This register is used to write the CRC initial value."]
    #[inline(always)]
    pub fn CRC_INIT(&mut self) -> CRC_INIT_W<'_, INIT_SPEC> {
        CRC_INIT_W::new(self, 0)
    }
}
#[doc = "CRC initial value\n\nYou can [`read`](crate::Reg::read) this register and get [`init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INIT_SPEC;
impl crate::RegisterSpec for INIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`init::R`](R) reader structure"]
impl crate::Readable for INIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`init::W`](W) writer structure"]
impl crate::Writable for INIT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INIT to value 0xffff_ffff"]
impl crate::Resettable for INIT_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

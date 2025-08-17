#[doc = "Register `FPUIMR` reader"]
pub type R = crate::R<FPUIMR_SPEC>;
#[doc = "Register `FPUIMR` writer"]
pub type W = crate::W<FPUIMR_SPEC>;
#[doc = "Field `FPU_IE` reader - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\\[5\\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\\[4\\]: input abnormal interrupt enable FPU_IE\\[3\\]: overflow interrupt enable FPU_IE\\[2\\]: underflow interrupt enable FPU_IE\\[1\\]: divide-by-zero interrupt enable FPU_IE\\[0\\]: invalid operation interrupt enable"]
pub type FPU_IE_R = crate::FieldReader;
#[doc = "Field `FPU_IE` writer - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\\[5\\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\\[4\\]: input abnormal interrupt enable FPU_IE\\[3\\]: overflow interrupt enable FPU_IE\\[2\\]: underflow interrupt enable FPU_IE\\[1\\]: divide-by-zero interrupt enable FPU_IE\\[0\\]: invalid operation interrupt enable"]
pub type FPU_IE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\\[5\\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\\[4\\]: input abnormal interrupt enable FPU_IE\\[3\\]: overflow interrupt enable FPU_IE\\[2\\]: underflow interrupt enable FPU_IE\\[1\\]: divide-by-zero interrupt enable FPU_IE\\[0\\]: invalid operation interrupt enable"]
    #[inline(always)]
    pub fn FPU_IE(&self) -> FPU_IE_R {
        FPU_IE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\\[5\\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\\[4\\]: input abnormal interrupt enable FPU_IE\\[3\\]: overflow interrupt enable FPU_IE\\[2\\]: underflow interrupt enable FPU_IE\\[1\\]: divide-by-zero interrupt enable FPU_IE\\[0\\]: invalid operation interrupt enable"]
    #[inline(always)]
    pub fn FPU_IE(&mut self) -> FPU_IE_W<'_, FPUIMR_SPEC> {
        FPU_IE_W::new(self, 0)
    }
}
#[doc = "SBS FPU interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpuimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpuimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPUIMR_SPEC;
impl crate::RegisterSpec for FPUIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpuimr::R`](R) reader structure"]
impl crate::Readable for FPUIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpuimr::W`](W) writer structure"]
impl crate::Writable for FPUIMR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FPUIMR to value 0x1f"]
impl crate::Resettable for FPUIMR_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}

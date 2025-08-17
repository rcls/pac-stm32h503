#[doc = "Register `CCSWCR` reader"]
pub type R = crate::R<CCSWCR_SPEC>;
#[doc = "Register `CCSWCR` writer"]
pub type W = crate::W<CCSWCR_SPEC>;
#[doc = "Field `SW_ANSRC1` reader - NMOS compensation code for VDD power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
pub type SW_ANSRC1_R = crate::FieldReader;
#[doc = "Field `SW_ANSRC1` writer - NMOS compensation code for VDD power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
pub type SW_ANSRC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_APSRC1` reader - PMOS compensation code for the VDD power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
pub type SW_APSRC1_R = crate::FieldReader;
#[doc = "Field `SW_APSRC1` writer - PMOS compensation code for the VDD power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
pub type SW_APSRC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_ANSRC2` reader - NMOS compensation code for VDDIO power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
pub type SW_ANSRC2_R = crate::FieldReader;
#[doc = "Field `SW_ANSRC2` writer - NMOS compensation code for VDDIO power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
pub type SW_ANSRC2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_APSRC2` reader - PMOS compensation code for the V DDIO power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
pub type SW_APSRC2_R = crate::FieldReader;
#[doc = "Field `SW_APSRC2` writer - PMOS compensation code for the V DDIO power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
pub type SW_APSRC2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - NMOS compensation code for VDD power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
    #[inline(always)]
    pub fn SW_ANSRC1(&self) -> SW_ANSRC1_R {
        SW_ANSRC1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PMOS compensation code for the VDD power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
    #[inline(always)]
    pub fn SW_APSRC1(&self) -> SW_APSRC1_R {
        SW_APSRC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - NMOS compensation code for VDDIO power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
    #[inline(always)]
    pub fn SW_ANSRC2(&self) -> SW_ANSRC2_R {
        SW_ANSRC2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PMOS compensation code for the V DDIO power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
    #[inline(always)]
    pub fn SW_APSRC2(&self) -> SW_APSRC2_R {
        SW_APSRC2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - NMOS compensation code for VDD power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
    #[inline(always)]
    pub fn SW_ANSRC1(&mut self) -> SW_ANSRC1_W<'_, CCSWCR_SPEC> {
        SW_ANSRC1_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - PMOS compensation code for the VDD power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR."]
    #[inline(always)]
    pub fn SW_APSRC1(&mut self) -> SW_APSRC1_W<'_, CCSWCR_SPEC> {
        SW_APSRC1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - NMOS compensation code for VDDIO power rails This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
    #[inline(always)]
    pub fn SW_ANSRC2(&mut self) -> SW_ANSRC2_W<'_, CCSWCR_SPEC> {
        SW_ANSRC2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - PMOS compensation code for the V DDIO power rails This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR."]
    #[inline(always)]
    pub fn SW_APSRC2(&mut self) -> SW_APSRC2_W<'_, CCSWCR_SPEC> {
        SW_APSRC2_W::new(self, 12)
    }
}
#[doc = "SBS compensation cell for I/Os software code register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccswcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccswcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCSWCR_SPEC;
impl crate::RegisterSpec for CCSWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccswcr::R`](R) reader structure"]
impl crate::Readable for CCSWCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccswcr::W`](W) writer structure"]
impl crate::Writable for CCSWCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCSWCR to value 0x7878"]
impl crate::Resettable for CCSWCR_SPEC {
    const RESET_VALUE: u32 = 0x7878;
}

#[doc = "Register `DHR12R1` reader"]
pub type R = crate::R<DHR12R1_SPEC>;
#[doc = "Register `DHR12R1` writer"]
pub type W = crate::W<DHR12R1_SPEC>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
pub type DACC1DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DACC1DHRB` reader - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode."]
pub type DACC1DHRB_R = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHRB` writer - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode."]
pub type DACC1DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn DACC1DHR(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode."]
    #[inline(always)]
    pub fn DACC1DHRB(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn DACC1DHR(&mut self) -> DACC1DHR_W<'_, DHR12R1_SPEC> {
        DACC1DHR_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode."]
    #[inline(always)]
    pub fn DACC1DHRB(&mut self) -> DACC1DHRB_W<'_, DHR12R1_SPEC> {
        DACC1DHRB_W::new(self, 16)
    }
}
#[doc = "DAC channel1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHR12R1_SPEC;
impl crate::RegisterSpec for DHR12R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12r1::R`](R) reader structure"]
impl crate::Readable for DHR12R1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dhr12r1::W`](W) writer structure"]
impl crate::Writable for DHR12R1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DHR12R1 to value 0"]
impl crate::Resettable for DHR12R1_SPEC {}

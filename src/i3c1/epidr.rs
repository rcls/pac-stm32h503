#[doc = "Register `EPIDR` reader"]
pub type R = crate::R<EPIDR_SPEC>;
#[doc = "Register `EPIDR` writer"]
pub type W = crate::W<EPIDR_SPEC>;
#[doc = "Field `MIPIID` reader - 4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\\[15:12\\] of the 48-bit provisioned ID. Note: The bits\\[11:0\\] of the provisioned ID may be 0."]
pub type MIPIID_R = crate::FieldReader;
#[doc = "Field `MIPIID` writer - 4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\\[15:12\\] of the 48-bit provisioned ID. Note: The bits\\[11:0\\] of the provisioned ID may be 0."]
pub type MIPIID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IDTSEL` reader - provisioned ID type selector This field is set as 0 i.e. vendor fixed value. This field represents the bit\\[32\\] of the 48-bit provisioned ID. Note: The bits\\[31:16\\] of the provisioned ID may be 0."]
pub type IDTSEL_R = crate::BitReader;
#[doc = "Field `MIPIMID` reader - 15-bit MIPI manufacturer ID This read field is the 15-bit STMicroelectronics MIPI ID i.e. 0x0104. This field represents the bits\\[47:33\\] of the 48-bit provisioned ID."]
pub type MIPIMID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 12:15 - 4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\\[15:12\\] of the 48-bit provisioned ID. Note: The bits\\[11:0\\] of the provisioned ID may be 0."]
    #[inline(always)]
    pub fn MIPIID(&self) -> MIPIID_R {
        MIPIID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - provisioned ID type selector This field is set as 0 i.e. vendor fixed value. This field represents the bit\\[32\\] of the 48-bit provisioned ID. Note: The bits\\[31:16\\] of the provisioned ID may be 0."]
    #[inline(always)]
    pub fn IDTSEL(&self) -> IDTSEL_R {
        IDTSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 15-bit MIPI manufacturer ID This read field is the 15-bit STMicroelectronics MIPI ID i.e. 0x0104. This field represents the bits\\[47:33\\] of the 48-bit provisioned ID."]
    #[inline(always)]
    pub fn MIPIMID(&self) -> MIPIMID_R {
        MIPIMID_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:15 - 4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\\[15:12\\] of the 48-bit provisioned ID. Note: The bits\\[11:0\\] of the provisioned ID may be 0."]
    #[inline(always)]
    pub fn MIPIID(&mut self) -> MIPIID_W<'_, EPIDR_SPEC> {
        MIPIID_W::new(self, 12)
    }
}
#[doc = "I3C extended provisioned ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`epidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPIDR_SPEC;
impl crate::RegisterSpec for EPIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epidr::R`](R) reader structure"]
impl crate::Readable for EPIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epidr::W`](W) writer structure"]
impl crate::Writable for EPIDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EPIDR to value 0x0208_0000"]
impl crate::Resettable for EPIDR_SPEC {
    const RESET_VALUE: u32 = 0x0208_0000;
}

#[doc = "Register `C6BR2` reader"]
pub type R = crate::R<C6BR2_SPEC>;
#[doc = "Register `C6BR2` writer"]
pub type W = crate::W<C6BR2_SPEC>;
#[doc = "Field `BRSAO` reader - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\\[2:0\\] versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRSAO\\[15:0\\] must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
pub type BRSAO_R = crate::FieldReader<u16>;
#[doc = "Field `BRSAO` writer - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\\[2:0\\] versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRSAO\\[15:0\\] must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
pub type BRSAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BRDAO` reader - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\\[2:0\\] versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRDAO\\[15:0\\] must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
pub type BRDAO_R = crate::FieldReader<u16>;
#[doc = "Field `BRDAO` writer - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\\[2:0\\] versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRDAO\\[15:0\\] must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
pub type BRDAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\\[2:0\\] versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRSAO\\[15:0\\] must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
    #[inline(always)]
    pub fn BRSAO(&self) -> BRSAO_R {
        BRSAO_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\\[2:0\\] versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRDAO\\[15:0\\] must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
    #[inline(always)]
    pub fn BRDAO(&self) -> BRDAO_R {
        BRDAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\\[2:0\\] versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRSAO\\[15:0\\] must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
    #[inline(always)]
    pub fn BRSAO(&mut self) -> BRSAO_W<'_, C6BR2_SPEC> {
        BRSAO_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\\[2:0\\] versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. Note: BRDAO\\[15:0\\] must be set to 0 in peripheral flow-control mode (if GPDMA_CxTR2.PFREQ = 1)."]
    #[inline(always)]
    pub fn BRDAO(&mut self) -> BRDAO_W<'_, C6BR2_SPEC> {
        BRDAO_W::new(self, 16)
    }
}
#[doc = "GPDMA channel 6 block register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c6br2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6br2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C6BR2_SPEC;
impl crate::RegisterSpec for C6BR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c6br2::R`](R) reader structure"]
impl crate::Readable for C6BR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c6br2::W`](W) writer structure"]
impl crate::Writable for C6BR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets C6BR2 to value 0"]
impl crate::Resettable for C6BR2_SPEC {}

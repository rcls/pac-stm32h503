#[doc = "Register `SWTRGR` writer"]
pub type W = crate::W<SWTRGR_SPEC>;
#[doc = "DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_hclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG1_A {
    #[doc = "0: No trigger"]
    B_0x0 = 0,
    #[doc = "1: Trigger"]
    B_0x1 = 1,
}
impl From<SWTRIG1_A> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_hclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
pub type SWTRIG1_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG1_A>;
impl<'a, REG> SWTRIG1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG1_A::B_0x0)
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG1_A::B_0x1)
    }
}
#[doc = "DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_hclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG2_A {
    #[doc = "0: No trigger"]
    B_0x0 = 0,
    #[doc = "1: Trigger"]
    B_0x1 = 1,
}
impl From<SWTRIG2_A> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTRIG2` writer - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_hclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation."]
pub type SWTRIG2_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG2_A>;
impl<'a, REG> SWTRIG2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG2_A::B_0x0)
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG2_A::B_0x1)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_hclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
    #[inline(always)]
    pub fn SWTRIG1(&mut self) -> SWTRIG1_W<'_, SWTRGR_SPEC> {
        SWTRIG1_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_hclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn SWTRIG2(&mut self) -> SWTRIG2_W<'_, SWTRGR_SPEC> {
        SWTRIG2_W::new(self, 1)
    }
}
#[doc = "DAC software trigger register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWTRGR_SPEC;
impl crate::RegisterSpec for SWTRGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swtrgr::W`](W) writer structure"]
impl crate::Writable for SWTRGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SWTRGR to value 0"]
impl crate::Resettable for SWTRGR_SPEC {}

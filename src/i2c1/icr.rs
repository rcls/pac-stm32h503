#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `ADDRCF` writer - Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register."]
pub type ADDRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKCF` writer - Not Acknowledge flag clear Writing 1 to this bit clears the NACKF flag in I2C_ISR register."]
pub type NACKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPCF` writer - STOP detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register."]
pub type STOPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRCF` writer - Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register."]
pub type BERRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOCF` writer - Arbitration lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register."]
pub type ARLOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRCF` writer - Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register."]
pub type OVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECCF` writer - PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type PECCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMOUTCF` writer - Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type TIMOUTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTCF` writer - Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type ALERTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 3 - Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register."]
    #[inline(always)]
    pub fn ADDRCF(&mut self) -> ADDRCF_W<'_, ICR_SPEC> {
        ADDRCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Not Acknowledge flag clear Writing 1 to this bit clears the NACKF flag in I2C_ISR register."]
    #[inline(always)]
    pub fn NACKCF(&mut self) -> NACKCF_W<'_, ICR_SPEC> {
        NACKCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - STOP detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register."]
    #[inline(always)]
    pub fn STOPCF(&mut self) -> STOPCF_W<'_, ICR_SPEC> {
        STOPCF_W::new(self, 5)
    }
    #[doc = "Bit 8 - Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register."]
    #[inline(always)]
    pub fn BERRCF(&mut self) -> BERRCF_W<'_, ICR_SPEC> {
        BERRCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register."]
    #[inline(always)]
    pub fn ARLOCF(&mut self) -> ARLOCF_W<'_, ICR_SPEC> {
        ARLOCF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register."]
    #[inline(always)]
    pub fn OVRCF(&mut self) -> OVRCF_W<'_, ICR_SPEC> {
        OVRCF_W::new(self, 10)
    }
    #[doc = "Bit 11 - PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn PECCF(&mut self) -> PECCF_W<'_, ICR_SPEC> {
        PECCF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn TIMOUTCF(&mut self) -> TIMOUTCF_W<'_, ICR_SPEC> {
        TIMOUTCF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn ALERTCF(&mut self) -> ALERTCF_W<'_, ICR_SPEC> {
        ALERTCF_W::new(self, 13)
    }
}
#[doc = "I2C interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {}

#[doc = "Register `OFR4` reader"]
pub type R = crate::R<OFR4_SPEC>;
#[doc = "Register `OFR4` writer"]
pub type W = crate::W<OFR4_SPEC>;
#[doc = "Field `OFFSET` reader - Data offset y for the channel programmed into bits OFFSET_CH\\[4:0\\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\\[4:0\\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\\[4:0\\] = 4 and OFFSET2_CH\\[4:0\\] = 4, this is OFFSET1\\[11:0\\] which is subtracted when converting channel 4."]
pub type OFFSET_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET` writer - Data offset y for the channel programmed into bits OFFSET_CH\\[4:0\\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\\[4:0\\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\\[4:0\\] = 4 and OFFSET2_CH\\[4:0\\] = 4, this is OFFSET1\\[11:0\\] which is subtracted when converting channel 4."]
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSETPOS_A {
    #[doc = "0: Negative offset"]
    B_0x0 = 0,
    #[doc = "1: Positive offset"]
    B_0x1 = 1,
}
impl From<OFFSETPOS_A> for bool {
    #[inline(always)]
    fn from(variant: OFFSETPOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFSETPOS` reader - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OFFSETPOS_R = crate::BitReader<OFFSETPOS_A>;
impl OFFSETPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OFFSETPOS_A {
        match self.bits {
            false => OFFSETPOS_A::B_0x0,
            true => OFFSETPOS_A::B_0x1,
        }
    }
    #[doc = "Negative offset"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OFFSETPOS_A::B_0x0
    }
    #[doc = "Positive offset"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OFFSETPOS_A::B_0x1
    }
}
#[doc = "Field `OFFSETPOS` writer - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OFFSETPOS_W<'a, REG> = crate::BitWriter<'a, REG, OFFSETPOS_A>;
impl<'a, REG> OFFSETPOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Negative offset"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSETPOS_A::B_0x0)
    }
    #[doc = "Positive offset"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSETPOS_A::B_0x1)
    }
}
#[doc = "Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SATEN_A {
    #[doc = "0: No saturation control, offset result can be signed"]
    B_0x0 = 0,
    #[doc = "1: Saturation enabled, offset result unsigned and saturated at 0x000 and 0xFFF"]
    B_0x1 = 1,
}
impl From<SATEN_A> for bool {
    #[inline(always)]
    fn from(variant: SATEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SATEN` reader - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SATEN_R = crate::BitReader<SATEN_A>;
impl SATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SATEN_A {
        match self.bits {
            false => SATEN_A::B_0x0,
            true => SATEN_A::B_0x1,
        }
    }
    #[doc = "No saturation control, offset result can be signed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SATEN_A::B_0x0
    }
    #[doc = "Saturation enabled, offset result unsigned and saturated at 0x000 and 0xFFF"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SATEN_A::B_0x1
    }
}
#[doc = "Field `SATEN` writer - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SATEN_W<'a, REG> = crate::BitWriter<'a, REG, SATEN_A>;
impl<'a, REG> SATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No saturation control, offset result can be signed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SATEN_A::B_0x0)
    }
    #[doc = "Saturation enabled, offset result unsigned and saturated at 0x000 and 0xFFF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SATEN_A::B_0x1)
    }
}
#[doc = "Field `OFFSET_CH` reader - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\\[11:0\\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers."]
pub type OFFSET_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET_CH` writer - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\\[11:0\\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers."]
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSET_EN` reader - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\\[11:0\\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OFFSET_EN_R = crate::BitReader;
#[doc = "Field `OFFSET_EN` writer - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\\[11:0\\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OFFSET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Data offset y for the channel programmed into bits OFFSET_CH\\[4:0\\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\\[4:0\\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\\[4:0\\] = 4 and OFFSET2_CH\\[4:0\\] = 4, this is OFFSET1\\[11:0\\] which is subtracted when converting channel 4."]
    #[inline(always)]
    pub fn OFFSET(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn OFFSETPOS(&self) -> OFFSETPOS_R {
        OFFSETPOS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn SATEN(&self) -> SATEN_R {
        SATEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\\[11:0\\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers."]
    #[inline(always)]
    pub fn OFFSET_CH(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\\[11:0\\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn OFFSET_EN(&self) -> OFFSET_EN_R {
        OFFSET_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset y for the channel programmed into bits OFFSET_CH\\[4:0\\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\\[4:0\\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\\[4:0\\] = 4 and OFFSET2_CH\\[4:0\\] = 4, this is OFFSET1\\[11:0\\] which is subtracted when converting channel 4."]
    #[inline(always)]
    pub fn OFFSET(&mut self) -> OFFSET_W<'_, OFR4_SPEC> {
        OFFSET_W::new(self, 0)
    }
    #[doc = "Bit 24 - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn OFFSETPOS(&mut self) -> OFFSETPOS_W<'_, OFR4_SPEC> {
        OFFSETPOS_W::new(self, 24)
    }
    #[doc = "Bit 25 - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn SATEN(&mut self) -> SATEN_W<'_, OFR4_SPEC> {
        SATEN_W::new(self, 25)
    }
    #[doc = "Bits 26:30 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\\[11:0\\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers."]
    #[inline(always)]
    pub fn OFFSET_CH(&mut self) -> OFFSET_CH_W<'_, OFR4_SPEC> {
        OFFSET_CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\\[11:0\\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn OFFSET_EN(&mut self) -> OFFSET_EN_W<'_, OFR4_SPEC> {
        OFFSET_EN_W::new(self, 31)
    }
}
#[doc = "ADC offset 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OFR4_SPEC;
impl crate::RegisterSpec for OFR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr4::R`](R) reader structure"]
impl crate::Readable for OFR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ofr4::W`](W) writer structure"]
impl crate::Writable for OFR4_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OFR4 to value 0"]
impl crate::Resettable for OFR4_SPEC {}

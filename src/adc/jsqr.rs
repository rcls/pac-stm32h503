#[doc = "Register `JSQR` reader"]
pub type R = crate::R<JSQR_SPEC>;
#[doc = "Register `JSQR` writer"]
pub type W = crate::W<JSQR_SPEC>;
#[doc = "Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JL_A {
    #[doc = "0: 1 conversion"]
    B_0x0 = 0,
    #[doc = "1: 2 conversions"]
    B_0x1 = 1,
    #[doc = "2: 3 conversions"]
    B_0x2 = 2,
    #[doc = "3: 4 conversions"]
    B_0x3 = 3,
}
impl From<JL_A> for u8 {
    #[inline(always)]
    fn from(variant: JL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JL_A {
    type Ux = u8;
}
impl crate::IsEnum for JL_A {}
#[doc = "Field `JL` reader - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JL_R = crate::FieldReader<JL_A>;
impl JL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JL_A {
        match self.bits {
            0 => JL_A::B_0x0,
            1 => JL_A::B_0x1,
            2 => JL_A::B_0x2,
            3 => JL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "1 conversion"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JL_A::B_0x0
    }
    #[doc = "2 conversions"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JL_A::B_0x1
    }
    #[doc = "3 conversions"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == JL_A::B_0x2
    }
    #[doc = "4 conversions"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == JL_A::B_0x3
    }
}
#[doc = "Field `JL` writer - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, JL_A, crate::Safe>;
impl<'a, REG> JL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 conversion"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JL_A::B_0x0)
    }
    #[doc = "2 conversions"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JL_A::B_0x1)
    }
    #[doc = "3 conversions"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(JL_A::B_0x2)
    }
    #[doc = "4 conversions"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(JL_A::B_0x3)
    }
}
#[doc = "External Trigger Selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL_A {
    #[doc = "0: adc_jext_trg0"]
    B_0x0 = 0,
    #[doc = "1: adc_jext_trg1"]
    B_0x1 = 1,
    #[doc = "2: adc_jext_trg2"]
    B_0x2 = 2,
    #[doc = "3: adc_jext_trg3"]
    B_0x3 = 3,
    #[doc = "4: adc_jext_trg4"]
    B_0x4 = 4,
    #[doc = "5: adc_jext_trg5"]
    B_0x5 = 5,
    #[doc = "6: adc_jext_trg6"]
    B_0x6 = 6,
    #[doc = "7: adc_jext_trg7"]
    B_0x7 = 7,
    #[doc = "31: adc_jext_trg31"]
    B_0x1F = 31,
}
impl From<JEXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for JEXTSEL_A {}
#[doc = "Field `JEXTSEL` reader - External Trigger Selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JEXTSEL_R = crate::FieldReader<JEXTSEL_A>;
impl JEXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<JEXTSEL_A> {
        match self.bits {
            0 => Some(JEXTSEL_A::B_0x0),
            1 => Some(JEXTSEL_A::B_0x1),
            2 => Some(JEXTSEL_A::B_0x2),
            3 => Some(JEXTSEL_A::B_0x3),
            4 => Some(JEXTSEL_A::B_0x4),
            5 => Some(JEXTSEL_A::B_0x5),
            6 => Some(JEXTSEL_A::B_0x6),
            7 => Some(JEXTSEL_A::B_0x7),
            31 => Some(JEXTSEL_A::B_0x1F),
            _ => None,
        }
    }
    #[doc = "adc_jext_trg0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JEXTSEL_A::B_0x0
    }
    #[doc = "adc_jext_trg1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JEXTSEL_A::B_0x1
    }
    #[doc = "adc_jext_trg2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == JEXTSEL_A::B_0x2
    }
    #[doc = "adc_jext_trg3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == JEXTSEL_A::B_0x3
    }
    #[doc = "adc_jext_trg4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == JEXTSEL_A::B_0x4
    }
    #[doc = "adc_jext_trg5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == JEXTSEL_A::B_0x5
    }
    #[doc = "adc_jext_trg6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == JEXTSEL_A::B_0x6
    }
    #[doc = "adc_jext_trg7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == JEXTSEL_A::B_0x7
    }
    #[doc = "adc_jext_trg31"]
    #[inline(always)]
    pub fn is_B_0x1F(&self) -> bool {
        *self == JEXTSEL_A::B_0x1F
    }
}
#[doc = "Field `JEXTSEL` writer - External Trigger Selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, JEXTSEL_A>;
impl<'a, REG> JEXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "adc_jext_trg0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL_A::B_0x0)
    }
    #[doc = "adc_jext_trg1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL_A::B_0x1)
    }
    #[doc = "adc_jext_trg2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL_A::B_0x2)
    }
    #[doc = "adc_jext_trg3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL_A::B_0x3)
    }
    #[doc = "adc_jext_trg4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL_A::B_0x4)
    }
    #[doc = "adc_jext_trg5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL_A::B_0x5)
    }
    #[doc = "adc_jext_trg6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL_A::B_0x6)
    }
    #[doc = "adc_jext_trg7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL_A::B_0x7)
    }
    #[doc = "adc_jext_trg31"]
    #[inline(always)]
    pub fn B_0x1F(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL_A::B_0x1F)
    }
}
#[doc = "External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing). If JQM = 1 and if the Queue of Context becomes empty, the software and hardware triggers of the injected sequence are both internally disabled (refer to Queue of context for injected conversions)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTEN_A {
    #[doc = "0: If JQDIS = 0 (queue enabled), hardware and software trigger detection disabled. Otherwise, the queue is disabled as well as hardware trigger detection (conversions can be launched by software)"]
    B_0x0 = 0,
    #[doc = "1: Hardware trigger detection on the rising edge"]
    B_0x1 = 1,
    #[doc = "2: Hardware trigger detection on the falling edge"]
    B_0x2 = 2,
    #[doc = "3: Hardware trigger detection on both the rising and falling edges"]
    B_0x3 = 3,
}
impl From<JEXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTEN_A {
    type Ux = u8;
}
impl crate::IsEnum for JEXTEN_A {}
#[doc = "Field `JEXTEN` reader - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing). If JQM = 1 and if the Queue of Context becomes empty, the software and hardware triggers of the injected sequence are both internally disabled (refer to Queue of context for injected conversions)"]
pub type JEXTEN_R = crate::FieldReader<JEXTEN_A>;
impl JEXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEXTEN_A {
        match self.bits {
            0 => JEXTEN_A::B_0x0,
            1 => JEXTEN_A::B_0x1,
            2 => JEXTEN_A::B_0x2,
            3 => JEXTEN_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "If JQDIS = 0 (queue enabled), hardware and software trigger detection disabled. Otherwise, the queue is disabled as well as hardware trigger detection (conversions can be launched by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JEXTEN_A::B_0x0
    }
    #[doc = "Hardware trigger detection on the rising edge"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JEXTEN_A::B_0x1
    }
    #[doc = "Hardware trigger detection on the falling edge"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == JEXTEN_A::B_0x2
    }
    #[doc = "Hardware trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == JEXTEN_A::B_0x3
    }
}
#[doc = "Field `JEXTEN` writer - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing). If JQM = 1 and if the Queue of Context becomes empty, the software and hardware triggers of the injected sequence are both internally disabled (refer to Queue of context for injected conversions)"]
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, JEXTEN_A, crate::Safe>;
impl<'a, REG> JEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If JQDIS = 0 (queue enabled), hardware and software trigger detection disabled. Otherwise, the queue is disabled as well as hardware trigger detection (conversions can be launched by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN_A::B_0x0)
    }
    #[doc = "Hardware trigger detection on the rising edge"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN_A::B_0x1)
    }
    #[doc = "Hardware trigger detection on the falling edge"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN_A::B_0x2)
    }
    #[doc = "Hardware trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN_A::B_0x3)
    }
}
#[doc = "Field `JSQ1` reader - 1st conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JSQ1_R = crate::FieldReader;
#[doc = "Field `JSQ1` writer - 1st conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JSQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ2` reader - 2nd conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JSQ2_R = crate::FieldReader;
#[doc = "Field `JSQ2` writer - 2nd conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JSQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ3` reader - 3rd conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JSQ3_R = crate::FieldReader;
#[doc = "Field `JSQ3` writer - 3rd conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JSQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ4` reader - 4th conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JSQ4_R = crate::FieldReader;
#[doc = "Field `JSQ4` writer - 4th conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JSQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JL(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - External Trigger Selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JEXTSEL(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:8 - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing). If JQM = 1 and if the Queue of Context becomes empty, the software and hardware triggers of the injected sequence are both internally disabled (refer to Queue of context for injected conversions)"]
    #[inline(always)]
    pub fn JEXTEN(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:13 - 1st conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JSQ1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 2nd conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JSQ2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - 3rd conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JSQ3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - 4th conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JSQ4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JL(&mut self) -> JL_W<'_, JSQR_SPEC> {
        JL_W::new(self, 0)
    }
    #[doc = "Bits 2:6 - External Trigger Selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JEXTSEL(&mut self) -> JEXTSEL_W<'_, JSQR_SPEC> {
        JEXTSEL_W::new(self, 2)
    }
    #[doc = "Bits 7:8 - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing). If JQM = 1 and if the Queue of Context becomes empty, the software and hardware triggers of the injected sequence are both internally disabled (refer to Queue of context for injected conversions)"]
    #[inline(always)]
    pub fn JEXTEN(&mut self) -> JEXTEN_W<'_, JSQR_SPEC> {
        JEXTEN_W::new(self, 7)
    }
    #[doc = "Bits 9:13 - 1st conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JSQ1(&mut self) -> JSQ1_W<'_, JSQR_SPEC> {
        JSQ1_W::new(self, 9)
    }
    #[doc = "Bits 15:19 - 2nd conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JSQ2(&mut self) -> JSQ2_W<'_, JSQR_SPEC> {
        JSQ2_W::new(self, 15)
    }
    #[doc = "Bits 21:25 - 3rd conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JSQ3(&mut self) -> JSQ3_W<'_, JSQR_SPEC> {
        JSQ3_W::new(self, 21)
    }
    #[doc = "Bits 27:31 - 4th conversion in the injected sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JSQ4(&mut self) -> JSQ4_W<'_, JSQR_SPEC> {
        JSQ4_W::new(self, 27)
    }
}
#[doc = "ADC injected sequence register\n\nYou can [`read`](crate::Reg::read) this register and get [`jsqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JSQR_SPEC;
impl crate::RegisterSpec for JSQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jsqr::R`](R) reader structure"]
impl crate::Readable for JSQR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`jsqr::W`](W) writer structure"]
impl crate::Writable for JSQR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets JSQR to value 0"]
impl crate::Resettable for JSQR_SPEC {}

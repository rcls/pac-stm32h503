#[doc = "Register `SQR1` reader"]
pub type R = crate::R<SQR1_SPEC>;
#[doc = "Register `SQR1` writer"]
pub type W = crate::W<SQR1_SPEC>;
#[doc = "Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum L_A {
    #[doc = "0: 1 conversion"]
    B_0x0 = 0,
    #[doc = "1: 2 conversions"]
    B_0x1 = 1,
    #[doc = "15: 16 conversions"]
    B_0xF = 15,
}
impl From<L_A> for u8 {
    #[inline(always)]
    fn from(variant: L_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for L_A {
    type Ux = u8;
}
impl crate::IsEnum for L_A {}
#[doc = "Field `L` reader - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type L_R = crate::FieldReader<L_A>;
impl L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<L_A> {
        match self.bits {
            0 => Some(L_A::B_0x0),
            1 => Some(L_A::B_0x1),
            15 => Some(L_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "1 conversion"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == L_A::B_0x0
    }
    #[doc = "2 conversions"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == L_A::B_0x1
    }
    #[doc = "16 conversions"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == L_A::B_0xF
    }
}
#[doc = "Field `L` writer - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type L_W<'a, REG> = crate::FieldWriter<'a, REG, 4, L_A>;
impl<'a, REG> L_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 conversion"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(L_A::B_0x0)
    }
    #[doc = "2 conversions"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(L_A::B_0x1)
    }
    #[doc = "16 conversions"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(L_A::B_0xF)
    }
}
#[doc = "Field `SQ1` reader - 1st conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ1_R = crate::FieldReader;
#[doc = "Field `SQ1` writer - 1st conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ2` reader - 2nd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ2_R = crate::FieldReader;
#[doc = "Field `SQ2` writer - 2nd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ3` reader - 3rd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ3_R = crate::FieldReader;
#[doc = "Field `SQ3` writer - 3rd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ4` reader - 4th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ4_R = crate::FieldReader;
#[doc = "Field `SQ4` writer - 4th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn L(&self) -> L_R {
        L_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - 1st conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ1(&self) -> SQ1_R {
        SQ1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - 2nd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - 3rd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 4th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn L(&mut self) -> L_W<'_, SQR1_SPEC> {
        L_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - 1st conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ1(&mut self) -> SQ1_W<'_, SQR1_SPEC> {
        SQ1_W::new(self, 6)
    }
    #[doc = "Bits 12:16 - 2nd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ2(&mut self) -> SQ2_W<'_, SQR1_SPEC> {
        SQ2_W::new(self, 12)
    }
    #[doc = "Bits 18:22 - 3rd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ3(&mut self) -> SQ3_W<'_, SQR1_SPEC> {
        SQ3_W::new(self, 18)
    }
    #[doc = "Bits 24:28 - 4th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ4(&mut self) -> SQ4_W<'_, SQR1_SPEC> {
        SQ4_W::new(self, 24)
    }
}
#[doc = "ADC regular sequence register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR1_SPEC;
impl crate::RegisterSpec for SQR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr1::R`](R) reader structure"]
impl crate::Readable for SQR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sqr1::W`](W) writer structure"]
impl crate::Writable for SQR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for SQR1_SPEC {}

#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TISEL_SPEC>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TISEL_SPEC>;
#[doc = "Selects tim_ti1\\[0..15\\] input ... Refer to for product specific implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1SEL_A {
    #[doc = "0: tim_ti1_in0: TIMx_CH1"]
    B_0x0 = 0,
    #[doc = "1: tim_ti1_in1"]
    B_0x1 = 1,
    #[doc = "15: tim_ti1_in15"]
    B_0xF = 15,
}
impl From<TI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI1SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for TI1SEL_A {}
#[doc = "Field `TI1SEL` reader - Selects tim_ti1\\[0..15\\] input ... Refer to for product specific implementation."]
pub type TI1SEL_R = crate::FieldReader<TI1SEL_A>;
impl TI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI1SEL_A> {
        match self.bits {
            0 => Some(TI1SEL_A::B_0x0),
            1 => Some(TI1SEL_A::B_0x1),
            15 => Some(TI1SEL_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "tim_ti1_in0: TIMx_CH1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TI1SEL_A::B_0x0
    }
    #[doc = "tim_ti1_in1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TI1SEL_A::B_0x1
    }
    #[doc = "tim_ti1_in15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == TI1SEL_A::B_0xF
    }
}
#[doc = "Field `TI1SEL` writer - Selects tim_ti1\\[0..15\\] input ... Refer to for product specific implementation."]
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI1SEL_A>;
impl<'a, REG> TI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "tim_ti1_in0: TIMx_CH1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL_A::B_0x0)
    }
    #[doc = "tim_ti1_in1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL_A::B_0x1)
    }
    #[doc = "tim_ti1_in15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL_A::B_0xF)
    }
}
#[doc = "Selects tim_ti2\\[0..15\\] input ... Refer to for product specific implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI2SEL_A {
    #[doc = "0: tim_ti2_in0: TIMx_CH2"]
    B_0x0 = 0,
    #[doc = "1: tim_ti2_in1"]
    B_0x1 = 1,
    #[doc = "15: tim_ti2_in15"]
    B_0xF = 15,
}
impl From<TI2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI2SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI2SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for TI2SEL_A {}
#[doc = "Field `TI2SEL` reader - Selects tim_ti2\\[0..15\\] input ... Refer to for product specific implementation."]
pub type TI2SEL_R = crate::FieldReader<TI2SEL_A>;
impl TI2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI2SEL_A> {
        match self.bits {
            0 => Some(TI2SEL_A::B_0x0),
            1 => Some(TI2SEL_A::B_0x1),
            15 => Some(TI2SEL_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "tim_ti2_in0: TIMx_CH2"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TI2SEL_A::B_0x0
    }
    #[doc = "tim_ti2_in1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TI2SEL_A::B_0x1
    }
    #[doc = "tim_ti2_in15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == TI2SEL_A::B_0xF
    }
}
#[doc = "Field `TI2SEL` writer - Selects tim_ti2\\[0..15\\] input ... Refer to for product specific implementation."]
pub type TI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI2SEL_A>;
impl<'a, REG> TI2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "tim_ti2_in0: TIMx_CH2"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL_A::B_0x0)
    }
    #[doc = "tim_ti2_in1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL_A::B_0x1)
    }
    #[doc = "tim_ti2_in15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL_A::B_0xF)
    }
}
#[doc = "Selects tim_ti3\\[0..15\\] input ... Refer to for product specific implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI3SEL_A {
    #[doc = "0: tim_ti3_in0: TIMx_CH3"]
    B_0x0 = 0,
    #[doc = "1: tim_ti3_in1"]
    B_0x1 = 1,
    #[doc = "15: tim_ti3_in15"]
    B_0xF = 15,
}
impl From<TI3SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI3SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI3SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for TI3SEL_A {}
#[doc = "Field `TI3SEL` reader - Selects tim_ti3\\[0..15\\] input ... Refer to for product specific implementation."]
pub type TI3SEL_R = crate::FieldReader<TI3SEL_A>;
impl TI3SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI3SEL_A> {
        match self.bits {
            0 => Some(TI3SEL_A::B_0x0),
            1 => Some(TI3SEL_A::B_0x1),
            15 => Some(TI3SEL_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "tim_ti3_in0: TIMx_CH3"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TI3SEL_A::B_0x0
    }
    #[doc = "tim_ti3_in1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TI3SEL_A::B_0x1
    }
    #[doc = "tim_ti3_in15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == TI3SEL_A::B_0xF
    }
}
#[doc = "Field `TI3SEL` writer - Selects tim_ti3\\[0..15\\] input ... Refer to for product specific implementation."]
pub type TI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI3SEL_A>;
impl<'a, REG> TI3SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "tim_ti3_in0: TIMx_CH3"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI3SEL_A::B_0x0)
    }
    #[doc = "tim_ti3_in1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI3SEL_A::B_0x1)
    }
    #[doc = "tim_ti3_in15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(TI3SEL_A::B_0xF)
    }
}
#[doc = "Selects tim_ti4\\[0..15\\] input ... Refer to for product specific implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI4SEL_A {
    #[doc = "0: tim_ti4_in0: TIMx_CH4"]
    B_0x0 = 0,
    #[doc = "1: tim_ti4_in1"]
    B_0x1 = 1,
    #[doc = "15: tim_ti4_in15"]
    B_0xF = 15,
}
impl From<TI4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI4SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI4SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for TI4SEL_A {}
#[doc = "Field `TI4SEL` reader - Selects tim_ti4\\[0..15\\] input ... Refer to for product specific implementation."]
pub type TI4SEL_R = crate::FieldReader<TI4SEL_A>;
impl TI4SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI4SEL_A> {
        match self.bits {
            0 => Some(TI4SEL_A::B_0x0),
            1 => Some(TI4SEL_A::B_0x1),
            15 => Some(TI4SEL_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "tim_ti4_in0: TIMx_CH4"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TI4SEL_A::B_0x0
    }
    #[doc = "tim_ti4_in1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TI4SEL_A::B_0x1
    }
    #[doc = "tim_ti4_in15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == TI4SEL_A::B_0xF
    }
}
#[doc = "Field `TI4SEL` writer - Selects tim_ti4\\[0..15\\] input ... Refer to for product specific implementation."]
pub type TI4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI4SEL_A>;
impl<'a, REG> TI4SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "tim_ti4_in0: TIMx_CH4"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI4SEL_A::B_0x0)
    }
    #[doc = "tim_ti4_in1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI4SEL_A::B_0x1)
    }
    #[doc = "tim_ti4_in15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(TI4SEL_A::B_0xF)
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects tim_ti1\\[0..15\\] input ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn TI1SEL(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Selects tim_ti2\\[0..15\\] input ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn TI2SEL(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Selects tim_ti3\\[0..15\\] input ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn TI3SEL(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Selects tim_ti4\\[0..15\\] input ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn TI4SEL(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects tim_ti1\\[0..15\\] input ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn TI1SEL(&mut self) -> TI1SEL_W<'_, TISEL_SPEC> {
        TI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Selects tim_ti2\\[0..15\\] input ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn TI2SEL(&mut self) -> TI2SEL_W<'_, TISEL_SPEC> {
        TI2SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Selects tim_ti3\\[0..15\\] input ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn TI3SEL(&mut self) -> TI3SEL_W<'_, TISEL_SPEC> {
        TI3SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Selects tim_ti4\\[0..15\\] input ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn TI4SEL(&mut self) -> TI4SEL_W<'_, TISEL_SPEC> {
        TI4SEL_W::new(self, 24)
    }
}
#[doc = "TIM3 timer input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TISEL_SPEC;
impl crate::RegisterSpec for TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisel::R`](R) reader structure"]
impl crate::Readable for TISEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tisel::W`](W) writer structure"]
impl crate::Writable for TISEL_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TISEL_SPEC {}

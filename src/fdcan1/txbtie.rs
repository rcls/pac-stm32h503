#[doc = "Register `TXBTIE` reader"]
pub type R = crate::R<TXBTIE_SPEC>;
#[doc = "Register `TXBTIE` writer"]
pub type W = crate::W<TXBTIE_SPEC>;
#[doc = "Transmission interrupt enable Each Tx buffer has its own TIE bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIE_A {
    #[doc = "0: Transmission interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Transmission interrupt enable"]
    B_0x1 = 1,
}
impl From<TIE_A> for u8 {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIE_A {
    type Ux = u8;
}
impl crate::IsEnum for TIE_A {}
#[doc = "Field `TIE` reader - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
pub type TIE_R = crate::FieldReader<TIE_A>;
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIE_A> {
        match self.bits {
            0 => Some(TIE_A::B_0x0),
            1 => Some(TIE_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "Transmission interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIE_A::B_0x0
    }
    #[doc = "Transmission interrupt enable"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIE_A::B_0x1
    }
}
#[doc = "Field `TIE` writer - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
pub type TIE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TIE_A>;
impl<'a, REG> TIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmission interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::B_0x0)
    }
    #[doc = "Transmission interrupt enable"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
    #[inline(always)]
    pub fn TIE(&self) -> TIE_R {
        TIE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
    #[inline(always)]
    pub fn TIE(&mut self) -> TIE_W<'_, TXBTIE_SPEC> {
        TIE_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer transmission interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbtie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBTIE_SPEC;
impl crate::RegisterSpec for TXBTIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbtie::R`](R) reader structure"]
impl crate::Readable for TXBTIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbtie::W`](W) writer structure"]
impl crate::Writable for TXBTIE_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TXBTIE to value 0"]
impl crate::Resettable for TXBTIE_SPEC {}

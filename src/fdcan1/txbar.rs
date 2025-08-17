#[doc = "Register `TXBAR` reader"]
pub type R = crate::R<TXBAR_SPEC>;
#[doc = "Register `TXBAR` writer"]
pub type W = crate::W<TXBAR_SPEC>;
#[doc = "Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AR_A {
    #[doc = "0: No transmission request added"]
    B_0x0 = 0,
    #[doc = "1: Transmission requested added."]
    B_0x1 = 1,
}
impl From<AR_A> for u8 {
    #[inline(always)]
    fn from(variant: AR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AR_A {
    type Ux = u8;
}
impl crate::IsEnum for AR_A {}
#[doc = "Field `AR` reader - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
pub type AR_R = crate::FieldReader<AR_A>;
impl AR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AR_A> {
        match self.bits {
            0 => Some(AR_A::B_0x0),
            1 => Some(AR_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "No transmission request added"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AR_A::B_0x0
    }
    #[doc = "Transmission requested added."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AR_A::B_0x1
    }
}
#[doc = "Field `AR` writer - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
pub type AR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, AR_A>;
impl<'a, REG> AR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No transmission request added"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AR_A::B_0x0)
    }
    #[doc = "Transmission requested added."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AR_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
    #[inline(always)]
    pub fn AR(&self) -> AR_R {
        AR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
    #[inline(always)]
    pub fn AR(&mut self) -> AR_W<'_, TXBAR_SPEC> {
        AR_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer add request register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBAR_SPEC;
impl crate::RegisterSpec for TXBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbar::R`](R) reader structure"]
impl crate::Readable for TXBAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbar::W`](W) writer structure"]
impl crate::Writable for TXBAR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TXBAR to value 0"]
impl crate::Resettable for TXBAR_SPEC {}

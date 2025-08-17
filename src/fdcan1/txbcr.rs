#[doc = "Register `TXBCR` reader"]
pub type R = crate::R<TXBCR_SPEC>;
#[doc = "Register `TXBCR` writer"]
pub type W = crate::W<TXBCR_SPEC>;
#[doc = "Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CR_A {
    #[doc = "0: No cancellation pending"]
    B_0x0 = 0,
    #[doc = "1: Cancellation pending"]
    B_0x1 = 1,
}
impl From<CR_A> for u8 {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CR_A {
    type Ux = u8;
}
impl crate::IsEnum for CR_A {}
#[doc = "Field `CR` reader - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
pub type CR_R = crate::FieldReader<CR_A>;
impl CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CR_A> {
        match self.bits {
            0 => Some(CR_A::B_0x0),
            1 => Some(CR_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "No cancellation pending"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CR_A::B_0x0
    }
    #[doc = "Cancellation pending"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CR_A::B_0x1
    }
}
#[doc = "Field `CR` writer - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CR_A>;
impl<'a, REG> CR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No cancellation pending"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CR_A::B_0x0)
    }
    #[doc = "Cancellation pending"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CR_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
    #[inline(always)]
    pub fn CR(&self) -> CR_R {
        CR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
    #[inline(always)]
    pub fn CR(&mut self) -> CR_W<'_, TXBCR_SPEC> {
        CR_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer cancellation request register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCR_SPEC;
impl crate::RegisterSpec for TXBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcr::R`](R) reader structure"]
impl crate::Readable for TXBCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbcr::W`](W) writer structure"]
impl crate::Writable for TXBCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TXBCR to value 0"]
impl crate::Resettable for TXBCR_SPEC {}

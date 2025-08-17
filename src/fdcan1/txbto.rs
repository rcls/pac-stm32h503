#[doc = "Register `TXBTO` reader"]
pub type R = crate::R<TXBTO_SPEC>;
#[doc = "Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TO_A {
    #[doc = "0: No transmission occurred"]
    B_0x0 = 0,
    #[doc = "1: Transmission occurred"]
    B_0x1 = 1,
}
impl From<TO_A> for u8 {
    #[inline(always)]
    fn from(variant: TO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TO_A {
    type Ux = u8;
}
impl crate::IsEnum for TO_A {}
#[doc = "Field `TO` reader - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
pub type TO_R = crate::FieldReader<TO_A>;
impl TO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TO_A> {
        match self.bits {
            0 => Some(TO_A::B_0x0),
            1 => Some(TO_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "No transmission occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TO_A::B_0x0
    }
    #[doc = "Transmission occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TO_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn TO(&self) -> TO_R {
        TO_R::new((self.bits & 7) as u8)
    }
}
#[doc = "FDCAN Tx buffer transmission occurred register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbto::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBTO_SPEC;
impl crate::RegisterSpec for TXBTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbto::R`](R) reader structure"]
impl crate::Readable for TXBTO_SPEC {}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TXBTO_SPEC {}

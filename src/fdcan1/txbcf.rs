#[doc = "Register `TXBCF` reader"]
pub type R = crate::R<TXBCF_SPEC>;
#[doc = "Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CF_A {
    #[doc = "0: No transmit buffer cancellation"]
    B_0x0 = 0,
    #[doc = "1: Transmit buffer cancellation finished"]
    B_0x1 = 1,
}
impl From<CF_A> for u8 {
    #[inline(always)]
    fn from(variant: CF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CF_A {
    type Ux = u8;
}
impl crate::IsEnum for CF_A {}
#[doc = "Field `CF` reader - Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
pub type CF_R = crate::FieldReader<CF_A>;
impl CF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CF_A> {
        match self.bits {
            0 => Some(CF_A::B_0x0),
            1 => Some(CF_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "No transmit buffer cancellation"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CF_A::B_0x0
    }
    #[doc = "Transmit buffer cancellation finished"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CF_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn CF(&self) -> CF_R {
        CF_R::new((self.bits & 7) as u8)
    }
}
#[doc = "FDCAN Tx buffer cancellation finished register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCF_SPEC;
impl crate::RegisterSpec for TXBCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcf::R`](R) reader structure"]
impl crate::Readable for TXBCF_SPEC {}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TXBCF_SPEC {}

#[doc = "Register `TXBRP` reader"]
pub type R = crate::R<TXBRP_SPEC>;
#[doc = "Transmission request pending Each Tx buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRP_A {
    #[doc = "0: No transmission request pending"]
    B_0x0 = 0,
    #[doc = "1: Transmission request pending"]
    B_0x1 = 1,
}
impl From<TRP_A> for u8 {
    #[inline(always)]
    fn from(variant: TRP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRP_A {
    type Ux = u8;
}
impl crate::IsEnum for TRP_A {}
#[doc = "Field `TRP` reader - Transmission request pending Each Tx buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions."]
pub type TRP_R = crate::FieldReader<TRP_A>;
impl TRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRP_A> {
        match self.bits {
            0 => Some(TRP_A::B_0x0),
            1 => Some(TRP_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "No transmission request pending"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TRP_A::B_0x0
    }
    #[doc = "Transmission request pending"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TRP_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmission request pending Each Tx buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions."]
    #[inline(always)]
    pub fn TRP(&self) -> TRP_R {
        TRP_R::new((self.bits & 7) as u8)
    }
}
#[doc = "FDCAN Tx buffer request pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBRP_SPEC;
impl crate::RegisterSpec for TXBRP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbrp::R`](R) reader structure"]
impl crate::Readable for TXBRP_SPEC {}
#[doc = "`reset()` method sets TXBRP to value 0"]
impl crate::Resettable for TXBRP_SPEC {}

#[doc = "Register `TXEFS` reader"]
pub type R = crate::R<TXEFS_SPEC>;
#[doc = "Field `EFFL` reader - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3."]
pub type EFFL_R = crate::FieldReader;
#[doc = "Field `EFGI` reader - Event FIFO get index Tx event FIFO read index pointer, range 0 to 3."]
pub type EFGI_R = crate::FieldReader;
#[doc = "Field `EFPI` reader - Event FIFO put index Tx event FIFO write index pointer, range 0 to 3."]
pub type EFPI_R = crate::FieldReader;
#[doc = "Event FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EFF_A {
    #[doc = "0: Tx event FIFO not full"]
    B_0x0 = 0,
    #[doc = "1: Tx event FIFO full"]
    B_0x1 = 1,
}
impl From<EFF_A> for bool {
    #[inline(always)]
    fn from(variant: EFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EFF` reader - Event FIFO full"]
pub type EFF_R = crate::BitReader<EFF_A>;
impl EFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EFF_A {
        match self.bits {
            false => EFF_A::B_0x0,
            true => EFF_A::B_0x1,
        }
    }
    #[doc = "Tx event FIFO not full"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EFF_A::B_0x0
    }
    #[doc = "Tx event FIFO full"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EFF_A::B_0x1
    }
}
#[doc = "Field `TEFL` reader - Tx event FIFO element lost This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\] is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0."]
pub type TEFL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3."]
    #[inline(always)]
    pub fn EFFL(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Event FIFO get index Tx event FIFO read index pointer, range 0 to 3."]
    #[inline(always)]
    pub fn EFGI(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Event FIFO put index Tx event FIFO write index pointer, range 0 to 3."]
    #[inline(always)]
    pub fn EFPI(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Event FIFO full"]
    #[inline(always)]
    pub fn EFF(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx event FIFO element lost This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\] is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0."]
    #[inline(always)]
    pub fn TEFL(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "FDCAN Tx event FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`txefs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEFS_SPEC;
impl crate::RegisterSpec for TXEFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefs::R`](R) reader structure"]
impl crate::Readable for TXEFS_SPEC {}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TXEFS_SPEC {}

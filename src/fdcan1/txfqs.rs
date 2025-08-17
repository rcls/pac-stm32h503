#[doc = "Register `TXFQS` reader"]
pub type R = crate::R<TXFQS_SPEC>;
#[doc = "Field `TFFL` reader - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\] = 1)."]
pub type TFFL_R = crate::FieldReader;
#[doc = "Field `TFGI` reader - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
pub type TFGI_R = crate::FieldReader;
#[doc = "Field `TFQPI` reader - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3"]
pub type TFQPI_R = crate::FieldReader;
#[doc = "Tx FIFO/queue full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFQF_A {
    #[doc = "0: Tx FIFO/queue not full"]
    B_0x0 = 0,
    #[doc = "1: Tx FIFO/queue full"]
    B_0x1 = 1,
}
impl From<TFQF_A> for bool {
    #[inline(always)]
    fn from(variant: TFQF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFQF` reader - Tx FIFO/queue full"]
pub type TFQF_R = crate::BitReader<TFQF_A>;
impl TFQF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFQF_A {
        match self.bits {
            false => TFQF_A::B_0x0,
            true => TFQF_A::B_0x1,
        }
    }
    #[doc = "Tx FIFO/queue not full"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TFQF_A::B_0x0
    }
    #[doc = "Tx FIFO/queue full"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TFQF_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\] = 1)."]
    #[inline(always)]
    pub fn TFFL(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
    #[inline(always)]
    pub fn TFGI(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3"]
    #[inline(always)]
    pub fn TFQPI(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/queue full"]
    #[inline(always)]
    pub fn TFQF(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "FDCAN Tx FIFO/queue status register\n\nYou can [`read`](crate::Reg::read) this register and get [`txfqs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFQS_SPEC;
impl crate::RegisterSpec for TXFQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfqs::R`](R) reader structure"]
impl crate::Readable for TXFQS_SPEC {}
#[doc = "`reset()` method sets TXFQS to value 0x03"]
impl crate::Resettable for TXFQS_SPEC {
    const RESET_VALUE: u32 = 0x03;
}

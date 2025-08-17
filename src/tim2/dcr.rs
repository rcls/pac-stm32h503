#[doc = "Register `DCR` reader"]
pub type R = crate::R<DCR_SPEC>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DCR_SPEC>;
#[doc = "DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBA_A {
    #[doc = "0: TIMx_CR1,"]
    B_0x0 = 0,
    #[doc = "1: TIMx_CR2,"]
    B_0x1 = 1,
    #[doc = "2: TIMx_SMCR,"]
    B_0x2 = 2,
}
impl From<DBA_A> for u8 {
    #[inline(always)]
    fn from(variant: DBA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBA_A {
    type Ux = u8;
}
impl crate::IsEnum for DBA_A {}
#[doc = "Field `DBA` reader - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
pub type DBA_R = crate::FieldReader<DBA_A>;
impl DBA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBA_A> {
        match self.bits {
            0 => Some(DBA_A::B_0x0),
            1 => Some(DBA_A::B_0x1),
            2 => Some(DBA_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "TIMx_CR1,"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBA_A::B_0x0
    }
    #[doc = "TIMx_CR2,"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBA_A::B_0x1
    }
    #[doc = "TIMx_SMCR,"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == DBA_A::B_0x2
    }
}
#[doc = "Field `DBA` writer - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
pub type DBA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DBA_A>;
impl<'a, REG> DBA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIMx_CR1,"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBA_A::B_0x0)
    }
    #[doc = "TIMx_CR2,"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBA_A::B_0x1)
    }
    #[doc = "TIMx_SMCR,"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DBA_A::B_0x2)
    }
}
#[doc = "DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIM2_CR1. If DBL = 7 bytes and DBA = TIM2_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data are copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data are transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data are also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBL_A {
    #[doc = "0: 1 transfer"]
    B_0x0 = 0,
    #[doc = "1: 2 transfers"]
    B_0x1 = 1,
    #[doc = "2: 3 transfers"]
    B_0x2 = 2,
    #[doc = "26: 26 transfers"]
    B_0x1A = 26,
}
impl From<DBL_A> for u8 {
    #[inline(always)]
    fn from(variant: DBL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBL_A {
    type Ux = u8;
}
impl crate::IsEnum for DBL_A {}
#[doc = "Field `DBL` reader - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIM2_CR1. If DBL = 7 bytes and DBA = TIM2_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data are copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data are transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data are also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA."]
pub type DBL_R = crate::FieldReader<DBL_A>;
impl DBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBL_A> {
        match self.bits {
            0 => Some(DBL_A::B_0x0),
            1 => Some(DBL_A::B_0x1),
            2 => Some(DBL_A::B_0x2),
            26 => Some(DBL_A::B_0x1A),
            _ => None,
        }
    }
    #[doc = "1 transfer"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBL_A::B_0x0
    }
    #[doc = "2 transfers"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBL_A::B_0x1
    }
    #[doc = "3 transfers"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == DBL_A::B_0x2
    }
    #[doc = "26 transfers"]
    #[inline(always)]
    pub fn is_B_0x1A(&self) -> bool {
        *self == DBL_A::B_0x1A
    }
}
#[doc = "Field `DBL` writer - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIM2_CR1. If DBL = 7 bytes and DBA = TIM2_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data are copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data are transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data are also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA."]
pub type DBL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DBL_A>;
impl<'a, REG> DBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 transfer"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBL_A::B_0x0)
    }
    #[doc = "2 transfers"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBL_A::B_0x1)
    }
    #[doc = "3 transfers"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DBL_A::B_0x2)
    }
    #[doc = "26 transfers"]
    #[inline(always)]
    pub fn B_0x1A(self) -> &'a mut crate::W<REG> {
        self.variant(DBL_A::B_0x1A)
    }
}
#[doc = "DMA burst source selection This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBSS_A {
    #[doc = "1: Update"]
    B_0x1 = 1,
    #[doc = "2: CC1"]
    B_0x2 = 2,
    #[doc = "3: CC2"]
    B_0x3 = 3,
    #[doc = "4: CC3"]
    B_0x4 = 4,
    #[doc = "5: CC4"]
    B_0x5 = 5,
    #[doc = "6: COM"]
    B_0x6 = 6,
    #[doc = "7: Trigger"]
    B_0x7 = 7,
}
impl From<DBSS_A> for u8 {
    #[inline(always)]
    fn from(variant: DBSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBSS_A {
    type Ux = u8;
}
impl crate::IsEnum for DBSS_A {}
#[doc = "Field `DBSS` reader - DMA burst source selection This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). Others: reserved"]
pub type DBSS_R = crate::FieldReader<DBSS_A>;
impl DBSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBSS_A> {
        match self.bits {
            1 => Some(DBSS_A::B_0x1),
            2 => Some(DBSS_A::B_0x2),
            3 => Some(DBSS_A::B_0x3),
            4 => Some(DBSS_A::B_0x4),
            5 => Some(DBSS_A::B_0x5),
            6 => Some(DBSS_A::B_0x6),
            7 => Some(DBSS_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "Update"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBSS_A::B_0x1
    }
    #[doc = "CC1"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == DBSS_A::B_0x2
    }
    #[doc = "CC2"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == DBSS_A::B_0x3
    }
    #[doc = "CC3"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == DBSS_A::B_0x4
    }
    #[doc = "CC4"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == DBSS_A::B_0x5
    }
    #[doc = "COM"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == DBSS_A::B_0x6
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == DBSS_A::B_0x7
    }
}
#[doc = "Field `DBSS` writer - DMA burst source selection This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). Others: reserved"]
pub type DBSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DBSS_A>;
impl<'a, REG> DBSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Update"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBSS_A::B_0x1)
    }
    #[doc = "CC1"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DBSS_A::B_0x2)
    }
    #[doc = "CC2"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(DBSS_A::B_0x3)
    }
    #[doc = "CC3"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(DBSS_A::B_0x4)
    }
    #[doc = "CC4"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(DBSS_A::B_0x5)
    }
    #[doc = "COM"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(DBSS_A::B_0x6)
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(DBSS_A::B_0x7)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
    #[inline(always)]
    pub fn DBA(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIM2_CR1. If DBL = 7 bytes and DBA = TIM2_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data are copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data are transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data are also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA."]
    #[inline(always)]
    pub fn DBL(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - DMA burst source selection This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). Others: reserved"]
    #[inline(always)]
    pub fn DBSS(&self) -> DBSS_R {
        DBSS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
    #[inline(always)]
    pub fn DBA(&mut self) -> DBA_W<'_, DCR_SPEC> {
        DBA_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIM2_CR1. If DBL = 7 bytes and DBA = TIM2_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data are copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data are transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data are also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA."]
    #[inline(always)]
    pub fn DBL(&mut self) -> DBL_W<'_, DCR_SPEC> {
        DBL_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - DMA burst source selection This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). Others: reserved"]
    #[inline(always)]
    pub fn DBSS(&mut self) -> DBSS_W<'_, DCR_SPEC> {
        DBSS_W::new(self, 16)
    }
}
#[doc = "TIM2 DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCR_SPEC;
impl crate::RegisterSpec for DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DCR_SPEC {}

#[doc = "Register `C3LLR` reader"]
pub type R = crate::R<C3LLR_SPEC>;
#[doc = "Register `C3LLR` writer"]
pub type W = crate::W<C3LLR_SPEC>;
#[doc = "Field `LA` reader - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
pub type LA_R = crate::FieldReader<u16>;
#[doc = "Field `LA` writer - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
pub type LA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULL_A {
    #[doc = "0: no GPDMA_CxLLR update"]
    B_0x0 = 0,
    #[doc = "1: GPDMA_CxLLR update"]
    B_0x1 = 1,
}
impl From<ULL_A> for bool {
    #[inline(always)]
    fn from(variant: ULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULL` reader - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
pub type ULL_R = crate::BitReader<ULL_A>;
impl ULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ULL_A {
        match self.bits {
            false => ULL_A::B_0x0,
            true => ULL_A::B_0x1,
        }
    }
    #[doc = "no GPDMA_CxLLR update"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ULL_A::B_0x0
    }
    #[doc = "GPDMA_CxLLR update"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ULL_A::B_0x1
    }
}
#[doc = "Field `ULL` writer - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
pub type ULL_W<'a, REG> = crate::BitWriter<'a, REG, ULL_A>;
impl<'a, REG> ULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no GPDMA_CxLLR update"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ULL_A::B_0x0)
    }
    #[doc = "GPDMA_CxLLR update"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ULL_A::B_0x1)
    }
}
#[doc = "Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDA_A {
    #[doc = "0: no GPDMA_CxDAR update"]
    B_0x0 = 0,
    #[doc = "1: GPDMA_CxDAR update"]
    B_0x1 = 1,
}
impl From<UDA_A> for bool {
    #[inline(always)]
    fn from(variant: UDA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDA` reader - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
pub type UDA_R = crate::BitReader<UDA_A>;
impl UDA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDA_A {
        match self.bits {
            false => UDA_A::B_0x0,
            true => UDA_A::B_0x1,
        }
    }
    #[doc = "no GPDMA_CxDAR update"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UDA_A::B_0x0
    }
    #[doc = "GPDMA_CxDAR update"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UDA_A::B_0x1
    }
}
#[doc = "Field `UDA` writer - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
pub type UDA_W<'a, REG> = crate::BitWriter<'a, REG, UDA_A>;
impl<'a, REG> UDA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no GPDMA_CxDAR update"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UDA_A::B_0x0)
    }
    #[doc = "GPDMA_CxDAR update"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UDA_A::B_0x1)
    }
}
#[doc = "update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USA_A {
    #[doc = "0: no GPDMA_CxSAR update"]
    B_0x0 = 0,
    #[doc = "1: GPDMA_CxSAR update"]
    B_0x1 = 1,
}
impl From<USA_A> for bool {
    #[inline(always)]
    fn from(variant: USA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USA` reader - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
pub type USA_R = crate::BitReader<USA_A>;
impl USA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USA_A {
        match self.bits {
            false => USA_A::B_0x0,
            true => USA_A::B_0x1,
        }
    }
    #[doc = "no GPDMA_CxSAR update"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USA_A::B_0x0
    }
    #[doc = "GPDMA_CxSAR update"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USA_A::B_0x1
    }
}
#[doc = "Field `USA` writer - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
pub type USA_W<'a, REG> = crate::BitWriter<'a, REG, USA_A>;
impl<'a, REG> USA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no GPDMA_CxSAR update"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USA_A::B_0x0)
    }
    #[doc = "GPDMA_CxSAR update"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USA_A::B_0x1)
    }
}
#[doc = "Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\] is then restored to the programmed value after data transfer is completed and before the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UB1_A {
    #[doc = "0: no GPDMA_CxBR1 update from memory (GPDMA_CxBR1.BNDT\\[15:0\\] restored if any link transfer)"]
    B_0x0 = 0,
    #[doc = "1: GPDMA_CxBR1 update"]
    B_0x1 = 1,
}
impl From<UB1_A> for bool {
    #[inline(always)]
    fn from(variant: UB1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UB1` reader - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\] is then restored to the programmed value after data transfer is completed and before the link transfer."]
pub type UB1_R = crate::BitReader<UB1_A>;
impl UB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UB1_A {
        match self.bits {
            false => UB1_A::B_0x0,
            true => UB1_A::B_0x1,
        }
    }
    #[doc = "no GPDMA_CxBR1 update from memory (GPDMA_CxBR1.BNDT\\[15:0\\] restored if any link transfer)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UB1_A::B_0x0
    }
    #[doc = "GPDMA_CxBR1 update"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UB1_A::B_0x1
    }
}
#[doc = "Field `UB1` writer - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\] is then restored to the programmed value after data transfer is completed and before the link transfer."]
pub type UB1_W<'a, REG> = crate::BitWriter<'a, REG, UB1_A>;
impl<'a, REG> UB1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no GPDMA_CxBR1 update from memory (GPDMA_CxBR1.BNDT\\[15:0\\] restored if any link transfer)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UB1_A::B_0x0)
    }
    #[doc = "GPDMA_CxBR1 update"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UB1_A::B_0x1)
    }
}
#[doc = "Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UT2_A {
    #[doc = "0: no GPDMA_CxTR2 update"]
    B_0x0 = 0,
    #[doc = "1: GPDMA_CxTR2 update"]
    B_0x1 = 1,
}
impl From<UT2_A> for bool {
    #[inline(always)]
    fn from(variant: UT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UT2` reader - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
pub type UT2_R = crate::BitReader<UT2_A>;
impl UT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UT2_A {
        match self.bits {
            false => UT2_A::B_0x0,
            true => UT2_A::B_0x1,
        }
    }
    #[doc = "no GPDMA_CxTR2 update"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UT2_A::B_0x0
    }
    #[doc = "GPDMA_CxTR2 update"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UT2_A::B_0x1
    }
}
#[doc = "Field `UT2` writer - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
pub type UT2_W<'a, REG> = crate::BitWriter<'a, REG, UT2_A>;
impl<'a, REG> UT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no GPDMA_CxTR2 update"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UT2_A::B_0x0)
    }
    #[doc = "GPDMA_CxTR2 update"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UT2_A::B_0x1)
    }
}
#[doc = "Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UT1_A {
    #[doc = "0: no GPDMA_CxTR1 update"]
    B_0x0 = 0,
    #[doc = "1: GPDMA_CxTR1 update"]
    B_0x1 = 1,
}
impl From<UT1_A> for bool {
    #[inline(always)]
    fn from(variant: UT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UT1` reader - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
pub type UT1_R = crate::BitReader<UT1_A>;
impl UT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UT1_A {
        match self.bits {
            false => UT1_A::B_0x0,
            true => UT1_A::B_0x1,
        }
    }
    #[doc = "no GPDMA_CxTR1 update"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UT1_A::B_0x0
    }
    #[doc = "GPDMA_CxTR1 update"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UT1_A::B_0x1
    }
}
#[doc = "Field `UT1` writer - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
pub type UT1_W<'a, REG> = crate::BitWriter<'a, REG, UT1_A>;
impl<'a, REG> UT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no GPDMA_CxTR1 update"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UT1_A::B_0x0)
    }
    #[doc = "GPDMA_CxTR1 update"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UT1_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
    #[inline(always)]
    pub fn LA(&self) -> LA_R {
        LA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
    #[inline(always)]
    pub fn ULL(&self) -> ULL_R {
        ULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 27 - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
    #[inline(always)]
    pub fn UDA(&self) -> UDA_R {
        UDA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
    #[inline(always)]
    pub fn USA(&self) -> USA_R {
        USA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\] is then restored to the programmed value after data transfer is completed and before the link transfer."]
    #[inline(always)]
    pub fn UB1(&self) -> UB1_R {
        UB1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
    #[inline(always)]
    pub fn UT2(&self) -> UT2_R {
        UT2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
    #[inline(always)]
    pub fn UT1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
    #[inline(always)]
    pub fn LA(&mut self) -> LA_W<'_, C3LLR_SPEC> {
        LA_W::new(self, 2)
    }
    #[doc = "Bit 16 - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
    #[inline(always)]
    pub fn ULL(&mut self) -> ULL_W<'_, C3LLR_SPEC> {
        ULL_W::new(self, 16)
    }
    #[doc = "Bit 27 - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
    #[inline(always)]
    pub fn UDA(&mut self) -> UDA_W<'_, C3LLR_SPEC> {
        UDA_W::new(self, 27)
    }
    #[doc = "Bit 28 - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
    #[inline(always)]
    pub fn USA(&mut self) -> USA_W<'_, C3LLR_SPEC> {
        USA_W::new(self, 28)
    }
    #[doc = "Bit 29 - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\] is then restored to the programmed value after data transfer is completed and before the link transfer."]
    #[inline(always)]
    pub fn UB1(&mut self) -> UB1_W<'_, C3LLR_SPEC> {
        UB1_W::new(self, 29)
    }
    #[doc = "Bit 30 - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
    #[inline(always)]
    pub fn UT2(&mut self) -> UT2_W<'_, C3LLR_SPEC> {
        UT2_W::new(self, 30)
    }
    #[doc = "Bit 31 - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
    #[inline(always)]
    pub fn UT1(&mut self) -> UT1_W<'_, C3LLR_SPEC> {
        UT1_W::new(self, 31)
    }
}
#[doc = "GPDMA channel 3 linked-list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3llr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3llr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3LLR_SPEC;
impl crate::RegisterSpec for C3LLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3llr::R`](R) reader structure"]
impl crate::Readable for C3LLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c3llr::W`](W) writer structure"]
impl crate::Writable for C3LLR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets C3LLR to value 0"]
impl crate::Resettable for C3LLR_SPEC {}

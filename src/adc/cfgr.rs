#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled"]
    B_0x0 = 0,
    #[doc = "1: DMA enabled"]
    B_0x1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::B_0x0,
            true => DMAEN_A::B_0x1,
        }
    }
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAEN_A::B_0x0
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAEN_A::B_0x1
    }
}
#[doc = "Field `DMAEN` writer - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN_A>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::B_0x0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::B_0x1)
    }
}
#[doc = "Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG_A {
    #[doc = "0: DMA One Shot mode selected"]
    B_0x0 = 0,
    #[doc = "1: DMA Circular mode selected"]
    B_0x1 = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACFG` reader - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type DMACFG_R = crate::BitReader<DMACFG_A>;
impl DMACFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::B_0x0,
            true => DMACFG_A::B_0x1,
        }
    }
    #[doc = "DMA One Shot mode selected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMACFG_A::B_0x0
    }
    #[doc = "DMA Circular mode selected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMACFG_A::B_0x1
    }
}
#[doc = "Field `DMACFG` writer - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG, DMACFG_A>;
impl<'a, REG> DMACFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA One Shot mode selected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG_A::B_0x0)
    }
    #[doc = "DMA Circular mode selected"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG_A::B_0x1)
    }
}
#[doc = "Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12-bit"]
    B_0x0 = 0,
    #[doc = "1: 10-bit"]
    B_0x1 = 1,
    #[doc = "2: 8-bit"]
    B_0x2 = 2,
    #[doc = "3: 6-bit"]
    B_0x3 = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES_A {
    type Ux = u8;
}
impl crate::IsEnum for RES_A {}
#[doc = "Field `RES` reader - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type RES_R = crate::FieldReader<RES_A>;
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::B_0x0,
            1 => RES_A::B_0x1,
            2 => RES_A::B_0x2,
            3 => RES_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RES_A::B_0x0
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RES_A::B_0x1
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == RES_A::B_0x2
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == RES_A::B_0x3
    }
}
#[doc = "Field `RES` writer - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RES_A, crate::Safe>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::B_0x0)
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::B_0x1)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::B_0x2)
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::B_0x3)
    }
}
#[doc = "External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL_A {
    #[doc = "0: adc_ext_trg0"]
    B_0x0 = 0,
    #[doc = "1: adc_ext_trg1"]
    B_0x1 = 1,
    #[doc = "2: adc_ext_trg2"]
    B_0x2 = 2,
    #[doc = "3: adc_ext_trg3"]
    B_0x3 = 3,
    #[doc = "4: adc_ext_trg4"]
    B_0x4 = 4,
    #[doc = "5: adc_ext_trg5"]
    B_0x5 = 5,
    #[doc = "6: adc_ext_trg6"]
    B_0x6 = 6,
    #[doc = "7: adc_ext_trg7"]
    B_0x7 = 7,
    #[doc = "8: adc_ext_trg8"]
    B_0x8 = 8,
    #[doc = "9: adc_ext_trg9"]
    B_0x9 = 9,
    #[doc = "10: adc_ext_trg10"]
    B_0xA = 10,
    #[doc = "11: adc_ext_trg11"]
    B_0xB = 11,
    #[doc = "12: adc_ext_trg12"]
    B_0xC = 12,
    #[doc = "13: adc_ext_trg13"]
    B_0xD = 13,
    #[doc = "14: adc_ext_trg14"]
    B_0xE = 14,
    #[doc = "15: adc_ext_trg15"]
    B_0xF = 15,
    #[doc = "16: adc_ext_trg16"]
    B_0x10 = 16,
    #[doc = "17: adc_ext_trg17"]
    B_0x11 = 17,
    #[doc = "18: adc_ext_trg18"]
    B_0x12 = 18,
    #[doc = "19: adc_ext_trg19"]
    B_0x13 = 19,
    #[doc = "20: adc_ext_trg20"]
    B_0x14 = 20,
    #[doc = "21: adc_ext_trg21"]
    B_0x15 = 21,
    #[doc = "22: adc_ext_trg22"]
    B_0x16 = 22,
    #[doc = "23: adc_ext_trg23"]
    B_0x17 = 23,
    #[doc = "24: adc_ext_trg24"]
    B_0x18 = 24,
    #[doc = "25: adc_ext_trg25"]
    B_0x19 = 25,
    #[doc = "26: adc_ext_trg26"]
    B_0x1A = 26,
    #[doc = "27: adc_ext_trg27"]
    B_0x1B = 27,
    #[doc = "28: adc_ext_trg28"]
    B_0x1C = 28,
    #[doc = "29: adc_ext_trg29"]
    B_0x1D = 29,
    #[doc = "30: adc_ext_trg30"]
    B_0x1E = 30,
    #[doc = "31: adc_ext_trg31"]
    B_0x1F = 31,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTSEL_A {}
#[doc = "Field `EXTSEL` reader - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL_R = crate::FieldReader<EXTSEL_A>;
impl EXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTSEL_A {
        match self.bits {
            0 => EXTSEL_A::B_0x0,
            1 => EXTSEL_A::B_0x1,
            2 => EXTSEL_A::B_0x2,
            3 => EXTSEL_A::B_0x3,
            4 => EXTSEL_A::B_0x4,
            5 => EXTSEL_A::B_0x5,
            6 => EXTSEL_A::B_0x6,
            7 => EXTSEL_A::B_0x7,
            8 => EXTSEL_A::B_0x8,
            9 => EXTSEL_A::B_0x9,
            10 => EXTSEL_A::B_0xA,
            11 => EXTSEL_A::B_0xB,
            12 => EXTSEL_A::B_0xC,
            13 => EXTSEL_A::B_0xD,
            14 => EXTSEL_A::B_0xE,
            15 => EXTSEL_A::B_0xF,
            16 => EXTSEL_A::B_0x10,
            17 => EXTSEL_A::B_0x11,
            18 => EXTSEL_A::B_0x12,
            19 => EXTSEL_A::B_0x13,
            20 => EXTSEL_A::B_0x14,
            21 => EXTSEL_A::B_0x15,
            22 => EXTSEL_A::B_0x16,
            23 => EXTSEL_A::B_0x17,
            24 => EXTSEL_A::B_0x18,
            25 => EXTSEL_A::B_0x19,
            26 => EXTSEL_A::B_0x1A,
            27 => EXTSEL_A::B_0x1B,
            28 => EXTSEL_A::B_0x1C,
            29 => EXTSEL_A::B_0x1D,
            30 => EXTSEL_A::B_0x1E,
            31 => EXTSEL_A::B_0x1F,
            _ => unreachable!(),
        }
    }
    #[doc = "adc_ext_trg0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTSEL_A::B_0x0
    }
    #[doc = "adc_ext_trg1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTSEL_A::B_0x1
    }
    #[doc = "adc_ext_trg2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTSEL_A::B_0x2
    }
    #[doc = "adc_ext_trg3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == EXTSEL_A::B_0x3
    }
    #[doc = "adc_ext_trg4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == EXTSEL_A::B_0x4
    }
    #[doc = "adc_ext_trg5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == EXTSEL_A::B_0x5
    }
    #[doc = "adc_ext_trg6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == EXTSEL_A::B_0x6
    }
    #[doc = "adc_ext_trg7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == EXTSEL_A::B_0x7
    }
    #[doc = "adc_ext_trg8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == EXTSEL_A::B_0x8
    }
    #[doc = "adc_ext_trg9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == EXTSEL_A::B_0x9
    }
    #[doc = "adc_ext_trg10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == EXTSEL_A::B_0xA
    }
    #[doc = "adc_ext_trg11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == EXTSEL_A::B_0xB
    }
    #[doc = "adc_ext_trg12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == EXTSEL_A::B_0xC
    }
    #[doc = "adc_ext_trg13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == EXTSEL_A::B_0xD
    }
    #[doc = "adc_ext_trg14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == EXTSEL_A::B_0xE
    }
    #[doc = "adc_ext_trg15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == EXTSEL_A::B_0xF
    }
    #[doc = "adc_ext_trg16"]
    #[inline(always)]
    pub fn is_B_0x10(&self) -> bool {
        *self == EXTSEL_A::B_0x10
    }
    #[doc = "adc_ext_trg17"]
    #[inline(always)]
    pub fn is_B_0x11(&self) -> bool {
        *self == EXTSEL_A::B_0x11
    }
    #[doc = "adc_ext_trg18"]
    #[inline(always)]
    pub fn is_B_0x12(&self) -> bool {
        *self == EXTSEL_A::B_0x12
    }
    #[doc = "adc_ext_trg19"]
    #[inline(always)]
    pub fn is_B_0x13(&self) -> bool {
        *self == EXTSEL_A::B_0x13
    }
    #[doc = "adc_ext_trg20"]
    #[inline(always)]
    pub fn is_B_0x14(&self) -> bool {
        *self == EXTSEL_A::B_0x14
    }
    #[doc = "adc_ext_trg21"]
    #[inline(always)]
    pub fn is_B_0x15(&self) -> bool {
        *self == EXTSEL_A::B_0x15
    }
    #[doc = "adc_ext_trg22"]
    #[inline(always)]
    pub fn is_B_0x16(&self) -> bool {
        *self == EXTSEL_A::B_0x16
    }
    #[doc = "adc_ext_trg23"]
    #[inline(always)]
    pub fn is_B_0x17(&self) -> bool {
        *self == EXTSEL_A::B_0x17
    }
    #[doc = "adc_ext_trg24"]
    #[inline(always)]
    pub fn is_B_0x18(&self) -> bool {
        *self == EXTSEL_A::B_0x18
    }
    #[doc = "adc_ext_trg25"]
    #[inline(always)]
    pub fn is_B_0x19(&self) -> bool {
        *self == EXTSEL_A::B_0x19
    }
    #[doc = "adc_ext_trg26"]
    #[inline(always)]
    pub fn is_B_0x1A(&self) -> bool {
        *self == EXTSEL_A::B_0x1A
    }
    #[doc = "adc_ext_trg27"]
    #[inline(always)]
    pub fn is_B_0x1B(&self) -> bool {
        *self == EXTSEL_A::B_0x1B
    }
    #[doc = "adc_ext_trg28"]
    #[inline(always)]
    pub fn is_B_0x1C(&self) -> bool {
        *self == EXTSEL_A::B_0x1C
    }
    #[doc = "adc_ext_trg29"]
    #[inline(always)]
    pub fn is_B_0x1D(&self) -> bool {
        *self == EXTSEL_A::B_0x1D
    }
    #[doc = "adc_ext_trg30"]
    #[inline(always)]
    pub fn is_B_0x1E(&self) -> bool {
        *self == EXTSEL_A::B_0x1E
    }
    #[doc = "adc_ext_trg31"]
    #[inline(always)]
    pub fn is_B_0x1F(&self) -> bool {
        *self == EXTSEL_A::B_0x1F
    }
}
#[doc = "Field `EXTSEL` writer - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, EXTSEL_A, crate::Safe>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "adc_ext_trg0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x0)
    }
    #[doc = "adc_ext_trg1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x1)
    }
    #[doc = "adc_ext_trg2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x2)
    }
    #[doc = "adc_ext_trg3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x3)
    }
    #[doc = "adc_ext_trg4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x4)
    }
    #[doc = "adc_ext_trg5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x5)
    }
    #[doc = "adc_ext_trg6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x6)
    }
    #[doc = "adc_ext_trg7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x7)
    }
    #[doc = "adc_ext_trg8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x8)
    }
    #[doc = "adc_ext_trg9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x9)
    }
    #[doc = "adc_ext_trg10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0xA)
    }
    #[doc = "adc_ext_trg11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0xB)
    }
    #[doc = "adc_ext_trg12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0xC)
    }
    #[doc = "adc_ext_trg13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0xD)
    }
    #[doc = "adc_ext_trg14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0xE)
    }
    #[doc = "adc_ext_trg15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0xF)
    }
    #[doc = "adc_ext_trg16"]
    #[inline(always)]
    pub fn B_0x10(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x10)
    }
    #[doc = "adc_ext_trg17"]
    #[inline(always)]
    pub fn B_0x11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x11)
    }
    #[doc = "adc_ext_trg18"]
    #[inline(always)]
    pub fn B_0x12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x12)
    }
    #[doc = "adc_ext_trg19"]
    #[inline(always)]
    pub fn B_0x13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x13)
    }
    #[doc = "adc_ext_trg20"]
    #[inline(always)]
    pub fn B_0x14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x14)
    }
    #[doc = "adc_ext_trg21"]
    #[inline(always)]
    pub fn B_0x15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x15)
    }
    #[doc = "adc_ext_trg22"]
    #[inline(always)]
    pub fn B_0x16(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x16)
    }
    #[doc = "adc_ext_trg23"]
    #[inline(always)]
    pub fn B_0x17(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x17)
    }
    #[doc = "adc_ext_trg24"]
    #[inline(always)]
    pub fn B_0x18(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x18)
    }
    #[doc = "adc_ext_trg25"]
    #[inline(always)]
    pub fn B_0x19(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x19)
    }
    #[doc = "adc_ext_trg26"]
    #[inline(always)]
    pub fn B_0x1A(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x1A)
    }
    #[doc = "adc_ext_trg27"]
    #[inline(always)]
    pub fn B_0x1B(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x1B)
    }
    #[doc = "adc_ext_trg28"]
    #[inline(always)]
    pub fn B_0x1C(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x1C)
    }
    #[doc = "adc_ext_trg29"]
    #[inline(always)]
    pub fn B_0x1D(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x1D)
    }
    #[doc = "adc_ext_trg30"]
    #[inline(always)]
    pub fn B_0x1E(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x1E)
    }
    #[doc = "adc_ext_trg31"]
    #[inline(always)]
    pub fn B_0x1F(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0x1F)
    }
}
#[doc = "External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN_A {
    #[doc = "0: Hardware trigger detection disabled (conversions can be launched by software)"]
    B_0x0 = 0,
    #[doc = "1: Hardware trigger detection on the rising edge"]
    B_0x1 = 1,
    #[doc = "2: Hardware trigger detection on the falling edge"]
    B_0x2 = 2,
    #[doc = "3: Hardware trigger detection on both the rising and falling edges"]
    B_0x3 = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTEN_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTEN_A {}
#[doc = "Field `EXTEN` reader - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTEN_R = crate::FieldReader<EXTEN_A>;
impl EXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::B_0x0,
            1 => EXTEN_A::B_0x1,
            2 => EXTEN_A::B_0x2,
            3 => EXTEN_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Hardware trigger detection disabled (conversions can be launched by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTEN_A::B_0x0
    }
    #[doc = "Hardware trigger detection on the rising edge"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTEN_A::B_0x1
    }
    #[doc = "Hardware trigger detection on the falling edge"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTEN_A::B_0x2
    }
    #[doc = "Hardware trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == EXTEN_A::B_0x3
    }
}
#[doc = "Field `EXTEN` writer - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTEN_A, crate::Safe>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Hardware trigger detection disabled (conversions can be launched by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN_A::B_0x0)
    }
    #[doc = "Hardware trigger detection on the rising edge"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN_A::B_0x1)
    }
    #[doc = "Hardware trigger detection on the falling edge"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN_A::B_0x2)
    }
    #[doc = "Hardware trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN_A::B_0x3)
    }
}
#[doc = "Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD_A {
    #[doc = "0: ADC_DR register is preserved with the old data when an overrun is detected."]
    B_0x0 = 0,
    #[doc = "1: ADC_DR register is overwritten with the last conversion result when an overrun is detected."]
    B_0x1 = 1,
}
impl From<OVRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRMOD` reader - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type OVRMOD_R = crate::BitReader<OVRMOD_A>;
impl OVRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRMOD_A {
        match self.bits {
            false => OVRMOD_A::B_0x0,
            true => OVRMOD_A::B_0x1,
        }
    }
    #[doc = "ADC_DR register is preserved with the old data when an overrun is detected."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OVRMOD_A::B_0x0
    }
    #[doc = "ADC_DR register is overwritten with the last conversion result when an overrun is detected."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OVRMOD_A::B_0x1
    }
}
#[doc = "Field `OVRMOD` writer - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG, OVRMOD_A>;
impl<'a, REG> OVRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC_DR register is preserved with the old data when an overrun is detected."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD_A::B_0x0)
    }
    #[doc = "ADC_DR register is overwritten with the last conversion result when an overrun is detected."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD_A::B_0x1)
    }
}
#[doc = "Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    #[doc = "0: Single conversion mode"]
    B_0x0 = 0,
    #[doc = "1: Continuous conversion mode"]
    B_0x1 = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type CONT_R = crate::BitReader<CONT_A>;
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::B_0x0,
            true => CONT_A::B_0x1,
        }
    }
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CONT_A::B_0x0
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CONT_A::B_0x1
    }
}
#[doc = "Field `CONT` writer - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT_A>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CONT_A::B_0x0)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CONT_A::B_0x1)
    }
}
#[doc = "Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTDLY_A {
    #[doc = "0: Auto-delayed conversion mode off"]
    B_0x0 = 0,
    #[doc = "1: Auto-delayed conversion mode on"]
    B_0x1 = 1,
}
impl From<AUTDLY_A> for bool {
    #[inline(always)]
    fn from(variant: AUTDLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTDLY` reader - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AUTDLY_R = crate::BitReader<AUTDLY_A>;
impl AUTDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUTDLY_A {
        match self.bits {
            false => AUTDLY_A::B_0x0,
            true => AUTDLY_A::B_0x1,
        }
    }
    #[doc = "Auto-delayed conversion mode off"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AUTDLY_A::B_0x0
    }
    #[doc = "Auto-delayed conversion mode on"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AUTDLY_A::B_0x1
    }
}
#[doc = "Field `AUTDLY` writer - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG, AUTDLY_A>;
impl<'a, REG> AUTDLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto-delayed conversion mode off"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AUTDLY_A::B_0x0)
    }
    #[doc = "Auto-delayed conversion mode on"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AUTDLY_A::B_0x1)
    }
}
#[doc = "Data alignment This bit is set and cleared by software to select right or left alignment. Refer to register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    #[doc = "0: Right alignment"]
    B_0x0 = 0,
    #[doc = "1: Left alignment"]
    B_0x1 = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN` reader - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
impl ALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::B_0x0,
            true => ALIGN_A::B_0x1,
        }
    }
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ALIGN_A::B_0x0
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ALIGN_A::B_0x1
    }
}
#[doc = "Field `ALIGN` writer - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG, ALIGN_A>;
impl<'a, REG> ALIGN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN_A::B_0x0)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN_A::B_0x1)
    }
}
#[doc = "Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN_A {
    #[doc = "0: Discontinuous mode for regular channels disabled"]
    B_0x0 = 0,
    #[doc = "1: Discontinuous mode for regular channels enabled"]
    B_0x1 = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCEN` reader - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type DISCEN_R = crate::BitReader<DISCEN_A>;
impl DISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::B_0x0,
            true => DISCEN_A::B_0x1,
        }
    }
    #[doc = "Discontinuous mode for regular channels disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DISCEN_A::B_0x0
    }
    #[doc = "Discontinuous mode for regular channels enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DISCEN_A::B_0x1
    }
}
#[doc = "Field `DISCEN` writer - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG, DISCEN_A>;
impl<'a, REG> DISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode for regular channels disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN_A::B_0x0)
    }
    #[doc = "Discontinuous mode for regular channels enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN_A::B_0x1)
    }
}
#[doc = "Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISCNUM_A {
    #[doc = "0: 1 channel"]
    B_0x0 = 0,
    #[doc = "1: 2 channels"]
    B_0x1 = 1,
    #[doc = "7: 8 channels"]
    B_0x7 = 7,
}
impl From<DISCNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: DISCNUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DISCNUM_A {
    type Ux = u8;
}
impl crate::IsEnum for DISCNUM_A {}
#[doc = "Field `DISCNUM` reader - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type DISCNUM_R = crate::FieldReader<DISCNUM_A>;
impl DISCNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DISCNUM_A> {
        match self.bits {
            0 => Some(DISCNUM_A::B_0x0),
            1 => Some(DISCNUM_A::B_0x1),
            7 => Some(DISCNUM_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "1 channel"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DISCNUM_A::B_0x0
    }
    #[doc = "2 channels"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DISCNUM_A::B_0x1
    }
    #[doc = "8 channels"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == DISCNUM_A::B_0x7
    }
}
#[doc = "Field `DISCNUM` writer - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DISCNUM_A>;
impl<'a, REG> DISCNUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 channel"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM_A::B_0x0)
    }
    #[doc = "2 channels"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM_A::B_0x1)
    }
    #[doc = "8 channels"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(DISCNUM_A::B_0x7)
    }
}
#[doc = "Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDISCEN_A {
    #[doc = "0: Discontinuous mode on injected channels disabled"]
    B_0x0 = 0,
    #[doc = "1: Discontinuous mode on injected channels enabled"]
    B_0x1 = 1,
}
impl From<JDISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JDISCEN` reader - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set."]
pub type JDISCEN_R = crate::BitReader<JDISCEN_A>;
impl JDISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JDISCEN_A {
        match self.bits {
            false => JDISCEN_A::B_0x0,
            true => JDISCEN_A::B_0x1,
        }
    }
    #[doc = "Discontinuous mode on injected channels disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JDISCEN_A::B_0x0
    }
    #[doc = "Discontinuous mode on injected channels enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JDISCEN_A::B_0x1
    }
}
#[doc = "Field `JDISCEN` writer - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set."]
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG, JDISCEN_A>;
impl<'a, REG> JDISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode on injected channels disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN_A::B_0x0)
    }
    #[doc = "Discontinuous mode on injected channels enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN_A::B_0x1)
    }
}
#[doc = "JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQM_A {
    #[doc = "0: JSQR mode 0: The Queue is never empty and maintains the last written configuration into JSQR."]
    B_0x0 = 0,
    #[doc = "1: JSQR mode 1: The Queue can be empty and when this occurs, the software and hardware triggers of the injected sequence are both internally disabled just after the completion of the last valid injected sequence."]
    B_0x1 = 1,
}
impl From<JQM_A> for bool {
    #[inline(always)]
    fn from(variant: JQM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQM` reader - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JQM_R = crate::BitReader<JQM_A>;
impl JQM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JQM_A {
        match self.bits {
            false => JQM_A::B_0x0,
            true => JQM_A::B_0x1,
        }
    }
    #[doc = "JSQR mode 0: The Queue is never empty and maintains the last written configuration into JSQR."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JQM_A::B_0x0
    }
    #[doc = "JSQR mode 1: The Queue can be empty and when this occurs, the software and hardware triggers of the injected sequence are both internally disabled just after the completion of the last valid injected sequence."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JQM_A::B_0x1
    }
}
#[doc = "Field `JQM` writer - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JQM_W<'a, REG> = crate::BitWriter<'a, REG, JQM_A>;
impl<'a, REG> JQM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "JSQR mode 0: The Queue is never empty and maintains the last written configuration into JSQR."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JQM_A::B_0x0)
    }
    #[doc = "JSQR mode 1: The Queue can be empty and when this occurs, the software and hardware triggers of the injected sequence are both internally disabled just after the completion of the last valid injected sequence."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JQM_A::B_0x1)
    }
}
#[doc = "Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\\[4:0\\] bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL_A {
    #[doc = "0: Analog watchdog 1 enabled on all channels"]
    B_0x0 = 0,
    #[doc = "1: Analog watchdog 1 enabled on a single channel"]
    B_0x1 = 1,
}
impl From<AWD1SGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1SGL` reader - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\\[4:0\\] bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD1SGL_R = crate::BitReader<AWD1SGL_A>;
impl AWD1SGL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1SGL_A {
        match self.bits {
            false => AWD1SGL_A::B_0x0,
            true => AWD1SGL_A::B_0x1,
        }
    }
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AWD1SGL_A::B_0x0
    }
    #[doc = "Analog watchdog 1 enabled on a single channel"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AWD1SGL_A::B_0x1
    }
}
#[doc = "Field `AWD1SGL` writer - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\\[4:0\\] bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG, AWD1SGL_A>;
impl<'a, REG> AWD1SGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL_A::B_0x0)
    }
    #[doc = "Analog watchdog 1 enabled on a single channel"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL_A::B_0x1)
    }
}
#[doc = "Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN_A {
    #[doc = "0: Analog watchdog 1 disabled on regular channels"]
    B_0x0 = 0,
    #[doc = "1: Analog watchdog 1 enabled on regular channels"]
    B_0x1 = 1,
}
impl From<AWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1EN` reader - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type AWD1EN_R = crate::BitReader<AWD1EN_A>;
impl AWD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1EN_A {
        match self.bits {
            false => AWD1EN_A::B_0x0,
            true => AWD1EN_A::B_0x1,
        }
    }
    #[doc = "Analog watchdog 1 disabled on regular channels"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AWD1EN_A::B_0x0
    }
    #[doc = "Analog watchdog 1 enabled on regular channels"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AWD1EN_A::B_0x1
    }
}
#[doc = "Field `AWD1EN` writer - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, AWD1EN_A>;
impl<'a, REG> AWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 disabled on regular channels"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN_A::B_0x0)
    }
    #[doc = "Analog watchdog 1 enabled on regular channels"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN_A::B_0x1)
    }
}
#[doc = "Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAWD1EN_A {
    #[doc = "0: Analog watchdog 1 disabled on injected channels"]
    B_0x0 = 0,
    #[doc = "1: Analog watchdog 1 enabled on injected channels"]
    B_0x1 = 1,
}
impl From<JAWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: JAWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAWD1EN` reader - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JAWD1EN_R = crate::BitReader<JAWD1EN_A>;
impl JAWD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JAWD1EN_A {
        match self.bits {
            false => JAWD1EN_A::B_0x0,
            true => JAWD1EN_A::B_0x1,
        }
    }
    #[doc = "Analog watchdog 1 disabled on injected channels"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JAWD1EN_A::B_0x0
    }
    #[doc = "Analog watchdog 1 enabled on injected channels"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JAWD1EN_A::B_0x1
    }
}
#[doc = "Field `JAWD1EN` writer - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, JAWD1EN_A>;
impl<'a, REG> JAWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 disabled on injected channels"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JAWD1EN_A::B_0x0)
    }
    #[doc = "Analog watchdog 1 enabled on injected channels"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JAWD1EN_A::B_0x1)
    }
}
#[doc = "Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAUTO_A {
    #[doc = "0: Automatic injected group conversion disabled"]
    B_0x0 = 0,
    #[doc = "1: Automatic injected group conversion enabled"]
    B_0x1 = 1,
}
impl From<JAUTO_A> for bool {
    #[inline(always)]
    fn from(variant: JAUTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAUTO` reader - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing)."]
pub type JAUTO_R = crate::BitReader<JAUTO_A>;
impl JAUTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JAUTO_A {
        match self.bits {
            false => JAUTO_A::B_0x0,
            true => JAUTO_A::B_0x1,
        }
    }
    #[doc = "Automatic injected group conversion disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JAUTO_A::B_0x0
    }
    #[doc = "Automatic injected group conversion enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JAUTO_A::B_0x1
    }
}
#[doc = "Field `JAUTO` writer - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing)."]
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG, JAUTO_A>;
impl<'a, REG> JAUTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic injected group conversion disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO_A::B_0x0)
    }
    #[doc = "Automatic injected group conversion enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO_A::B_0x1)
    }
}
#[doc = "Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\\[4:0\\] setting to the reset value. The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWD1CH_A {
    #[doc = "0: ADC analog input channel 0 monitored by AWD1"]
    B_0x0 = 0,
    #[doc = "1: ADC analog input channel 1 monitored by AWD1"]
    B_0x1 = 1,
    #[doc = "19: ADC analog input channel 19 monitored by AWD1"]
    B_0x13 = 19,
}
impl From<AWD1CH_A> for u8 {
    #[inline(always)]
    fn from(variant: AWD1CH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWD1CH_A {
    type Ux = u8;
}
impl crate::IsEnum for AWD1CH_A {}
#[doc = "Field `AWD1CH` reader - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\\[4:0\\] setting to the reset value. The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD1CH_R = crate::FieldReader<AWD1CH_A>;
impl AWD1CH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AWD1CH_A> {
        match self.bits {
            0 => Some(AWD1CH_A::B_0x0),
            1 => Some(AWD1CH_A::B_0x1),
            19 => Some(AWD1CH_A::B_0x13),
            _ => None,
        }
    }
    #[doc = "ADC analog input channel 0 monitored by AWD1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AWD1CH_A::B_0x0
    }
    #[doc = "ADC analog input channel 1 monitored by AWD1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AWD1CH_A::B_0x1
    }
    #[doc = "ADC analog input channel 19 monitored by AWD1"]
    #[inline(always)]
    pub fn is_B_0x13(&self) -> bool {
        *self == AWD1CH_A::B_0x13
    }
}
#[doc = "Field `AWD1CH` writer - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\\[4:0\\] setting to the reset value. The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, AWD1CH_A>;
impl<'a, REG> AWD1CH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC analog input channel 0 monitored by AWD1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1CH_A::B_0x0)
    }
    #[doc = "ADC analog input channel 1 monitored by AWD1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1CH_A::B_0x1)
    }
    #[doc = "ADC analog input channel 19 monitored by AWD1"]
    #[inline(always)]
    pub fn B_0x13(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1CH_A::B_0x13)
    }
}
#[doc = "Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism : Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQDIS_A {
    #[doc = "0: Injected Queue enabled"]
    B_0x0 = 0,
    #[doc = "1: Injected Queue disabled"]
    B_0x1 = 1,
}
impl From<JQDIS_A> for bool {
    #[inline(always)]
    fn from(variant: JQDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQDIS` reader - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism : Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared."]
pub type JQDIS_R = crate::BitReader<JQDIS_A>;
impl JQDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JQDIS_A {
        match self.bits {
            false => JQDIS_A::B_0x0,
            true => JQDIS_A::B_0x1,
        }
    }
    #[doc = "Injected Queue enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JQDIS_A::B_0x0
    }
    #[doc = "Injected Queue disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JQDIS_A::B_0x1
    }
}
#[doc = "Field `JQDIS` writer - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism : Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared."]
pub type JQDIS_W<'a, REG> = crate::BitWriter<'a, REG, JQDIS_A>;
impl<'a, REG> JQDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected Queue enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JQDIS_A::B_0x0)
    }
    #[doc = "Injected Queue disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JQDIS_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn DMAEN(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn DMACFG(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn RES(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:9 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn EXTSEL(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn EXTEN(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn OVRMOD(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn CONT(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AUTDLY(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ALIGN(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn DISCEN(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn DISCNUM(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set."]
    #[inline(always)]
    pub fn JDISCEN(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JQM(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\\[4:0\\] bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD1SGL(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD1EN(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JAWD1EN(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JAUTO(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\\[4:0\\] setting to the reset value. The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD1CH(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism : Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared."]
    #[inline(always)]
    pub fn JQDIS(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn DMAEN(&mut self) -> DMAEN_W<'_, CFGR_SPEC> {
        DMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn DMACFG(&mut self) -> DMACFG_W<'_, CFGR_SPEC> {
        DMACFG_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn RES(&mut self) -> RES_W<'_, CFGR_SPEC> {
        RES_W::new(self, 3)
    }
    #[doc = "Bits 5:9 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn EXTSEL(&mut self) -> EXTSEL_W<'_, CFGR_SPEC> {
        EXTSEL_W::new(self, 5)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn EXTEN(&mut self) -> EXTEN_W<'_, CFGR_SPEC> {
        EXTEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn OVRMOD(&mut self) -> OVRMOD_W<'_, CFGR_SPEC> {
        OVRMOD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn CONT(&mut self) -> CONT_W<'_, CFGR_SPEC> {
        CONT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AUTDLY(&mut self) -> AUTDLY_W<'_, CFGR_SPEC> {
        AUTDLY_W::new(self, 14)
    }
    #[doc = "Bit 15 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ALIGN(&mut self) -> ALIGN_W<'_, CFGR_SPEC> {
        ALIGN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn DISCEN(&mut self) -> DISCEN_W<'_, CFGR_SPEC> {
        DISCEN_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn DISCNUM(&mut self) -> DISCNUM_W<'_, CFGR_SPEC> {
        DISCNUM_W::new(self, 17)
    }
    #[doc = "Bit 20 - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set."]
    #[inline(always)]
    pub fn JDISCEN(&mut self) -> JDISCEN_W<'_, CFGR_SPEC> {
        JDISCEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JQM(&mut self) -> JQM_W<'_, CFGR_SPEC> {
        JQM_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\\[4:0\\] bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD1SGL(&mut self) -> AWD1SGL_W<'_, CFGR_SPEC> {
        AWD1SGL_W::new(self, 22)
    }
    #[doc = "Bit 23 - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD1EN(&mut self) -> AWD1EN_W<'_, CFGR_SPEC> {
        AWD1EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JAWD1EN(&mut self) -> JAWD1EN_W<'_, CFGR_SPEC> {
        JAWD1EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JAUTO(&mut self) -> JAUTO_W<'_, CFGR_SPEC> {
        JAUTO_W::new(self, 25)
    }
    #[doc = "Bits 26:30 - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\\[4:0\\] setting to the reset value. The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD1CH(&mut self) -> AWD1CH_W<'_, CFGR_SPEC> {
        AWD1CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism : Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared."]
    #[inline(always)]
    pub fn JQDIS(&mut self) -> JQDIS_W<'_, CFGR_SPEC> {
        JQDIS_W::new(self, 31)
    }
}
#[doc = "ADC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR to value 0x8000_0000"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}

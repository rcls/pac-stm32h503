#[doc = "Register `BCDR` reader"]
pub type R = crate::R<BCDR_SPEC>;
#[doc = "Register `BCDR` writer"]
pub type W = crate::W<BCDR_SPEC>;
#[doc = "Field `BCDEN` reader - Battery charging detector (BCD) enable Device mode This bit is set by the software to enable the BCD support within the USB Device. When enabled, the USB PHY is fully controlled by BCD and cannot be used for normal communication. Once the BCD discovery is finished, the BCD should be placed in OFF mode by clearing this bit to 0 in order to allow the normal USB operation."]
pub type BCDEN_R = crate::BitReader;
#[doc = "Field `BCDEN` writer - Battery charging detector (BCD) enable Device mode This bit is set by the software to enable the BCD support within the USB Device. When enabled, the USB PHY is fully controlled by BCD and cannot be used for normal communication. Once the BCD discovery is finished, the BCD should be placed in OFF mode by clearing this bit to 0 in order to allow the normal USB operation."]
pub type BCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDEN` reader - Data contact detection (DCD) mode enable Device mode This bit is set by the software to put the BCD into DCD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type DCDEN_R = crate::BitReader;
#[doc = "Field `DCDEN` writer - Data contact detection (DCD) mode enable Device mode This bit is set by the software to put the BCD into DCD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type DCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - Primary detection (PD) mode enable Device mode This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type PDEN_R = crate::BitReader;
#[doc = "Field `PDEN` writer - Primary detection (PD) mode enable Device mode This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEN` reader - Secondary detection (SD) mode enable Device mode This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type SDEN_R = crate::BitReader;
#[doc = "Field `SDEN` writer - Secondary detection (SD) mode enable Device mode This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data contact detection (DCD) status Device mode This bit gives the result of DCD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDET_A {
    #[doc = "0: data lines contact not detected."]
    B_0x0 = 0,
    #[doc = "1: data lines contact detected."]
    B_0x1 = 1,
}
impl From<DCDET_A> for bool {
    #[inline(always)]
    fn from(variant: DCDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDET` reader - Data contact detection (DCD) status Device mode This bit gives the result of DCD."]
pub type DCDET_R = crate::BitReader<DCDET_A>;
impl DCDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCDET_A {
        match self.bits {
            false => DCDET_A::B_0x0,
            true => DCDET_A::B_0x1,
        }
    }
    #[doc = "data lines contact not detected."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DCDET_A::B_0x0
    }
    #[doc = "data lines contact detected."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DCDET_A::B_0x1
    }
}
#[doc = "Primary detection (PD) status Device mode This bit gives the result of PD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDET_A {
    #[doc = "0: no BCD support detected (connected to SDP or proprietary device)."]
    B_0x0 = 0,
    #[doc = "1: BCD support detected (connected to ACA, CDP or DCP)."]
    B_0x1 = 1,
}
impl From<PDET_A> for bool {
    #[inline(always)]
    fn from(variant: PDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDET` reader - Primary detection (PD) status Device mode This bit gives the result of PD."]
pub type PDET_R = crate::BitReader<PDET_A>;
impl PDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDET_A {
        match self.bits {
            false => PDET_A::B_0x0,
            true => PDET_A::B_0x1,
        }
    }
    #[doc = "no BCD support detected (connected to SDP or proprietary device)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PDET_A::B_0x0
    }
    #[doc = "BCD support detected (connected to ACA, CDP or DCP)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PDET_A::B_0x1
    }
}
#[doc = "Secondary detection (SD) status Device mode This bit gives the result of SD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDET_A {
    #[doc = "0: CDP detected."]
    B_0x0 = 0,
    #[doc = "1: DCP detected."]
    B_0x1 = 1,
}
impl From<SDET_A> for bool {
    #[inline(always)]
    fn from(variant: SDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDET` reader - Secondary detection (SD) status Device mode This bit gives the result of SD."]
pub type SDET_R = crate::BitReader<SDET_A>;
impl SDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDET_A {
        match self.bits {
            false => SDET_A::B_0x0,
            true => SDET_A::B_0x1,
        }
    }
    #[doc = "CDP detected."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SDET_A::B_0x0
    }
    #[doc = "DCP detected."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SDET_A::B_0x1
    }
}
#[doc = "DM pull-up detection status Device mode This bit is active only during PD and gives the result of comparison between DM voltage level and VLGC threshold. In normal situation, the DM level should be below this threshold. If it is above, it means that the DM is externally pulled high. This can be caused by connection to a PS2 port (which pulls-up both DP and DM lines) or to some proprietary charger not following the BCD specification.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS2DET_A {
    #[doc = "0: Normal port detected (connected to SDP, ACA, CDP or DCP)."]
    B_0x0 = 0,
    #[doc = "1: PS2 port or proprietary charger detected."]
    B_0x1 = 1,
}
impl From<PS2DET_A> for bool {
    #[inline(always)]
    fn from(variant: PS2DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS2DET` reader - DM pull-up detection status Device mode This bit is active only during PD and gives the result of comparison between DM voltage level and VLGC threshold. In normal situation, the DM level should be below this threshold. If it is above, it means that the DM is externally pulled high. This can be caused by connection to a PS2 port (which pulls-up both DP and DM lines) or to some proprietary charger not following the BCD specification."]
pub type PS2DET_R = crate::BitReader<PS2DET_A>;
impl PS2DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PS2DET_A {
        match self.bits {
            false => PS2DET_A::B_0x0,
            true => PS2DET_A::B_0x1,
        }
    }
    #[doc = "Normal port detected (connected to SDP, ACA, CDP or DCP)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PS2DET_A::B_0x0
    }
    #[doc = "PS2 port or proprietary charger detected."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PS2DET_A::B_0x1
    }
}
#[doc = "Field `DPPU_DPD` reader - DP pull-up / DPDM pull-down Device mode This bit is set by software to enable the embedded pull-up on DP line. Clearing it to 0 can be used to signal disconnect to the host when needed by the user software. Host mode This bit is set by software to enable the embedded pull-down on DP and DM lines."]
pub type DPPU_DPD_R = crate::BitReader;
#[doc = "Field `DPPU_DPD` writer - DP pull-up / DPDM pull-down Device mode This bit is set by software to enable the embedded pull-up on DP line. Clearing it to 0 can be used to signal disconnect to the host when needed by the user software. Host mode This bit is set by software to enable the embedded pull-down on DP and DM lines."]
pub type DPPU_DPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Battery charging detector (BCD) enable Device mode This bit is set by the software to enable the BCD support within the USB Device. When enabled, the USB PHY is fully controlled by BCD and cannot be used for normal communication. Once the BCD discovery is finished, the BCD should be placed in OFF mode by clearing this bit to 0 in order to allow the normal USB operation."]
    #[inline(always)]
    pub fn BCDEN(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data contact detection (DCD) mode enable Device mode This bit is set by the software to put the BCD into DCD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    pub fn DCDEN(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Primary detection (PD) mode enable Device mode This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    pub fn PDEN(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secondary detection (SD) mode enable Device mode This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    pub fn SDEN(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data contact detection (DCD) status Device mode This bit gives the result of DCD."]
    #[inline(always)]
    pub fn DCDET(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Primary detection (PD) status Device mode This bit gives the result of PD."]
    #[inline(always)]
    pub fn PDET(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secondary detection (SD) status Device mode This bit gives the result of SD."]
    #[inline(always)]
    pub fn SDET(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DM pull-up detection status Device mode This bit is active only during PD and gives the result of comparison between DM voltage level and VLGC threshold. In normal situation, the DM level should be below this threshold. If it is above, it means that the DM is externally pulled high. This can be caused by connection to a PS2 port (which pulls-up both DP and DM lines) or to some proprietary charger not following the BCD specification."]
    #[inline(always)]
    pub fn PS2DET(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - DP pull-up / DPDM pull-down Device mode This bit is set by software to enable the embedded pull-up on DP line. Clearing it to 0 can be used to signal disconnect to the host when needed by the user software. Host mode This bit is set by software to enable the embedded pull-down on DP and DM lines."]
    #[inline(always)]
    pub fn DPPU_DPD(&self) -> DPPU_DPD_R {
        DPPU_DPD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery charging detector (BCD) enable Device mode This bit is set by the software to enable the BCD support within the USB Device. When enabled, the USB PHY is fully controlled by BCD and cannot be used for normal communication. Once the BCD discovery is finished, the BCD should be placed in OFF mode by clearing this bit to 0 in order to allow the normal USB operation."]
    #[inline(always)]
    pub fn BCDEN(&mut self) -> BCDEN_W<'_, BCDR_SPEC> {
        BCDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data contact detection (DCD) mode enable Device mode This bit is set by the software to put the BCD into DCD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    pub fn DCDEN(&mut self) -> DCDEN_W<'_, BCDR_SPEC> {
        DCDEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Primary detection (PD) mode enable Device mode This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    pub fn PDEN(&mut self) -> PDEN_W<'_, BCDR_SPEC> {
        PDEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Secondary detection (SD) mode enable Device mode This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD, PD, SD or OFF) should be selected to work correctly."]
    #[inline(always)]
    pub fn SDEN(&mut self) -> SDEN_W<'_, BCDR_SPEC> {
        SDEN_W::new(self, 3)
    }
    #[doc = "Bit 15 - DP pull-up / DPDM pull-down Device mode This bit is set by software to enable the embedded pull-up on DP line. Clearing it to 0 can be used to signal disconnect to the host when needed by the user software. Host mode This bit is set by software to enable the embedded pull-down on DP and DM lines."]
    #[inline(always)]
    pub fn DPPU_DPD(&mut self) -> DPPU_DPD_W<'_, BCDR_SPEC> {
        DPPU_DPD_W::new(self, 15)
    }
}
#[doc = "USB_BCDR\n\nYou can [`read`](crate::Reg::read) this register and get [`bcdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCDR_SPEC;
impl crate::RegisterSpec for BCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdr::R`](R) reader structure"]
impl crate::Readable for BCDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcdr::W`](W) writer structure"]
impl crate::Writable for BCDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BCDR to value 0"]
impl crate::Resettable for BCDR_SPEC {}

#[doc = "Register `VMSR` reader"]
pub type R = crate::R<VMSR_SPEC>;
#[doc = "analog voltage detector output on V DDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVDO_A {
    #[doc = "0: V DDA is equal or higher than the AVD threshold selected with the ALS\\[2:0\\] bits."]
    B_0x0 = 0,
    #[doc = "1: V DDA is lower than the AVD threshold selected with the ALS\\[2:0\\] bits."]
    B_0x1 = 1,
}
impl From<AVDO_A> for bool {
    #[inline(always)]
    fn from(variant: AVDO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVDO` reader - analog voltage detector output on V DDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set."]
pub type AVDO_R = crate::BitReader<AVDO_A>;
impl AVDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AVDO_A {
        match self.bits {
            false => AVDO_A::B_0x0,
            true => AVDO_A::B_0x1,
        }
    }
    #[doc = "V DDA is equal or higher than the AVD threshold selected with the ALS\\[2:0\\] bits."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AVDO_A::B_0x0
    }
    #[doc = "V DDA is lower than the AVD threshold selected with the ALS\\[2:0\\] bits."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AVDO_A::B_0x1
    }
}
#[doc = "voltage detector output on V DDIO2 This bit is set and cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDIO2RDY_A {
    #[doc = "0: V DDIO2 is below 1.2 V."]
    B_0x0 = 0,
    #[doc = "1: V DDIO2 is above or equal to 1.2 V."]
    B_0x1 = 1,
}
impl From<VDDIO2RDY_A> for bool {
    #[inline(always)]
    fn from(variant: VDDIO2RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDIO2RDY` reader - voltage detector output on V DDIO2 This bit is set and cleared by hardware."]
pub type VDDIO2RDY_R = crate::BitReader<VDDIO2RDY_A>;
impl VDDIO2RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VDDIO2RDY_A {
        match self.bits {
            false => VDDIO2RDY_A::B_0x0,
            true => VDDIO2RDY_A::B_0x1,
        }
    }
    #[doc = "V DDIO2 is below 1.2 V."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VDDIO2RDY_A::B_0x0
    }
    #[doc = "V DDIO2 is above or equal to 1.2 V."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VDDIO2RDY_A::B_0x1
    }
}
#[doc = "programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDO_A {
    #[doc = "0: V DD is equal or higher than the PVD threshold selected through the PLS\\[2:0\\] bits."]
    B_0x0 = 0,
    #[doc = "1: V DD is lower than the PVD threshold selected through the PLS\\[2:0\\] bits."]
    B_0x1 = 1,
}
impl From<PVDO_A> for bool {
    #[inline(always)]
    fn from(variant: PVDO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDO` reader - programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
pub type PVDO_R = crate::BitReader<PVDO_A>;
impl PVDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVDO_A {
        match self.bits {
            false => PVDO_A::B_0x0,
            true => PVDO_A::B_0x1,
        }
    }
    #[doc = "V DD is equal or higher than the PVD threshold selected through the PLS\\[2:0\\] bits."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PVDO_A::B_0x0
    }
    #[doc = "V DD is lower than the PVD threshold selected through the PLS\\[2:0\\] bits."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PVDO_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 19 - analog voltage detector output on V DDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set."]
    #[inline(always)]
    pub fn AVDO(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - voltage detector output on V DDIO2 This bit is set and cleared by hardware."]
    #[inline(always)]
    pub fn VDDIO2RDY(&self) -> VDDIO2RDY_R {
        VDDIO2RDY_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
    #[inline(always)]
    pub fn PVDO(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "PWR voltage monitor status register\n\nYou can [`read`](crate::Reg::read) this register and get [`vmsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMSR_SPEC;
impl crate::RegisterSpec for VMSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmsr::R`](R) reader structure"]
impl crate::Readable for VMSR_SPEC {}
#[doc = "`reset()` method sets VMSR to value 0"]
impl crate::Resettable for VMSR_SPEC {}

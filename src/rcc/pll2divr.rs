#[doc = "Register `PLL2DIVR` reader"]
pub type R = crate::R<PLL2DIVR_SPEC>;
#[doc = "Register `PLL2DIVR` writer"]
pub type W = crate::W<PLL2DIVR_SPEC>;
#[doc = "Multiplication factor for PLL2VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PLL2N_A {
    #[doc = "3: PLL2N = 4"]
    B_0x3 = 3,
    #[doc = "4: PLL2N = 5"]
    B_0x4 = 4,
    #[doc = "5: PLL2N = 6"]
    B_0x5 = 5,
    #[doc = "128: PLL2N = 129 (default after reset)"]
    B_0x80 = 128,
    #[doc = "511: PLL2N = 512"]
    B_0x1FF = 511,
}
impl From<PLL2N_A> for u16 {
    #[inline(always)]
    fn from(variant: PLL2N_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2N_A {
    type Ux = u16;
}
impl crate::IsEnum for PLL2N_A {}
#[doc = "Field `PLL2N` reader - Multiplication factor for PLL2VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved"]
pub type PLL2N_R = crate::FieldReader<PLL2N_A>;
impl PLL2N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL2N_A> {
        match self.bits {
            3 => Some(PLL2N_A::B_0x3),
            4 => Some(PLL2N_A::B_0x4),
            5 => Some(PLL2N_A::B_0x5),
            128 => Some(PLL2N_A::B_0x80),
            511 => Some(PLL2N_A::B_0x1FF),
            _ => None,
        }
    }
    #[doc = "PLL2N = 4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL2N_A::B_0x3
    }
    #[doc = "PLL2N = 5"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PLL2N_A::B_0x4
    }
    #[doc = "PLL2N = 6"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PLL2N_A::B_0x5
    }
    #[doc = "PLL2N = 129 (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x80(&self) -> bool {
        *self == PLL2N_A::B_0x80
    }
    #[doc = "PLL2N = 512"]
    #[inline(always)]
    pub fn is_B_0x1FF(&self) -> bool {
        *self == PLL2N_A::B_0x1FF
    }
}
#[doc = "Field `PLL2N` writer - Multiplication factor for PLL2VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved"]
pub type PLL2N_W<'a, REG> = crate::FieldWriter<'a, REG, 9, PLL2N_A>;
impl<'a, REG> PLL2N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "PLL2N = 4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2N_A::B_0x3)
    }
    #[doc = "PLL2N = 5"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2N_A::B_0x4)
    }
    #[doc = "PLL2N = 6"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2N_A::B_0x5)
    }
    #[doc = "PLL2N = 129 (default after reset)"]
    #[inline(always)]
    pub fn B_0x80(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2N_A::B_0x80)
    }
    #[doc = "PLL2N = 512"]
    #[inline(always)]
    pub fn B_0x1FF(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2N_A::B_0x1FF)
    }
}
#[doc = "PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2P_A {
    #[doc = "0: pll2_p_ck = vco2_ck"]
    B_0x0 = 0,
    #[doc = "1: pll2_p_ck = vco2_ck / 2 (default after reset)"]
    B_0x1 = 1,
    #[doc = "2: pll2_p_ck = vco2_ck / 3"]
    B_0x2 = 2,
    #[doc = "3: pll2_p_ck = vco2_ck / 4"]
    B_0x3 = 3,
    #[doc = "127: pll2_p_ck = vco2_ck / 128"]
    B_0x7F = 127,
}
impl From<PLL2P_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL2P_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2P_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL2P_A {}
#[doc = "Field `PLL2P` reader - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2P_R = crate::FieldReader<PLL2P_A>;
impl PLL2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL2P_A> {
        match self.bits {
            0 => Some(PLL2P_A::B_0x0),
            1 => Some(PLL2P_A::B_0x1),
            2 => Some(PLL2P_A::B_0x2),
            3 => Some(PLL2P_A::B_0x3),
            127 => Some(PLL2P_A::B_0x7F),
            _ => None,
        }
    }
    #[doc = "pll2_p_ck = vco2_ck"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2P_A::B_0x0
    }
    #[doc = "pll2_p_ck = vco2_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2P_A::B_0x1
    }
    #[doc = "pll2_p_ck = vco2_ck / 3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL2P_A::B_0x2
    }
    #[doc = "pll2_p_ck = vco2_ck / 4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL2P_A::B_0x3
    }
    #[doc = "pll2_p_ck = vco2_ck / 128"]
    #[inline(always)]
    pub fn is_B_0x7F(&self) -> bool {
        *self == PLL2P_A::B_0x7F
    }
}
#[doc = "Field `PLL2P` writer - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2P_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PLL2P_A>;
impl<'a, REG> PLL2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll2_p_ck = vco2_ck"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P_A::B_0x0)
    }
    #[doc = "pll2_p_ck = vco2_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P_A::B_0x1)
    }
    #[doc = "pll2_p_ck = vco2_ck / 3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P_A::B_0x2)
    }
    #[doc = "pll2_p_ck = vco2_ck / 4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P_A::B_0x3)
    }
    #[doc = "pll2_p_ck = vco2_ck / 128"]
    #[inline(always)]
    pub fn B_0x7F(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P_A::B_0x7F)
    }
}
#[doc = "PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2Q_A {
    #[doc = "0: pll2_q_ck = vco2_ck"]
    B_0x0 = 0,
    #[doc = "1: pll2_q_ck = vco2_ck / 2 (default after reset)"]
    B_0x1 = 1,
    #[doc = "2: pll2_q_ck = vco2_ck / 3"]
    B_0x2 = 2,
    #[doc = "3: pll2_q_ck = vco2_ck / 4"]
    B_0x3 = 3,
    #[doc = "127: pll2_q_ck = vco2_ck / 128"]
    B_0x7F = 127,
}
impl From<PLL2Q_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL2Q_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2Q_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL2Q_A {}
#[doc = "Field `PLL2Q` reader - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2Q_R = crate::FieldReader<PLL2Q_A>;
impl PLL2Q_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL2Q_A> {
        match self.bits {
            0 => Some(PLL2Q_A::B_0x0),
            1 => Some(PLL2Q_A::B_0x1),
            2 => Some(PLL2Q_A::B_0x2),
            3 => Some(PLL2Q_A::B_0x3),
            127 => Some(PLL2Q_A::B_0x7F),
            _ => None,
        }
    }
    #[doc = "pll2_q_ck = vco2_ck"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2Q_A::B_0x0
    }
    #[doc = "pll2_q_ck = vco2_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2Q_A::B_0x1
    }
    #[doc = "pll2_q_ck = vco2_ck / 3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL2Q_A::B_0x2
    }
    #[doc = "pll2_q_ck = vco2_ck / 4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL2Q_A::B_0x3
    }
    #[doc = "pll2_q_ck = vco2_ck / 128"]
    #[inline(always)]
    pub fn is_B_0x7F(&self) -> bool {
        *self == PLL2Q_A::B_0x7F
    }
}
#[doc = "Field `PLL2Q` writer - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2Q_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PLL2Q_A>;
impl<'a, REG> PLL2Q_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll2_q_ck = vco2_ck"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q_A::B_0x0)
    }
    #[doc = "pll2_q_ck = vco2_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q_A::B_0x1)
    }
    #[doc = "pll2_q_ck = vco2_ck / 3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q_A::B_0x2)
    }
    #[doc = "pll2_q_ck = vco2_ck / 4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q_A::B_0x3)
    }
    #[doc = "pll2_q_ck = vco2_ck / 128"]
    #[inline(always)]
    pub fn B_0x7F(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q_A::B_0x7F)
    }
}
#[doc = "PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2R_A {
    #[doc = "0: pll2_r_ck = vco2_ck"]
    B_0x0 = 0,
    #[doc = "1: pll2_r_ck = vco2_ck / 2 (default after reset)"]
    B_0x1 = 1,
    #[doc = "2: pll2_r_ck = vco2_ck / 3"]
    B_0x2 = 2,
    #[doc = "3: pll2_r_ck = vco2_ck / 4"]
    B_0x3 = 3,
    #[doc = "127: pll2_r_ck = vco2_ck / 128"]
    B_0x7F = 127,
}
impl From<PLL2R_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL2R_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2R_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL2R_A {}
#[doc = "Field `PLL2R` reader - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2R_R = crate::FieldReader<PLL2R_A>;
impl PLL2R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL2R_A> {
        match self.bits {
            0 => Some(PLL2R_A::B_0x0),
            1 => Some(PLL2R_A::B_0x1),
            2 => Some(PLL2R_A::B_0x2),
            3 => Some(PLL2R_A::B_0x3),
            127 => Some(PLL2R_A::B_0x7F),
            _ => None,
        }
    }
    #[doc = "pll2_r_ck = vco2_ck"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2R_A::B_0x0
    }
    #[doc = "pll2_r_ck = vco2_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2R_A::B_0x1
    }
    #[doc = "pll2_r_ck = vco2_ck / 3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL2R_A::B_0x2
    }
    #[doc = "pll2_r_ck = vco2_ck / 4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL2R_A::B_0x3
    }
    #[doc = "pll2_r_ck = vco2_ck / 128"]
    #[inline(always)]
    pub fn is_B_0x7F(&self) -> bool {
        *self == PLL2R_A::B_0x7F
    }
}
#[doc = "Field `PLL2R` writer - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2R_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PLL2R_A>;
impl<'a, REG> PLL2R_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll2_r_ck = vco2_ck"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R_A::B_0x0)
    }
    #[doc = "pll2_r_ck = vco2_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R_A::B_0x1)
    }
    #[doc = "pll2_r_ck = vco2_ck / 3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R_A::B_0x2)
    }
    #[doc = "pll2_r_ck = vco2_ck / 4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R_A::B_0x3)
    }
    #[doc = "pll2_r_ck = vco2_ck / 128"]
    #[inline(always)]
    pub fn B_0x7F(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R_A::B_0x7F)
    }
}
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL2VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    pub fn PLL2N(&self) -> PLL2N_R {
        PLL2N_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn PLL2P(&self) -> PLL2P_R {
        PLL2P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn PLL2Q(&self) -> PLL2Q_R {
        PLL2Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn PLL2R(&self) -> PLL2R_R {
        PLL2R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL2VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    pub fn PLL2N(&mut self) -> PLL2N_W<'_, PLL2DIVR_SPEC> {
        PLL2N_W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn PLL2P(&mut self) -> PLL2P_W<'_, PLL2DIVR_SPEC> {
        PLL2P_W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn PLL2Q(&mut self) -> PLL2Q_W<'_, PLL2DIVR_SPEC> {
        PLL2Q_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn PLL2R(&mut self) -> PLL2R_W<'_, PLL2DIVR_SPEC> {
        PLL2R_W::new(self, 24)
    }
}
#[doc = "RCC PLL1 dividers register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll2divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL2DIVR_SPEC;
impl crate::RegisterSpec for PLL2DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll2divr::R`](R) reader structure"]
impl crate::Readable for PLL2DIVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll2divr::W`](W) writer structure"]
impl crate::Writable for PLL2DIVR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PLL2DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL2DIVR_SPEC {
    const RESET_VALUE: u32 = 0x0101_0280;
}

#[doc = "Register `CEVR` writer"]
pub type W = crate::W<CEVR_SPEC>;
#[doc = "clear frame complete flag (whatever the I3C is acting as controller/target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFCF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.FCF"]
    B_0x1 = 1,
}
impl From<CFCF_A> for bool {
    #[inline(always)]
    fn from(variant: CFCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFCF` writer - clear frame complete flag (whatever the I3C is acting as controller/target)"]
pub type CFCF_W<'a, REG> = crate::BitWriter<'a, REG, CFCF_A>;
impl<'a, REG> CFCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CFCF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.FCF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CFCF_A::B_0x1)
    }
}
#[doc = "clear target-initiated read end flag (when the I3C is acting as controller)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRXTGTENDF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.RXTGTENDF"]
    B_0x1 = 1,
}
impl From<CRXTGTENDF_A> for bool {
    #[inline(always)]
    fn from(variant: CRXTGTENDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRXTGTENDF` writer - clear target-initiated read end flag (when the I3C is acting as controller)"]
pub type CRXTGTENDF_W<'a, REG> = crate::BitWriter<'a, REG, CRXTGTENDF_A>;
impl<'a, REG> CRXTGTENDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRXTGTENDF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.RXTGTENDF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRXTGTENDF_A::B_0x1)
    }
}
#[doc = "clear error flag (whatever the I3C is acting as controller/target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERRF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.ERRF"]
    B_0x1 = 1,
}
impl From<CERRF_A> for bool {
    #[inline(always)]
    fn from(variant: CERRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERRF` writer - clear error flag (whatever the I3C is acting as controller/target)"]
pub type CERRF_W<'a, REG> = crate::BitWriter<'a, REG, CERRF_A>;
impl<'a, REG> CERRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CERRF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.ERRF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CERRF_A::B_0x1)
    }
}
#[doc = "clear IBI request flag (when the I3C is acting as controller)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIBIF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.IBIF"]
    B_0x1 = 1,
}
impl From<CIBIF_A> for bool {
    #[inline(always)]
    fn from(variant: CIBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIBIF` writer - clear IBI request flag (when the I3C is acting as controller)"]
pub type CIBIF_W<'a, REG> = crate::BitWriter<'a, REG, CIBIF_A>;
impl<'a, REG> CIBIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CIBIF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.IBIF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CIBIF_A::B_0x1)
    }
}
#[doc = "clear IBI end flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIBIENDF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.IBIENDF"]
    B_0x1 = 1,
}
impl From<CIBIENDF_A> for bool {
    #[inline(always)]
    fn from(variant: CIBIENDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIBIENDF` writer - clear IBI end flag (when the I3C is acting as target)"]
pub type CIBIENDF_W<'a, REG> = crate::BitWriter<'a, REG, CIBIENDF_A>;
impl<'a, REG> CIBIENDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CIBIENDF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.IBIENDF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CIBIENDF_A::B_0x1)
    }
}
#[doc = "clear controller-role request flag (when the I3C is acting as controller)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.CRF"]
    B_0x1 = 1,
}
impl From<CCRF_A> for bool {
    #[inline(always)]
    fn from(variant: CCRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRF` writer - clear controller-role request flag (when the I3C is acting as controller)"]
pub type CCRF_W<'a, REG> = crate::BitWriter<'a, REG, CCRF_A>;
impl<'a, REG> CCRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCRF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.CRF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCRF_A::B_0x1)
    }
}
#[doc = "clear controller-role update flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRUPDF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.CRUPDF"]
    B_0x1 = 1,
}
impl From<CCRUPDF_A> for bool {
    #[inline(always)]
    fn from(variant: CCRUPDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRUPDF` writer - clear controller-role update flag (when the I3C is acting as target)"]
pub type CCRUPDF_W<'a, REG> = crate::BitWriter<'a, REG, CCRUPDF_A>;
impl<'a, REG> CCRUPDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCRUPDF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.CRUPDF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCRUPDF_A::B_0x1)
    }
}
#[doc = "clear hot-join flag (when the I3C is acting as controller)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHJF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.HJF"]
    B_0x1 = 1,
}
impl From<CHJF_A> for bool {
    #[inline(always)]
    fn from(variant: CHJF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHJF` writer - clear hot-join flag (when the I3C is acting as controller)"]
pub type CHJF_W<'a, REG> = crate::BitWriter<'a, REG, CHJF_A>;
impl<'a, REG> CHJF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHJF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.HJF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHJF_A::B_0x1)
    }
}
#[doc = "clear wakeup flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWKPF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.WKPF"]
    B_0x1 = 1,
}
impl From<CWKPF_A> for bool {
    #[inline(always)]
    fn from(variant: CWKPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWKPF` writer - clear wakeup flag (when the I3C is acting as target)"]
pub type CWKPF_W<'a, REG> = crate::BitWriter<'a, REG, CWKPF_A>;
impl<'a, REG> CWKPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CWKPF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.WKPF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CWKPF_A::B_0x1)
    }
}
#[doc = "clear GETxxx CCC flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CGETF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.GETF"]
    B_0x1 = 1,
}
impl From<CGETF_A> for bool {
    #[inline(always)]
    fn from(variant: CGETF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGETF` writer - clear GETxxx CCC flag (when the I3C is acting as target)"]
pub type CGETF_W<'a, REG> = crate::BitWriter<'a, REG, CGETF_A>;
impl<'a, REG> CGETF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CGETF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.GETF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CGETF_A::B_0x1)
    }
}
#[doc = "clear GETSTATUS CCC flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTAF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.STAF"]
    B_0x1 = 1,
}
impl From<CSTAF_A> for bool {
    #[inline(always)]
    fn from(variant: CSTAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTAF` writer - clear GETSTATUS CCC flag (when the I3C is acting as target)"]
pub type CSTAF_W<'a, REG> = crate::BitWriter<'a, REG, CSTAF_A>;
impl<'a, REG> CSTAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTAF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.STAF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTAF_A::B_0x1)
    }
}
#[doc = "clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDAUPDF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.DAUPDF"]
    B_0x1 = 1,
}
impl From<CDAUPDF_A> for bool {
    #[inline(always)]
    fn from(variant: CDAUPDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDAUPDF` writer - clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)"]
pub type CDAUPDF_W<'a, REG> = crate::BitWriter<'a, REG, CDAUPDF_A>;
impl<'a, REG> CDAUPDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CDAUPDF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.DAUPDF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CDAUPDF_A::B_0x1)
    }
}
#[doc = "clear SETMWL CCC flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMWLUPDF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.MWLUPDF"]
    B_0x1 = 1,
}
impl From<CMWLUPDF_A> for bool {
    #[inline(always)]
    fn from(variant: CMWLUPDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMWLUPDF` writer - clear SETMWL CCC flag (when the I3C is acting as target)"]
pub type CMWLUPDF_W<'a, REG> = crate::BitWriter<'a, REG, CMWLUPDF_A>;
impl<'a, REG> CMWLUPDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CMWLUPDF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.MWLUPDF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CMWLUPDF_A::B_0x1)
    }
}
#[doc = "clear SETMRL CCC flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMRLUPDF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.MRLUPDF"]
    B_0x1 = 1,
}
impl From<CMRLUPDF_A> for bool {
    #[inline(always)]
    fn from(variant: CMRLUPDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMRLUPDF` writer - clear SETMRL CCC flag (when the I3C is acting as target)"]
pub type CMRLUPDF_W<'a, REG> = crate::BitWriter<'a, REG, CMRLUPDF_A>;
impl<'a, REG> CMRLUPDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CMRLUPDF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.MRLUPDF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CMRLUPDF_A::B_0x1)
    }
}
#[doc = "clear reset pattern flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSTF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.RSTF"]
    B_0x1 = 1,
}
impl From<CRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: CRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSTF` writer - clear reset pattern flag (when the I3C is acting as target)"]
pub type CRSTF_W<'a, REG> = crate::BitWriter<'a, REG, CRSTF_A>;
impl<'a, REG> CRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRSTF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.RSTF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRSTF_A::B_0x1)
    }
}
#[doc = "clear ENTASx CCC flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASUPDF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.ASUPDF"]
    B_0x1 = 1,
}
impl From<CASUPDF_A> for bool {
    #[inline(always)]
    fn from(variant: CASUPDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASUPDF` writer - clear ENTASx CCC flag (when the I3C is acting as target)"]
pub type CASUPDF_W<'a, REG> = crate::BitWriter<'a, REG, CASUPDF_A>;
impl<'a, REG> CASUPDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CASUPDF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.ASUPDF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CASUPDF_A::B_0x1)
    }
}
#[doc = "clear ENEC/DISEC CCC flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINTUPDF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.CINTUPDF"]
    B_0x1 = 1,
}
impl From<CINTUPDF_A> for bool {
    #[inline(always)]
    fn from(variant: CINTUPDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINTUPDF` writer - clear ENEC/DISEC CCC flag (when the I3C is acting as target)"]
pub type CINTUPDF_W<'a, REG> = crate::BitWriter<'a, REG, CINTUPDF_A>;
impl<'a, REG> CINTUPDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CINTUPDF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.CINTUPDF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CINTUPDF_A::B_0x1)
    }
}
#[doc = "clear DEFTGTS CCC flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDEFF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.DEFF"]
    B_0x1 = 1,
}
impl From<CDEFF_A> for bool {
    #[inline(always)]
    fn from(variant: CDEFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEFF` writer - clear DEFTGTS CCC flag (when the I3C is acting as target)"]
pub type CDEFF_W<'a, REG> = crate::BitWriter<'a, REG, CDEFF_A>;
impl<'a, REG> CDEFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CDEFF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.DEFF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CDEFF_A::B_0x1)
    }
}
#[doc = "clear DEFGRPA CCC flag (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CGRPF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clear I3C_EVR.GRPF"]
    B_0x1 = 1,
}
impl From<CGRPF_A> for bool {
    #[inline(always)]
    fn from(variant: CGRPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGRPF` writer - clear DEFGRPA CCC flag (when the I3C is acting as target)"]
pub type CGRPF_W<'a, REG> = crate::BitWriter<'a, REG, CGRPF_A>;
impl<'a, REG> CGRPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CGRPF_A::B_0x0)
    }
    #[doc = "clear I3C_EVR.GRPF"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CGRPF_A::B_0x1)
    }
}
impl W {
    #[doc = "Bit 9 - clear frame complete flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn CFCF(&mut self) -> CFCF_W<'_, CEVR_SPEC> {
        CFCF_W::new(self, 9)
    }
    #[doc = "Bit 10 - clear target-initiated read end flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn CRXTGTENDF(&mut self) -> CRXTGTENDF_W<'_, CEVR_SPEC> {
        CRXTGTENDF_W::new(self, 10)
    }
    #[doc = "Bit 11 - clear error flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn CERRF(&mut self) -> CERRF_W<'_, CEVR_SPEC> {
        CERRF_W::new(self, 11)
    }
    #[doc = "Bit 15 - clear IBI request flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn CIBIF(&mut self) -> CIBIF_W<'_, CEVR_SPEC> {
        CIBIF_W::new(self, 15)
    }
    #[doc = "Bit 16 - clear IBI end flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CIBIENDF(&mut self) -> CIBIENDF_W<'_, CEVR_SPEC> {
        CIBIENDF_W::new(self, 16)
    }
    #[doc = "Bit 17 - clear controller-role request flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn CCRF(&mut self) -> CCRF_W<'_, CEVR_SPEC> {
        CCRF_W::new(self, 17)
    }
    #[doc = "Bit 18 - clear controller-role update flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CCRUPDF(&mut self) -> CCRUPDF_W<'_, CEVR_SPEC> {
        CCRUPDF_W::new(self, 18)
    }
    #[doc = "Bit 19 - clear hot-join flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn CHJF(&mut self) -> CHJF_W<'_, CEVR_SPEC> {
        CHJF_W::new(self, 19)
    }
    #[doc = "Bit 21 - clear wakeup flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CWKPF(&mut self) -> CWKPF_W<'_, CEVR_SPEC> {
        CWKPF_W::new(self, 21)
    }
    #[doc = "Bit 22 - clear GETxxx CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CGETF(&mut self) -> CGETF_W<'_, CEVR_SPEC> {
        CGETF_W::new(self, 22)
    }
    #[doc = "Bit 23 - clear GETSTATUS CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CSTAF(&mut self) -> CSTAF_W<'_, CEVR_SPEC> {
        CSTAF_W::new(self, 23)
    }
    #[doc = "Bit 24 - clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CDAUPDF(&mut self) -> CDAUPDF_W<'_, CEVR_SPEC> {
        CDAUPDF_W::new(self, 24)
    }
    #[doc = "Bit 25 - clear SETMWL CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CMWLUPDF(&mut self) -> CMWLUPDF_W<'_, CEVR_SPEC> {
        CMWLUPDF_W::new(self, 25)
    }
    #[doc = "Bit 26 - clear SETMRL CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CMRLUPDF(&mut self) -> CMRLUPDF_W<'_, CEVR_SPEC> {
        CMRLUPDF_W::new(self, 26)
    }
    #[doc = "Bit 27 - clear reset pattern flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CRSTF(&mut self) -> CRSTF_W<'_, CEVR_SPEC> {
        CRSTF_W::new(self, 27)
    }
    #[doc = "Bit 28 - clear ENTASx CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CASUPDF(&mut self) -> CASUPDF_W<'_, CEVR_SPEC> {
        CASUPDF_W::new(self, 28)
    }
    #[doc = "Bit 29 - clear ENEC/DISEC CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CINTUPDF(&mut self) -> CINTUPDF_W<'_, CEVR_SPEC> {
        CINTUPDF_W::new(self, 29)
    }
    #[doc = "Bit 30 - clear DEFTGTS CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CDEFF(&mut self) -> CDEFF_W<'_, CEVR_SPEC> {
        CDEFF_W::new(self, 30)
    }
    #[doc = "Bit 31 - clear DEFGRPA CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CGRPF(&mut self) -> CGRPF_W<'_, CEVR_SPEC> {
        CGRPF_W::new(self, 31)
    }
}
#[doc = "I3C clear event register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cevr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEVR_SPEC;
impl crate::RegisterSpec for CEVR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cevr::W`](W) writer structure"]
impl crate::Writable for CEVR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CEVR to value 0"]
impl crate::Resettable for CEVR_SPEC {}

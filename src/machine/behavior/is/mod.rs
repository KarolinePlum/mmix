pub mod arithmetic;
pub mod bitwise;
pub mod bytewise;
pub mod conditional;
pub mod control;
pub mod float;
pub mod immediate;
pub mod interrupt;
pub mod memory;
pub mod other;
pub mod subroutine;
pub mod system;

use machine::state::State;

pub fn get(_opcode: u8) -> Semantic {
    unimplemented!();
    // SEMANTICS[opcode]
}

type Semantic = fn(&mut State, u8, u8, u8);

pub const SEMANTICS: [Semantic; 256] = [
    interrupt::trap,    
    float::fcmp,      
    float::fun,       
    float::feql,      
    float::fadd,      
    float::fix,       
    float::fsub,      
    float::fixu,        
    float::flot,
    float::floti,
    float::flotu,
    float::flotui,
    float::sflot,
    float::sfloti,
    float::sflotu,
    float::sflotui,
    float::fmul,
    float::fcmpe,
    float::fune,
    float::feqle,
    float::fdiv,
    float::fsqrt,
    float::frem,
    float::fint,
    arithmetic::mul,
    arithmetic::muli,
    arithmetic::mulu,
    arithmetic::mului,
    arithmetic::div,
    arithmetic::divi,
    arithmetic::divu,
    arithmetic::divui,
    arithmetic::add,
    arithmetic::addi,
    arithmetic::addu,
    arithmetic::addui,
    arithmetic::sub,
    arithmetic::subi,
    arithmetic::subu,
    arithmetic::subui,
    arithmetic::addu2,
    arithmetic::addu2i,
    arithmetic::addu4,
    arithmetic::addu4i,
    arithmetic::addu8,
    arithmetic::addu8i,
    arithmetic::addu16,
    arithmetic::addu16i,
    arithmetic::cmp,
    arithmetic::cmpi,
    arithmetic::cmpu,
    arithmetic::cmpui,
    arithmetic::neg,
    arithmetic::negi,
    arithmetic::negu,
    arithmetic::negui,
    arithmetic::sl,
    arithmetic::sli,
    arithmetic::slu,
    arithmetic::slui,
    arithmetic::sr,
    arithmetic::sri,
    arithmetic::sru,
    arithmetic::srui,
    control::bn,
    control::bnb,
    control::bz,
    control::bzb,
    control::bp,
    control::bpb,
    control::bod,
    control::bodb,
    control::bnn,
    control::bnnb,
    control::bnz,
    control::bnzb,
    control::bnp,
    control::bnpb,
    control::bev,
    control::bevb,
    control::pbn,
    control::pbnb,
    control::pbz,
    control::pbzb,
    control::pbp,
    control::pbpb,
    control::pbod,
    control::pbodb,
    control::pbnn,
    control::pbnnb,
    control::pbnz,
    control::pbnzb,
    control::pbnp,
    control::pbnpb,
    control::pbev,
    control::pbevb,
    conditional::csn,
    conditional::csni,
    conditional::csz,
    conditional::cszi,
    conditional::csp,
    conditional::cspi,
    conditional::csod,
    conditional::csodi,
    conditional::csnn,
    conditional::csnni,
    conditional::csnz,
    conditional::csnzi,
    conditional::csnp,
    conditional::csnpi,
    conditional::csev,
    conditional::csevi,
    conditional::zsn,
    conditional::zsni,
    conditional::zsz,
    conditional::zszi,
    conditional::zsp,
    conditional::zspi,
    conditional::zsod,
    conditional::zsodi,
    conditional::zsnn,
    conditional::zsnni,
    conditional::zsnz,
    conditional::zsnzi,
    conditional::zsnp,
    conditional::zsnpi,
    conditional::zsev,
    conditional::zsevi,
    memory::ldb,
    memory::ldbi,
    memory::ldbu,
    memory::ldbui,
    memory::ldw,
    memory::ldwi,
    memory::ldwu,
    memory::ldwui,
    memory::ldt,
    memory::ldti,
    memory::ldtu,
    memory::ldtui,
    memory::ldo,
    memory::ldoi,
    memory::ldou,
    memory::ldoui,
    float::ldsf,
    float::ldsfi,
    memory::ldht,
    memory::ldhti,
    system::cswap,
    system::cswapi,
    system::ldunc,
    system::ldunci,
    system::ldvts,
    system::ldvtsi,
    system::preld,
    system::preldi,
    system::prego,
    system::pregoi,
    control::go,
    control::goi,
    memory::stb,
    memory::stbi,
    memory::stbu,
    memory::stbui,
    memory::stw,
    memory::stwi,
    memory::stwu,
    memory::stwui,
    memory::stt,
    memory::stti,
    memory::sttu,
    memory::sttui,
    memory::sto,
    memory::stoi,
    memory::stou,
    memory::stoui,
    float::stsf,
    float::stsfi,
    memory::stht,
    memory::sthti,
    memory::stco,
    memory::stcoi,
    system::stunc,
    system::stunci,
    system::syncd,
    system::syncdi,
    system::prest,
    system::presti,
    system::syncid,
    system::syncidi,
    subroutine::pushgo,
    subroutine::pushgoi,
    bitwise::or,
    bitwise::ori,
    bitwise::orn,
    bitwise::orni,
    bitwise::nor,
    bitwise::nori,
    bitwise::xor,
    bitwise::xori,
    bitwise::and,
    bitwise::andi,
    bitwise::andn,
    bitwise::andni,
    bitwise::nand,
    bitwise::nandi,
    bitwise::nxor,
    bitwise::nxori,
    bytewise::bdif,
    bytewise::bdifi,
    bytewise::wdif,
    bytewise::wdifi,
    bytewise::tdif,
    bytewise::tdifi,
    bytewise::odif,
    bytewise::odifi,
    bitwise::mux,
    bitwise::muxi,
    bitwise::sadd,
    bitwise::saddi,
    bytewise::mor,
    bytewise::mori,
    bytewise::mxor,
    bytewise::mxori,
    immediate::seth,
    immediate::setmh,
    immediate::setml,
    immediate::setl,
    immediate::inch,
    immediate::incmh,
    immediate::incml,
    immediate::incl,
    immediate::orh,
    immediate::ormh,
    immediate::orml,
    immediate::orl,
    immediate::andnh,
    immediate::andnmh,
    immediate::andnml,
    immediate::andnl,
    control::jmp,
    control::jmpb,
    subroutine::pushj,
    subroutine::pushjb,
    other::geta,
    other::getab,
    other::put,
    other::puti,
    subroutine::pop,
    interrupt::resume,
    subroutine::save,
    subroutine::unsave,
    system::sync,
    other::swym,
    other::get,
    interrupt::trip,
];


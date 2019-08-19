pub enum OpCodes {
    OpStop,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpSdiv,
    OpMod,
    OpSmod,
    OpExp,
    OpNot,
    OpLt,
    OpGt,
    OpSlt,
    OpSgt,
    OpEq,
    OpIszero,
    OpAnd,
    OpOr,
    OpXor,
    OpByte,
    OpShl,
    OpShr,
    OpSar,
    OpAddmod,
    OpMulmod,
    OpSignextend,
    OpKeccak256,
    OpJump,
    OpJumpi,
    OpPc,
    OpPop,
    OpDup1,
    OpDup2,
    OpDup3,
    OpDup4,
    OpDup5,
    OpDup6,
    OpDup7,
    OpDup8,
    OpDup9,
    OpDup10,
    OpDup11,
    OpDup12,
    OpDup13,
    OpDup14,
    OpDup15,
    OpDup16,
    OpSwap1,
    OpSwap2,
    OpSwap3,
    OpSwap4,
    OpSwap5,
    OpSwap6,
    OpSwap7,
    OpSwap8,
    OpSwap9,
    OpSwap10,
    OpSwap11,
    OpSwap12,
    OpSwap13,
    OpSwap14,
    OpSwap15,
    OpSwap16,
    OpMload,
    OpMstore,
    OpMstore8,
    OpSload,
    OpSstore,
    OpMsize,
    OpGas,
    OpAddress,
    OpBalance,
    OpCaller,
    OpCallvalue,
    OpCalldataload,
    OpCalldatasize,
    OpCalldatacopy,
    OpCodesize,
    OpCodecopy,
    OpExtcodesize,
    OpExtcodecop,
    OpReturndatasize,
    OpReturndatacopy,
    OpExtcodehash,
    OpCreate,
    OpCreate2,
    OpCall,
    OpCallcode,
    OpDelegatecall,
    OpStaticcall,
    OpReturn,
    OpRevert,
    OpSelfdestruct,
}
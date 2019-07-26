use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub enum BlockItemDeclaration {
    Data(Box<BlockItemDeclarationData>),
    LocalParameter(Box<BlockItemDeclarationLocalParameter>),
    Parameter(Box<BlockItemDeclarationParameter>),
    Let(Box<BlockItemDeclarationLet>),
}

#[derive(Clone, Debug, Node)]
pub struct BlockItemDeclarationData {
    pub nodes: (Vec<AttributeInstance>, DataDeclaration),
}

#[derive(Clone, Debug, Node)]
pub struct BlockItemDeclarationLocalParameter {
    pub nodes: (Vec<AttributeInstance>, LocalParameterDeclaration, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct BlockItemDeclarationParameter {
    pub nodes: (Vec<AttributeInstance>, ParameterDeclaration, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct BlockItemDeclarationLet {
    pub nodes: (Vec<AttributeInstance>, LetDeclaration),
}

use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub struct SpecifyInputTerminalDescriptor {
    pub nodes: (InputIdentifier, Option<Bracket<ConstantRangeExpression>>),
}

#[derive(Clone, Debug, Node)]
pub struct SpecifyOutputTerminalDescriptor {
    pub nodes: (OutputIdentifier, Option<Bracket<ConstantRangeExpression>>),
}

#[derive(Clone, Debug, Node)]
pub enum InputIdentifier {
    InputPortIdentifier(Box<InputPortIdentifier>),
    InoutPortIdentifier(Box<InoutPortIdentifier>),
    Interface(Box<InputIdentifierInterface>),
}

#[derive(Clone, Debug, Node)]
pub struct InputIdentifierInterface {
    pub nodes: (InterfaceIdentifier, Symbol, PortIdentifier),
}

#[derive(Clone, Debug, Node)]
pub enum OutputIdentifier {
    OutputPortIdentifier(Box<OutputPortIdentifier>),
    InoutPortIdentifier(Box<InoutPortIdentifier>),
    Interface(Box<OutputIdentifierInterface>),
}

#[derive(Clone, Debug, Node)]
pub struct OutputIdentifierInterface {
    pub nodes: (InterfaceIdentifier, Symbol, PortIdentifier),
}

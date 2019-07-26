use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub enum PathDelayValue {
    ListOfPathDelayExpressions(Box<ListOfPathDelayExpressions>),
    Paren(Box<PathDelayValueParen>),
}

#[derive(Clone, Debug, Node)]
pub struct PathDelayValueParen {
    pub nodes: (Paren<ListOfPathDelayExpressions>,),
}

#[derive(Clone, Debug, Node)]
pub struct ListOfPathDelayExpressions {
    pub nodes: (List<Symbol, TPathDelayExpression>,),
}

#[derive(Clone, Debug, Node)]
pub struct TPathDelayExpression {
    pub nodes: (PathDelayExpression,),
}
#[derive(Clone, Debug, Node)]
pub struct PathDelayExpression {
    pub nodes: (ConstantMintypmaxExpression,),
}

#[derive(Clone, Debug, Node)]
pub enum EdgeSensitivePathDeclaration {
    Parallel(Box<EdgeSensitivePathDeclarationParallel>),
    Full(Box<EdgeSensitivePathDeclarationFull>),
}

#[derive(Clone, Debug, Node)]
pub struct EdgeSensitivePathDeclarationParallel {
    pub nodes: (ParallelEdgeSensitivePathDescription, Symbol, PathDelayValue),
}

#[derive(Clone, Debug, Node)]
pub struct EdgeSensitivePathDeclarationFull {
    pub nodes: (FullEdgeSensitivePathDescription, Symbol, PathDelayValue),
}

#[derive(Clone, Debug, Node)]
pub struct ParallelEdgeSensitivePathDescription {
    pub nodes: (
        Paren<(
            Option<EdgeIdentifier>,
            SpecifyInputTerminalDescriptor,
            Option<PolarityOperator>,
            Symbol,
            Paren<(
                SpecifyOutputTerminalDescriptor,
                Option<PolarityOperator>,
                Symbol,
                DataSourceExpression,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct FullEdgeSensitivePathDescription {
    pub nodes: (
        Paren<(
            Option<EdgeIdentifier>,
            ListOfPathInputs,
            Option<PolarityOperator>,
            Symbol,
            Paren<(
                ListOfPathOutputs,
                Option<PolarityOperator>,
                Symbol,
                DataSourceExpression,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct DataSourceExpression {
    pub nodes: (Expression,),
}

#[derive(Clone, Debug, Node)]
pub enum EdgeIdentifier {
    Posedge(Box<Keyword>),
    Negedge(Box<Keyword>),
    Edge(Box<Keyword>),
}

#[derive(Clone, Debug, Node)]
pub enum StateDependentPathDeclaration {
    IfSimple(Box<StateDependentPathDeclarationIfSimple>),
    IfEdgeSensitive(Box<StateDependentPathDeclarationIfEdgeSensitive>),
    IfNone(Box<StateDependentPathDeclarationIfNone>),
}

#[derive(Clone, Debug, Node)]
pub struct StateDependentPathDeclarationIfSimple {
    pub nodes: (Keyword, Paren<ModulePathExpression>, SimplePathDeclaration),
}

#[derive(Clone, Debug, Node)]
pub struct StateDependentPathDeclarationIfEdgeSensitive {
    pub nodes: (
        Keyword,
        Paren<ModulePathExpression>,
        EdgeSensitivePathDeclaration,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct StateDependentPathDeclarationIfNone {
    pub nodes: (Keyword, SimplePathDeclaration),
}

#[derive(Clone, Debug, Node)]
pub struct PolarityOperator {
    pub nodes: (Symbol,),
}

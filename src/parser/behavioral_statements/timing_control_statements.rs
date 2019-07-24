use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub struct ProceduralTimingControlStatement {
    pub nodes: (ProceduralTimingControl, StatementOrNull),
}

#[derive(Clone, Debug, Node)]
pub enum DelayOrEventControl {
    Delay(Box<DelayControl>),
    Event(Box<EventControl>),
    Repeat(Box<DelayOrEventControlRepeat>),
}

#[derive(Clone, Debug, Node)]
pub struct DelayOrEventControlRepeat {
    pub nodes: (Keyword, Paren<Expression>, EventControl),
}

#[derive(Clone, Debug, Node)]
pub enum DelayControl {
    Delay(Box<DelayControlDelay>),
    Mintypmax(Box<DelayControlMintypmax>),
}

#[derive(Clone, Debug, Node)]
pub struct DelayControlDelay {
    pub nodes: (Symbol, DelayValue),
}

#[derive(Clone, Debug, Node)]
pub struct DelayControlMintypmax {
    pub nodes: (Symbol, Paren<MintypmaxExpression>),
}

#[derive(Clone, Debug, Node)]
pub enum EventControl {
    EventIdentifier(Box<EventControlEventIdentifier>),
    EventExpression(Box<EventControlEventExpression>),
    Asterisk(Box<EventControlAsterisk>),
    ParenAsterisk(Box<EventControlParenAsterisk>),
    SequenceIdentifier(Box<EventControlSequenceIdentifier>),
}

#[derive(Clone, Debug, Node)]
pub struct EventControlEventIdentifier {
    pub nodes: (Symbol, HierarchicalEventIdentifier),
}

#[derive(Clone, Debug, Node)]
pub struct EventControlEventExpression {
    pub nodes: (Symbol, Paren<EventExpression>),
}

#[derive(Clone, Debug, Node)]
pub struct EventControlAsterisk {
    pub nodes: (Symbol,),
}

#[derive(Clone, Debug, Node)]
pub struct EventControlParenAsterisk {
    pub nodes: (Symbol, Paren<Symbol>),
}

#[derive(Clone, Debug, Node)]
pub struct EventControlSequenceIdentifier {
    pub nodes: (Symbol, PsOrHierarchicalSequenceIdentifier),
}

#[derive(Clone, Debug, Node)]
pub enum EventExpression {
    Expression(Box<EventExpressionExpression>),
    Sequence(Box<EventExpressionSequence>),
    Or(Box<EventExpressionOr>),
    Comma(Box<EventExpressionComma>),
    Paren(Box<EventExpressionParen>),
}

#[derive(Clone, Debug, Node)]
pub struct EventExpressionExpression {
    pub nodes: (
        Option<EdgeIdentifier>,
        Expression,
        Option<(Keyword, Expression)>,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct EventExpressionSequence {
    pub nodes: (SequenceInstance, Option<(Keyword, Expression)>),
}

#[derive(Clone, Debug, Node)]
pub struct EventExpressionOr {
    pub nodes: (EventExpression, Keyword, EventExpression),
}

#[derive(Clone, Debug, Node)]
pub struct EventExpressionComma {
    pub nodes: (EventExpression, Symbol, EventExpression),
}

#[derive(Clone, Debug, Node)]
pub struct EventExpressionParen {
    pub nodes: (Paren<EventExpression>,),
}

#[derive(Clone, Debug, Node)]
pub enum ProceduralTimingControl {
    DelayControl(Box<DelayControl>),
    EventControl(Box<EventControl>),
    CycleDelay(Box<CycleDelay>),
}

#[derive(Clone, Debug, Node)]
pub enum JumpStatement {
    Return(Box<JumpStatementReturn>),
    Break(Box<JumpStatementBreak>),
    Continue(Box<JumpStatementContinue>),
}

#[derive(Clone, Debug, Node)]
pub struct JumpStatementReturn {
    pub nodes: (Keyword, Option<Expression>, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct JumpStatementBreak {
    pub nodes: (Keyword, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct JumpStatementContinue {
    pub nodes: (Keyword, Symbol),
}

#[derive(Clone, Debug, Node)]
pub enum WaitStatement {
    Wait(Box<WaitStatementWait>),
    Fork(Box<WaitStatementFork>),
    Order(Box<WaitStatementOrder>),
}

#[derive(Clone, Debug, Node)]
pub struct WaitStatementWait {
    pub nodes: (Keyword, Paren<Expression>, StatementOrNull),
}

#[derive(Clone, Debug, Node)]
pub struct WaitStatementFork {
    pub nodes: (Keyword, Keyword, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct WaitStatementOrder {
    pub nodes: (
        Keyword,
        Paren<List<Symbol, HierarchicalIdentifier>>,
        ActionBlock,
    ),
}

#[derive(Clone, Debug, Node)]
pub enum EventTrigger {
    Named(Box<EventTriggerNamed>),
    Nonblocking(Box<EventTriggerNonblocking>),
}

#[derive(Clone, Debug, Node)]
pub struct EventTriggerNamed {
    pub nodes: (Symbol, HierarchicalEventIdentifier, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct EventTriggerNonblocking {
    pub nodes: (
        Symbol,
        Option<DelayOrEventControl>,
        HierarchicalEventIdentifier,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub enum DisableStatement {
    Task(Box<DisableStatementTask>),
    Block(Box<DisableStatementBlock>),
    Fork(Box<DisableStatementFork>),
}

#[derive(Clone, Debug, Node)]
pub struct DisableStatementTask {
    pub nodes: (Keyword, HierarchicalTaskIdentifier, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct DisableStatementBlock {
    pub nodes: (Keyword, HierarchicalBlockIdentifier, Symbol),
}

#[derive(Clone, Debug, Node)]
pub struct DisableStatementFork {
    pub nodes: (Keyword, Keyword, Symbol),
}

// -----------------------------------------------------------------------------

#[parser]
pub fn procedural_timing_control_statement(
    s: Span,
) -> IResult<Span, ProceduralTimingControlStatement> {
    let (s, a) = procedural_timing_control(s)?;
    let (s, b) = statement_or_null(s)?;
    Ok((s, ProceduralTimingControlStatement { nodes: (a, b) }))
}

#[parser]
pub fn delay_or_event_control(s: Span) -> IResult<Span, DelayOrEventControl> {
    alt((
        map(delay_control, |x| DelayOrEventControl::Delay(Box::new(x))),
        map(event_control, |x| DelayOrEventControl::Event(Box::new(x))),
        delay_or_event_control_repeat,
    ))(s)
}

#[parser]
pub fn delay_or_event_control_repeat(s: Span) -> IResult<Span, DelayOrEventControl> {
    let (s, a) = keyword("repeat")(s)?;
    let (s, b) = paren(expression)(s)?;
    let (s, c) = event_control(s)?;
    Ok((
        s,
        DelayOrEventControl::Repeat(Box::new(DelayOrEventControlRepeat { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn delay_control(s: Span) -> IResult<Span, DelayControl> {
    alt((delay_control_delay, delay_control_mintypmax))(s)
}

#[parser]
pub fn delay_control_delay(s: Span) -> IResult<Span, DelayControl> {
    let (s, a) = symbol("#")(s)?;
    let (s, b) = delay_value(s)?;
    Ok((
        s,
        DelayControl::Delay(Box::new(DelayControlDelay { nodes: (a, b) })),
    ))
}

#[parser]
pub fn delay_control_mintypmax(s: Span) -> IResult<Span, DelayControl> {
    let (s, a) = symbol("#")(s)?;
    let (s, b) = paren(mintypmax_expression)(s)?;
    Ok((
        s,
        DelayControl::Mintypmax(Box::new(DelayControlMintypmax { nodes: (a, b) })),
    ))
}

#[parser]
pub fn event_control(s: Span) -> IResult<Span, EventControl> {
    alt((
        event_control_event_identifier,
        event_control_event_expression,
        event_control_asterisk,
        event_control_paren_asterisk,
        event_control_sequence_identifier,
    ))(s)
}

#[parser]
pub fn event_control_event_identifier(s: Span) -> IResult<Span, EventControl> {
    let (s, a) = symbol("@")(s)?;
    let (s, b) = hierarchical_event_identifier(s)?;
    Ok((
        s,
        EventControl::EventIdentifier(Box::new(EventControlEventIdentifier { nodes: (a, b) })),
    ))
}

#[parser]
pub fn event_control_event_expression(s: Span) -> IResult<Span, EventControl> {
    let (s, a) = symbol("@")(s)?;
    let (s, b) = paren(event_expression)(s)?;
    Ok((
        s,
        EventControl::EventExpression(Box::new(EventControlEventExpression { nodes: (a, b) })),
    ))
}

#[parser]
pub fn event_control_asterisk(s: Span) -> IResult<Span, EventControl> {
    let (s, a) = symbol("@*")(s)?;
    Ok((
        s,
        EventControl::Asterisk(Box::new(EventControlAsterisk { nodes: (a,) })),
    ))
}

#[parser]
pub fn event_control_paren_asterisk(s: Span) -> IResult<Span, EventControl> {
    let (s, a) = symbol("@")(s)?;
    let (s, b) = paren(symbol("*"))(s)?;
    Ok((
        s,
        EventControl::ParenAsterisk(Box::new(EventControlParenAsterisk { nodes: (a, b) })),
    ))
}

#[parser]
pub fn event_control_sequence_identifier(s: Span) -> IResult<Span, EventControl> {
    let (s, a) = symbol("@")(s)?;
    let (s, b) = ps_or_hierarchical_sequence_identifier(s)?;
    Ok((
        s,
        EventControl::SequenceIdentifier(Box::new(EventControlSequenceIdentifier {
            nodes: (a, b),
        })),
    ))
}

#[parser]
pub fn event_expression(s: Span) -> IResult<Span, EventExpression> {
    alt((
        event_expression_expression,
        event_expression_sequence,
        event_expression_or,
        event_expression_comma,
        event_expression_paren,
    ))(s)
}

#[parser(MaybeRecursive)]
pub fn event_expression_expression(s: Span) -> IResult<Span, EventExpression> {
    let (s, a) = opt(edge_identifier)(s)?;
    let (s, b) = expression(s)?;
    let (s, c) = opt(pair(keyword("iff"), expression))(s)?;
    Ok((
        s,
        EventExpression::Expression(Box::new(EventExpressionExpression { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn event_expression_sequence(s: Span) -> IResult<Span, EventExpression> {
    let (s, a) = sequence_instance(s)?;
    let (s, b) = opt(pair(keyword("iff"), expression))(s)?;
    Ok((
        s,
        EventExpression::Sequence(Box::new(EventExpressionSequence { nodes: (a, b) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn event_expression_or(s: Span) -> IResult<Span, EventExpression> {
    let (s, a) = event_expression(s)?;
    let (s, b) = keyword("or")(s)?;
    let (s, c) = event_expression(s)?;
    Ok((
        s,
        EventExpression::Or(Box::new(EventExpressionOr { nodes: (a, b, c) })),
    ))
}

#[parser(MaybeRecursive)]
pub fn event_expression_comma(s: Span) -> IResult<Span, EventExpression> {
    let (s, a) = event_expression(s)?;
    let (s, b) = symbol(",")(s)?;
    let (s, c) = event_expression(s)?;
    Ok((
        s,
        EventExpression::Comma(Box::new(EventExpressionComma { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn event_expression_paren(s: Span) -> IResult<Span, EventExpression> {
    let (s, a) = paren(event_expression)(s)?;
    Ok((
        s,
        EventExpression::Paren(Box::new(EventExpressionParen { nodes: (a,) })),
    ))
}

#[parser]
pub fn procedural_timing_control(s: Span) -> IResult<Span, ProceduralTimingControl> {
    alt((
        map(delay_control, |x| {
            ProceduralTimingControl::DelayControl(Box::new(x))
        }),
        map(event_control, |x| {
            ProceduralTimingControl::EventControl(Box::new(x))
        }),
        map(cycle_delay, |x| {
            ProceduralTimingControl::CycleDelay(Box::new(x))
        }),
    ))(s)
}

#[parser]
pub fn jump_statement(s: Span) -> IResult<Span, JumpStatement> {
    alt((
        jump_statement_return,
        jump_statement_break,
        jump_statement_continue,
    ))(s)
}

#[parser]
pub fn jump_statement_return(s: Span) -> IResult<Span, JumpStatement> {
    let (s, a) = keyword("return")(s)?;
    let (s, b) = opt(expression)(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        JumpStatement::Return(Box::new(JumpStatementReturn { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn jump_statement_break(s: Span) -> IResult<Span, JumpStatement> {
    let (s, a) = keyword("break")(s)?;
    let (s, b) = symbol(";")(s)?;
    Ok((
        s,
        JumpStatement::Break(Box::new(JumpStatementBreak { nodes: (a, b) })),
    ))
}

#[parser]
pub fn jump_statement_continue(s: Span) -> IResult<Span, JumpStatement> {
    let (s, a) = keyword("continue")(s)?;
    let (s, b) = symbol(";")(s)?;
    Ok((
        s,
        JumpStatement::Continue(Box::new(JumpStatementContinue { nodes: (a, b) })),
    ))
}

#[parser]
pub fn wait_statement(s: Span) -> IResult<Span, WaitStatement> {
    alt((
        wait_statement_wait,
        wait_statement_fork,
        wait_statement_order,
    ))(s)
}

#[parser]
pub fn wait_statement_wait(s: Span) -> IResult<Span, WaitStatement> {
    let (s, a) = keyword("wait")(s)?;
    let (s, b) = paren(expression)(s)?;
    let (s, c) = statement_or_null(s)?;
    Ok((
        s,
        WaitStatement::Wait(Box::new(WaitStatementWait { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn wait_statement_fork(s: Span) -> IResult<Span, WaitStatement> {
    let (s, a) = keyword("wait")(s)?;
    let (s, b) = keyword("fork")(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        WaitStatement::Fork(Box::new(WaitStatementFork { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn wait_statement_order(s: Span) -> IResult<Span, WaitStatement> {
    let (s, a) = keyword("wait_order")(s)?;
    let (s, b) = paren(list(symbol(","), hierarchical_identifier))(s)?;
    let (s, c) = action_block(s)?;
    Ok((
        s,
        WaitStatement::Order(Box::new(WaitStatementOrder { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn event_trigger(s: Span) -> IResult<Span, EventTrigger> {
    alt((event_trigger_named, event_trigger_nonblocking))(s)
}

#[parser]
pub fn event_trigger_named(s: Span) -> IResult<Span, EventTrigger> {
    let (s, a) = symbol("->")(s)?;
    let (s, b) = hierarchical_event_identifier(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        EventTrigger::Named(Box::new(EventTriggerNamed { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn event_trigger_nonblocking(s: Span) -> IResult<Span, EventTrigger> {
    let (s, a) = symbol("->>")(s)?;
    let (s, b) = opt(delay_or_event_control)(s)?;
    let (s, c) = hierarchical_event_identifier(s)?;
    let (s, d) = symbol(";")(s)?;
    Ok((
        s,
        EventTrigger::Nonblocking(Box::new(EventTriggerNonblocking {
            nodes: (a, b, c, d),
        })),
    ))
}

#[parser]
pub fn disable_statement(s: Span) -> IResult<Span, DisableStatement> {
    alt((
        disable_statement_task,
        disable_statement_block,
        disable_statement_fork,
    ))(s)
}

#[parser]
pub fn disable_statement_task(s: Span) -> IResult<Span, DisableStatement> {
    let (s, a) = keyword("disable")(s)?;
    let (s, b) = hierarchical_task_identifier(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        DisableStatement::Task(Box::new(DisableStatementTask { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn disable_statement_block(s: Span) -> IResult<Span, DisableStatement> {
    let (s, a) = keyword("disable")(s)?;
    let (s, b) = hierarchical_block_identifier(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        DisableStatement::Block(Box::new(DisableStatementBlock { nodes: (a, b, c) })),
    ))
}

#[parser]
pub fn disable_statement_fork(s: Span) -> IResult<Span, DisableStatement> {
    let (s, a) = keyword("disable")(s)?;
    let (s, b) = keyword("fork")(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        DisableStatement::Fork(Box::new(DisableStatementFork { nodes: (a, b, c) })),
    ))
}

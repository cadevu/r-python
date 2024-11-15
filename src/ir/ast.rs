pub type Name = String;

pub enum Expression {
    CInt(i32),
    Var(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

pub enum Statement {
    VarDeclaration(Box<Name>),
    ValDeclaration(Box<Name>),
    Assignment(Box<Name>, Box<Expression>),
    IfThenElse(Box<Expression>, Box<Statement>, Box<Statement>),
    While(Box<Expression>, Box<Statement>),
    Sequence(Box<Statement>, Box<Statement>),
}

use syntax::Span;

#[derive(Debug)]
pub struct TypeExpr {
	pub span: Span,
	pub path: Vec<String>
}

#[derive(Debug)]
pub struct FunctionAnnotation {
	pub span: Span,
	pub name: String
}

#[derive(Debug)]
pub struct Function {
	pub span: Span,
	pub name: String,
	pub params: Vec<FunctionParam>,
	pub code: Vec<Code>,
	pub annotations: Vec<FunctionAnnotation>,
	pub return_types: Vec<TypeExpr>
}

#[derive(Debug)]
pub struct FunctionParam {
	pub span: Span,
	pub name: String,
	pub param_type: TypeExpr
}

#[derive(Debug)]
pub enum Expr {
	BinaryExpr(BinaryExpr),
	Name(NameExpr),
	Closed(ClosedExpr),
	NumberLit(NumberLitExpr),
	Call(CallExpr)
}

#[derive(Debug)]
pub struct CallExpr {
	pub span: Span,
	pub object: Box<Expr>,
	pub args: Vec<Expr>
}

#[derive(Debug)]
pub struct ClosedExpr {
	pub span: Span,
	pub expr: Box<Expr>
}

#[derive(Debug)]
pub struct NameExpr {
	pub span: Span,
	pub name: String
}

#[derive(Debug)]
pub struct NumberLitExpr {
	pub span: Span,
	pub number: String
}

#[derive(Debug)]
pub struct BinaryExpr {
	pub span: Span,
	pub op: BinaryOp,
	pub left: Box<Expr>,
	pub right: Box<Expr>
}

#[derive(Debug)]
pub enum BinaryOp {
	Add, Mul, Div, Sub,
	Eq, Ne, Lt, Le, Gt, Ge
}

#[derive(Debug)]
pub struct ReturnStmt {
	pub span: Span,
	pub expr: Option<Expr>
}

#[derive(Debug)]
pub struct VarDeclaration {
	pub span: Span,
	pub name: String,
	pub expr: Option<Expr>,
	pub var_type: Option<TypeExpr>
}

#[derive(Debug)]
pub enum Code {
	ReturnStmt(ReturnStmt),
	VarDeclaration(VarDeclaration),
	ExprStmt(Expr),
	Assignment(Assignment),
	IfStmt(IfStmt),
	ForStmt(ForStmt)
}

#[derive(Debug)]
pub struct IfStmt {
	pub span: Span,
	pub condition: Expr,
	pub code: Vec<Code>,
	pub else_code: Option<Vec<Code>>
}

#[derive(Debug)]
pub struct ForStmt {
	pub span: Span,
	pub init: Option<Box<Code>>,
	pub condition: Option<Expr>,
	pub inc: Option<Box<Code>>,
	pub code: Vec<Code>,
}

#[derive(Debug)]
pub struct Assignment {
	pub span: Span,
	pub left: Expr,
	pub right: Expr
}

#[derive(Debug)]
pub enum TopLevelNode {
	Function(Function),
}

#[derive(Debug)]
pub struct TranslationUnit {
	pub nodes: Vec<TopLevelNode>
}
# zlang
a language implemented in rust

## grammar

### Node Objects
#### Node
```
struct Node {
  type: String,
  loc: SourceLocation | None
}
```

#### Location
```
struct Location {
  row: usize,
  column: usize
}
```

#### SourceLocation
```
struct SourceLocation {
  source: String | None,
  start: Location,
  end: Location
}
```

### Programs
```
struct Program <: Node {
  type: "Program",
  body: Vec<Statement>
}
```

### Functions
```
struct Property <: Node {
  type: "property",
  id: Identifier | None
}

struct Function <: Node {
  type: "Function",
  id: Identifier | None,
  params: Vec<Property>,
  body: BlockStatement
}
```

### Statements
```
struct Statement <: Node { }
```

#### EmptyStatement
```
// empty statement
struct EmptyStatement <: Statement {
  type: "EmptyStatement"
}

```
#### BlockStatement
```
struct BlockStatement <: Statement {
  type: "BlockStatement",
  body: Vec<Statement>
}
```

#### ExpressionStatement
```
struct ExpressionStatement <: Statement {
  type: "ExpressionStatement",
  expression: Expression
}
```

#### IfStatement
```
struct IfStatement <: Statement {
  type: "IfStatement",
  test: Expression,
  consequent: Statement,
  alternate: Statement | None
}
```

#### ReturnStatement
```
struct ReturnStatement <: Statement {
  type: "ReturnStatement",
  argument: Expression | None
}
```

#### WhileStatement
```
struct WhileStatement <: Statement {
  type: "WhileStatement",
  test: Expression,
  body: BlockStatement
}
```

#### ForInStatement
```
struct ForInStatement <: Statement {
  type: "ForInStatement",
  left: Expression,
  right: Expression,
  body: BlockStatement,
  each: boolean
}
```

### Expression
```
struct Expression <: Node {
}
```

#### ArrayExpression
```
struct ArrayExpression <: Expression {
  type: "ArrayExpression",
  elements: Vec<Expression>
}
```

#### AssignmentExpression
```
struct AssignmentExpression <: Expression {
  type: "AssignmentExpression",
  opertaor: AssignmentOperator,
  left: Expression,
  right: Expression
}
```

#### LogicalExpression
```
struct LogicalExpression <: Expression {
  type: "LogicalExpression",
  operator: LogicalOperator,
  left: Expression,
  right: Expression
}
```

#### CallExpression
```
struct CallExpression <: Expression {
  type: "CallExpression",
  callee: Expression,
  arguments: [ Expression | None ]
}
```

### Miscellaneous

#### AssignmentOperator
```
enum AssignmentOperator {
  "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "<<=" | ">>=" | "|=" | "^=" | "&="
}
```

#### IdentifierOperator
```
struct Identifier <: Node, Expression {
  type: "Identifier",
  name: String
}
```

#### Literal


#### LogicalOperator


#### Property


#### UnaryOperator


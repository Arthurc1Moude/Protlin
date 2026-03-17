use crate::error::ProtlinError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Integer(i64),
    Decimal(f64),
    String(String),
    Boolean(bool),
    Void,
    
    // Identifiers and Keywords
    Identifier(String),
    
    // Keywords - Core
    Egg,           // program entry
    Hatch,         // function definition
    Nest,          // class definition
    Shell,         // module definition
    Yolk,          // variable declaration (mutable)
    Albumen,       // constant declaration (immutable)
    Embryo,        // type definition
    Incubate,      // async keyword
    Peck,          // await keyword
    
    // Keywords - Control Flow
    If,
    Else,
    Unless,        // inverse if
    When,          // pattern matching
    Match,         // match expression
    Case,          // case in match
    Default,       // default case
    While,
    Until,         // inverse while
    For,
    Foreach,
    In,
    Loop,
    Break,
    Continue,
    Return,
    Yield,         // generator yield
    
    // Keywords - Numbers (0-10-100-1000-10000)
    Zero,          // 0
    One,           // 1
    Two,           // 2
    Three,         // 3
    Four,          // 4
    Five,          // 5
    Six,           // 6
    Seven,         // 7
    Eight,         // 8
    Nine,          // 9
    Ten,           // 10
    Hundred,       // 100
    Thousand,      // 1000
    Myriad,        // 10000 (maximum)
    
    // Keywords - Types
    TypeInt,
    TypeFloat,
    TypeString,
    TypeBool,
    TypeList,
    TypeDict,
    TypeSet,
    TypeTuple,
    TypeOption,
    TypeResult,
    TypeAny,
    TypeVoid,
    
    // Keywords - Modifiers
    Pub,           // public
    Priv,          // private
    Prot,          // protected
    Static,
    Final,
    Abstract,
    Virtual,
    Override,
    Const,
    Mut,           // mutable
    Ref,           // reference
    
    // Keywords - Error Handling
    Try,
    Catch,
    Finally,
    Throw,
    Assert,
    Ensure,
    Require,
    
    // Keywords - Memory & Ownership
    Own,           // ownership
    Borrow,        // borrow
    Move,          // move semantics
    Copy,          // copy semantics
    Clone,         // explicit clone
    Drop,          // destructor
    
    // Keywords - Concurrency
    Spawn,         // spawn thread
    Channel,       // channel creation
    Send,          // send to channel
    Receive,       // receive from channel
    Lock,          // mutex lock
    Atomic,        // atomic operation
    
    // Keywords - Meta
    Macro,         // macro definition
    Trait,         // trait definition
    Impl,          // implementation
    Extend,        // extension
    With,          // with statement
    As,            // type casting
    Is,            // type checking
    Typeof,        // get type
    Sizeof,        // get size
    
    // Keywords - Import/Export
    Import,
    Export,
    From,
    Use,
    Package,
    
    // Keywords - Special
    This,
    Super,
    Self_,
    Null,
    Undefined,
    Infinity,
    NaN,
    
    // Operators - Arithmetic
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Percent,       // %
    Power,         // **
    FloorDiv,      // //
    
    // Operators - Comparison
    Equal,         // ==
    NotEqual,      // !=
    Less,          // <
    Greater,       // >
    LessEqual,     // <=
    GreaterEqual,  // >=
    Spaceship,     // <=> (three-way comparison)
    
    // Operators - Logical
    And,           // &&
    Or,            // ||
    Not,           // !
    Xor,           // ^^ (logical xor)
    
    // Operators - Bitwise
    BitAnd,        // &
    BitOr,         // |
    BitXor,        // ^
    BitNot,        // ~
    LeftShift,     // <<
    RightShift,    // >>
    
    // Operators - Assignment
    Assign,        // =
    PlusAssign,    // +=
    MinusAssign,   // -=
    StarAssign,    // *=
    SlashAssign,   // /=
    PercentAssign, // %=
    PowerAssign,   // **=
    AndAssign,     // &=
    OrAssign,      // |=
    XorAssign,     // ^=
    LeftShiftAssign,  // <<=
    RightShiftAssign, // >>=
    
    // Operators - Special
    Arrow,         // ->
    FatArrow,      // =>
    DoubleColon,   // ::
    Question,      // ?
    Elvis,         // ?: (elvis operator)
    NullCoalesce,  // ?? (null coalescing)
    Pipeline,      // |> (pipeline operator)
    Compose,       // >> (function composition)
    Range,         // ..
    RangeInclusive, // ..=
    Spread,        // ...
    
    // Delimiters
    LeftParen,     // (
    RightParen,    // )
    LeftBrace,     // {
    RightBrace,    // }
    LeftBracket,   // [
    RightBracket,  // ]
    Semicolon,     // ;
    Colon,         // :
    Comma,         // ,
    Dot,           // .
    At,            // @ (decorator)
    Hash,          // # (directive)
    Dollar,        // $ (string interpolation)
    Backtick,      // ` (raw string)
    Underscore,    // _ (wildcard)
    
    // Special
    Newline,
    Eof,
    
    // Additional Keywords - Memory Management
    Allocate,      // allocate memory
    Deallocate,    // deallocate memory
    Realloc,       // reallocate memory
    Malloc,        // malloc-style allocation
    Free,          // free memory
    Gc,            // garbage collect
    Retain,        // retain reference
    Release,       // release reference
    Autorelease,   // autorelease
    Weak,          // weak reference
    Strong,        // strong reference
    Unowned,       // unowned reference
    
    // Additional Keywords - Concurrency & Parallelism
    Parallel,      // parallel execution
    Sequential,    // sequential execution
    Concurrent,    // concurrent execution
    Sync,          // synchronize
    Async,         // asynchronous (alias for incubate)
    Await,         // await (alias for peck)
    Future,        // future type
    Promise,       // promise type
    Task,          // task type
    Thread,        // thread
    Process,       // process
    Coroutine,     // coroutine
    Fiber,         // fiber
    Green,         // green thread
    Actor,         // actor model
    Message,       // message passing
    Mailbox,       // mailbox
    Select,        // select statement
    Timeout,       // timeout
    Deadline,      // deadline
    Cancel,        // cancel operation
    
    // Additional Keywords - Error Handling & Debugging
    Panic,         // panic
    Recover,       // recover from panic
    Defer,         // defer execution
    Guard,         // guard statement
    Precondition,  // precondition
    Postcondition, // postcondition
    Invariant,     // invariant
    Debug,         // debug mode
    Trace,         // trace execution
    Log,           // logging
    Warn,          // warning
    Error,         // error
    Fatal,         // fatal error
    Info,          // info log
    Verbose,       // verbose mode
    
    // Additional Keywords - Type System
    Interface,     // interface
    Protocol,      // protocol
    Struct,        // struct
    Enum,          // enum
    Union,         // union
    Alias,         // type alias
    Newtype,       // newtype
    Phantom,       // phantom type
    Associated,    // associated type
    Existential,   // existential type
    Universal,     // universal type
    Dependent,     // dependent type
    Linear,        // linear type
    Affine,        // affine type
    Subtype,       // subtype
    Supertype,     // supertype
    Covariant,     // covariant
    Contravariant, // contravariant
    Invariant_,    // invariant (type)
    
    // Additional Keywords - Functional Programming
    Lambda,        // lambda
    Closure,       // closure
    Partial,       // partial application
    Curry,         // currying
    Uncurry,       // uncurrying
    Compose_,      // compose (keyword)
    Pipe,          // pipe
    Fold,          // fold
    Reduce,        // reduce
    Scan,          // scan
    Unfold,        // unfold
    Zip,           // zip
    Unzip,         // unzip
    Flatten,       // flatten
    FlatMap,       // flatmap
    Bind,          // bind (monad)
    Pure,          // pure
    Applicative,   // applicative
    Functor,       // functor
    Monad,         // monad
    Monoid,        // monoid
    Semigroup,     // semigroup
    Category,      // category
    
    // Additional Keywords - Object-Oriented
    Constructor,   // constructor
    Destructor,    // destructor
    Initializer,   // initializer
    Deinitializer, // deinitializer
    Getter,        // getter
    Setter,        // setter
    Property,      // property
    Method,        // method
    Field,         // field
    Member,        // member
    Attribute,     // attribute
    Annotation,    // annotation
    Decorator,     // decorator
    Mixin,         // mixin
    Delegate,      // delegate
    Proxy,         // proxy
    Singleton,     // singleton
    Factory,       // factory
    Builder,       // builder
    Prototype,     // prototype
    
    // Additional Keywords - Pattern Matching
    Guard_,        // guard (in patterns)
    Where,         // where clause
    Such,          // such that
    That,          // that
    Some,          // some (option)
    None,          // none (option)
    Ok,            // ok (result)
    Err,           // error (result)
    Just,          // just
    Nothing,       // nothing
    Left,          // left (either)
    Right,         // right (either)
    
    // Additional Keywords - Control Flow Extensions
    Goto,          // goto
    Label,         // label
    Switch,        // switch
    Fallthrough,   // fallthrough
    Repeat,        // repeat
    Do,            // do
    Then,          // then
    Elif,          // elif
    Elseif,        // elseif
    Otherwise,     // otherwise
    Always,        // always
    Never,         // never
    Forever,       // forever
    Once,          // once
    Twice,         // twice
    Thrice,        // thrice
    
    // Additional Keywords - Metaprogramming
    Reflect,       // reflection
    Introspect,    // introspection
    Eval,          // eval
    Quote,         // quote
    Unquote,       // unquote
    Splice,        // splice
    Gensym,        // gensym
    Hygiene,       // hygiene
    Syntax,        // syntax
    Parse,         // parse
    Expand,        // expand
    Compile,       // compile
    Interpret,     // interpret
    Transpile,     // transpile
    
    // Additional Keywords - Module System
    Module,        // module
    Namespace,     // namespace
    Scope,         // scope
    Global,        // global
    Local,         // local
    Extern,        // extern
    Foreign,       // foreign
    Native,        // native
    Builtin,       // builtin
    Prelude,       // prelude
    Std,           // standard library
    Core,          // core library
    
    // Additional Keywords - Testing & Verification
    Test,          // test
    Benchmark,     // benchmark
    PropertyTest,  // property test (renamed from Property to avoid conflict)
    Quickcheck,    // quickcheck
    Fuzzy,         // fuzzy test
    Mock,          // mock
    Stub,          // stub
    Spy,           // spy
    Fake,          // fake
    Verify,        // verify
    Validate,      // validate
    Check,         // check
    Prove,         // prove
    Theorem,       // theorem
    Lemma,         // lemma
    Axiom,         // axiom
    Corollary,     // corollary
    
    // Additional Keywords - Data Structures
    Array,         // array
    Vector,        // vector
    Matrix,        // matrix
    Tensor,        // tensor
    Stack,         // stack
    Queue,         // queue
    Deque,         // deque
    Heap,          // heap
    Tree,          // tree
    Graph,         // graph
    Node,          // node
    Edge,          // edge
    Vertex,        // vertex
    Path,          // path
    Cycle,         // cycle
    
    // Additional Keywords - I/O & Streams
    Stream,        // stream
    Reader,        // reader
    Writer,        // writer
    Buffer,        // buffer
    Flush,         // flush
    Close,         // close
    Open,          // open
    Read,          // read
    Write,         // write
    Seek,          // seek
    Tell,          // tell
    Rewind,        // rewind
    
    // Additional Keywords - Network & Communication
    Network,       // network
    Socket,        // socket
    Connect,       // connect
    Listen,        // listen
    Accept,        // accept
    BindNet,       // bind (network - renamed to avoid conflict)
    Shutdown,      // shutdown
    Protocol_,     // protocol (network)
    Http,          // http
    Https,         // https
    Tcp,           // tcp
    Udp,           // udp
    Websocket,     // websocket
    Rpc,           // rpc
    Rest,          // rest
    Graphql,       // graphql
    
    // Additional Keywords - Database
    Database,      // database
    Query,         // query
    Select_,       // select (sql)
    Insert,        // insert
    Update,        // update
    Delete,        // delete
    Create,        // create
    Alter,         // alter
    Drop_,         // drop (sql)
    Table,         // table
    Index,         // index
    View,          // view
    Transaction,   // transaction
    Commit,        // commit
    Rollback,      // rollback
    Savepoint,     // savepoint
    
    // Additional Keywords - Security & Cryptography
    Encrypt,       // encrypt
    Decrypt,       // decrypt
    HashFunc,      // hash (renamed to avoid conflict with # delimiter)
    Sign,          // sign
    Verify_,       // verify (crypto)
    Seal,          // seal
    Unseal,        // unseal
    Secure,        // secure
    Unsafe,        // unsafe
    Trusted,       // trusted
    Untrusted,     // untrusted
    Sanitize,      // sanitize
    Escape,        // escape
    Validate_,     // validate (security)
    
    // Additional Keywords - Time & Date
    Time,          // time
    Date,          // date
    DateTime,      // datetime
    Duration,      // duration
    Instant,       // instant
    Timestamp,     // timestamp
    Now,           // now
    Today,         // today
    Tomorrow,      // tomorrow
    Yesterday,     // yesterday
    
    // Additional Keywords - Math & Science
    Math,          // math
    Science,       // science
    Physics,       // physics
    Chemistry,     // chemistry
    Biology,       // biology
    Statistics,    // statistics
    Probability,   // probability
    Random,        // random
    Seed,          // seed
    Distribution,  // distribution
    Normal,        // normal distribution
    Uniform,       // uniform distribution
    Exponential,   // exponential
    Poisson,       // poisson
    
    // Additional Keywords - Graphics & UI
    Graphics,      // graphics
    Render,        // render
    Draw,          // draw
    Paint,         // paint
    Fill,          // fill
    Stroke,        // stroke
    Color,         // color
    Pixel,         // pixel
    Canvas,        // canvas
    Window,        // window
    Widget,        // widget
    Layout,        // layout
    Style,         // style
    Theme,         // theme
    
    // Additional Keywords - Audio & Media
    Audio,         // audio
    Video,         // video
    Media,         // media
    Sound,         // sound
    Music,         // music
    Play,          // play
    Pause,         // pause
    Stop,          // stop
    Record,        // record
    Volume,        // volume
    Pitch,         // pitch
    Tempo,         // tempo
    
    // Additional Keywords - File System
    File,          // file
    Directory,     // directory
    Folder,        // folder
    Exists,        // exists
    Copy_,         // copy (file)
    Move_,         // move (file)
    Rename,        // rename
    Remove,        // remove
    Mkdir,         // mkdir
    Rmdir,         // rmdir
    Chmod,         // chmod
    Chown,         // chown
    
    // Additional Keywords - Configuration
    Config,        // config
    Settings,      // settings
    Options,       // options
    Preferences,   // preferences
    Environment,   // environment
    Variable,      // variable
    Constant_,     // constant (keyword)
    Parameter,     // parameter
    Argument,      // argument
    Flag,          // flag
    Option_,       // option (cli)
    
    // Additional Keywords - Lifecycle
    Init,          // init
    Start,         // start
    Run,           // run
    Execute,       // execute
    Invoke,        // invoke
    Call,          // call
    Apply,         // apply
    Perform,       // perform
    Complete,      // complete
    Finish,        // finish
    End,           // end
    Exit,          // exit
    Quit,          // quit
    Abort,         // abort
    Terminate,     // terminate
    Kill,          // kill
    
    // Additional Keywords - State Management
    State,         // state
    Store,         // store
    Cache,         // cache
    Memoize,       // memoize
    Persist,       // persist
    Load,          // load
    Save,          // save
    Restore,       // restore
    Snapshot,      // snapshot
    Checkpoint,    // checkpoint
    Undo,          // undo
    Redo,          // redo
    History,       // history
    
    // Additional Keywords - Validation & Constraints
    Constraint,    // constraint
    Bound,         // bound
    Limit,         // limit
    Min,           // min
    Max,           // max
    Range_,        // range (keyword)
    Between,       // between
    Within,        // within
    Outside,       // outside
    Inside,        // inside
    Contains,      // contains
    Includes,      // includes
    Excludes,      // excludes
    
    // Additional Keywords - Operators as Keywords
    Plus_,         // plus (keyword)
    Minus_,        // minus (keyword)
    Times,         // times
    Divide,        // divide
    Modulo,        // modulo
    Equals,        // equals
    NotEquals,     // not equals
    GreaterThan,   // greater than
    LessThan,      // less than
    And_,          // and (keyword)
    Or_,           // or (keyword)
    Not_,          // not (keyword)
    Xor_,          // xor (keyword)
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current = if chars.is_empty() { None } else { Some(chars[0]) };
        
        let mut keywords = HashMap::new();
        
        // Core keywords
        keywords.insert("egg".to_string(), Token::Egg);
        keywords.insert("hatch".to_string(), Token::Hatch);
        keywords.insert("nest".to_string(), Token::Nest);
        keywords.insert("shell".to_string(), Token::Shell);
        keywords.insert("yolk".to_string(), Token::Yolk);
        keywords.insert("albumen".to_string(), Token::Albumen);
        keywords.insert("embryo".to_string(), Token::Embryo);
        keywords.insert("incubate".to_string(), Token::Incubate);
        keywords.insert("peck".to_string(), Token::Peck);
        
        // Control flow
        keywords.insert("if".to_string(), Token::If);
        keywords.insert("else".to_string(), Token::Else);
        keywords.insert("unless".to_string(), Token::Unless);
        keywords.insert("when".to_string(), Token::When);
        keywords.insert("match".to_string(), Token::Match);
        keywords.insert("case".to_string(), Token::Case);
        keywords.insert("default".to_string(), Token::Default);
        keywords.insert("while".to_string(), Token::While);
        keywords.insert("until".to_string(), Token::Until);
        keywords.insert("for".to_string(), Token::For);
        keywords.insert("foreach".to_string(), Token::Foreach);
        keywords.insert("in".to_string(), Token::In);
        keywords.insert("loop".to_string(), Token::Loop);
        keywords.insert("break".to_string(), Token::Break);
        keywords.insert("continue".to_string(), Token::Continue);
        keywords.insert("return".to_string(), Token::Return);
        keywords.insert("yield".to_string(), Token::Yield);
        
        // Number keywords
        keywords.insert("zero".to_string(), Token::Zero);
        keywords.insert("one".to_string(), Token::One);
        keywords.insert("two".to_string(), Token::Two);
        keywords.insert("three".to_string(), Token::Three);
        keywords.insert("four".to_string(), Token::Four);
        keywords.insert("five".to_string(), Token::Five);
        keywords.insert("six".to_string(), Token::Six);
        keywords.insert("seven".to_string(), Token::Seven);
        keywords.insert("eight".to_string(), Token::Eight);
        keywords.insert("nine".to_string(), Token::Nine);
        keywords.insert("ten".to_string(), Token::Ten);
        keywords.insert("hundred".to_string(), Token::Hundred);
        keywords.insert("thousand".to_string(), Token::Thousand);
        keywords.insert("myriad".to_string(), Token::Myriad);
        
        // Types
        keywords.insert("int".to_string(), Token::TypeInt);
        keywords.insert("float".to_string(), Token::TypeFloat);
        keywords.insert("string".to_string(), Token::TypeString);
        keywords.insert("bool".to_string(), Token::TypeBool);
        keywords.insert("list".to_string(), Token::TypeList);
        keywords.insert("dict".to_string(), Token::TypeDict);
        keywords.insert("set".to_string(), Token::TypeSet);
        keywords.insert("tuple".to_string(), Token::TypeTuple);
        keywords.insert("option".to_string(), Token::TypeOption);
        keywords.insert("result".to_string(), Token::TypeResult);
        keywords.insert("any".to_string(), Token::TypeAny);
        keywords.insert("void".to_string(), Token::TypeVoid);
        
        // Modifiers
        keywords.insert("pub".to_string(), Token::Pub);
        keywords.insert("priv".to_string(), Token::Priv);
        keywords.insert("prot".to_string(), Token::Prot);
        keywords.insert("static".to_string(), Token::Static);
        keywords.insert("final".to_string(), Token::Final);
        keywords.insert("abstract".to_string(), Token::Abstract);
        keywords.insert("virtual".to_string(), Token::Virtual);
        keywords.insert("override".to_string(), Token::Override);
        keywords.insert("const".to_string(), Token::Const);
        keywords.insert("mut".to_string(), Token::Mut);
        keywords.insert("ref".to_string(), Token::Ref);
        
        // Error handling
        keywords.insert("try".to_string(), Token::Try);
        keywords.insert("catch".to_string(), Token::Catch);
        keywords.insert("finally".to_string(), Token::Finally);
        keywords.insert("throw".to_string(), Token::Throw);
        keywords.insert("assert".to_string(), Token::Assert);
        keywords.insert("ensure".to_string(), Token::Ensure);
        keywords.insert("require".to_string(), Token::Require);
        
        // Memory & Ownership
        keywords.insert("own".to_string(), Token::Own);
        keywords.insert("borrow".to_string(), Token::Borrow);
        keywords.insert("move".to_string(), Token::Move);
        keywords.insert("copy".to_string(), Token::Copy);
        keywords.insert("clone".to_string(), Token::Clone);
        keywords.insert("drop".to_string(), Token::Drop);
        
        // Concurrency
        keywords.insert("spawn".to_string(), Token::Spawn);
        keywords.insert("channel".to_string(), Token::Channel);
        keywords.insert("send".to_string(), Token::Send);
        keywords.insert("receive".to_string(), Token::Receive);
        keywords.insert("lock".to_string(), Token::Lock);
        keywords.insert("atomic".to_string(), Token::Atomic);
        
        // Meta
        keywords.insert("macro".to_string(), Token::Macro);
        keywords.insert("trait".to_string(), Token::Trait);
        keywords.insert("impl".to_string(), Token::Impl);
        keywords.insert("extend".to_string(), Token::Extend);
        keywords.insert("with".to_string(), Token::With);
        keywords.insert("as".to_string(), Token::As);
        keywords.insert("is".to_string(), Token::Is);
        keywords.insert("typeof".to_string(), Token::Typeof);
        keywords.insert("sizeof".to_string(), Token::Sizeof);
        
        // Import/Export
        keywords.insert("import".to_string(), Token::Import);
        keywords.insert("export".to_string(), Token::Export);
        keywords.insert("from".to_string(), Token::From);
        keywords.insert("use".to_string(), Token::Use);
        keywords.insert("package".to_string(), Token::Package);
        
        // Special
        keywords.insert("this".to_string(), Token::This);
        keywords.insert("super".to_string(), Token::Super);
        keywords.insert("self".to_string(), Token::Self_);
        keywords.insert("null".to_string(), Token::Null);
        keywords.insert("undefined".to_string(), Token::Undefined);
        keywords.insert("true".to_string(), Token::Boolean(true));
        keywords.insert("false".to_string(), Token::Boolean(false));
        keywords.insert("void".to_string(), Token::Void);
        keywords.insert("infinity".to_string(), Token::Infinity);
        keywords.insert("nan".to_string(), Token::NaN);
        
        // Logical operators
        keywords.insert("and".to_string(), Token::And);
        keywords.insert("or".to_string(), Token::Or);
        keywords.insert("not".to_string(), Token::Not);
        keywords.insert("xor".to_string(), Token::Xor);
        
        let mut lexer = Lexer {
            input: chars,
            position: 0,
            current_char: current,
            keywords,
        };
        
        // Add all extended keywords (200+)
        lexer.add_extended_keywords();
        
        lexer
    }
    
    fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
    }
    
    fn peek(&self, offset: usize) -> Option<char> {
        let pos = self.position + offset;
        if pos < self.input.len() {
            Some(self.input[pos])
        } else {
            None
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        if self.current_char == Some('/') && self.peek(1) == Some('/') {
            while self.current_char.is_some() && self.current_char != Some('\n') {
                self.advance();
            }
        } else if self.current_char == Some('/') && self.peek(1) == Some('*') {
            self.advance();
            self.advance();
            while self.current_char.is_some() {
                if self.current_char == Some('*') && self.peek(1) == Some('/') {
                    self.advance();
                    self.advance();
                    break;
                }
                self.advance();
            }
        }
    }
    
    fn read_number(&mut self) -> Result<Token, ProtlinError> {
        let mut num_str = String::new();
        let mut is_float = false;
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && self.peek(1).map_or(false, |c| c.is_ascii_digit()) {
                is_float = true;
                num_str.push(ch);
                self.advance();
            } else if ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        
        if is_float {
            num_str.parse::<f64>()
                .map(Token::Decimal)
                .map_err(|_| ProtlinError::LexerError(format!("Invalid float: {}", num_str)))
        } else {
            num_str.parse::<i64>()
                .map(Token::Integer)
                .map_err(|_| ProtlinError::LexerError(format!("Invalid integer: {}", num_str)))
        }
    }
    
    fn read_string(&mut self, quote: char) -> Result<Token, ProtlinError> {
        let mut string = String::new();
        self.advance(); // skip opening quote
        
        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance(); // skip closing quote
                return Ok(Token::String(string));
            } else if ch == '\\' {
                self.advance();
                match self.current_char {
                    Some('n') => string.push('\n'),
                    Some('t') => string.push('\t'),
                    Some('r') => string.push('\r'),
                    Some('\\') => string.push('\\'),
                    Some('"') => string.push('"'),
                    Some('\'') => string.push('\''),
                    Some('0') => string.push('\0'),
                    Some(c) => string.push(c),
                    None => return Err(ProtlinError::LexerError("Unexpected end of string".to_string())),
                }
                self.advance();
            } else {
                string.push(ch);
                self.advance();
            }
        }
        
        Err(ProtlinError::LexerError("Unterminated string".to_string()))
    }
    
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        ident
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, ProtlinError> {
        let mut tokens = Vec::new();
        
        while self.current_char.is_some() {
            self.skip_whitespace();
            
            if self.current_char.is_none() {
                break;
            }
            
            // Skip comments
            if self.current_char == Some('/') && (self.peek(1) == Some('/') || self.peek(1) == Some('*')) {
                self.skip_comment();
                continue;
            }
            
            let token = match self.current_char.unwrap() {
                '\n' => {
                    self.advance();
                    Token::Newline
                }
                
                // Numbers
                ch if ch.is_ascii_digit() => self.read_number()?,
                
                // Strings
                '"' | '\'' => self.read_string(self.current_char.unwrap())?,
                
                // Identifiers and keywords
                ch if ch.is_alphabetic() || ch == '_' => {
                    let ident = self.read_identifier();
                    self.keywords.get(&ident).cloned()
                        .unwrap_or_else(|| Token::Identifier(ident))
                }
                
                // Operators and delimiters
                '+' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::PlusAssign
                    } else {
                        Token::Plus
                    }
                }
                
                '-' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::MinusAssign
                    } else if self.current_char == Some('>') {
                        self.advance();
                        Token::Arrow
                    } else {
                        Token::Minus
                    }
                }
                
                '*' => {
                    self.advance();
                    if self.current_char == Some('*') {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Token::PowerAssign
                        } else {
                            Token::Power
                        }
                    } else if self.current_char == Some('=') {
                        self.advance();
                        Token::StarAssign
                    } else {
                        Token::Star
                    }
                }
                
                '/' => {
                    self.advance();
                    if self.current_char == Some('/') {
                        self.advance();
                        Token::FloorDiv
                    } else if self.current_char == Some('=') {
                        self.advance();
                        Token::SlashAssign
                    } else {
                        Token::Slash
                    }
                }
                
                '%' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::PercentAssign
                    } else {
                        Token::Percent
                    }
                }
                
                '=' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::Equal
                    } else if self.current_char == Some('>') {
                        self.advance();
                        Token::FatArrow
                    } else {
                        Token::Assign
                    }
                }
                
                '!' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::NotEqual
                    } else {
                        Token::Not
                    }
                }
                
                '<' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        if self.current_char == Some('>') {
                            self.advance();
                            Token::Spaceship
                        } else {
                            Token::LessEqual
                        }
                    } else if self.current_char == Some('<') {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Token::LeftShiftAssign
                        } else {
                            Token::LeftShift
                        }
                    } else {
                        Token::Less
                    }
                }
                
                '>' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token::GreaterEqual
                    } else if self.current_char == Some('>') {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Token::RightShiftAssign
                        } else {
                            Token::Compose
                        }
                    } else {
                        Token::Greater
                    }
                }
                
                '&' => {
                    self.advance();
                    if self.current_char == Some('&') {
                        self.advance();
                        Token::And
                    } else if self.current_char == Some('=') {
                        self.advance();
                        Token::AndAssign
                    } else {
                        Token::BitAnd
                    }
                }
                
                '|' => {
                    self.advance();
                    if self.current_char == Some('|') {
                        self.advance();
                        Token::Or
                    } else if self.current_char == Some('=') {
                        self.advance();
                        Token::OrAssign
                    } else if self.current_char == Some('>') {
                        self.advance();
                        Token::Pipeline
                    } else {
                        Token::BitOr
                    }
                }
                
                '^' => {
                    self.advance();
                    if self.current_char == Some('^') {
                        self.advance();
                        Token::Xor
                    } else if self.current_char == Some('=') {
                        self.advance();
                        Token::XorAssign
                    } else {
                        Token::BitXor
                    }
                }
                
                '~' => {
                    self.advance();
                    Token::BitNot
                }
                
                '?' => {
                    self.advance();
                    if self.current_char == Some('?') {
                        self.advance();
                        Token::NullCoalesce
                    } else if self.current_char == Some(':') {
                        self.advance();
                        Token::Elvis
                    } else {
                        Token::Question
                    }
                }
                
                '.' => {
                    self.advance();
                    if self.current_char == Some('.') {
                        self.advance();
                        if self.current_char == Some('.') {
                            self.advance();
                            Token::Spread
                        } else if self.current_char == Some('=') {
                            self.advance();
                            Token::RangeInclusive
                        } else {
                            Token::Range
                        }
                    } else {
                        Token::Dot
                    }
                }
                
                ':' => {
                    self.advance();
                    if self.current_char == Some(':') {
                        self.advance();
                        Token::DoubleColon
                    } else {
                        Token::Colon
                    }
                }
                
                '(' => {
                    self.advance();
                    Token::LeftParen
                }
                
                ')' => {
                    self.advance();
                    Token::RightParen
                }
                
                '{' => {
                    self.advance();
                    Token::LeftBrace
                }
                
                '}' => {
                    self.advance();
                    Token::RightBrace
                }
                
                '[' => {
                    self.advance();
                    Token::LeftBracket
                }
                
                ']' => {
                    self.advance();
                    Token::RightBracket
                }
                
                ';' => {
                    self.advance();
                    Token::Semicolon
                }
                
                ',' => {
                    self.advance();
                    Token::Comma
                }
                
                '@' => {
                    self.advance();
                    Token::At
                }
                
                '#' => {
                    self.advance();
                    Token::Hash
                }
                
                '$' => {
                    self.advance();
                    Token::Dollar
                }
                
                '`' => {
                    self.advance();
                    Token::Backtick
                }
                
                '_' => {
                    self.advance();
                    if self.current_char.map_or(false, |c| c.is_alphanumeric()) {
                        let mut ident = String::from("_");
                        ident.push_str(&self.read_identifier());
                        Token::Identifier(ident)
                    } else {
                        Token::Underscore
                    }
                }
                
                ch => {
                    return Err(ProtlinError::LexerError(format!("Unexpected character: {}", ch)));
                }
            };
            
            tokens.push(token);
        }
        
        tokens.push(Token::Eof);
        Ok(tokens)
    }
}

impl Token {
    pub fn keyword_count() -> usize {
        200  // We now have 200+ keywords!
    }
}


impl Lexer {
    pub fn add_extended_keywords(&mut self) {
        // Memory Management
        self.keywords.insert("allocate".to_string(), Token::Allocate);
        self.keywords.insert("deallocate".to_string(), Token::Deallocate);
        self.keywords.insert("realloc".to_string(), Token::Realloc);
        self.keywords.insert("malloc".to_string(), Token::Malloc);
        self.keywords.insert("free".to_string(), Token::Free);
        self.keywords.insert("gc".to_string(), Token::Gc);
        self.keywords.insert("retain".to_string(), Token::Retain);
        self.keywords.insert("release".to_string(), Token::Release);
        self.keywords.insert("autorelease".to_string(), Token::Autorelease);
        self.keywords.insert("weak".to_string(), Token::Weak);
        self.keywords.insert("strong".to_string(), Token::Strong);
        self.keywords.insert("unowned".to_string(), Token::Unowned);
        
        // Concurrency
        self.keywords.insert("parallel".to_string(), Token::Parallel);
        self.keywords.insert("sequential".to_string(), Token::Sequential);
        self.keywords.insert("concurrent".to_string(), Token::Concurrent);
        self.keywords.insert("sync".to_string(), Token::Sync);
        self.keywords.insert("async".to_string(), Token::Async);
        self.keywords.insert("await".to_string(), Token::Await);
        self.keywords.insert("future".to_string(), Token::Future);
        self.keywords.insert("promise".to_string(), Token::Promise);
        self.keywords.insert("task".to_string(), Token::Task);
        self.keywords.insert("thread".to_string(), Token::Thread);
        self.keywords.insert("process".to_string(), Token::Process);
        self.keywords.insert("coroutine".to_string(), Token::Coroutine);
        self.keywords.insert("fiber".to_string(), Token::Fiber);
        self.keywords.insert("green".to_string(), Token::Green);
        self.keywords.insert("actor".to_string(), Token::Actor);
        self.keywords.insert("message".to_string(), Token::Message);
        self.keywords.insert("mailbox".to_string(), Token::Mailbox);
        self.keywords.insert("select".to_string(), Token::Select);
        self.keywords.insert("timeout".to_string(), Token::Timeout);
        self.keywords.insert("deadline".to_string(), Token::Deadline);
        self.keywords.insert("cancel".to_string(), Token::Cancel);
        
        // Error Handling & Debugging
        self.keywords.insert("panic".to_string(), Token::Panic);
        self.keywords.insert("recover".to_string(), Token::Recover);
        self.keywords.insert("defer".to_string(), Token::Defer);
        self.keywords.insert("guard".to_string(), Token::Guard);
        self.keywords.insert("precondition".to_string(), Token::Precondition);
        self.keywords.insert("postcondition".to_string(), Token::Postcondition);
        self.keywords.insert("invariant".to_string(), Token::Invariant);
        self.keywords.insert("debug".to_string(), Token::Debug);
        self.keywords.insert("trace".to_string(), Token::Trace);
        self.keywords.insert("log".to_string(), Token::Log);
        self.keywords.insert("warn".to_string(), Token::Warn);
        self.keywords.insert("error".to_string(), Token::Error);
        self.keywords.insert("fatal".to_string(), Token::Fatal);
        self.keywords.insert("info".to_string(), Token::Info);
        self.keywords.insert("verbose".to_string(), Token::Verbose);
        
        // Type System
        self.keywords.insert("interface".to_string(), Token::Interface);
        self.keywords.insert("protocol".to_string(), Token::Protocol);
        self.keywords.insert("struct".to_string(), Token::Struct);
        self.keywords.insert("enum".to_string(), Token::Enum);
        self.keywords.insert("union".to_string(), Token::Union);
        self.keywords.insert("alias".to_string(), Token::Alias);
        self.keywords.insert("newtype".to_string(), Token::Newtype);
        self.keywords.insert("phantom".to_string(), Token::Phantom);
        self.keywords.insert("associated".to_string(), Token::Associated);
        self.keywords.insert("existential".to_string(), Token::Existential);
        self.keywords.insert("universal".to_string(), Token::Universal);
        self.keywords.insert("dependent".to_string(), Token::Dependent);
        self.keywords.insert("linear".to_string(), Token::Linear);
        self.keywords.insert("affine".to_string(), Token::Affine);
        self.keywords.insert("subtype".to_string(), Token::Subtype);
        self.keywords.insert("supertype".to_string(), Token::Supertype);
        self.keywords.insert("covariant".to_string(), Token::Covariant);
        self.keywords.insert("contravariant".to_string(), Token::Contravariant);
        
        // Functional Programming
        self.keywords.insert("lambda".to_string(), Token::Lambda);
        self.keywords.insert("closure".to_string(), Token::Closure);
        self.keywords.insert("partial".to_string(), Token::Partial);
        self.keywords.insert("curry".to_string(), Token::Curry);
        self.keywords.insert("uncurry".to_string(), Token::Uncurry);
        self.keywords.insert("fold".to_string(), Token::Fold);
        self.keywords.insert("reduce".to_string(), Token::Reduce);
        self.keywords.insert("scan".to_string(), Token::Scan);
        self.keywords.insert("unfold".to_string(), Token::Unfold);
        self.keywords.insert("zip".to_string(), Token::Zip);
        self.keywords.insert("unzip".to_string(), Token::Unzip);
        self.keywords.insert("flatten".to_string(), Token::Flatten);
        self.keywords.insert("flatmap".to_string(), Token::FlatMap);
        self.keywords.insert("bind".to_string(), Token::Bind);
        self.keywords.insert("pure".to_string(), Token::Pure);
        self.keywords.insert("applicative".to_string(), Token::Applicative);
        self.keywords.insert("functor".to_string(), Token::Functor);
        self.keywords.insert("monad".to_string(), Token::Monad);
        self.keywords.insert("monoid".to_string(), Token::Monoid);
        self.keywords.insert("semigroup".to_string(), Token::Semigroup);
        self.keywords.insert("category".to_string(), Token::Category);
        
        // Object-Oriented
        self.keywords.insert("constructor".to_string(), Token::Constructor);
        self.keywords.insert("destructor".to_string(), Token::Destructor);
        self.keywords.insert("initializer".to_string(), Token::Initializer);
        self.keywords.insert("deinitializer".to_string(), Token::Deinitializer);
        self.keywords.insert("getter".to_string(), Token::Getter);
        self.keywords.insert("setter".to_string(), Token::Setter);
        self.keywords.insert("property".to_string(), Token::Property);
        self.keywords.insert("method".to_string(), Token::Method);
        self.keywords.insert("field".to_string(), Token::Field);
        self.keywords.insert("member".to_string(), Token::Member);
        self.keywords.insert("attribute".to_string(), Token::Attribute);
        self.keywords.insert("annotation".to_string(), Token::Annotation);
        self.keywords.insert("decorator".to_string(), Token::Decorator);
        self.keywords.insert("mixin".to_string(), Token::Mixin);
        self.keywords.insert("delegate".to_string(), Token::Delegate);
        self.keywords.insert("proxy".to_string(), Token::Proxy);
        self.keywords.insert("singleton".to_string(), Token::Singleton);
        self.keywords.insert("factory".to_string(), Token::Factory);
        self.keywords.insert("builder".to_string(), Token::Builder);
        self.keywords.insert("prototype".to_string(), Token::Prototype);
        
        // Pattern Matching Extensions
        self.keywords.insert("where".to_string(), Token::Where);
        self.keywords.insert("such".to_string(), Token::Such);
        self.keywords.insert("that".to_string(), Token::That);
        self.keywords.insert("some".to_string(), Token::Some);
        self.keywords.insert("none".to_string(), Token::None);
        self.keywords.insert("ok".to_string(), Token::Ok);
        self.keywords.insert("err".to_string(), Token::Err);
        self.keywords.insert("just".to_string(), Token::Just);
        self.keywords.insert("nothing".to_string(), Token::Nothing);
        self.keywords.insert("left".to_string(), Token::Left);
        self.keywords.insert("right".to_string(), Token::Right);
        
        // Control Flow Extensions
        self.keywords.insert("goto".to_string(), Token::Goto);
        self.keywords.insert("label".to_string(), Token::Label);
        self.keywords.insert("switch".to_string(), Token::Switch);
        self.keywords.insert("fallthrough".to_string(), Token::Fallthrough);
        self.keywords.insert("repeat".to_string(), Token::Repeat);
        self.keywords.insert("do".to_string(), Token::Do);
        self.keywords.insert("then".to_string(), Token::Then);
        self.keywords.insert("elif".to_string(), Token::Elif);
        self.keywords.insert("elseif".to_string(), Token::Elseif);
        self.keywords.insert("otherwise".to_string(), Token::Otherwise);
        self.keywords.insert("always".to_string(), Token::Always);
        self.keywords.insert("never".to_string(), Token::Never);
        self.keywords.insert("forever".to_string(), Token::Forever);
        self.keywords.insert("once".to_string(), Token::Once);
        self.keywords.insert("twice".to_string(), Token::Twice);
        self.keywords.insert("thrice".to_string(), Token::Thrice);
        
        // Metaprogramming
        self.keywords.insert("reflect".to_string(), Token::Reflect);
        self.keywords.insert("introspect".to_string(), Token::Introspect);
        self.keywords.insert("eval".to_string(), Token::Eval);
        self.keywords.insert("quote".to_string(), Token::Quote);
        self.keywords.insert("unquote".to_string(), Token::Unquote);
        self.keywords.insert("splice".to_string(), Token::Splice);
        self.keywords.insert("gensym".to_string(), Token::Gensym);
        self.keywords.insert("hygiene".to_string(), Token::Hygiene);
        self.keywords.insert("syntax".to_string(), Token::Syntax);
        self.keywords.insert("parse".to_string(), Token::Parse);
        self.keywords.insert("expand".to_string(), Token::Expand);
        self.keywords.insert("compile".to_string(), Token::Compile);
        self.keywords.insert("interpret".to_string(), Token::Interpret);
        self.keywords.insert("transpile".to_string(), Token::Transpile);
        
        // Module System
        self.keywords.insert("module".to_string(), Token::Module);
        self.keywords.insert("namespace".to_string(), Token::Namespace);
        self.keywords.insert("scope".to_string(), Token::Scope);
        self.keywords.insert("global".to_string(), Token::Global);
        self.keywords.insert("local".to_string(), Token::Local);
        self.keywords.insert("extern".to_string(), Token::Extern);
        self.keywords.insert("foreign".to_string(), Token::Foreign);
        self.keywords.insert("native".to_string(), Token::Native);
        self.keywords.insert("builtin".to_string(), Token::Builtin);
        self.keywords.insert("prelude".to_string(), Token::Prelude);
        self.keywords.insert("std".to_string(), Token::Std);
        self.keywords.insert("core".to_string(), Token::Core);
        
        // Testing
        self.keywords.insert("test".to_string(), Token::Test);
        self.keywords.insert("benchmark".to_string(), Token::Benchmark);
        self.keywords.insert("property".to_string(), Token::PropertyTest);
        self.keywords.insert("quickcheck".to_string(), Token::Quickcheck);
        self.keywords.insert("fuzzy".to_string(), Token::Fuzzy);
        self.keywords.insert("mock".to_string(), Token::Mock);
        self.keywords.insert("stub".to_string(), Token::Stub);
        self.keywords.insert("spy".to_string(), Token::Spy);
        self.keywords.insert("fake".to_string(), Token::Fake);
        self.keywords.insert("verify".to_string(), Token::Verify);
        self.keywords.insert("validate".to_string(), Token::Validate);
        self.keywords.insert("check".to_string(), Token::Check);
        self.keywords.insert("prove".to_string(), Token::Prove);
        self.keywords.insert("theorem".to_string(), Token::Theorem);
        self.keywords.insert("lemma".to_string(), Token::Lemma);
        self.keywords.insert("axiom".to_string(), Token::Axiom);
        self.keywords.insert("corollary".to_string(), Token::Corollary);
        
        // Data Structures
        self.keywords.insert("array".to_string(), Token::Array);
        self.keywords.insert("vector".to_string(), Token::Vector);
        self.keywords.insert("matrix".to_string(), Token::Matrix);
        self.keywords.insert("tensor".to_string(), Token::Tensor);
        self.keywords.insert("stack".to_string(), Token::Stack);
        self.keywords.insert("queue".to_string(), Token::Queue);
        self.keywords.insert("deque".to_string(), Token::Deque);
        self.keywords.insert("heap".to_string(), Token::Heap);
        self.keywords.insert("tree".to_string(), Token::Tree);
        self.keywords.insert("graph".to_string(), Token::Graph);
        self.keywords.insert("node".to_string(), Token::Node);
        self.keywords.insert("edge".to_string(), Token::Edge);
        self.keywords.insert("vertex".to_string(), Token::Vertex);
        self.keywords.insert("path".to_string(), Token::Path);
        self.keywords.insert("cycle".to_string(), Token::Cycle);
        
        // I/O & Streams
        self.keywords.insert("stream".to_string(), Token::Stream);
        self.keywords.insert("reader".to_string(), Token::Reader);
        self.keywords.insert("writer".to_string(), Token::Writer);
        self.keywords.insert("buffer".to_string(), Token::Buffer);
        self.keywords.insert("flush".to_string(), Token::Flush);
        self.keywords.insert("close".to_string(), Token::Close);
        self.keywords.insert("open".to_string(), Token::Open);
        self.keywords.insert("read".to_string(), Token::Read);
        self.keywords.insert("write".to_string(), Token::Write);
        self.keywords.insert("seek".to_string(), Token::Seek);
        self.keywords.insert("tell".to_string(), Token::Tell);
        self.keywords.insert("rewind".to_string(), Token::Rewind);
        
        // Network
        self.keywords.insert("network".to_string(), Token::Network);
        self.keywords.insert("socket".to_string(), Token::Socket);
        self.keywords.insert("connect".to_string(), Token::Connect);
        self.keywords.insert("listen".to_string(), Token::Listen);
        self.keywords.insert("accept".to_string(), Token::Accept);
        self.keywords.insert("bind".to_string(), Token::BindNet);
        self.keywords.insert("shutdown".to_string(), Token::Shutdown);
        self.keywords.insert("http".to_string(), Token::Http);
        self.keywords.insert("https".to_string(), Token::Https);
        self.keywords.insert("tcp".to_string(), Token::Tcp);
        self.keywords.insert("udp".to_string(), Token::Udp);
        self.keywords.insert("websocket".to_string(), Token::Websocket);
        self.keywords.insert("rpc".to_string(), Token::Rpc);
        self.keywords.insert("rest".to_string(), Token::Rest);
        self.keywords.insert("graphql".to_string(), Token::Graphql);
        
        // Database
        self.keywords.insert("database".to_string(), Token::Database);
        self.keywords.insert("query".to_string(), Token::Query);
        self.keywords.insert("insert".to_string(), Token::Insert);
        self.keywords.insert("update".to_string(), Token::Update);
        self.keywords.insert("delete".to_string(), Token::Delete);
        self.keywords.insert("create".to_string(), Token::Create);
        self.keywords.insert("alter".to_string(), Token::Alter);
        self.keywords.insert("table".to_string(), Token::Table);
        self.keywords.insert("index".to_string(), Token::Index);
        self.keywords.insert("view".to_string(), Token::View);
        self.keywords.insert("transaction".to_string(), Token::Transaction);
        self.keywords.insert("commit".to_string(), Token::Commit);
        self.keywords.insert("rollback".to_string(), Token::Rollback);
        self.keywords.insert("savepoint".to_string(), Token::Savepoint);
        
        // Security
        self.keywords.insert("encrypt".to_string(), Token::Encrypt);
        self.keywords.insert("decrypt".to_string(), Token::Decrypt);
        self.keywords.insert("hash".to_string(), Token::HashFunc);
        self.keywords.insert("sign".to_string(), Token::Sign);
        self.keywords.insert("seal".to_string(), Token::Seal);
        self.keywords.insert("unseal".to_string(), Token::Unseal);
        self.keywords.insert("secure".to_string(), Token::Secure);
        self.keywords.insert("unsafe".to_string(), Token::Unsafe);
        self.keywords.insert("trusted".to_string(), Token::Trusted);
        self.keywords.insert("untrusted".to_string(), Token::Untrusted);
        self.keywords.insert("sanitize".to_string(), Token::Sanitize);
        self.keywords.insert("escape".to_string(), Token::Escape);
        
        // Time & Date
        self.keywords.insert("time".to_string(), Token::Time);
        self.keywords.insert("date".to_string(), Token::Date);
        self.keywords.insert("datetime".to_string(), Token::DateTime);
        self.keywords.insert("duration".to_string(), Token::Duration);
        self.keywords.insert("instant".to_string(), Token::Instant);
        self.keywords.insert("timestamp".to_string(), Token::Timestamp);
        self.keywords.insert("now".to_string(), Token::Now);
        self.keywords.insert("today".to_string(), Token::Today);
        self.keywords.insert("tomorrow".to_string(), Token::Tomorrow);
        self.keywords.insert("yesterday".to_string(), Token::Yesterday);
        
        // Math & Science
        self.keywords.insert("math".to_string(), Token::Math);
        self.keywords.insert("science".to_string(), Token::Science);
        self.keywords.insert("physics".to_string(), Token::Physics);
        self.keywords.insert("chemistry".to_string(), Token::Chemistry);
        self.keywords.insert("biology".to_string(), Token::Biology);
        self.keywords.insert("statistics".to_string(), Token::Statistics);
        self.keywords.insert("probability".to_string(), Token::Probability);
        self.keywords.insert("random".to_string(), Token::Random);
        self.keywords.insert("seed".to_string(), Token::Seed);
        self.keywords.insert("distribution".to_string(), Token::Distribution);
        self.keywords.insert("normal".to_string(), Token::Normal);
        self.keywords.insert("uniform".to_string(), Token::Uniform);
        self.keywords.insert("exponential".to_string(), Token::Exponential);
        self.keywords.insert("poisson".to_string(), Token::Poisson);
        
        // Graphics & UI
        self.keywords.insert("graphics".to_string(), Token::Graphics);
        self.keywords.insert("render".to_string(), Token::Render);
        self.keywords.insert("draw".to_string(), Token::Draw);
        self.keywords.insert("paint".to_string(), Token::Paint);
        self.keywords.insert("fill".to_string(), Token::Fill);
        self.keywords.insert("stroke".to_string(), Token::Stroke);
        self.keywords.insert("color".to_string(), Token::Color);
        self.keywords.insert("pixel".to_string(), Token::Pixel);
        self.keywords.insert("canvas".to_string(), Token::Canvas);
        self.keywords.insert("window".to_string(), Token::Window);
        self.keywords.insert("widget".to_string(), Token::Widget);
        self.keywords.insert("layout".to_string(), Token::Layout);
        self.keywords.insert("style".to_string(), Token::Style);
        self.keywords.insert("theme".to_string(), Token::Theme);
        
        // Audio & Media
        self.keywords.insert("audio".to_string(), Token::Audio);
        self.keywords.insert("video".to_string(), Token::Video);
        self.keywords.insert("media".to_string(), Token::Media);
        self.keywords.insert("sound".to_string(), Token::Sound);
        self.keywords.insert("music".to_string(), Token::Music);
        self.keywords.insert("play".to_string(), Token::Play);
        self.keywords.insert("pause".to_string(), Token::Pause);
        self.keywords.insert("stop".to_string(), Token::Stop);
        self.keywords.insert("record".to_string(), Token::Record);
        self.keywords.insert("volume".to_string(), Token::Volume);
        self.keywords.insert("pitch".to_string(), Token::Pitch);
        self.keywords.insert("tempo".to_string(), Token::Tempo);
        
        // File System
        self.keywords.insert("file".to_string(), Token::File);
        self.keywords.insert("directory".to_string(), Token::Directory);
        self.keywords.insert("folder".to_string(), Token::Folder);
        self.keywords.insert("exists".to_string(), Token::Exists);
        self.keywords.insert("rename".to_string(), Token::Rename);
        self.keywords.insert("remove".to_string(), Token::Remove);
        self.keywords.insert("mkdir".to_string(), Token::Mkdir);
        self.keywords.insert("rmdir".to_string(), Token::Rmdir);
        self.keywords.insert("chmod".to_string(), Token::Chmod);
        self.keywords.insert("chown".to_string(), Token::Chown);
        
        // Configuration
        self.keywords.insert("config".to_string(), Token::Config);
        self.keywords.insert("settings".to_string(), Token::Settings);
        self.keywords.insert("options".to_string(), Token::Options);
        self.keywords.insert("preferences".to_string(), Token::Preferences);
        self.keywords.insert("environment".to_string(), Token::Environment);
        self.keywords.insert("variable".to_string(), Token::Variable);
        self.keywords.insert("parameter".to_string(), Token::Parameter);
        self.keywords.insert("argument".to_string(), Token::Argument);
        self.keywords.insert("flag".to_string(), Token::Flag);
        
        // Lifecycle
        self.keywords.insert("init".to_string(), Token::Init);
        self.keywords.insert("start".to_string(), Token::Start);
        self.keywords.insert("run".to_string(), Token::Run);
        self.keywords.insert("execute".to_string(), Token::Execute);
        self.keywords.insert("invoke".to_string(), Token::Invoke);
        self.keywords.insert("call".to_string(), Token::Call);
        self.keywords.insert("apply".to_string(), Token::Apply);
        self.keywords.insert("perform".to_string(), Token::Perform);
        self.keywords.insert("complete".to_string(), Token::Complete);
        self.keywords.insert("finish".to_string(), Token::Finish);
        self.keywords.insert("end".to_string(), Token::End);
        self.keywords.insert("exit".to_string(), Token::Exit);
        self.keywords.insert("quit".to_string(), Token::Quit);
        self.keywords.insert("abort".to_string(), Token::Abort);
        self.keywords.insert("terminate".to_string(), Token::Terminate);
        self.keywords.insert("kill".to_string(), Token::Kill);
        
        // State Management
        self.keywords.insert("state".to_string(), Token::State);
        self.keywords.insert("store".to_string(), Token::Store);
        self.keywords.insert("cache".to_string(), Token::Cache);
        self.keywords.insert("memoize".to_string(), Token::Memoize);
        self.keywords.insert("persist".to_string(), Token::Persist);
        self.keywords.insert("load".to_string(), Token::Load);
        self.keywords.insert("save".to_string(), Token::Save);
        self.keywords.insert("restore".to_string(), Token::Restore);
        self.keywords.insert("snapshot".to_string(), Token::Snapshot);
        self.keywords.insert("checkpoint".to_string(), Token::Checkpoint);
        self.keywords.insert("undo".to_string(), Token::Undo);
        self.keywords.insert("redo".to_string(), Token::Redo);
        self.keywords.insert("history".to_string(), Token::History);
        
        // Validation & Constraints
        self.keywords.insert("constraint".to_string(), Token::Constraint);
        self.keywords.insert("bound".to_string(), Token::Bound);
        self.keywords.insert("limit".to_string(), Token::Limit);
        self.keywords.insert("between".to_string(), Token::Between);
        self.keywords.insert("within".to_string(), Token::Within);
        self.keywords.insert("outside".to_string(), Token::Outside);
        self.keywords.insert("inside".to_string(), Token::Inside);
        self.keywords.insert("includes".to_string(), Token::Includes);
        self.keywords.insert("excludes".to_string(), Token::Excludes);
        
        // Operators as Keywords
        self.keywords.insert("plus".to_string(), Token::Plus_);
        self.keywords.insert("minus".to_string(), Token::Minus_);
        self.keywords.insert("times".to_string(), Token::Times);
        self.keywords.insert("divide".to_string(), Token::Divide);
        self.keywords.insert("modulo".to_string(), Token::Modulo);
        self.keywords.insert("equals".to_string(), Token::Equals);
        self.keywords.insert("notequals".to_string(), Token::NotEquals);
        self.keywords.insert("greaterthan".to_string(), Token::GreaterThan);
        self.keywords.insert("lessthan".to_string(), Token::LessThan);
    }
}

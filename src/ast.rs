use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration {
        name: String,
        mutable: bool,
        type_annotation: Option<Type>,
        value: Expression,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Statement>,
        is_async: bool,
    },
    ClassDeclaration {
        name: String,
        superclass: Option<String>,
        traits: Vec<String>,
        members: Vec<ClassMember>,
    },
    TraitDeclaration {
        name: String,
        methods: Vec<TraitMethod>,
    },
    ImplBlock {
        trait_name: Option<String>,
        type_name: String,
        methods: Vec<Statement>,
    },
    ModuleDeclaration {
        name: String,
        body: Vec<Statement>,
    },
    Import {
        module: String,
        items: Vec<String>,
    },
    Export {
        items: Vec<String>,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    Unless {
        condition: Expression,
        body: Vec<Statement>,
    },
    Match {
        value: Expression,
        cases: Vec<MatchCase>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Until {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    Loop {
        body: Vec<Statement>,
    },
    Repeat {
        count: Expression,
        body: Vec<Statement>,
    },
    Break,
    Continue,
    Return(Option<Expression>),
    Yield(Expression),
    Try {
        body: Vec<Statement>,
        catch_clauses: Vec<CatchClause>,
        finally: Option<Vec<Statement>>,
    },
    Throw(Expression),
    Assert {
        condition: Expression,
        message: Option<Expression>,
    },
    Block(Vec<Statement>),
    
    // Time/Date statements
    Timestamp {
        variable: String,
    },
    
    // Database statements
    Select {
        table: String,
        columns: Vec<String>,
        condition: Option<Expression>,
    },
    Insert {
        table: String,
        columns: Vec<String>,
        values: Vec<Expression>,
    },
    Update {
        table: String,
        assignments: Vec<(String, Expression)>,
        condition: Option<Expression>,
    },
    Delete {
        table: String,
        condition: Option<Expression>,
    },
    
    // Graphics/UI statements
    Window {
        name: String,
        width: Expression,
        height: Expression,
        properties: Vec<(String, Expression)>,
    },
    Canvas {
        name: String,
        width: Expression,
        height: Expression,
    },
    Draw {
        target: String,
        shape: String,
        parameters: Vec<Expression>,
    },
    
    // Concurrency statements
    Spawn {
        body: Vec<Statement>,
    },
    Channel {
        name: String,
    },
    Send {
        channel: String,
        value: Expression,
    },
    Receive {
        channel: String,
        variable: String,
    },
    Lock {
        resource: String,
        body: Vec<Statement>,
    },
    
    // Error handling statements
    Panic {
        message: Expression,
    },
    Recover {
        try_body: Vec<Statement>,
        catch_body: Vec<Statement>,
    },
    Defer {
        body: Vec<Statement>,
    },
    Require {
        condition: Expression,
        message: Option<Expression>,
    },
    Ensure {
        condition: Expression,
        message: Option<Expression>,
    },
    
    // Testing statements
    Test {
        name: String,
        body: Vec<Statement>,
    },
    Bench {
        name: String,
        body: Vec<Statement>,
    },
    Mock {
        name: String,
        methods: Vec<(String, Vec<Statement>)>,
    },
    
    // Security statements
    Encrypt {
        variable: String,
        data: Expression,
        algorithm: Expression,
    },
    Decrypt {
        variable: String,
        data: Expression,
        algorithm: Expression,
    },
    Hash {
        variable: String,
        data: Expression,
        algorithm: Expression,
    },
    Sign {
        variable: String,
        data: Expression,
        key: Expression,
    },
    Verify {
        signature: Expression,
        data: Expression,
        key: Expression,
    },
    
    // Network statements
    Connect {
        name: String,
        address: Expression,
        body: Option<Vec<Statement>>,
    },
    Listen {
        port: Expression,
        body: Vec<Statement>,
    },
    
    // Graphics statements
    Render {
        scene: String,
        camera: String,
    },
    
    // Memory management statements
    Allocate {
        variable: String,
        size: Expression,
    },
    Deallocate {
        variable: String,
    },
    Free {
        variable: String,
    },
    
    // Module statements
    Module {
        name: String,
        body: Vec<Statement>,
    },
    Namespace {
        name: String,
        body: Vec<Statement>,
    },
    Package {
        name: String,
    },
    Use {
        path: String,
    },
    
    // Type definition statements
    Enum {
        name: String,
        variants: Vec<String>,
    },
    Struct {
        name: String,
        fields: Vec<(String, Option<Type>)>,
    },
    Union {
        name: String,
        variants: Vec<(String, Option<Type>)>,
    },
    Alias {
        name: String,
        target: Type,
    },
    Interface {
        name: String,
        methods: Vec<TraitMethod>,
    },
    
    // Async statements
    Async {
        body: Vec<Statement>,
    },
    Await {
        expression: Expression,
    },
    Future {
        name: String,
        body: Vec<Statement>,
    },
    Promise {
        name: String,
        body: Vec<Statement>,
    },
    Coroutine {
        name: String,
        body: Vec<Statement>,
    },
    
    // Advanced control flow
    Forever {
        body: Vec<Statement>,
    },
    Times {
        count: Expression,
        body: Vec<Statement>,
    },
    Twice {
        body: Vec<Statement>,
    },
    Thrice {
        body: Vec<Statement>,
    },
    When {
        condition: Expression,
        body: Vec<Statement>,
    },
    Otherwise {
        body: Vec<Statement>,
    },
    Switch {
        value: Expression,
        cases: Vec<MatchCase>,
    },
    Goto {
        label: String,
    },
    Label {
        name: String,
    },
    Abort {
        message: Option<Expression>,
    },
    Exit {
        code: Option<Expression>,
    },
    Quit,
    
    // Logging statements
    Fatal {
        message: Expression,
    },
    Warn {
        message: Expression,
    },
    Error {
        message: Expression,
    },
    Info {
        message: Expression,
    },
    Debug {
        message: Expression,
    },
    Trace {
        message: Expression,
    },
    
    // Data structures & collections
    Array {
        name: String,
        size: Option<Expression>,
        elements: Vec<Expression>,
    },
    Vector {
        name: String,
        elements: Vec<Expression>,
    },
    Deque {
        name: String,
        elements: Vec<Expression>,
    },
    Stack {
        name: String,
    },
    Queue {
        name: String,
    },
    Tree {
        name: String,
    },
    Graph {
        name: String,
        nodes: Vec<String>,
    },
    Matrix {
        name: String,
        rows: Expression,
        cols: Expression,
    },
    Tensor {
        name: String,
        dimensions: Vec<Expression>,
    },
    
    // Collection operations (only those with tokens)
    
    // Functional operations
    Reduce {
        variable: String,
        collection: Expression,
        function: Expression,
        initial: Expression,
    },
    Fold {
        variable: String,
        collection: Expression,
        function: Expression,
        initial: Expression,
    },
    Zip {
        variable: String,
        collections: Vec<Expression>,
    },
    Flatten {
        variable: String,
        collection: Expression,
    },
    
    // I/O operations
    Read {
        variable: String,
        source: Expression,
    },
    Write {
        destination: Expression,
        data: Expression,
    },
    Open {
        variable: String,
        path: Expression,
        mode: Expression,
    },
    Close {
        handle: String,
    },
    Flush {
        handle: String,
    },
    
    // File system operations
    File {
        name: String,
        path: Expression,
    },
    Folder {
        name: String,
        path: Expression,
    },
    Directory {
        name: String,
        path: Expression,
    },
    Path {
        variable: String,
        components: Vec<Expression>,
    },
    Mkdir {
        path: Expression,
    },
    Rmdir {
        path: Expression,
    },
    Remove {
        path: Expression,
    },
    Rename {
        old_path: Expression,
        new_path: Expression,
    },
    Copy {
        source: Expression,
        destination: Expression,
    },
    Move {
        source: Expression,
        destination: Expression,
    },
    
    // Database operations
    Database {
        name: String,
        connection: Expression,
    },
    Table {
        name: String,
        columns: Vec<(String, Option<Type>)>,
    },
    Query {
        variable: String,
        sql: Expression,
    },
    Transaction {
        body: Vec<Statement>,
    },
    Commit,
    Rollback,
    
    // Math & Science
    Math {
        operation: String,
        operands: Vec<Expression>,
    },
    Random {
        variable: String,
        min: Option<Expression>,
        max: Option<Expression>,
    },
    Seed {
        value: Expression,
    },
    
    // Time operations
    Now {
        variable: String,
    },
    Today {
        variable: String,
    },
    Timeout {
        duration: Expression,
        body: Vec<Statement>,
    },
    
    // Additional concurrency & parallelism
    Parallel {
        body: Vec<Statement>,
    },
    Sequential {
        body: Vec<Statement>,
    },
    Concurrent {
        tasks: Vec<Vec<Statement>>,
    },
    Sync {
        body: Vec<Statement>,
    },
    Task {
        name: String,
        body: Vec<Statement>,
    },
    Thread {
        name: String,
        body: Vec<Statement>,
    },
    Process {
        name: String,
        command: Expression,
    },
    Fiber {
        name: String,
        body: Vec<Statement>,
    },
    Green {
        name: String,
        body: Vec<Statement>,
    },
    Actor {
        name: String,
        mailbox: String,
        body: Vec<Statement>,
    },
    Message {
        actor: String,
        content: Expression,
    },
    Mailbox {
        name: String,
    },
    Deadline {
        time: Expression,
        body: Vec<Statement>,
    },
    Cancel {
        task: String,
    },
    
    // Additional error handling
    Guard {
        condition: Expression,
        body: Vec<Statement>,
    },
    Precondition {
        condition: Expression,
        message: Option<Expression>,
    },
    Postcondition {
        condition: Expression,
        message: Option<Expression>,
    },
    Invariant {
        condition: Expression,
        message: Option<Expression>,
    },
    Verbose {
        message: Expression,
    },
    Log {
        level: String,
        message: Expression,
    },
    
    // Additional type system
    Newtype {
        name: String,
        base_type: Type,
    },
    Phantom {
        name: String,
        type_param: String,
    },
    Associated {
        name: String,
        type_annotation: Type,
    },
    Existential {
        name: String,
        constraints: Vec<String>,
    },
    Universal {
        name: String,
        type_param: String,
    },
    Dependent {
        name: String,
        param: String,
        type_annotation: Type,
    },
    Linear {
        name: String,
        value: Expression,
    },
    Affine {
        name: String,
        value: Expression,
    },
    Subtype {
        name: String,
        parent: String,
    },
    Supertype {
        name: String,
        child: String,
    },
    Covariant {
        type_param: String,
    },
    Contravariant {
        type_param: String,
    },
    InvariantType {
        type_param: String,
    },
    
    // Functional programming
    Lambda {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
    },
    Closure {
        name: String,
        captures: Vec<String>,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
    },
    Partial {
        variable: String,
        function: Expression,
        args: Vec<Expression>,
    },
    Curry {
        variable: String,
        function: Expression,
    },
    Uncurry {
        variable: String,
        function: Expression,
    },
    Compose {
        variable: String,
        functions: Vec<Expression>,
    },
    Pipe {
        variable: String,
        value: Expression,
        functions: Vec<Expression>,
    },
    Scan {
        variable: String,
        collection: Expression,
        function: Expression,
        initial: Expression,
    },
    Unfold {
        variable: String,
        seed: Expression,
        function: Expression,
    },
    Unzip {
        variables: Vec<String>,
        collection: Expression,
    },
    FlatMap {
        variable: String,
        collection: Expression,
        function: Expression,
    },
    Bind {
        variable: String,
        monad: Expression,
        function: Expression,
    },
    Pure {
        variable: String,
        value: Expression,
    },
    Applicative {
        variable: String,
        function: Expression,
        value: Expression,
    },
    Functor {
        name: String,
        map_function: Expression,
    },
    Monad {
        name: String,
        bind_function: Expression,
    },
    Monoid {
        name: String,
        identity: Expression,
        combine: Expression,
    },
    Semigroup {
        name: String,
        combine: Expression,
    },
    Category {
        name: String,
        objects: Vec<String>,
        morphisms: Vec<(String, String, Expression)>,
    },
    
    // Object-oriented
    Constructor {
        class_name: String,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
    },
    Destructor {
        class_name: String,
        body: Vec<Statement>,
    },
    Initializer {
        name: String,
        body: Vec<Statement>,
    },
    Deinitializer {
        name: String,
        body: Vec<Statement>,
    },
    Getter {
        property: String,
        body: Vec<Statement>,
    },
    Setter {
        property: String,
        parameter: String,
        body: Vec<Statement>,
    },
    Property {
        name: String,
        type_annotation: Option<Type>,
        getter: Option<Vec<Statement>>,
        setter: Option<Vec<Statement>>,
    },
    Method {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Statement>,
    },
    Field {
        name: String,
        type_annotation: Option<Type>,
        value: Option<Expression>,
    },
    Member {
        object: String,
        name: String,
    },
    Attribute {
        name: String,
        value: Expression,
    },
    Annotation {
        name: String,
        args: Vec<Expression>,
    },
    Decorator {
        name: String,
        target: Box<Statement>,
    },
    Mixin {
        name: String,
        traits: Vec<String>,
        body: Vec<Statement>,
    },
    Delegate {
        target: String,
        method: String,
        to: String,
    },
    Proxy {
        name: String,
        target: String,
        handlers: Vec<(String, Vec<Statement>)>,
    },
    Singleton {
        name: String,
        body: Vec<Statement>,
    },
    Factory {
        name: String,
        product_type: String,
        body: Vec<Statement>,
    },
    Builder {
        name: String,
        fields: Vec<String>,
        body: Vec<Statement>,
    },
    Prototype {
        name: String,
        base: String,
    },
    
    // Pattern matching extensions
    Where {
        condition: Expression,
    },
    Such {
        condition: Expression,
    },
    That {
        condition: Expression,
    },
    Some {
        variable: String,
        value: Expression,
    },
    None {
        variable: String,
    },
    Ok {
        variable: String,
        value: Expression,
    },
    Err {
        variable: String,
        error: Expression,
    },
    Just {
        variable: String,
        value: Expression,
    },
    Nothing {
        variable: String,
    },
    Left {
        variable: String,
        value: Expression,
    },
    Right {
        variable: String,
        value: Expression,
    },
    
    // Control flow extensions
    Fallthrough,
    Do {
        body: Vec<Statement>,
    },
    Then {
        body: Vec<Statement>,
    },
    Elif {
        condition: Expression,
        body: Vec<Statement>,
    },
    Elseif {
        condition: Expression,
        body: Vec<Statement>,
    },
    Always {
        body: Vec<Statement>,
    },
    Never {
        body: Vec<Statement>,
    },
    Once {
        body: Vec<Statement>,
    },
    
    // Metaprogramming
    Reflect {
        target: Expression,
        variable: String,
    },
    Introspect {
        target: Expression,
        variable: String,
    },
    Eval {
        code: Expression,
    },
    Quote {
        variable: String,
        expression: Expression,
    },
    Unquote {
        variable: String,
    },
    Splice {
        target: String,
        code: Expression,
    },
    Gensym {
        variable: String,
        prefix: Option<String>,
    },
    Hygiene {
        enabled: bool,
    },
    Syntax {
        name: String,
        pattern: String,
        template: String,
    },
    Parse {
        variable: String,
        input: Expression,
    },
    Expand {
        macro_name: String,
        args: Vec<Expression>,
    },
    Compile {
        source: Expression,
        output: String,
    },
    Interpret {
        code: Expression,
    },
    Transpile {
        source: Expression,
        target_lang: String,
        output: String,
    },
    
    // Module system extensions
    Scope {
        name: String,
        body: Vec<Statement>,
    },
    Global {
        name: String,
        value: Expression,
    },
    Local {
        name: String,
        value: Expression,
    },
    Extern {
        name: String,
        signature: String,
    },
    Foreign {
        language: String,
        code: Expression,
    },
    Native {
        name: String,
        library: String,
    },
    Builtin {
        name: String,
    },
    Prelude {
        items: Vec<String>,
    },
    Std {
        module: String,
    },
    Core {
        feature: String,
    },
    
    // Testing & verification
    PropertyTest {
        name: String,
        property: Expression,
        body: Vec<Statement>,
    },
    Quickcheck {
        name: String,
        property: Expression,
    },
    Fuzzy {
        name: String,
        input_generator: Expression,
        body: Vec<Statement>,
    },
    Stub {
        name: String,
        return_value: Expression,
    },
    Spy {
        name: String,
        target: String,
    },
    Fake {
        name: String,
        behavior: Vec<Statement>,
    },
    Validate {
        condition: Expression,
        message: Option<Expression>,
    },
    Check {
        condition: Expression,
        message: Option<Expression>,
    },
    Prove {
        theorem: String,
        proof: Vec<Statement>,
    },
    Theorem {
        name: String,
        statement: Expression,
    },
    Lemma {
        name: String,
        statement: Expression,
    },
    Axiom {
        name: String,
        statement: Expression,
    },
    Corollary {
        name: String,
        theorem: String,
        statement: Expression,
    },
    
    // Additional data structures
    Heap {
        name: String,
        heap_type: String,
    },
    Node {
        name: String,
        value: Expression,
        children: Vec<String>,
    },
    Edge {
        from: String,
        to: String,
        weight: Option<Expression>,
    },
    Vertex {
        name: String,
        value: Expression,
    },
    Cycle {
        name: String,
        nodes: Vec<String>,
    },
    
    // I/O & Streams
    Stream {
        name: String,
        source: Expression,
    },
    Reader {
        name: String,
        source: Expression,
    },
    Writer {
        name: String,
        destination: Expression,
    },
    Buffer {
        name: String,
        size: Expression,
    },
    Seek {
        handle: String,
        position: Expression,
    },
    Tell {
        handle: String,
        variable: String,
    },
    Rewind {
        handle: String,
    },
    
    // Network extensions
    Network {
        name: String,
        protocol: String,
    },
    Socket {
        name: String,
        socket_type: String,
    },
    Accept {
        socket: String,
        variable: String,
    },
    BindNet {
        socket: String,
        address: Expression,
    },
    Shutdown {
        connection: String,
    },
    ProtocolDef {
        name: String,
        methods: Vec<String>,
    },
    Http {
        method: String,
        url: Expression,
        body: Option<Expression>,
    },
    Https {
        method: String,
        url: Expression,
        body: Option<Expression>,
    },
    Tcp {
        address: Expression,
        body: Vec<Statement>,
    },
    Udp {
        address: Expression,
        body: Vec<Statement>,
    },
    Websocket {
        url: Expression,
        handlers: Vec<(String, Vec<Statement>)>,
    },
    Rpc {
        service: String,
        method: String,
        args: Vec<Expression>,
    },
    Rest {
        endpoint: String,
        method: String,
        data: Option<Expression>,
    },
    Graphql {
        query: Expression,
        variables: Option<Expression>,
    },
    
    // Database extensions
    Create {
        entity_type: String,
        name: String,
        definition: Expression,
    },
    Alter {
        entity_type: String,
        name: String,
        changes: Expression,
    },
    DropTable {
        name: String,
    },
    Index {
        table: String,
        columns: Vec<String>,
    },
    View {
        name: String,
        query: Expression,
    },
    Savepoint {
        name: String,
    },
    
    // Security extensions
    Seal {
        variable: String,
        data: Expression,
    },
    Unseal {
        variable: String,
        sealed_data: Expression,
    },
    Secure {
        body: Vec<Statement>,
    },
    Unsafe {
        body: Vec<Statement>,
    },
    Trusted {
        source: String,
    },
    Untrusted {
        source: String,
    },
    Sanitize {
        variable: String,
        data: Expression,
    },
    Escape {
        variable: String,
        data: Expression,
    },
    
    // Time & Date extensions
    Time {
        variable: String,
        format: Option<Expression>,
    },
    Date {
        variable: String,
        format: Option<Expression>,
    },
    DateTime {
        variable: String,
        format: Option<Expression>,
    },
    Duration {
        variable: String,
        amount: Expression,
        unit: String,
    },
    Instant {
        variable: String,
    },
    Tomorrow {
        variable: String,
    },
    Yesterday {
        variable: String,
    },
    
    // Math & Science extensions
    Science {
        domain: String,
        operation: String,
        args: Vec<Expression>,
    },
    Physics {
        calculation: String,
        params: Vec<Expression>,
    },
    Chemistry {
        reaction: String,
        reactants: Vec<Expression>,
    },
    Biology {
        process: String,
        params: Vec<Expression>,
    },
    Statistics {
        operation: String,
        data: Expression,
    },
    Probability {
        event: Expression,
        space: Expression,
    },
    Distribution {
        dist_type: String,
        parameters: Vec<Expression>,
    },
    Normal {
        mean: Expression,
        std_dev: Expression,
    },
    Uniform {
        min: Expression,
        max: Expression,
    },
    Exponential {
        rate: Expression,
    },
    Poisson {
        lambda: Expression,
    },
    
    // Graphics & UI extensions
    Graphics {
        context: String,
        operations: Vec<Statement>,
    },
    Paint {
        target: String,
        color: Expression,
    },
    Fill {
        shape: String,
        color: Expression,
    },
    Stroke {
        shape: String,
        color: Expression,
        width: Expression,
    },
    Color {
        variable: String,
        r: Expression,
        g: Expression,
        b: Expression,
        a: Option<Expression>,
    },
    Pixel {
        x: Expression,
        y: Expression,
        color: Expression,
    },
    Widget {
        name: String,
        widget_type: String,
        properties: Vec<(String, Expression)>,
    },
    Layout {
        name: String,
        layout_type: String,
        children: Vec<String>,
    },
    Style {
        target: String,
        properties: Vec<(String, Expression)>,
    },
    Theme {
        name: String,
        colors: Vec<(String, Expression)>,
    },
    
    // Audio & Media
    Audio {
        name: String,
        source: Expression,
    },
    Video {
        name: String,
        source: Expression,
    },
    Media {
        name: String,
        media_type: String,
        source: Expression,
    },
    Sound {
        name: String,
        frequency: Expression,
        duration: Expression,
    },
    Music {
        name: String,
        notes: Vec<Expression>,
    },
    Play {
        media: String,
    },
    Pause {
        media: String,
    },
    Stop {
        media: String,
    },
    Record {
        name: String,
        source: String,
        duration: Option<Expression>,
    },
    Volume {
        media: String,
        level: Expression,
    },
    Pitch {
        sound: String,
        frequency: Expression,
    },
    Tempo {
        music: String,
        bpm: Expression,
    },
    
    // File system extensions
    Exists {
        path: Expression,
        variable: String,
    },
    Chmod {
        path: Expression,
        mode: Expression,
    },
    Chown {
        path: Expression,
        owner: Expression,
    },
    
    // Configuration
    Config {
        name: String,
        settings: Vec<(String, Expression)>,
    },
    Settings {
        name: String,
        values: Vec<(String, Expression)>,
    },
    Options {
        name: String,
        values: Vec<(String, Expression)>,
    },
    Preferences {
        name: String,
        values: Vec<(String, Expression)>,
    },
    Environment {
        name: String,
        variables: Vec<(String, Expression)>,
    },
    Variable {
        name: String,
        value: Expression,
    },
    Parameter {
        name: String,
        type_annotation: Option<Type>,
        default: Option<Expression>,
    },
    Argument {
        name: String,
        value: Expression,
    },
    Flag {
        name: String,
        enabled: bool,
    },
    
    // Lifecycle extensions
    Init {
        name: String,
        body: Vec<Statement>,
    },
    Start {
        name: String,
    },
    Run {
        name: String,
        args: Vec<Expression>,
    },
    Execute {
        command: Expression,
        args: Vec<Expression>,
    },
    Invoke {
        function: Expression,
        args: Vec<Expression>,
    },
    Call {
        function: Expression,
        args: Vec<Expression>,
    },
    Apply {
        function: Expression,
        args: Vec<Expression>,
    },
    Perform {
        action: String,
        params: Vec<Expression>,
    },
    Complete {
        task: String,
    },
    Finish {
        task: String,
    },
    End {
        scope: String,
    },
    Terminate {
        process: String,
    },
    Kill {
        process: String,
        signal: Option<Expression>,
    },
    
    // State management
    State {
        name: String,
        initial: Expression,
    },
    Store {
        name: String,
        value: Expression,
    },
    Cache {
        name: String,
        value: Expression,
        ttl: Option<Expression>,
    },
    Memoize {
        function: String,
    },
    Persist {
        name: String,
        data: Expression,
    },
    Load {
        name: String,
        source: Expression,
    },
    Save {
        name: String,
        destination: Expression,
    },
    Restore {
        name: String,
        checkpoint: String,
    },
    Snapshot {
        name: String,
        state: Expression,
    },
    Checkpoint {
        name: String,
    },
    Undo {
        steps: Option<Expression>,
    },
    Redo {
        steps: Option<Expression>,
    },
    History {
        name: String,
        max_size: Option<Expression>,
    },
    
    // Validation & Constraints
    Constraint {
        name: String,
        condition: Expression,
    },
    Bound {
        variable: String,
        lower: Expression,
        upper: Expression,
    },
    Limit {
        variable: String,
        max: Expression,
    },
    Min {
        variable: String,
        value: Expression,
    },
    Max {
        variable: String,
        value: Expression,
    },
    RangeConstraint {
        variable: String,
        min: Expression,
        max: Expression,
    },
    Between {
        value: Expression,
        lower: Expression,
        upper: Expression,
    },
    Within {
        value: Expression,
        bounds: Expression,
    },
    Outside {
        value: Expression,
        bounds: Expression,
    },
    Inside {
        value: Expression,
        container: Expression,
    },
    Contains {
        container: Expression,
        element: Expression,
    },
    Includes {
        collection: Expression,
        element: Expression,
    },
    Excludes {
        collection: Expression,
        element: Expression,
    },
    
    // Operators as keywords
    Plus {
        left: Expression,
        right: Expression,
        result: String,
    },
    Minus {
        left: Expression,
        right: Expression,
        result: String,
    },
    TimesOp {
        left: Expression,
        right: Expression,
        result: String,
    },
    Divide {
        left: Expression,
        right: Expression,
        result: String,
    },
    Modulo {
        left: Expression,
        right: Expression,
        result: String,
    },
    Equals {
        left: Expression,
        right: Expression,
        result: String,
    },
    NotEquals {
        left: Expression,
        right: Expression,
        result: String,
    },
    GreaterThan {
        left: Expression,
        right: Expression,
        result: String,
    },
    LessThan {
        left: Expression,
        right: Expression,
        result: String,
    },
    AndOp {
        left: Expression,
        right: Expression,
        result: String,
    },
    OrOp {
        left: Expression,
        right: Expression,
        result: String,
    },
    NotOp {
        operand: Expression,
        result: String,
    },
    XorOp {
        left: Expression,
        right: Expression,
        result: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub default_value: Option<Expression>,
    pub is_ref: bool,
    pub is_mut: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassMember {
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_final: bool,
    pub member_type: ClassMemberType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMemberType {
    Field {
        name: String,
        type_annotation: Option<Type>,
        value: Option<Expression>,
    },
    Method {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Statement>,
        is_virtual: bool,
        is_override: bool,
    },
    Constructor {
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethod {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub default_body: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(Value),
    Identifier(String),
    Tuple(Vec<Pattern>),
    List(Vec<Pattern>),
    Wildcard,
    Range(Box<Expression>, Box<Expression>),
    Constructor {
        name: String,
        fields: Vec<Pattern>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub exception_type: Option<String>,
    pub variable: Option<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Integer(i64),
    Decimal(f64),
    String(String),
    Boolean(bool),
    Null,
    Void,
    Identifier(String),
    
    // Binary operations
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    FloorDiv(Box<Expression>, Box<Expression>),
    
    // Comparison
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    LessEqual(Box<Expression>, Box<Expression>),
    GreaterEqual(Box<Expression>, Box<Expression>),
    Spaceship(Box<Expression>, Box<Expression>),
    
    // Logical
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
    
    // Bitwise
    BitAnd(Box<Expression>, Box<Expression>),
    BitOr(Box<Expression>, Box<Expression>),
    BitXor(Box<Expression>, Box<Expression>),
    BitNot(Box<Expression>),
    LeftShift(Box<Expression>, Box<Expression>),
    RightShift(Box<Expression>, Box<Expression>),
    
    // Assignment
    Assign(Box<Expression>, Box<Expression>),
    AddAssign(Box<Expression>, Box<Expression>),
    SubAssign(Box<Expression>, Box<Expression>),
    MulAssign(Box<Expression>, Box<Expression>),
    DivAssign(Box<Expression>, Box<Expression>),
    ModAssign(Box<Expression>, Box<Expression>),
    
    // Special operators
    Pipeline(Box<Expression>, Box<Expression>),
    Compose(Box<Expression>, Box<Expression>),
    NullCoalesce(Box<Expression>, Box<Expression>),
    Elvis(Box<Expression>, Box<Expression>, Box<Expression>),
    Range(Box<Expression>, Box<Expression>),
    RangeInclusive(Box<Expression>, Box<Expression>),
    
    // Member access
    MemberAccess(Box<Expression>, String),
    Index(Box<Expression>, Box<Expression>),
    
    // Function call
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    
    // Method call
    MethodCall {
        object: Box<Expression>,
        method: String,
        arguments: Vec<Expression>,
    },
    
    // Collections
    List(Vec<Expression>),
    Dict(Vec<(Expression, Expression)>),
    Set(Vec<Expression>),
    Tuple(Vec<Expression>),
    
    // Lambda
    Lambda {
        parameters: Vec<Parameter>,
        body: Box<Expression>,
    },
    
    // Type operations
    Cast(Box<Expression>, Type),
    TypeCheck(Box<Expression>, Type),
    TypeOf(Box<Expression>),
    SizeOf(Box<Expression>),
    
    // Special
    This,
    Super,
    Spread(Box<Expression>),
    Await(Box<Expression>),
    Clone(Box<Expression>),
    Move(Box<Expression>),
    Borrow(Box<Expression>),
    
    // Ternary
    Ternary {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Void,
    Any,
    List(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Set(Box<Type>),
    Tuple(Vec<Type>),
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
    Custom(String),
    Generic {
        name: String,
        type_params: Vec<Type>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Decimal(f64),
    String(String),
    Boolean(bool),
    Null,
    Void,
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Set(Vec<Value>),
    Tuple(Vec<Value>),
    Function {
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
        closure: HashMap<String, Value>,
    },
    NativeFunction {
        name: String,
        arity: usize,
    },
    Object {
        class_name: String,
        fields: HashMap<String, Value>,
    },
    Range {
        start: i64,
        end: i64,
        inclusive: bool,
    },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Decimal(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Void => write!(f, "void"),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Dict(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Set(items) => {
                write!(f, "{{")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "}}")
            }
            Value::Tuple(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Value::Function { .. } => write!(f, "<function>"),
            Value::NativeFunction { name, .. } => write!(f, "<native function: {}>", name),
            Value::Object { class_name, .. } => write!(f, "<{} instance>", class_name),
            Value::Range { start, end, inclusive } => {
                if *inclusive {
                    write!(f, "{}..={}", start, end)
                } else {
                    write!(f, "{}..{}", start, end)
                }
            }
        }
    }
}

pub type Program = Vec<Statement>;

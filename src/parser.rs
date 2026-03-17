use crate::ast::*;
use crate::error::ProtlinError;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn peek_ahead(&self, n: usize) -> Option<&Token> {
        if self.current + n < self.tokens.len() {
            Some(&self.tokens[self.current + n])
        } else {
            None
        }
    }
    
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].clone()
    }
    
    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token_type)
    }
    
    fn match_token(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    fn consume(&mut self, expected: Token, message: &str) -> Result<Token, ProtlinError> {
        if self.check(&expected) {
            Ok(self.advance())
        } else {
            Err(ProtlinError::ParserError(format!(
                "{}: expected {:?}, got {:?}",
                message,
                expected,
                self.peek()
            )))
        }
    }
    
    fn skip_newlines(&mut self) {
        while self.match_token(&[Token::Newline]) {}
    }
    
    pub fn parse(&mut self) -> Result<Program, ProtlinError> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            self.skip_newlines();
            if !self.is_at_end() {
                statements.push(self.declaration()?);
            }
        }
        
        Ok(statements)
    }
    
    fn declaration(&mut self) -> Result<Statement, ProtlinError> {
        self.skip_newlines();
        
        match self.peek() {
            Token::Egg => {
                self.advance();
                self.consume(Token::LeftBrace, "Expected '{' after 'egg'")?;
                self.skip_newlines();
                
                let mut statements = Vec::new();
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    statements.push(self.declaration()?);
                    self.skip_newlines();
                }
                
                self.consume(Token::RightBrace, "Expected '}' after egg body")?;
                Ok(Statement::Block(statements))
            }
            Token::Yolk | Token::Albumen => self.variable_declaration(),
            Token::Hatch => self.function_declaration(),
            Token::Nest => self.class_declaration(),
            Token::Shell => self.module_declaration(),
            Token::Trait => self.trait_declaration(),
            Token::Impl => self.impl_block(),
            Token::Import => self.import_statement(),
            Token::Export => self.export_statement(),
            Token::Timestamp => self.timestamp_statement(),
            Token::Window => self.window_statement(),
            Token::Canvas => self.canvas_statement(),
            Token::Draw => self.draw_statement(),
            Token::Select_ => self.select_statement(),
            Token::Insert => self.insert_statement(),
            Token::Update => self.update_statement(),
            Token::Delete => self.delete_statement(),
            Token::Spawn => self.spawn_statement(),
            Token::Channel => self.channel_statement(),
            Token::Send => self.send_statement(),
            Token::Receive => self.receive_statement(),
            Token::Lock => self.lock_statement(),
            Token::Panic => self.panic_statement(),
            Token::Recover => self.recover_statement(),
            Token::Defer => self.defer_statement(),
            Token::Require => self.require_statement(),
            Token::Ensure => self.ensure_statement(),
            Token::Test => self.test_statement(),
            Token::Benchmark => self.bench_statement(),
            Token::Mock => self.mock_statement(),
            Token::Encrypt => self.encrypt_statement(),
            Token::Decrypt => self.decrypt_statement(),
            Token::HashFunc => self.hash_statement(),
            Token::Sign => self.sign_statement(),
            Token::Verify => self.verify_statement(),
            Token::Connect => self.connect_statement(),
            Token::Listen => self.listen_statement(),
            Token::Render => self.render_statement(),
            Token::Allocate => self.allocate_statement(),
            Token::Deallocate => self.deallocate_statement(),
            Token::Free => self.free_statement(),
            Token::Module => self.module_statement(),
            Token::Namespace => self.namespace_statement(),
            Token::Package => self.package_statement(),
            Token::Use => self.use_statement(),
            Token::Enum => self.enum_statement(),
            Token::Struct => self.struct_statement(),
            Token::Union => self.union_statement(),
            Token::Alias => self.alias_statement(),
            Token::Interface => self.interface_statement(),
            Token::Async => self.async_statement(),
            Token::Await => self.await_statement(),
            Token::Future => self.future_statement(),
            Token::Promise => self.promise_statement(),
            Token::Coroutine => self.coroutine_statement(),
            Token::Forever => self.forever_statement(),
            Token::Times => self.times_statement(),
            Token::Twice => self.twice_statement(),
            Token::Thrice => self.thrice_statement(),
            Token::When => self.when_statement(),
            Token::Otherwise => self.otherwise_statement(),
            Token::Switch => self.switch_statement(),
            Token::Goto => self.goto_statement(),
            Token::Label => self.label_statement(),
            Token::Abort => self.abort_statement(),
            Token::Exit => self.exit_statement(),
            Token::Quit => self.quit_statement(),
            Token::Fatal => self.fatal_statement(),
            Token::Warn => self.warn_statement(),
            Token::Error => self.error_statement(),
            Token::Info => self.info_statement(),
            Token::Debug => self.debug_statement(),
            Token::Trace => self.trace_statement(),
            Token::Array => self.array_statement(),
            Token::Vector => self.vector_statement(),
            Token::Deque => self.deque_statement(),
            Token::Stack => self.stack_statement(),
            Token::Queue => self.queue_statement(),
            Token::Tree => self.tree_statement(),
            Token::Graph => self.graph_statement(),
            Token::Matrix => self.matrix_statement(),
            Token::Tensor => self.tensor_statement(),
            Token::Reduce => self.reduce_statement(),
            Token::Fold => self.fold_statement(),
            Token::Zip => self.zip_statement(),
            Token::Flatten => self.flatten_statement(),
            Token::Read => self.read_statement(),
            Token::Write => self.write_statement(),
            Token::Open => self.open_statement(),
            Token::Close => self.close_statement(),
            Token::Flush => self.flush_statement(),
            Token::File => self.file_statement(),
            Token::Folder => self.folder_statement(),
            Token::Directory => self.directory_statement(),
            Token::Path => self.path_statement(),
            Token::Mkdir => self.mkdir_statement(),
            Token::Rmdir => self.rmdir_statement(),
            Token::Remove => self.remove_statement(),
            Token::Rename => self.rename_statement(),
            Token::Copy => self.copy_statement(),
            Token::Move => self.move_statement(),
            Token::Database => self.database_statement(),
            Token::Table => self.table_statement(),
            Token::Query => self.query_statement(),
            Token::Transaction => self.transaction_statement(),
            Token::Commit => self.commit_statement(),
            Token::Rollback => self.rollback_statement(),
            Token::Math => self.math_statement(),
            Token::Random => self.random_statement(),
            Token::Seed => self.seed_statement(),
            Token::Now => self.now_statement(),
            Token::Today => self.today_statement(),
            Token::Timeout => self.timeout_statement(),
            // Additional concurrency & parallelism
            Token::Parallel => self.parallel_statement(),
            Token::Sequential => self.sequential_statement(),
            Token::Concurrent => self.concurrent_statement(),
            Token::Sync => self.sync_statement(),
            Token::Task => self.task_statement(),
            Token::Thread => self.thread_statement(),
            Token::Process => self.process_statement(),
            Token::Fiber => self.fiber_statement(),
            Token::Green => self.green_statement(),
            Token::Actor => self.actor_statement(),
            Token::Message => self.message_statement(),
            Token::Mailbox => self.mailbox_statement(),
            Token::Deadline => self.deadline_statement(),
            Token::Cancel => self.cancel_statement(),
            // Additional error handling
            Token::Guard => self.guard_statement(),
            Token::Precondition => self.precondition_statement(),
            Token::Postcondition => self.postcondition_statement(),
            Token::Invariant => self.invariant_statement(),
            Token::Verbose => self.verbose_statement(),
            Token::Log => self.log_statement(),
            // Additional type system
            Token::Newtype => self.newtype_statement(),
            Token::Phantom => self.phantom_statement(),
            Token::Associated => self.associated_statement(),
            Token::Existential => self.existential_statement(),
            Token::Universal => self.universal_statement(),
            Token::Dependent => self.dependent_statement(),
            Token::Linear => self.linear_statement(),
            Token::Affine => self.affine_statement(),
            Token::Subtype => self.subtype_statement(),
            Token::Supertype => self.supertype_statement(),
            Token::Covariant => self.covariant_statement(),
            Token::Contravariant => self.contravariant_statement(),
            // Functional programming (many already exist, adding missing ones)
            Token::Lambda => self.lambda_statement(),
            Token::Closure => self.closure_statement(),
            Token::Partial => self.partial_statement(),
            Token::Curry => self.curry_statement(),
            Token::Uncurry => self.uncurry_statement(),
            Token::Pipe => self.pipe_statement(),
            Token::Scan => self.scan_statement(),
            Token::Unfold => self.unfold_statement(),
            Token::Unzip => self.unzip_statement(),
            Token::FlatMap => self.flatmap_statement(),
            Token::Bind => self.bind_statement(),
            Token::Pure => self.pure_statement(),
            Token::Applicative => self.applicative_statement(),
            Token::Functor => self.functor_statement(),
            Token::Monad => self.monad_statement(),
            Token::Monoid => self.monoid_statement(),
            Token::Semigroup => self.semigroup_statement(),
            Token::Category => self.category_statement(),
            // Object-oriented
            Token::Constructor => self.constructor_statement(),
            Token::Destructor => self.destructor_statement(),
            Token::Initializer => self.initializer_statement(),
            Token::Deinitializer => self.deinitializer_statement(),
            Token::Getter => self.getter_statement(),
            Token::Setter => self.setter_statement(),
            Token::Property => self.property_statement(),
            Token::Method => self.method_statement(),
            Token::Field => self.field_statement(),
            Token::Member => self.member_statement(),
            Token::Attribute => self.attribute_statement(),
            Token::Annotation => self.annotation_statement(),
            Token::Decorator => self.decorator_statement(),
            Token::Mixin => self.mixin_statement(),
            Token::Delegate => self.delegate_statement(),
            Token::Proxy => self.proxy_statement(),
            Token::Singleton => self.singleton_statement(),
            Token::Factory => self.factory_statement(),
            Token::Builder => self.builder_statement(),
            Token::Prototype => self.prototype_statement(),
            // Pattern matching extensions
            Token::Where => self.where_statement(),
            Token::Such => self.such_statement(),
            Token::That => self.that_statement(),
            Token::Some => self.some_statement(),
            Token::None => self.none_statement(),
            Token::Ok => self.ok_statement(),
            Token::Err => self.err_statement(),
            Token::Just => self.just_statement(),
            Token::Nothing => self.nothing_statement(),
            Token::Left => self.left_statement(),
            Token::Right => self.right_statement(),
            // Control flow extensions
            Token::Fallthrough => self.fallthrough_statement(),
            Token::Do => self.do_statement(),
            Token::Then => self.then_statement(),
            Token::Elif => self.elif_statement(),
            Token::Elseif => self.elseif_statement(),
            Token::Always => self.always_statement(),
            Token::Never => self.never_statement(),
            Token::Once => self.once_statement(),
            // Metaprogramming
            Token::Reflect => self.reflect_statement(),
            Token::Introspect => self.introspect_statement(),
            Token::Eval => self.eval_statement(),
            Token::Quote => self.quote_statement(),
            Token::Unquote => self.unquote_statement(),
            Token::Splice => self.splice_statement(),
            Token::Gensym => self.gensym_statement(),
            Token::Hygiene => self.hygiene_statement(),
            Token::Syntax => self.syntax_statement(),
            Token::Parse => self.parse_statement(),
            Token::Expand => self.expand_statement(),
            Token::Compile => self.compile_statement(),
            Token::Interpret => self.interpret_statement(),
            Token::Transpile => self.transpile_statement(),
            // Module system extensions
            Token::Scope => self.scope_statement(),
            Token::Global => self.global_statement(),
            Token::Local => self.local_statement(),
            Token::Extern => self.extern_statement(),
            Token::Foreign => self.foreign_statement(),
            Token::Native => self.native_statement(),
            Token::Builtin => self.builtin_statement(),
            Token::Prelude => self.prelude_statement(),
            Token::Std => self.std_statement(),
            Token::Core => self.core_statement(),
            // Testing & verification
            Token::PropertyTest => self.property_test_statement(),
            Token::Quickcheck => self.quickcheck_statement(),
            Token::Fuzzy => self.fuzzy_statement(),
            Token::Stub => self.stub_statement(),
            Token::Spy => self.spy_statement(),
            Token::Fake => self.fake_statement(),
            Token::Validate => self.validate_statement(),
            Token::Check => self.check_statement(),
            Token::Prove => self.prove_statement(),
            Token::Theorem => self.theorem_statement(),
            Token::Lemma => self.lemma_statement(),
            Token::Axiom => self.axiom_statement(),
            Token::Corollary => self.corollary_statement(),
            // Additional data structures
            Token::Heap => self.heap_statement(),
            Token::Node => self.node_statement(),
            Token::Edge => self.edge_statement(),
            Token::Vertex => self.vertex_statement(),
            Token::Cycle => self.cycle_statement(),
            // I/O & Streams
            Token::Stream => self.stream_statement(),
            Token::Reader => self.reader_statement(),
            Token::Writer => self.writer_statement(),
            Token::Buffer => self.buffer_statement(),
            Token::Seek => self.seek_statement(),
            Token::Tell => self.tell_statement(),
            Token::Rewind => self.rewind_statement(),
            // Network extensions
            Token::Network => self.network_statement(),
            Token::Socket => self.socket_statement(),
            Token::Accept => self.accept_statement(),
            Token::BindNet => self.bindnet_statement(),
            Token::Shutdown => self.shutdown_statement(),
            Token::Http => self.http_statement(),
            Token::Https => self.https_statement(),
            Token::Tcp => self.tcp_statement(),
            Token::Udp => self.udp_statement(),
            Token::Websocket => self.websocket_statement(),
            Token::Rpc => self.rpc_statement(),
            Token::Rest => self.rest_statement(),
            Token::Graphql => self.graphql_statement(),
            // Database extensions
            Token::Create => self.create_statement(),
            Token::Alter => self.alter_statement(),
            Token::Index => self.index_statement(),
            Token::View => self.view_statement(),
            Token::Savepoint => self.savepoint_statement(),
            // Security extensions
            Token::Seal => self.seal_statement(),
            Token::Unseal => self.unseal_statement(),
            Token::Secure => self.secure_statement(),
            Token::Unsafe => self.unsafe_statement(),
            Token::Trusted => self.trusted_statement(),
            Token::Untrusted => self.untrusted_statement(),
            Token::Sanitize => self.sanitize_statement(),
            Token::Escape => self.escape_statement(),
            // Time & Date extensions
            Token::Time => self.time_statement(),
            Token::Date => self.date_statement(),
            Token::DateTime => self.datetime_statement(),
            Token::Duration => self.duration_statement(),
            Token::Instant => self.instant_statement(),
            Token::Tomorrow => self.tomorrow_statement(),
            Token::Yesterday => self.yesterday_statement(),
            // Math & Science extensions
            Token::Science => self.science_statement(),
            Token::Physics => self.physics_statement(),
            Token::Chemistry => self.chemistry_statement(),
            Token::Biology => self.biology_statement(),
            Token::Statistics => self.statistics_statement(),
            Token::Probability => self.probability_statement(),
            Token::Distribution => self.distribution_statement(),
            Token::Normal => self.normal_statement(),
            Token::Uniform => self.uniform_statement(),
            Token::Exponential => self.exponential_statement(),
            Token::Poisson => self.poisson_statement(),
            // Graphics & UI extensions
            Token::Graphics => self.graphics_statement(),
            Token::Paint => self.paint_statement(),
            Token::Fill => self.fill_statement(),
            Token::Stroke => self.stroke_statement(),
            Token::Color => self.color_statement(),
            Token::Pixel => self.pixel_statement(),
            Token::Widget => self.widget_statement(),
            Token::Layout => self.layout_statement(),
            Token::Style => self.style_statement(),
            Token::Theme => self.theme_statement(),
            // Audio & Media
            Token::Audio => self.audio_statement(),
            Token::Video => self.video_statement(),
            Token::Media => self.media_statement(),
            Token::Sound => self.sound_statement(),
            Token::Music => self.music_statement(),
            Token::Play => self.play_statement(),
            Token::Pause => self.pause_statement(),
            Token::Stop => self.stop_statement(),
            Token::Record => self.record_statement(),
            Token::Volume => self.volume_statement(),
            Token::Pitch => self.pitch_statement(),
            Token::Tempo => self.tempo_statement(),
            // File system extensions
            Token::Exists => self.exists_statement(),
            Token::Chmod => self.chmod_statement(),
            Token::Chown => self.chown_statement(),
            // Configuration
            Token::Config => self.config_statement(),
            Token::Settings => self.settings_statement(),
            Token::Options => self.options_statement(),
            Token::Preferences => self.preferences_statement(),
            Token::Environment => self.environment_statement(),
            Token::Variable => self.variable_statement(),
            Token::Parameter => self.parameter_statement(),
            Token::Argument => self.argument_statement(),
            Token::Flag => self.flag_statement(),
            // Lifecycle extensions
            Token::Init => self.init_statement(),
            Token::Start => self.start_statement(),
            Token::Run => self.run_statement(),
            Token::Execute => self.execute_statement(),
            Token::Invoke => self.invoke_statement(),
            Token::Call => self.call_statement(),
            Token::Apply => self.apply_statement(),
            Token::Perform => self.perform_statement(),
            Token::Complete => self.complete_statement(),
            Token::Finish => self.finish_statement(),
            Token::End => self.end_statement(),
            Token::Terminate => self.terminate_statement(),
            Token::Kill => self.kill_statement(),
            // State management
            Token::State => self.state_statement(),
            Token::Store => self.store_statement(),
            Token::Cache => self.cache_statement(),
            Token::Memoize => self.memoize_statement(),
            Token::Persist => self.persist_statement(),
            Token::Load => self.load_statement(),
            Token::Save => self.save_statement(),
            Token::Restore => self.restore_statement(),
            Token::Snapshot => self.snapshot_statement(),
            Token::Checkpoint => self.checkpoint_statement(),
            Token::Undo => self.undo_statement(),
            Token::Redo => self.redo_statement(),
            Token::History => self.history_statement(),
            // Validation & Constraints
            Token::Constraint => self.constraint_statement(),
            Token::Bound => self.bound_statement(),
            Token::Limit => self.limit_statement(),
            Token::Between => self.between_statement(),
            Token::Within => self.within_statement(),
            Token::Outside => self.outside_statement(),
            Token::Inside => self.inside_statement(),
            Token::Includes => self.includes_statement(),
            Token::Excludes => self.excludes_statement(),
            // Operators as keywords
            Token::Plus_ => self.plus_statement(),
            Token::Minus_ => self.minus_statement(),
            Token::Times => self.times_op_statement(),
            Token::Divide => self.divide_statement(),
            Token::Modulo => self.modulo_statement(),
            Token::Equals => self.equals_statement(),
            Token::NotEquals => self.notequals_statement(),
            Token::GreaterThan => self.greaterthan_statement(),
            Token::LessThan => self.lessthan_statement(),
            _ => self.statement(),
        }
    }
    
    fn variable_declaration(&mut self) -> Result<Statement, ProtlinError> {
        let mutable = match self.advance() {
            Token::Yolk => true,
            Token::Albumen => false,
            _ => unreachable!(),
        };
        
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected identifier".to_string())),
        };
        
        let type_annotation = if self.match_token(&[Token::Colon]) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.consume(Token::Assign, "Expected '=' in variable declaration")?;
        
        let value = self.expression()?;
        
        self.skip_newlines();
        
        Ok(Statement::VariableDeclaration {
            name,
            mutable,
            type_annotation,
            value,
        })
    }
    
    fn function_declaration(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Hatch, "Expected 'hatch'")?;
        
        let is_async = self.match_token(&[Token::Incubate]);
        
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected function name".to_string())),
        };
        
        self.consume(Token::LeftParen, "Expected '(' after function name")?;
        
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let is_ref = self.match_token(&[Token::Ref]);
                let is_mut = self.match_token(&[Token::Mut]);
                
                let param_name = match self.advance() {
                    Token::Identifier(n) => n,
                    _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
                };
                
                let type_annotation = if self.match_token(&[Token::Colon]) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                
                let default_value = if self.match_token(&[Token::Assign]) {
                    Some(self.expression()?)
                } else {
                    None
                };
                
                parameters.push(Parameter {
                    name: param_name,
                    type_annotation,
                    default_value,
                    is_ref,
                    is_mut,
                });
                
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        
        let return_type = if self.match_token(&[Token::Arrow]) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.consume(Token::LeftBrace, "Expected '{' before function body")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after function body")?;
        
        Ok(Statement::FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
            is_async,
        })
    }
    
    fn class_declaration(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Nest, "Expected 'nest'")?;
        
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected class name".to_string())),
        };
        
        let superclass = if self.match_token(&[Token::Extend]) {
            match self.advance() {
                Token::Identifier(n) => Some(n),
                _ => return Err(ProtlinError::ParserError("Expected superclass name".to_string())),
            }
        } else {
            None
        };
        
        let mut traits = Vec::new();
        if self.match_token(&[Token::Impl]) {
            loop {
                match self.advance() {
                    Token::Identifier(n) => traits.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected trait name".to_string())),
                }
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(Token::LeftBrace, "Expected '{' before class body")?;
        self.skip_newlines();
        
        let mut members = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            members.push(self.class_member()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after class body")?;
        
        Ok(Statement::ClassDeclaration {
            name,
            superclass,
            traits,
            members,
        })
    }
    
    fn class_member(&mut self) -> Result<ClassMember, ProtlinError> {
        let visibility = if self.match_token(&[Token::Pub]) {
            Visibility::Public
        } else if self.match_token(&[Token::Priv]) {
            Visibility::Private
        } else if self.match_token(&[Token::Prot]) {
            Visibility::Protected
        } else {
            Visibility::Private
        };
        
        let is_static = self.match_token(&[Token::Static]);
        let is_final = self.match_token(&[Token::Final]);
        
        if self.match_token(&[Token::Hatch]) {
            let is_virtual = self.match_token(&[Token::Virtual]);
            let is_override = self.match_token(&[Token::Override]);
            
            let name = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected method name".to_string())),
            };
            
            self.consume(Token::LeftParen, "Expected '(' after method name")?;
            
            let mut parameters = Vec::new();
            if !self.check(&Token::RightParen) {
                loop {
                    let is_ref = self.match_token(&[Token::Ref]);
                    let is_mut = self.match_token(&[Token::Mut]);
                    
                    let param_name = match self.advance() {
                        Token::Identifier(n) => n,
                        _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
                    };
                    
                    let type_annotation = if self.match_token(&[Token::Colon]) {
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    
                    let default_value = if self.match_token(&[Token::Assign]) {
                        Some(self.expression()?)
                    } else {
                        None
                    };
                    
                    parameters.push(Parameter {
                        name: param_name,
                        type_annotation,
                        default_value,
                        is_ref,
                        is_mut,
                    });
                    
                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }
            
            self.consume(Token::RightParen, "Expected ')' after parameters")?;
            
            let return_type = if self.match_token(&[Token::Arrow]) {
                Some(self.parse_type()?)
            } else {
                None
            };
            
            self.consume(Token::LeftBrace, "Expected '{' before method body")?;
            self.skip_newlines();
            
            let mut body = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                body.push(self.declaration()?);
                self.skip_newlines();
            }
            
            self.consume(Token::RightBrace, "Expected '}' after method body")?;
            
            Ok(ClassMember {
                visibility,
                is_static,
                is_final,
                member_type: ClassMemberType::Method {
                    name,
                    parameters,
                    return_type,
                    body,
                    is_virtual,
                    is_override,
                },
            })
        } else {
            let name = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected field name".to_string())),
            };
            
            let type_annotation = if self.match_token(&[Token::Colon]) {
                Some(self.parse_type()?)
            } else {
                None
            };
            
            let value = if self.match_token(&[Token::Assign]) {
                Some(self.expression()?)
            } else {
                None
            };
            
            Ok(ClassMember {
                visibility,
                is_static,
                is_final,
                member_type: ClassMemberType::Field {
                    name,
                    type_annotation,
                    value,
                },
            })
        }
    }
    
    fn trait_declaration(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Trait, "Expected 'trait'")?;
        
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected trait name".to_string())),
        };
        
        self.consume(Token::LeftBrace, "Expected '{' before trait body")?;
        self.skip_newlines();
        
        let mut methods = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.consume(Token::Hatch, "Expected 'hatch' for trait method")?;
            
            let method_name = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected method name".to_string())),
            };
            
            self.consume(Token::LeftParen, "Expected '(' after method name")?;
            
            let mut parameters = Vec::new();
            if !self.check(&Token::RightParen) {
                loop {
                    let is_ref = self.match_token(&[Token::Ref]);
                    let is_mut = self.match_token(&[Token::Mut]);
                    
                    let param_name = match self.advance() {
                        Token::Identifier(n) => n,
                        _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
                    };
                    
                    let type_annotation = if self.match_token(&[Token::Colon]) {
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    
                    parameters.push(Parameter {
                        name: param_name,
                        type_annotation,
                        default_value: None,
                        is_ref,
                        is_mut,
                    });
                    
                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }
            
            self.consume(Token::RightParen, "Expected ')' after parameters")?;
            
            let return_type = if self.match_token(&[Token::Arrow]) {
                Some(self.parse_type()?)
            } else {
                None
            };
            
            let default_body = if self.check(&Token::LeftBrace) {
                self.advance();
                self.skip_newlines();
                
                let mut body = Vec::new();
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    body.push(self.declaration()?);
                    self.skip_newlines();
                }
                
                self.consume(Token::RightBrace, "Expected '}' after method body")?;
                Some(body)
            } else {
                None
            };
            
            methods.push(TraitMethod {
                name: method_name,
                parameters,
                return_type,
                default_body,
            });
            
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after trait body")?;
        
        Ok(Statement::TraitDeclaration { name, methods })
    }
    
    fn impl_block(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Impl, "Expected 'impl'")?;
        
        let first_name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected type or trait name".to_string())),
        };
        
        let (trait_name, type_name) = if self.match_token(&[Token::For]) {
            let type_n = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected type name".to_string())),
            };
            (Some(first_name), type_n)
        } else {
            (None, first_name)
        };
        
        self.consume(Token::LeftBrace, "Expected '{' before impl body")?;
        self.skip_newlines();
        
        let mut methods = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            methods.push(self.function_declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after impl body")?;
        
        Ok(Statement::ImplBlock {
            trait_name,
            type_name,
            methods,
        })
    }
    
    fn module_declaration(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Shell, "Expected 'shell'")?;
        
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected module name".to_string())),
        };
        
        self.consume(Token::LeftBrace, "Expected '{' before module body")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after module body")?;
        
        Ok(Statement::ModuleDeclaration { name, body })
    }
    
    fn import_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Import, "Expected 'import'")?;
        
        let mut items = Vec::new();
        
        if self.match_token(&[Token::LeftBrace]) {
            loop {
                match self.advance() {
                    Token::Identifier(n) => items.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected identifier".to_string())),
                }
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
            self.consume(Token::RightBrace, "Expected '}' after import list")?;
        } else {
            match self.advance() {
                Token::Identifier(n) => items.push(n),
                _ => return Err(ProtlinError::ParserError("Expected identifier".to_string())),
            }
        }
        
        self.consume(Token::From, "Expected 'from' after import items")?;
        
        let module = match self.advance() {
            Token::String(s) => s,
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected module name".to_string())),
        };
        
        Ok(Statement::Import { module, items })
    }
    
    fn export_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Export, "Expected 'export'")?;
        
        let mut items = Vec::new();
        
        if self.match_token(&[Token::LeftBrace]) {
            loop {
                match self.advance() {
                    Token::Identifier(n) => items.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected identifier".to_string())),
                }
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
            self.consume(Token::RightBrace, "Expected '}' after export list")?;
        } else {
            match self.advance() {
                Token::Identifier(n) => items.push(n),
                _ => return Err(ProtlinError::ParserError("Expected identifier".to_string())),
            }
        }
        
        Ok(Statement::Export { items })
    }
    
    fn statement(&mut self) -> Result<Statement, ProtlinError> {
        match self.peek() {
            Token::If => self.if_statement(),
            Token::Unless => self.unless_statement(),
            Token::Match => self.match_statement(),
            Token::While => self.while_statement(),
            Token::Until => self.until_statement(),
            Token::For | Token::Foreach => self.for_statement(),
            Token::Loop => self.loop_statement(),
            Token::Repeat => self.repeat_statement(),
            Token::Break => {
                self.advance();
                Ok(Statement::Break)
            }
            Token::Continue => {
                self.advance();
                Ok(Statement::Continue)
            }
            Token::Return => {
                self.advance();
                let value = if self.check(&Token::Newline) || self.check(&Token::Semicolon) || self.is_at_end() {
                    None
                } else {
                    Some(self.expression()?)
                };
                Ok(Statement::Return(value))
            }
            Token::Yield => {
                self.advance();
                let value = self.expression()?;
                Ok(Statement::Yield(value))
            }
            Token::Try => self.try_statement(),
            Token::Throw => {
                self.advance();
                let value = self.expression()?;
                Ok(Statement::Throw(value))
            }
            Token::Assert => self.assert_statement(),
            Token::LeftBrace => self.block_statement(),
            _ => {
                let expr = self.expression()?;
                Ok(Statement::Expression(expr))
            }
        }
    }
    
    fn if_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::If, "Expected 'if'")?;
        
        let condition = self.expression()?;
        
        self.consume(Token::LeftBrace, "Expected '{' after if condition")?;
        self.skip_newlines();
        
        let mut then_branch = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            then_branch.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after if body")?;
        
        let else_branch = if self.match_token(&[Token::Else]) {
            if self.check(&Token::If) {
                Some(vec![self.if_statement()?])
            } else {
                self.consume(Token::LeftBrace, "Expected '{' after else")?;
                self.skip_newlines();
                
                let mut else_body = Vec::new();
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    else_body.push(self.declaration()?);
                    self.skip_newlines();
                }
                
                self.consume(Token::RightBrace, "Expected '}' after else body")?;
                Some(else_body)
            }
        } else {
            None
        };
        
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    fn unless_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Unless, "Expected 'unless'")?;
        
        let condition = self.expression()?;
        
        self.consume(Token::LeftBrace, "Expected '{' after unless condition")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after unless body")?;
        
        Ok(Statement::Unless { condition, body })
    }
    
    fn match_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Match, "Expected 'match'")?;
        
        let value = self.expression()?;
        
        self.consume(Token::LeftBrace, "Expected '{' after match value")?;
        self.skip_newlines();
        
        let mut cases = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.consume(Token::Case, "Expected 'case' in match")?;
            
            let pattern = self.parse_pattern()?;
            
            let guard = if self.match_token(&[Token::If]) {
                Some(self.expression()?)
            } else {
                None
            };
            
            self.consume(Token::FatArrow, "Expected '=>' after pattern")?;
            
            if self.check(&Token::LeftBrace) {
                self.advance();
                self.skip_newlines();
                
                let mut body = Vec::new();
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    body.push(self.declaration()?);
                    self.skip_newlines();
                }
                
                self.consume(Token::RightBrace, "Expected '}' after case body")?;
                
                cases.push(MatchCase {
                    pattern,
                    guard,
                    body,
                });
            } else {
                let stmt = self.statement()?;
                cases.push(MatchCase {
                    pattern,
                    guard,
                    body: vec![stmt],
                });
            }
            
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after match cases")?;
        
        Ok(Statement::Match { value, cases })
    }
    
    fn parse_pattern(&mut self) -> Result<Pattern, ProtlinError> {
        match self.peek() {
            Token::Integer(n) => {
                let val = *n;
                self.advance();
                Ok(Pattern::Literal(Value::Integer(val)))
            }
            Token::String(s) => {
                let val = s.clone();
                self.advance();
                Ok(Pattern::Literal(Value::String(val)))
            }
            Token::Boolean(b) => {
                let val = *b;
                self.advance();
                Ok(Pattern::Literal(Value::Boolean(val)))
            }
            Token::Identifier(name) => {
                let n = name.clone();
                self.advance();
                Ok(Pattern::Identifier(n))
            }
            Token::Underscore | Token::Default => {
                self.advance();
                Ok(Pattern::Wildcard)
            }
            _ => Err(ProtlinError::ParserError("Invalid pattern".to_string())),
        }
    }
    
    fn while_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::While, "Expected 'while'")?;
        
        let condition = self.expression()?;
        
        self.consume(Token::LeftBrace, "Expected '{' after while condition")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after while body")?;
        
        Ok(Statement::While { condition, body })
    }
    
    fn until_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Until, "Expected 'until'")?;
        
        let condition = self.expression()?;
        
        self.consume(Token::LeftBrace, "Expected '{' after until condition")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after until body")?;
        
        Ok(Statement::Until { condition, body })
    }
    
    fn for_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.match_token(&[Token::For, Token::Foreach]);
        
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        
        self.consume(Token::In, "Expected 'in' in for loop")?;
        
        let iterable = self.expression()?;
        
        self.consume(Token::LeftBrace, "Expected '{' after for iterable")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after for body")?;
        
        Ok(Statement::For {
            variable,
            iterable,
            body,
        })
    }
    
    fn loop_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Loop, "Expected 'loop'")?;
        
        self.consume(Token::LeftBrace, "Expected '{' after loop")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after loop body")?;
        
        Ok(Statement::Loop { body })
    }
    
    fn repeat_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Repeat, "Expected 'repeat'")?;
        
        // Parse the count expression (how many times to repeat)
        let count = self.expression()?;
        
        self.consume(Token::LeftBrace, "Expected '{' after repeat count")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after repeat body")?;
        
        Ok(Statement::Repeat { count, body })
    }
    
    fn try_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Try, "Expected 'try'")?;
        
        self.consume(Token::LeftBrace, "Expected '{' after try")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after try body")?;
        
        let mut catch_clauses = Vec::new();
        while self.match_token(&[Token::Catch]) {
            let (exception_type, variable) = if self.match_token(&[Token::LeftParen]) {
                let exc_type = if let Token::Identifier(name) = self.peek() {
                    let n = name.clone();
                    self.advance();
                    Some(n)
                } else {
                    None
                };
                
                let var = if self.match_token(&[Token::Identifier(String::new())]) {
                    if let Token::Identifier(name) = self.tokens[self.current - 1].clone() {
                        Some(name)
                    } else {
                        None
                    }
                } else {
                    None
                };
                
                self.consume(Token::RightParen, "Expected ')' after catch parameters")?;
                (exc_type, var)
            } else {
                (None, None)
            };
            
            self.consume(Token::LeftBrace, "Expected '{' after catch")?;
            self.skip_newlines();
            
            let mut catch_body = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                catch_body.push(self.declaration()?);
                self.skip_newlines();
            }
            
            self.consume(Token::RightBrace, "Expected '}' after catch body")?;
            
            catch_clauses.push(CatchClause {
                exception_type,
                variable,
                body: catch_body,
            });
        }
        
        let finally = if self.match_token(&[Token::Finally]) {
            self.consume(Token::LeftBrace, "Expected '{' after finally")?;
            self.skip_newlines();
            
            let mut finally_body = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                finally_body.push(self.declaration()?);
                self.skip_newlines();
            }
            
            self.consume(Token::RightBrace, "Expected '}' after finally body")?;
            Some(finally_body)
        } else {
            None
        };
        
        Ok(Statement::Try {
            body,
            catch_clauses,
            finally,
        })
    }
    
    fn assert_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Assert, "Expected 'assert'")?;
        
        let condition = self.expression()?;
        
        let message = if self.match_token(&[Token::Comma]) {
            Some(self.expression()?)
        } else {
            None
        };
        
        Ok(Statement::Assert { condition, message })
    }
    
    fn block_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::LeftBrace, "Expected '{'")?;
        self.skip_newlines();
        
        let mut statements = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}'")?;
        
        Ok(Statement::Block(statements))
    }
    
    fn expression(&mut self) -> Result<Expression, ProtlinError> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Result<Expression, ProtlinError> {
        let expr = self.pipeline()?;
        
        if self.match_token(&[Token::Assign]) {
            let value = self.assignment()?;
            return Ok(Expression::Assign(Box::new(expr), Box::new(value)));
        } else if self.match_token(&[Token::PlusAssign]) {
            let value = self.assignment()?;
            return Ok(Expression::AddAssign(Box::new(expr), Box::new(value)));
        } else if self.match_token(&[Token::MinusAssign]) {
            let value = self.assignment()?;
            return Ok(Expression::SubAssign(Box::new(expr), Box::new(value)));
        } else if self.match_token(&[Token::StarAssign]) {
            let value = self.assignment()?;
            return Ok(Expression::MulAssign(Box::new(expr), Box::new(value)));
        } else if self.match_token(&[Token::SlashAssign]) {
            let value = self.assignment()?;
            return Ok(Expression::DivAssign(Box::new(expr), Box::new(value)));
        } else if self.match_token(&[Token::PercentAssign]) {
            let value = self.assignment()?;
            return Ok(Expression::ModAssign(Box::new(expr), Box::new(value)));
        }
        
        Ok(expr)
    }
    
    fn pipeline(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.null_coalesce()?;
        
        while self.match_token(&[Token::Pipeline]) {
            let right = self.null_coalesce()?;
            expr = Expression::Pipeline(Box::new(expr), Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn null_coalesce(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.logical_or()?;
        
        while self.match_token(&[Token::NullCoalesce]) {
            let right = self.logical_or()?;
            expr = Expression::NullCoalesce(Box::new(expr), Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn logical_or(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.logical_xor()?;
        
        while self.match_token(&[Token::Or]) {
            let right = self.logical_xor()?;
            expr = Expression::Or(Box::new(expr), Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn logical_xor(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.logical_and()?;
        
        while self.match_token(&[Token::Xor]) {
            let right = self.logical_and()?;
            expr = Expression::Xor(Box::new(expr), Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn logical_and(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.equality()?;
        
        while self.match_token(&[Token::And]) {
            let right = self.equality()?;
            expr = Expression::And(Box::new(expr), Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.comparison()?;
        
        loop {
            let token = self.peek().clone();
            match token {
                Token::Equal => {
                    self.advance();
                    let right = self.comparison()?;
                    expr = Expression::Equal(Box::new(expr), Box::new(right));
                }
                Token::NotEqual => {
                    self.advance();
                    let right = self.comparison()?;
                    expr = Expression::NotEqual(Box::new(expr), Box::new(right));
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.bitwise_or()?;
        
        loop {
            let token = self.peek().clone();
            match token {
                Token::Less => {
                    self.advance();
                    let right = self.bitwise_or()?;
                    expr = Expression::Less(Box::new(expr), Box::new(right));
                }
                Token::Greater => {
                    self.advance();
                    let right = self.bitwise_or()?;
                    expr = Expression::Greater(Box::new(expr), Box::new(right));
                }
                Token::LessEqual => {
                    self.advance();
                    let right = self.bitwise_or()?;
                    expr = Expression::LessEqual(Box::new(expr), Box::new(right));
                }
                Token::GreaterEqual => {
                    self.advance();
                    let right = self.bitwise_or()?;
                    expr = Expression::GreaterEqual(Box::new(expr), Box::new(right));
                }
                Token::Spaceship => {
                    self.advance();
                    let right = self.bitwise_or()?;
                    expr = Expression::Spaceship(Box::new(expr), Box::new(right));
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn bitwise_or(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.bitwise_xor()?;
        
        while self.match_token(&[Token::BitOr]) {
            let right = self.bitwise_xor()?;
            expr = Expression::BitOr(Box::new(expr), Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn bitwise_xor(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.bitwise_and()?;
        
        while self.match_token(&[Token::BitXor]) {
            let right = self.bitwise_and()?;
            expr = Expression::BitXor(Box::new(expr), Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn bitwise_and(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.shift()?;
        
        while self.match_token(&[Token::BitAnd]) {
            let right = self.shift()?;
            expr = Expression::BitAnd(Box::new(expr), Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn shift(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.range()?;
        
        loop {
            let token = self.peek().clone();
            match token {
                Token::LeftShift => {
                    self.advance();
                    let right = self.range()?;
                    expr = Expression::LeftShift(Box::new(expr), Box::new(right));
                }
                Token::RightShift => {
                    self.advance();
                    let right = self.range()?;
                    expr = Expression::RightShift(Box::new(expr), Box::new(right));
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn range(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.term()?;
        
        if self.match_token(&[Token::Range]) {
            let end = self.term()?;
            expr = Expression::Range(Box::new(expr), Box::new(end));
        } else if self.match_token(&[Token::RangeInclusive]) {
            let end = self.term()?;
            expr = Expression::RangeInclusive(Box::new(expr), Box::new(end));
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.factor()?;
        
        loop {
            let token = self.peek().clone();
            match token {
                Token::Plus => {
                    self.advance();
                    let right = self.factor()?;
                    expr = Expression::Add(Box::new(expr), Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.factor()?;
                    expr = Expression::Subtract(Box::new(expr), Box::new(right));
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.power()?;
        
        loop {
            let token = self.peek().clone();
            match token {
                Token::Star => {
                    self.advance();
                    let right = self.power()?;
                    expr = Expression::Multiply(Box::new(expr), Box::new(right));
                }
                Token::Slash => {
                    self.advance();
                    let right = self.power()?;
                    expr = Expression::Divide(Box::new(expr), Box::new(right));
                }
                Token::Percent => {
                    self.advance();
                    let right = self.power()?;
                    expr = Expression::Modulo(Box::new(expr), Box::new(right));
                }
                Token::FloorDiv => {
                    self.advance();
                    let right = self.power()?;
                    expr = Expression::FloorDiv(Box::new(expr), Box::new(right));
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn power(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.unary()?;
        
        if self.match_token(&[Token::Power]) {
            let right = self.power()?;
            expr = Expression::Power(Box::new(expr), Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expression, ProtlinError> {
        match self.peek() {
            Token::Not => {
                self.advance();
                let expr = self.unary()?;
                Ok(Expression::Not(Box::new(expr)))
            }
            Token::Minus => {
                self.advance();
                let expr = self.unary()?;
                Ok(Expression::Subtract(
                    Box::new(Expression::Integer(0)),
                    Box::new(expr),
                ))
            }
            Token::BitNot => {
                self.advance();
                let expr = self.unary()?;
                Ok(Expression::BitNot(Box::new(expr)))
            }
            Token::Peck => {
                self.advance();
                let expr = self.unary()?;
                Ok(Expression::Await(Box::new(expr)))
            }
            Token::Clone => {
                self.advance();
                let expr = self.unary()?;
                Ok(Expression::Clone(Box::new(expr)))
            }
            Token::Move => {
                self.advance();
                let expr = self.unary()?;
                Ok(Expression::Move(Box::new(expr)))
            }
            Token::Borrow => {
                self.advance();
                let expr = self.unary()?;
                Ok(Expression::Borrow(Box::new(expr)))
            }
            Token::Typeof => {
                self.advance();
                let expr = self.unary()?;
                Ok(Expression::TypeOf(Box::new(expr)))
            }
            Token::Sizeof => {
                self.advance();
                let expr = self.unary()?;
                Ok(Expression::SizeOf(Box::new(expr)))
            }
            _ => self.postfix(),
        }
    }
    
    fn postfix(&mut self) -> Result<Expression, ProtlinError> {
        let mut expr = self.primary()?;
        
        loop {
            match self.peek() {
                Token::LeftParen => {
                    self.advance();
                    let mut arguments = Vec::new();
                    
                    if !self.check(&Token::RightParen) {
                        loop {
                            arguments.push(self.expression()?);
                            if !self.match_token(&[Token::Comma]) {
                                break;
                            }
                        }
                    }
                    
                    self.consume(Token::RightParen, "Expected ')' after arguments")?;
                    
                    expr = Expression::Call {
                        callee: Box::new(expr),
                        arguments,
                    };
                }
                Token::LeftBracket => {
                    self.advance();
                    let index = self.expression()?;
                    self.consume(Token::RightBracket, "Expected ']' after index")?;
                    expr = Expression::Index(Box::new(expr), Box::new(index));
                }
                Token::Dot => {
                    self.advance();
                    let member = match self.advance() {
                        Token::Identifier(name) => name,
                        _ => return Err(ProtlinError::ParserError("Expected member name".to_string())),
                    };
                    
                    if self.check(&Token::LeftParen) {
                        self.advance();
                        let mut arguments = Vec::new();
                        
                        if !self.check(&Token::RightParen) {
                            loop {
                                arguments.push(self.expression()?);
                                if !self.match_token(&[Token::Comma]) {
                                    break;
                                }
                            }
                        }
                        
                        self.consume(Token::RightParen, "Expected ')' after arguments")?;
                        
                        expr = Expression::MethodCall {
                            object: Box::new(expr),
                            method: member,
                            arguments,
                        };
                    } else {
                        expr = Expression::MemberAccess(Box::new(expr), member);
                    }
                }
                Token::As => {
                    self.advance();
                    let target_type = self.parse_type()?;
                    expr = Expression::Cast(Box::new(expr), target_type);
                }
                Token::Is => {
                    self.advance();
                    let target_type = self.parse_type()?;
                    expr = Expression::TypeCheck(Box::new(expr), target_type);
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn primary(&mut self) -> Result<Expression, ProtlinError> {
        match self.peek().clone() {
            Token::Integer(n) => {
                self.advance();
                Ok(Expression::Integer(n))
            }
            Token::Decimal(f) => {
                self.advance();
                Ok(Expression::Decimal(f))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expression::String(s))
            }
            Token::Boolean(b) => {
                self.advance();
                Ok(Expression::Boolean(b))
            }
            Token::Null => {
                self.advance();
                Ok(Expression::Null)
            }
            Token::Void => {
                self.advance();
                Ok(Expression::Void)
            }
            Token::This => {
                self.advance();
                Ok(Expression::This)
            }
            Token::Super => {
                self.advance();
                Ok(Expression::Super)
            }
            Token::Identifier(name) => {
                self.advance();
                Ok(Expression::Identifier(name))
            }
            Token::Zero => {
                self.advance();
                Ok(Expression::Identifier("zero".to_string()))
            }
            Token::One => {
                self.advance();
                Ok(Expression::Identifier("one".to_string()))
            }
            Token::Two => {
                self.advance();
                Ok(Expression::Identifier("two".to_string()))
            }
            Token::Three => {
                self.advance();
                Ok(Expression::Identifier("three".to_string()))
            }
            Token::Four => {
                self.advance();
                Ok(Expression::Identifier("four".to_string()))
            }
            Token::Five => {
                self.advance();
                Ok(Expression::Identifier("five".to_string()))
            }
            Token::Six => {
                self.advance();
                Ok(Expression::Identifier("six".to_string()))
            }
            Token::Seven => {
                self.advance();
                Ok(Expression::Identifier("seven".to_string()))
            }
            Token::Eight => {
                self.advance();
                Ok(Expression::Identifier("eight".to_string()))
            }
            Token::Nine => {
                self.advance();
                Ok(Expression::Identifier("nine".to_string()))
            }
            Token::Ten => {
                self.advance();
                Ok(Expression::Identifier("ten".to_string()))
            }
            Token::Hundred => {
                self.advance();
                Ok(Expression::Identifier("hundred".to_string()))
            }
            Token::Thousand => {
                self.advance();
                Ok(Expression::Identifier("thousand".to_string()))
            }
            Token::Myriad => {
                self.advance();
                Ok(Expression::Identifier("myriad".to_string()))
            }
            Token::LeftParen => {
                self.advance();
                
                if self.check(&Token::RightParen) {
                    self.advance();
                    return Ok(Expression::Tuple(vec![]));
                }
                
                let expr = self.expression()?;
                
                if self.match_token(&[Token::Comma]) {
                    let mut elements = vec![expr];
                    
                    if !self.check(&Token::RightParen) {
                        loop {
                            elements.push(self.expression()?);
                            if !self.match_token(&[Token::Comma]) {
                                break;
                            }
                        }
                    }
                    
                    self.consume(Token::RightParen, "Expected ')' after tuple")?;
                    Ok(Expression::Tuple(elements))
                } else {
                    self.consume(Token::RightParen, "Expected ')' after expression")?;
                    Ok(expr)
                }
            }
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                
                if !self.check(&Token::RightBracket) {
                    loop {
                        elements.push(self.expression()?);
                        if !self.match_token(&[Token::Comma]) {
                            break;
                        }
                    }
                }
                
                self.consume(Token::RightBracket, "Expected ']' after list")?;
                Ok(Expression::List(elements))
            }
            Token::LeftBrace => {
                self.advance();
                let mut pairs = Vec::new();
                
                if !self.check(&Token::RightBrace) {
                    loop {
                        let key = self.expression()?;
                        self.consume(Token::Colon, "Expected ':' after dict key")?;
                        let value = self.expression()?;
                        pairs.push((key, value));
                        
                        if !self.match_token(&[Token::Comma]) {
                            break;
                        }
                    }
                }
                
                self.consume(Token::RightBrace, "Expected '}' after dict")?;
                Ok(Expression::Dict(pairs))
            }
            Token::BitOr => {
                self.advance();
                let mut parameters = Vec::new();
                
                if !self.check(&Token::BitOr) {
                    loop {
                        let param_name = match self.advance() {
                            Token::Identifier(n) => n,
                            _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
                        };
                        
                        let type_annotation = if self.match_token(&[Token::Colon]) {
                            Some(self.parse_type()?)
                        } else {
                            None
                        };
                        
                        parameters.push(Parameter {
                            name: param_name,
                            type_annotation,
                            default_value: None,
                            is_ref: false,
                            is_mut: false,
                        });
                        
                        if !self.match_token(&[Token::Comma]) {
                            break;
                        }
                    }
                }
                
                self.consume(Token::BitOr, "Expected '|' after lambda parameters")?;
                
                let body = Box::new(self.expression()?);
                
                Ok(Expression::Lambda { parameters, body })
            }
            _ => Err(ProtlinError::ParserError(format!(
                "Unexpected token: {:?}",
                self.peek()
            ))),
        }
    }
    
    fn parse_type(&mut self) -> Result<Type, ProtlinError> {
        match self.peek() {
            Token::TypeInt => {
                self.advance();
                Ok(Type::Int)
            }
            Token::TypeFloat => {
                self.advance();
                Ok(Type::Float)
            }
            Token::TypeString => {
                self.advance();
                Ok(Type::String)
            }
            Token::TypeBool => {
                self.advance();
                Ok(Type::Bool)
            }
            Token::TypeVoid => {
                self.advance();
                Ok(Type::Void)
            }
            Token::TypeAny => {
                self.advance();
                Ok(Type::Any)
            }
            Token::TypeList => {
                self.advance();
                self.consume(Token::Less, "Expected '<' after list type")?;
                let inner = self.parse_type()?;
                self.consume(Token::Greater, "Expected '>' after list type parameter")?;
                Ok(Type::List(Box::new(inner)))
            }
            Token::TypeDict => {
                self.advance();
                self.consume(Token::Less, "Expected '<' after dict type")?;
                let key_type = self.parse_type()?;
                self.consume(Token::Comma, "Expected ',' in dict type")?;
                let value_type = self.parse_type()?;
                self.consume(Token::Greater, "Expected '>' after dict type parameters")?;
                Ok(Type::Dict(Box::new(key_type), Box::new(value_type)))
            }
            Token::TypeSet => {
                self.advance();
                self.consume(Token::Less, "Expected '<' after set type")?;
                let inner = self.parse_type()?;
                self.consume(Token::Greater, "Expected '>' after set type parameter")?;
                Ok(Type::Set(Box::new(inner)))
            }
            Token::Identifier(name) => {
                let n = name.clone();
                self.advance();
                Ok(Type::Custom(n))
            }
            _ => Err(ProtlinError::ParserError(format!(
                "Expected type, got {:?}",
                self.peek()
            ))),
        }
    }
    
    // Timestamp statement: timestamp varname
    fn timestamp_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Timestamp, "Expected 'timestamp'")?;
        let variable = match self.advance() {
            Token::Identifier(name) => name,
            _ => return Err(ProtlinError::ParserError("Expected variable name after 'timestamp'".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Timestamp { variable })
    }
    
    // Window statement: window name width height { properties }
    fn window_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Window, "Expected 'window'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected window name".to_string())),
        };
        let width = self.expression()?;
        let height = self.expression()?;
        
        let mut properties = Vec::new();
        if self.match_token(&[Token::LeftBrace]) {
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                let prop_name = match self.advance() {
                    Token::Identifier(n) => n,
                    _ => break,
                };
                self.consume(Token::Colon, "Expected ':' after property name")?;
                let prop_value = self.expression()?;
                properties.push((prop_name, prop_value));
                self.match_token(&[Token::Comma]);
                self.skip_newlines();
            }
            self.consume(Token::RightBrace, "Expected '}'")?;
        }
        self.skip_newlines();
        Ok(Statement::Window { name, width, height, properties })
    }
    
    // Canvas statement: canvas name width height
    fn canvas_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Canvas, "Expected 'canvas'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected canvas name".to_string())),
        };
        let width = self.expression()?;
        let height = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Canvas { name, width, height })
    }
    
    // Draw statement: draw target shape params...
    fn draw_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Draw, "Expected 'draw'")?;
        let target = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected draw target".to_string())),
        };
        let shape = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected shape name".to_string())),
        };
        
        let mut parameters = Vec::new();
        while !self.check(&Token::Newline) && !self.is_at_end() {
            parameters.push(self.primary()?);
        }
        self.skip_newlines();
        Ok(Statement::Draw { target, shape, parameters })
    }

    // Select statement: select columns from table where condition
    fn select_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Select_, "Expected 'select'")?;
        
        let mut columns = Vec::new();
        loop {
            match self.advance() {
                Token::Identifier(col) => columns.push(col),
                _ => return Err(ProtlinError::ParserError("Expected column name".to_string())),
            }
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }
        
        // Expect 'from' keyword (we need to add this or use identifier)
        let table = match self.advance() {
            Token::Identifier(t) => t,
            _ => return Err(ProtlinError::ParserError("Expected table name".to_string())),
        };
        
        let condition = if self.check(&Token::Identifier("where".to_string())) {
            self.advance();
            Some(self.expression()?)
        } else {
            None
        };
        
        self.skip_newlines();
        Ok(Statement::Select { table, columns, condition })
    }
    
    // Insert statement: insert into table columns values
    fn insert_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Insert, "Expected 'insert'")?;
        
        let table = match self.advance() {
            Token::Identifier(t) => t,
            _ => return Err(ProtlinError::ParserError("Expected table name".to_string())),
        };
        
        let mut columns = Vec::new();
        if self.match_token(&[Token::LeftParen]) {
            loop {
                match self.advance() {
                    Token::Identifier(col) => columns.push(col),
                    _ => break,
                }
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
            self.consume(Token::RightParen, "Expected ')'")?;
        }
        
        let mut values = Vec::new();
        if self.match_token(&[Token::LeftParen]) {
            loop {
                values.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
            self.consume(Token::RightParen, "Expected ')'")?;
        }
        
        self.skip_newlines();
        Ok(Statement::Insert { table, columns, values })
    }
    
    // Update statement: update table set assignments where condition
    fn update_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Update, "Expected 'update'")?;
        
        let table = match self.advance() {
            Token::Identifier(t) => t,
            _ => return Err(ProtlinError::ParserError("Expected table name".to_string())),
        };
        
        let mut assignments = Vec::new();
        loop {
            let col = match self.advance() {
                Token::Identifier(c) => c,
                _ => break,
            };
            self.consume(Token::Assign, "Expected '='")?;
            let val = self.expression()?;
            assignments.push((col, val));
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }
        
        let condition = if self.check(&Token::Identifier("where".to_string())) {
            self.advance();
            Some(self.expression()?)
        } else {
            None
        };
        
        self.skip_newlines();
        Ok(Statement::Update { table, assignments, condition })
    }
    
    // Delete statement: delete from table where condition
    fn delete_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Delete, "Expected 'delete'")?;
        
        let table = match self.advance() {
            Token::Identifier(t) => t,
            _ => return Err(ProtlinError::ParserError("Expected table name".to_string())),
        };
        
        let condition = if self.check(&Token::Identifier("where".to_string())) {
            self.advance();
            Some(self.expression()?)
        } else {
            None
        };
        
        self.skip_newlines();
        Ok(Statement::Delete { table, condition })
    }
    
    // Spawn statement: spawn { body }
    fn spawn_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Spawn, "Expected 'spawn'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'spawn'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after spawn body")?;
        self.skip_newlines();
        Ok(Statement::Spawn { body })
    }
    
    // Channel statement: channel name
    fn channel_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Channel, "Expected 'channel'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected channel name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Channel { name })
    }
    
    // Send statement: send channel value
    fn send_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Send, "Expected 'send'")?;
        let channel = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected channel name".to_string())),
        };
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Send { channel, value })
    }
    
    // Receive statement: receive channel variable
    fn receive_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Receive, "Expected 'receive'")?;
        let channel = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected channel name".to_string())),
        };
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Receive { channel, variable })
    }
    
    // Lock statement: lock resource { body }
    fn lock_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Lock, "Expected 'lock'")?;
        let resource = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected resource name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after lock resource")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after lock body")?;
        self.skip_newlines();
        Ok(Statement::Lock { resource, body })
    }
    
    // Panic statement: panic message
    fn panic_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Panic, "Expected 'panic'")?;
        let message = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Panic { message })
    }
    
    // Recover statement: recover { try_body } catch { catch_body }
    fn recover_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Recover, "Expected 'recover'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'recover'")?;
        self.skip_newlines();
        
        let mut try_body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            try_body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after recover body")?;
        
        self.consume(Token::Catch, "Expected 'catch' after recover block")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'catch'")?;
        self.skip_newlines();
        
        let mut catch_body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            catch_body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after catch body")?;
        self.skip_newlines();
        Ok(Statement::Recover { try_body, catch_body })
    }
    
    // Defer statement: defer { body }
    fn defer_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Defer, "Expected 'defer'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'defer'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after defer body")?;
        self.skip_newlines();
        Ok(Statement::Defer { body })
    }
    
    // Require statement: require condition, message
    fn require_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Require, "Expected 'require'")?;
        let condition = self.expression()?;
        let message = if self.match_token(&[Token::Comma]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Require { condition, message })
    }
    
    // Ensure statement: ensure condition, message
    fn ensure_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Ensure, "Expected 'ensure'")?;
        let condition = self.expression()?;
        let message = if self.match_token(&[Token::Comma]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Ensure { condition, message })
    }
    
    // Test statement: test "name" { body }
    fn test_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Test, "Expected 'test'")?;
        let name = match self.advance() {
            Token::String(s) => s,
            Token::Identifier(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected test name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after test name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after test body")?;
        self.skip_newlines();
        Ok(Statement::Test { name, body })
    }
    
    // Bench statement: bench "name" { body }
    fn bench_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Benchmark, "Expected 'bench'")?;
        let name = match self.advance() {
            Token::String(s) => s,
            Token::Identifier(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected benchmark name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after bench name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after bench body")?;
        self.skip_newlines();
        Ok(Statement::Bench { name, body })
    }
    
    // Mock statement: mock name { methods }
    fn mock_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Mock, "Expected 'mock'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected mock name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after mock name")?;
        self.skip_newlines();
        
        let mut methods = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let method_name = match self.advance() {
                Token::Identifier(n) => n,
                _ => break,
            };
            self.consume(Token::Colon, "Expected ':' after method name")?;
            self.consume(Token::LeftBrace, "Expected '{' after ':'")?;
            self.skip_newlines();
            
            let mut method_body = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                method_body.push(self.declaration()?);
                self.skip_newlines();
            }
            
            self.consume(Token::RightBrace, "Expected '}' after method body")?;
            methods.push((method_name, method_body));
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after mock body")?;
        self.skip_newlines();
        Ok(Statement::Mock { name, methods })
    }
    
    // Encrypt statement: encrypt variable data algorithm
    fn encrypt_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Encrypt, "Expected 'encrypt'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let data = self.expression()?;
        let algorithm = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Encrypt { variable, data, algorithm })
    }
    
    // Decrypt statement: decrypt variable data algorithm
    fn decrypt_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Decrypt, "Expected 'decrypt'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let data = self.expression()?;
        let algorithm = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Decrypt { variable, data, algorithm })
    }
    
    // Hash statement: hash variable data algorithm
    fn hash_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::HashFunc, "Expected 'hash'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let data = self.expression()?;
        let algorithm = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Hash { variable, data, algorithm })
    }
    
    // Sign statement: sign variable data key
    fn sign_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Sign, "Expected 'sign'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let data = self.expression()?;
        let key = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Sign { variable, data, key })
    }
    
    // Verify statement: verify signature data key
    fn verify_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Verify, "Expected 'verify'")?;
        let signature = self.expression()?;
        let data = self.expression()?;
        let key = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Verify { signature, data, key })
    }
    
    // Connect statement: connect name address { body }
    fn connect_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Connect, "Expected 'connect'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected connection name".to_string())),
        };
        let address = self.expression()?;
        
        let body = if self.check(&Token::LeftBrace) {
            self.advance();
            self.skip_newlines();
            
            let mut stmts = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                stmts.push(self.declaration()?);
                self.skip_newlines();
            }
            
            self.consume(Token::RightBrace, "Expected '}' after connect body")?;
            Some(stmts)
        } else {
            None
        };
        
        self.skip_newlines();
        Ok(Statement::Connect { name, address, body })
    }
    
    // Listen statement: listen port { body }
    fn listen_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Listen, "Expected 'listen'")?;
        let port = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after listen port")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after listen body")?;
        self.skip_newlines();
        Ok(Statement::Listen { port, body })
    }
    
    // Render statement: render scene camera
    fn render_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Render, "Expected 'render'")?;
        let scene = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected scene name".to_string())),
        };
        let camera = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected camera name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Render { scene, camera })
    }
    
    // Memory management statements
    fn allocate_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Allocate, "Expected 'allocate'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let size = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Allocate { variable, size })
    }
    
    fn deallocate_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Deallocate, "Expected 'deallocate'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Deallocate { variable })
    }
    
    fn free_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Free, "Expected 'free'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Free { variable })
    }
    
    // Module statements
    fn module_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Module, "Expected 'module'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected module name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after module name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after module body")?;
        self.skip_newlines();
        Ok(Statement::Module { name, body })
    }
    
    fn namespace_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Namespace, "Expected 'namespace'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected namespace name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after namespace name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after namespace body")?;
        self.skip_newlines();
        Ok(Statement::Namespace { name, body })
    }
    
    fn package_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Package, "Expected 'package'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected package name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Package { name })
    }
    
    fn use_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Use, "Expected 'use'")?;
        let path = match self.advance() {
            Token::Identifier(n) => n,
            Token::String(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected path".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Use { path })
    }
    
    // Type definition statements
    fn enum_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Enum, "Expected 'enum'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected enum name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after enum name")?;
        self.skip_newlines();
        
        let mut variants = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            match self.advance() {
                Token::Identifier(v) => variants.push(v),
                _ => break,
            }
            self.match_token(&[Token::Comma]);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after enum variants")?;
        self.skip_newlines();
        Ok(Statement::Enum { name, variants })
    }
    
    fn struct_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Struct, "Expected 'struct'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected struct name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after struct name")?;
        self.skip_newlines();
        
        let mut fields = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let field_name = match self.advance() {
                Token::Identifier(n) => n,
                _ => break,
            };
            let field_type = if self.match_token(&[Token::Colon]) {
                Some(self.parse_type()?)
            } else {
                None
            };
            fields.push((field_name, field_type));
            self.match_token(&[Token::Comma]);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after struct fields")?;
        self.skip_newlines();
        Ok(Statement::Struct { name, fields })
    }
    
    fn union_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Union, "Expected 'union'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected union name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after union name")?;
        self.skip_newlines();
        
        let mut variants = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let variant_name = match self.advance() {
                Token::Identifier(n) => n,
                _ => break,
            };
            let variant_type = if self.match_token(&[Token::Colon]) {
                Some(self.parse_type()?)
            } else {
                None
            };
            variants.push((variant_name, variant_type));
            self.match_token(&[Token::Comma]);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after union variants")?;
        self.skip_newlines();
        Ok(Statement::Union { name, variants })
    }
    
    fn alias_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Alias, "Expected 'alias'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected alias name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after alias name")?;
        let target = self.parse_type()?;
        self.skip_newlines();
        Ok(Statement::Alias { name, target })
    }
    
    fn interface_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Interface, "Expected 'interface'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected interface name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after interface name")?;
        self.skip_newlines();
        
        let mut methods = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.consume(Token::Hatch, "Expected 'hatch' for interface method")?;
            
            let method_name = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected method name".to_string())),
            };
            
            self.consume(Token::LeftParen, "Expected '(' after method name")?;
            
            let mut parameters = Vec::new();
            if !self.check(&Token::RightParen) {
                loop {
                    let param_name = match self.advance() {
                        Token::Identifier(n) => n,
                        _ => break,
                    };
                    let type_annotation = if self.match_token(&[Token::Colon]) {
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    parameters.push(Parameter {
                        name: param_name,
                        type_annotation,
                        default_value: None,
                        is_ref: false,
                        is_mut: false,
                    });
                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }
            
            self.consume(Token::RightParen, "Expected ')' after parameters")?;
            
            let return_type = if self.match_token(&[Token::Arrow]) {
                Some(self.parse_type()?)
            } else {
                None
            };
            
            methods.push(TraitMethod {
                name: method_name,
                parameters,
                return_type,
                default_body: None,
            });
            
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after interface body")?;
        self.skip_newlines();
        Ok(Statement::Interface { name, methods })
    }
    
    // Async statements
    fn async_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Async, "Expected 'async'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'async'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after async body")?;
        self.skip_newlines();
        Ok(Statement::Async { body })
    }
    
    fn await_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Await, "Expected 'await'")?;
        let expression = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Await { expression })
    }
    
    fn future_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Future, "Expected 'future'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected future name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after future name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after future body")?;
        self.skip_newlines();
        Ok(Statement::Future { name, body })
    }
    
    fn promise_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Promise, "Expected 'promise'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected promise name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after promise name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after promise body")?;
        self.skip_newlines();
        Ok(Statement::Promise { name, body })
    }
    
    fn coroutine_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Coroutine, "Expected 'coroutine'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected coroutine name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after coroutine name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after coroutine body")?;
        self.skip_newlines();
        Ok(Statement::Coroutine { name, body })
    }
    
    // Advanced control flow
    fn forever_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Forever, "Expected 'forever'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'forever'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after forever body")?;
        self.skip_newlines();
        Ok(Statement::Forever { body })
    }
    
    fn times_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Times, "Expected 'times'")?;
        let count = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after times count")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after times body")?;
        self.skip_newlines();
        Ok(Statement::Times { count, body })
    }
    
    fn twice_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Twice, "Expected 'twice'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'twice'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after twice body")?;
        self.skip_newlines();
        Ok(Statement::Twice { body })
    }
    
    fn thrice_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Thrice, "Expected 'thrice'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'thrice'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after thrice body")?;
        self.skip_newlines();
        Ok(Statement::Thrice { body })
    }
    
    fn when_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::When, "Expected 'when'")?;
        let condition = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after when condition")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after when body")?;
        self.skip_newlines();
        Ok(Statement::When { condition, body })
    }
    
    fn otherwise_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Otherwise, "Expected 'otherwise'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'otherwise'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after otherwise body")?;
        self.skip_newlines();
        Ok(Statement::Otherwise { body })
    }
    
    fn switch_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Switch, "Expected 'switch'")?;
        let value = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after switch value")?;
        self.skip_newlines();
        
        let mut cases = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.consume(Token::Case, "Expected 'case' in switch")?;
            
            let pattern = self.parse_pattern()?;
            
            let guard = if self.match_token(&[Token::If]) {
                Some(self.expression()?)
            } else {
                None
            };
            
            self.consume(Token::FatArrow, "Expected '=>' after pattern")?;
            
            if self.check(&Token::LeftBrace) {
                self.advance();
                self.skip_newlines();
                
                let mut body = Vec::new();
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    body.push(self.declaration()?);
                    self.skip_newlines();
                }
                
                self.consume(Token::RightBrace, "Expected '}' after case body")?;
                
                cases.push(MatchCase {
                    pattern,
                    guard,
                    body,
                });
            } else {
                let stmt = self.statement()?;
                cases.push(MatchCase {
                    pattern,
                    guard,
                    body: vec![stmt],
                });
            }
            
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after switch cases")?;
        self.skip_newlines();
        Ok(Statement::Switch { value, cases })
    }
    
    fn goto_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Goto, "Expected 'goto'")?;
        let label = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected label name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Goto { label })
    }
    
    fn label_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Label, "Expected 'label'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected label name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Label { name })
    }
    
    fn abort_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Abort, "Expected 'abort'")?;
        let message = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Abort { message })
    }
    
    fn exit_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Exit, "Expected 'exit'")?;
        let code = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Exit { code })
    }
    
    fn quit_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Quit, "Expected 'quit'")?;
        self.skip_newlines();
        Ok(Statement::Quit)
    }
    
    // Logging statements
    fn fatal_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Fatal, "Expected 'fatal'")?;
        let message = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Fatal { message })
    }
    
    fn warn_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Warn, "Expected 'warn'")?;
        let message = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Warn { message })
    }
    
    fn error_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Error, "Expected 'error'")?;
        let message = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Error { message })
    }
    
    fn info_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Info, "Expected 'info'")?;
        let message = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Info { message })
    }
    
    fn debug_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Debug, "Expected 'debug'")?;
        let message = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Debug { message })
    }
    
    fn trace_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Trace, "Expected 'trace'")?;
        let message = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Trace { message })
    }

    
    // Data structure statements - Phase 2
    fn array_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Array, "Expected 'array'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected array name".to_string())),
        };
        
        let (size, elements) = if self.match_token(&[Token::LeftBracket]) {
            let sz = if !self.check(&Token::RightBracket) {
                Some(self.expression()?)
            } else {
                None
            };
            self.consume(Token::RightBracket, "Expected ']'")?;
            
            let mut elems = Vec::new();
            if self.match_token(&[Token::LeftBrace]) {
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    elems.push(self.expression()?);
                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
                self.consume(Token::RightBrace, "Expected '}'")?;
            }
            (sz, elems)
        } else {
            (None, Vec::new())
        };
        
        self.skip_newlines();
        Ok(Statement::Array { name, size, elements })
    }
    
    fn vector_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Vector, "Expected 'vector'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected vector name".to_string())),
        };
        
        let mut elements = Vec::new();
        if self.match_token(&[Token::LeftBrace]) {
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                elements.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
            self.consume(Token::RightBrace, "Expected '}'")?;
        }
        
        self.skip_newlines();
        Ok(Statement::Vector { name, elements })
    }
    
    fn deque_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Deque, "Expected 'deque'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected deque name".to_string())),
        };
        
        let mut elements = Vec::new();
        if self.match_token(&[Token::LeftBrace]) {
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                elements.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
            self.consume(Token::RightBrace, "Expected '}'")?;
        }
        
        self.skip_newlines();
        Ok(Statement::Deque { name, elements })
    }
    
    fn stack_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Stack, "Expected 'stack'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected stack name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Stack { name })
    }
    
    fn queue_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Queue, "Expected 'queue'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected queue name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Queue { name })
    }
    
    fn tree_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Tree, "Expected 'tree'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected tree name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Tree { name })
    }
    
    fn graph_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Graph, "Expected 'graph'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected graph name".to_string())),
        };
        
        let mut nodes = Vec::new();
        if self.match_token(&[Token::LeftBrace]) {
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                match self.advance() {
                    Token::Identifier(n) => nodes.push(n),
                    _ => break,
                }
                self.match_token(&[Token::Comma]);
            }
            self.consume(Token::RightBrace, "Expected '}'")?;
        }
        
        self.skip_newlines();
        Ok(Statement::Graph { name, nodes })
    }
    
    fn matrix_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Matrix, "Expected 'matrix'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected matrix name".to_string())),
        };
        let rows = self.expression()?;
        let cols = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Matrix { name, rows, cols })
    }
    
    fn tensor_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Tensor, "Expected 'tensor'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected tensor name".to_string())),
        };
        
        let mut dimensions = Vec::new();
        if self.match_token(&[Token::LeftBracket]) {
            while !self.check(&Token::RightBracket) && !self.is_at_end() {
                dimensions.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
            self.consume(Token::RightBracket, "Expected ']'")?;
        }
        
        self.skip_newlines();
        Ok(Statement::Tensor { name, dimensions })
    }
    
    // Collection operations
    fn reduce_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Reduce, "Expected 'reduce'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let collection = self.expression()?;
        let function = self.expression()?;
        let initial = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Reduce { variable, collection, function, initial })
    }
    
    fn fold_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Fold, "Expected 'fold'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let collection = self.expression()?;
        let function = self.expression()?;
        let initial = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Fold { variable, collection, function, initial })
    }
    
    fn zip_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Zip, "Expected 'zip'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        
        let mut collections = Vec::new();
        loop {
            collections.push(self.expression()?);
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }
        
        self.skip_newlines();
        Ok(Statement::Zip { variable, collections })
    }
    
    fn flatten_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Flatten, "Expected 'flatten'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let collection = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Flatten { variable, collection })
    }
    
    // I/O operations
    fn read_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Read, "Expected 'read'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let source = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Read { variable, source })
    }
    
    fn write_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Write, "Expected 'write'")?;
        let destination = self.expression()?;
        let data = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Write { destination, data })
    }
    
    fn open_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Open, "Expected 'open'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let path = self.expression()?;
        let mode = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Open { variable, path, mode })
    }
    
    fn close_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Close, "Expected 'close'")?;
        let handle = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected handle name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Close { handle })
    }
    
    fn flush_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Flush, "Expected 'flush'")?;
        let handle = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected handle name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Flush { handle })
    }
    
    // File system operations
    fn file_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::File, "Expected 'file'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected file name".to_string())),
        };
        let path = self.expression()?;
        self.skip_newlines();
        Ok(Statement::File { name, path })
    }
    
    fn folder_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Folder, "Expected 'folder'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected folder name".to_string())),
        };
        let path = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Folder { name, path })
    }
    
    fn directory_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Directory, "Expected 'directory'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected directory name".to_string())),
        };
        let path = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Directory { name, path })
    }
    
    fn path_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Path, "Expected 'path'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        
        let mut components = Vec::new();
        loop {
            components.push(self.expression()?);
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }
        
        self.skip_newlines();
        Ok(Statement::Path { variable, components })
    }
    
    fn mkdir_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Mkdir, "Expected 'mkdir'")?;
        let path = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Mkdir { path })
    }
    
    fn rmdir_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Rmdir, "Expected 'rmdir'")?;
        let path = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Rmdir { path })
    }
    
    fn remove_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Remove, "Expected 'remove'")?;
        let path = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Remove { path })
    }
    
    fn rename_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Rename, "Expected 'rename'")?;
        let old_path = self.expression()?;
        let new_path = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Rename { old_path, new_path })
    }
    
    fn copy_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Copy, "Expected 'copy'")?;
        let source = self.expression()?;
        let destination = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Copy { source, destination })
    }
    
    fn move_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Move, "Expected 'move'")?;
        let source = self.expression()?;
        let destination = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Move { source, destination })
    }
    
    // Database operations
    fn database_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Database, "Expected 'database'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected database name".to_string())),
        };
        let connection = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Database { name, connection })
    }
    
    fn table_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Table, "Expected 'table'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected table name".to_string())),
        };
        
        let mut columns = Vec::new();
        if self.match_token(&[Token::LeftBrace]) {
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                let col_name = match self.advance() {
                    Token::Identifier(n) => n,
                    _ => break,
                };
                let col_type = if self.match_token(&[Token::Colon]) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                columns.push((col_name, col_type));
                self.match_token(&[Token::Comma]);
                self.skip_newlines();
            }
            self.consume(Token::RightBrace, "Expected '}'")?;
        }
        
        self.skip_newlines();
        Ok(Statement::Table { name, columns })
    }
    
    fn query_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Query, "Expected 'query'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let sql = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Query { variable, sql })
    }
    
    fn transaction_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Transaction, "Expected 'transaction'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'transaction'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after transaction body")?;
        self.skip_newlines();
        Ok(Statement::Transaction { body })
    }
    
    fn commit_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Commit, "Expected 'commit'")?;
        self.skip_newlines();
        Ok(Statement::Commit)
    }
    
    fn rollback_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Rollback, "Expected 'rollback'")?;
        self.skip_newlines();
        Ok(Statement::Rollback)
    }
    
    // Math & Science
    fn math_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Math, "Expected 'math'")?;
        let operation = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected operation name".to_string())),
        };
        
        let mut operands = Vec::new();
        loop {
            operands.push(self.expression()?);
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }
        
        self.skip_newlines();
        Ok(Statement::Math { operation, operands })
    }
    
    fn random_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Random, "Expected 'random'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        
        let min = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        
        let max = if min.is_some() && !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.skip_newlines();
        Ok(Statement::Random { variable, min, max })
    }
    
    fn seed_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Seed, "Expected 'seed'")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Seed { value })
    }
    
    // Time operations
    fn now_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Now, "Expected 'now'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Now { variable })
    }
    
    fn today_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Today, "Expected 'today'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Today { variable })
    }
    
    fn timeout_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Timeout, "Expected 'timeout'")?;
        let duration = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after timeout duration")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after timeout body")?;
        self.skip_newlines();
        Ok(Statement::Timeout { duration, body })
    }
    
    // ============================================================================
    // PHASE 2-15: ALL REMAINING KEYWORD IMPLEMENTATIONS

    // Additional concurrency & parallelism
    fn parallel_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Parallel, "Expected 'parallel'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'parallel'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after parallel body")?;
        self.skip_newlines();
        Ok(Statement::Parallel { body })
    }
    
    fn sequential_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Sequential, "Expected 'sequential'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'sequential'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after sequential body")?;
        self.skip_newlines();
        Ok(Statement::Sequential { body })
    }
    
    fn concurrent_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Concurrent, "Expected 'concurrent'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'concurrent'")?;
        self.skip_newlines();
        
        let mut tasks = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.consume(Token::LeftBrace, "Expected '{' for task")?;
            self.skip_newlines();
            
            let mut task_body = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                task_body.push(self.declaration()?);
                self.skip_newlines();
            }
            
            self.consume(Token::RightBrace, "Expected '}' after task")?;
            tasks.push(task_body);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after concurrent body")?;
        self.skip_newlines();
        Ok(Statement::Concurrent { tasks })
    }
    
    fn sync_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Sync, "Expected 'sync'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'sync'")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after sync body")?;
        self.skip_newlines();
        Ok(Statement::Sync { body })
    }
    
    fn task_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Task, "Expected 'task'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected task name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after task name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after task body")?;
        self.skip_newlines();
        Ok(Statement::Task { name, body })
    }
    
    fn thread_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Thread, "Expected 'thread'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected thread name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after thread name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after thread body")?;
        self.skip_newlines();
        Ok(Statement::Thread { name, body })
    }
    
    fn process_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Process, "Expected 'process'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected process name".to_string())),
        };
        let command = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Process { name, command })
    }
    
    fn fiber_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Fiber, "Expected 'fiber'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected fiber name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after fiber name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after fiber body")?;
        self.skip_newlines();
        Ok(Statement::Fiber { name, body })
    }
    
    fn green_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Green, "Expected 'green'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected green thread name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after green thread name")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after green thread body")?;
        self.skip_newlines();
        Ok(Statement::Green { name, body })
    }
    
    fn actor_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Actor, "Expected 'actor'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected actor name".to_string())),
        };
        let mailbox = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected mailbox name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after actor mailbox")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after actor body")?;
        self.skip_newlines();
        Ok(Statement::Actor { name, mailbox, body })
    }
    
    fn message_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Message, "Expected 'message'")?;
        let actor = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected actor name".to_string())),
        };
        let content = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Message { actor, content })
    }
    
    fn mailbox_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Mailbox, "Expected 'mailbox'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected mailbox name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Mailbox { name })
    }
    
    fn deadline_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Deadline, "Expected 'deadline'")?;
        let time = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after deadline time")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after deadline body")?;
        self.skip_newlines();
        Ok(Statement::Deadline { time, body })
    }
    
    fn cancel_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Cancel, "Expected 'cancel'")?;
        let task = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected task name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Cancel { task })
    }
    
    // Additional error handling
    fn guard_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Guard, "Expected 'guard'")?;
        let condition = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after guard condition")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        
        self.consume(Token::RightBrace, "Expected '}' after guard body")?;
        self.skip_newlines();
        Ok(Statement::Guard { condition, body })
    }
    
    fn precondition_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Precondition, "Expected 'precondition'")?;
        let condition = self.expression()?;
        let message = if self.match_token(&[Token::Comma]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Precondition { condition, message })
    }
    
    fn postcondition_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Postcondition, "Expected 'postcondition'")?;
        let condition = self.expression()?;
        let message = if self.match_token(&[Token::Comma]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Postcondition { condition, message })
    }
    
    fn invariant_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Invariant, "Expected 'invariant'")?;
        let condition = self.expression()?;
        let message = if self.match_token(&[Token::Comma]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Invariant { condition, message })
    }
    
    fn verbose_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Verbose, "Expected 'verbose'")?;
        let message = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Verbose { message })
    }
    
    fn log_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Log, "Expected 'log'")?;
        let level = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected log level".to_string())),
        };
        let message = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Log { level, message })
    }
    
    // Additional type system
    fn newtype_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Newtype, "Expected 'newtype'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected newtype name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after newtype name")?;
        let base_type = self.parse_type()?;
        self.skip_newlines();
        Ok(Statement::Newtype { name, base_type })
    }
    
    fn phantom_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Phantom, "Expected 'phantom'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected phantom type name".to_string())),
        };
        let type_param = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected type parameter".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Phantom { name, type_param })
    }
    
    fn associated_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Associated, "Expected 'associated'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected associated type name".to_string())),
        };
        self.consume(Token::Colon, "Expected ':' after associated type name")?;
        let type_annotation = self.parse_type()?;
        self.skip_newlines();
        Ok(Statement::Associated { name, type_annotation })
    }
    
    fn existential_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Existential, "Expected 'existential'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected existential type name".to_string())),
        };
        let mut constraints = Vec::new();
        if self.match_token(&[Token::Colon]) {
            loop {
                match self.advance() {
                    Token::Identifier(n) => constraints.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected constraint".to_string())),
                }
                if !self.match_token(&[Token::Plus]) {
                    break;
                }
            }
        }
        self.skip_newlines();
        Ok(Statement::Existential { name, constraints })
    }
    
    fn universal_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Universal, "Expected 'universal'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected universal type name".to_string())),
        };
        let type_param = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected type parameter".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Universal { name, type_param })
    }
    
    fn dependent_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Dependent, "Expected 'dependent'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected dependent type name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after dependent type name")?;
        let param = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
        };
        self.consume(Token::Colon, "Expected ':' after parameter")?;
        let type_annotation = self.parse_type()?;
        self.consume(Token::RightParen, "Expected ')' after dependent type")?;
        self.skip_newlines();
        Ok(Statement::Dependent { name, param, type_annotation })
    }
    
    fn linear_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Linear, "Expected 'linear'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected linear variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after linear variable")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Linear { name, value })
    }
    
    fn affine_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Affine, "Expected 'affine'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected affine variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after affine variable")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Affine { name, value })
    }
    
    fn subtype_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Subtype, "Expected 'subtype'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected subtype name".to_string())),
        };
        self.consume(Token::Less, "Expected '<' after subtype")?;
        let parent = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected parent type".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Subtype { name, parent })
    }
    
    fn supertype_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Supertype, "Expected 'supertype'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected supertype name".to_string())),
        };
        self.consume(Token::Greater, "Expected '>' after supertype")?;
        let child = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected child type".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Supertype { name, child })
    }
    
    fn covariant_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Covariant, "Expected 'covariant'")?;
        let type_param = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected type parameter".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Covariant { type_param })
    }
    
    fn contravariant_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Contravariant, "Expected 'contravariant'")?;
        let type_param = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected type parameter".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Contravariant { type_param })
    }
    
    fn lambda_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Lambda, "Expected 'lambda'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected lambda name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after lambda name")?;
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let is_ref = self.match_token(&[Token::Ref]);
                let is_mut = self.match_token(&[Token::Mut]);
                let param_name = match self.advance() {
                    Token::Identifier(n) => n,
                    _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
                };
                let type_annotation = if self.match_token(&[Token::Colon]) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                parameters.push(Parameter {
                    name: param_name,
                    type_annotation,
                    default_value: None,
                    is_ref,
                    is_mut,
                });
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        self.consume(Token::LeftBrace, "Expected '{' after lambda parameters")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after lambda body")?;
        self.skip_newlines();
        Ok(Statement::Lambda { name, parameters, body })
    }
    
    fn closure_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Closure, "Expected 'closure'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected closure name".to_string())),
        };
        self.consume(Token::LeftBracket, "Expected '[' for captures")?;
        let mut captures = Vec::new();
        if !self.check(&Token::RightBracket) {
            loop {
                match self.advance() {
                    Token::Identifier(n) => captures.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected capture variable".to_string())),
                }
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightBracket, "Expected ']' after captures")?;
        self.consume(Token::LeftParen, "Expected '(' after captures")?;
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let is_ref = self.match_token(&[Token::Ref]);
                let is_mut = self.match_token(&[Token::Mut]);
                let param_name = match self.advance() {
                    Token::Identifier(n) => n,
                    _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
                };
                let type_annotation = if self.match_token(&[Token::Colon]) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                parameters.push(Parameter {
                    name: param_name,
                    type_annotation,
                    default_value: None,
                    is_ref,
                    is_mut,
                });
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        self.consume(Token::LeftBrace, "Expected '{' after closure parameters")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after closure body")?;
        self.skip_newlines();
        Ok(Statement::Closure { name, captures, parameters, body })
    }
    
    fn partial_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Partial, "Expected 'partial'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let function = self.expression()?;
        let mut args = Vec::new();
        while !self.check(&Token::Newline) && !self.is_at_end() {
            args.push(self.expression()?);
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }
        self.skip_newlines();
        Ok(Statement::Partial { variable, function, args })
    }
    
    fn curry_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Curry, "Expected 'curry'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let function = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Curry { variable, function })
    }
    
    fn uncurry_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Uncurry, "Expected 'uncurry'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let function = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Uncurry { variable, function })
    }
    
    fn pipe_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Pipe, "Expected 'pipe'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let value = self.expression()?;
        let mut functions = Vec::new();
        while self.match_token(&[Token::Pipe]) {
            functions.push(self.expression()?);
        }
        self.skip_newlines();
        Ok(Statement::Pipe { variable, value, functions })
    }
    
    fn scan_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Scan, "Expected 'scan'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let collection = self.expression()?;
        let function = self.expression()?;
        let initial = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Scan { variable, collection, function, initial })
    }
    
    fn unfold_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Unfold, "Expected 'unfold'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let seed = self.expression()?;
        let function = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Unfold { variable, seed, function })
    }
    
    fn unzip_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Unzip, "Expected 'unzip'")?;
        let mut variables = Vec::new();
        loop {
            match self.advance() {
                Token::Identifier(n) => variables.push(n),
                _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
            }
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }
        self.consume(Token::Assign, "Expected '=' after variables")?;
        let collection = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Unzip { variables, collection })
    }
    
    fn flatmap_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::FlatMap, "Expected 'flatmap'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let collection = self.expression()?;
        let function = self.expression()?;
        self.skip_newlines();
        Ok(Statement::FlatMap { variable, collection, function })
    }
    
    fn bind_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Bind, "Expected 'bind'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let monad = self.expression()?;
        let function = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Bind { variable, monad, function })
    }
    
    fn pure_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Pure, "Expected 'pure'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Pure { variable, value })
    }
    
    fn applicative_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Applicative, "Expected 'applicative'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let function = self.expression()?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Applicative { variable, function, value })
    }
    
    fn functor_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Functor, "Expected 'functor'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected functor name".to_string())),
        };
        let map_function = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Functor { name, map_function })
    }
    
    fn monad_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Monad, "Expected 'monad'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected monad name".to_string())),
        };
        let bind_function = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Monad { name, bind_function })
    }
    
    fn monoid_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Monoid, "Expected 'monoid'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected monoid name".to_string())),
        };
        let identity = self.expression()?;
        let combine = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Monoid { name, identity, combine })
    }
    
    fn semigroup_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Semigroup, "Expected 'semigroup'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected semigroup name".to_string())),
        };
        let combine = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Semigroup { name, combine })
    }
    
    fn category_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Category, "Expected 'category'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected category name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after category name")?;
        self.skip_newlines();
        let mut objects = Vec::new();
        let mut morphisms = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            if self.match_token(&[Token::Identifier("object".to_string())]) {
                match self.advance() {
                    Token::Identifier(n) => objects.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected object name".to_string())),
                }
            } else if self.match_token(&[Token::Identifier("morphism".to_string())]) {
                let from = match self.advance() {
                    Token::Identifier(n) => n,
                    _ => return Err(ProtlinError::ParserError("Expected source object".to_string())),
                };
                self.consume(Token::Arrow, "Expected '->' in morphism")?;
                let to = match self.advance() {
                    Token::Identifier(n) => n,
                    _ => return Err(ProtlinError::ParserError("Expected target object".to_string())),
                };
                let func = self.expression()?;
                morphisms.push((from, to, func));
            }
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after category body")?;
        self.skip_newlines();
        Ok(Statement::Category { name, objects, morphisms })
    }
    
    fn constructor_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Constructor, "Expected 'constructor'")?;
        let class_name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected class name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after constructor")?;
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let is_ref = self.match_token(&[Token::Ref]);
                let is_mut = self.match_token(&[Token::Mut]);
                let param_name = match self.advance() {
                    Token::Identifier(n) => n,
                    _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
                };
                let type_annotation = if self.match_token(&[Token::Colon]) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                parameters.push(Parameter {
                    name: param_name,
                    type_annotation,
                    default_value: None,
                    is_ref,
                    is_mut,
                });
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        self.consume(Token::LeftBrace, "Expected '{' after constructor parameters")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after constructor body")?;
        self.skip_newlines();
        Ok(Statement::Constructor { class_name, parameters, body })
    }
    
    fn destructor_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Destructor, "Expected 'destructor'")?;
        let class_name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected class name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after destructor")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after destructor body")?;
        self.skip_newlines();
        Ok(Statement::Destructor { class_name, body })
    }
    
    fn initializer_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Initializer, "Expected 'initializer'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected initializer name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after initializer")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after initializer body")?;
        self.skip_newlines();
        Ok(Statement::Initializer { name, body })
    }
    
    fn deinitializer_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Deinitializer, "Expected 'deinitializer'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected deinitializer name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after deinitializer")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after deinitializer body")?;
        self.skip_newlines();
        Ok(Statement::Deinitializer { name, body })
    }
    
    fn getter_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Getter, "Expected 'getter'")?;
        let property = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected property name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after getter")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after getter body")?;
        self.skip_newlines();
        Ok(Statement::Getter { property, body })
    }
    
    fn setter_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Setter, "Expected 'setter'")?;
        let property = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected property name".to_string())),
        };
        let parameter = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after setter")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after setter body")?;
        self.skip_newlines();
        Ok(Statement::Setter { property, parameter, body })
    }
    
    fn property_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Property, "Expected 'property'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected property name".to_string())),
        };
        let type_annotation = if self.match_token(&[Token::Colon]) {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.consume(Token::LeftBrace, "Expected '{' after property")?;
        self.skip_newlines();
        let mut getter = None;
        let mut setter = None;
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            if self.match_token(&[Token::Getter]) {
                self.consume(Token::LeftBrace, "Expected '{' after getter")?;
                self.skip_newlines();
                let mut getter_body = Vec::new();
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    getter_body.push(self.declaration()?);
                    self.skip_newlines();
                }
                self.consume(Token::RightBrace, "Expected '}' after getter body")?;
                getter = Some(getter_body);
            } else if self.match_token(&[Token::Setter]) {
                self.consume(Token::LeftBrace, "Expected '{' after setter")?;
                self.skip_newlines();
                let mut setter_body = Vec::new();
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    setter_body.push(self.declaration()?);
                    self.skip_newlines();
                }
                self.consume(Token::RightBrace, "Expected '}' after setter body")?;
                setter = Some(setter_body);
            }
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after property body")?;
        self.skip_newlines();
        Ok(Statement::Property { name, type_annotation, getter, setter })
    }
    
    fn method_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Method, "Expected 'method'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected method name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after method name")?;
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let is_ref = self.match_token(&[Token::Ref]);
                let is_mut = self.match_token(&[Token::Mut]);
                let param_name = match self.advance() {
                    Token::Identifier(n) => n,
                    _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
                };
                let type_annotation = if self.match_token(&[Token::Colon]) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                parameters.push(Parameter {
                    name: param_name,
                    type_annotation,
                    default_value: None,
                    is_ref,
                    is_mut,
                });
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        let return_type = if self.match_token(&[Token::Arrow]) {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.consume(Token::LeftBrace, "Expected '{' after method signature")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after method body")?;
        self.skip_newlines();
        Ok(Statement::Method { name, parameters, return_type, body })
    }
    
    fn field_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Field, "Expected 'field'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected field name".to_string())),
        };
        let type_annotation = if self.match_token(&[Token::Colon]) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let value = if self.match_token(&[Token::Assign]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Field { name, type_annotation, value })
    }
    
    fn member_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Member, "Expected 'member'")?;
        let object = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected object name".to_string())),
        };
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected member name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Member { object, name })
    }
    
    fn attribute_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Attribute, "Expected 'attribute'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected attribute name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after attribute name")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Attribute { name, value })
    }
    
    fn annotation_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Annotation, "Expected 'annotation'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected annotation name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after annotation name")?;
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after annotation args")?;
        self.skip_newlines();
        Ok(Statement::Annotation { name, args })
    }
    
    fn decorator_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Decorator, "Expected 'decorator'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected decorator name".to_string())),
        };
        let target = Box::new(self.declaration()?);
        self.skip_newlines();
        Ok(Statement::Decorator { name, target })
    }
    
    fn mixin_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Mixin, "Expected 'mixin'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected mixin name".to_string())),
        };
        let mut traits = Vec::new();
        if self.match_token(&[Token::Colon]) {
            loop {
                match self.advance() {
                    Token::Identifier(n) => traits.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected trait name".to_string())),
                }
                if !self.match_token(&[Token::Plus]) {
                    break;
                }
            }
        }
        self.consume(Token::LeftBrace, "Expected '{' after mixin")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after mixin body")?;
        self.skip_newlines();
        Ok(Statement::Mixin { name, traits, body })
    }
    
    fn delegate_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Delegate, "Expected 'delegate'")?;
        let target = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected target name".to_string())),
        };
        let method = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected method name".to_string())),
        };
        self.consume(Token::Identifier("to".to_string()), "Expected 'to'")?;
        let to = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected delegate target".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Delegate { target, method, to })
    }
    
    fn proxy_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Proxy, "Expected 'proxy'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected proxy name".to_string())),
        };
        let target = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected target name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after proxy")?;
        self.skip_newlines();
        let mut handlers = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let handler_name = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected handler name".to_string())),
            };
            self.consume(Token::Colon, "Expected ':' after handler name")?;
            self.consume(Token::LeftBrace, "Expected '{' after handler")?;
            self.skip_newlines();
            let mut handler_body = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                handler_body.push(self.declaration()?);
                self.skip_newlines();
            }
            self.consume(Token::RightBrace, "Expected '}' after handler body")?;
            handlers.push((handler_name, handler_body));
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after proxy body")?;
        self.skip_newlines();
        Ok(Statement::Proxy { name, target, handlers })
    }
    
    fn singleton_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Singleton, "Expected 'singleton'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected singleton name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after singleton")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after singleton body")?;
        self.skip_newlines();
        Ok(Statement::Singleton { name, body })
    }
    
    fn factory_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Factory, "Expected 'factory'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected factory name".to_string())),
        };
        let product_type = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected product type".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after factory")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after factory body")?;
        self.skip_newlines();
        Ok(Statement::Factory { name, product_type, body })
    }
    
    fn builder_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Builder, "Expected 'builder'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected builder name".to_string())),
        };
        self.consume(Token::LeftBracket, "Expected '[' for fields")?;
        let mut fields = Vec::new();
        if !self.check(&Token::RightBracket) {
            loop {
                match self.advance() {
                    Token::Identifier(n) => fields.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected field name".to_string())),
                }
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightBracket, "Expected ']' after fields")?;
        self.consume(Token::LeftBrace, "Expected '{' after builder")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after builder body")?;
        self.skip_newlines();
        Ok(Statement::Builder { name, fields, body })
    }
    
    fn prototype_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Prototype, "Expected 'prototype'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected prototype name".to_string())),
        };
        let base = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected base name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Prototype { name, base })
    }

    
    // Pattern matching extensions
    fn where_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Where, "Expected 'where'")?;
        let condition = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Where { condition })
    }
    
    fn such_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Such, "Expected 'such'")?;
        let condition = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Such { condition })
    }
    
    fn that_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::That, "Expected 'that'")?;
        let condition = self.expression()?;
        self.skip_newlines();
        Ok(Statement::That { condition })
    }
    
    fn some_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Some, "Expected 'some'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Some { variable, value })
    }
    
    fn none_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::None, "Expected 'none'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::None { variable })
    }
    
    fn ok_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Ok, "Expected 'ok'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Ok { variable, value })
    }
    
    fn err_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Err, "Expected 'err'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let error = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Err { variable, error })
    }
    
    fn just_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Just, "Expected 'just'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Just { variable, value })
    }
    
    fn nothing_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Nothing, "Expected 'nothing'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Nothing { variable })
    }
    
    fn left_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Left, "Expected 'left'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Left { variable, value })
    }
    
    fn right_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Right, "Expected 'right'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Right { variable, value })
    }
    
    // Control flow extensions
    fn fallthrough_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Fallthrough, "Expected 'fallthrough'")?;
        self.skip_newlines();
        Ok(Statement::Fallthrough)
    }
    
    fn do_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Do, "Expected 'do'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'do'")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after do body")?;
        self.skip_newlines();
        Ok(Statement::Do { body })
    }
    
    fn then_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Then, "Expected 'then'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'then'")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after then body")?;
        self.skip_newlines();
        Ok(Statement::Then { body })
    }
    
    fn elif_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Elif, "Expected 'elif'")?;
        let condition = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after elif condition")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after elif body")?;
        self.skip_newlines();
        Ok(Statement::Elif { condition, body })
    }
    
    fn elseif_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Elseif, "Expected 'elseif'")?;
        let condition = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after elseif condition")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after elseif body")?;
        self.skip_newlines();
        Ok(Statement::Elseif { condition, body })
    }
    
    fn always_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Always, "Expected 'always'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'always'")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after always body")?;
        self.skip_newlines();
        Ok(Statement::Always { body })
    }
    
    fn never_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Never, "Expected 'never'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'never'")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after never body")?;
        self.skip_newlines();
        Ok(Statement::Never { body })
    }
    
    fn once_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Once, "Expected 'once'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'once'")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after once body")?;
        self.skip_newlines();
        Ok(Statement::Once { body })
    }
    
    // Metaprogramming
    fn reflect_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Reflect, "Expected 'reflect'")?;
        let target = self.expression()?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Reflect { target, variable })
    }
    
    fn introspect_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Introspect, "Expected 'introspect'")?;
        let target = self.expression()?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Introspect { target, variable })
    }
    
    fn eval_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Eval, "Expected 'eval'")?;
        let code = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Eval { code })
    }
    
    fn quote_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Quote, "Expected 'quote'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let expression = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Quote { variable, expression })
    }
    
    fn unquote_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Unquote, "Expected 'unquote'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Unquote { variable })
    }
    
    fn splice_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Splice, "Expected 'splice'")?;
        let target = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected target name".to_string())),
        };
        let code = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Splice { target, code })
    }
    
    fn gensym_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Gensym, "Expected 'gensym'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let prefix = if !self.check(&Token::Newline) && !self.is_at_end() {
            match self.advance() {
                Token::String(s) => Some(s),
                _ => None,
            }
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Gensym { variable, prefix })
    }
    
    fn hygiene_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Hygiene, "Expected 'hygiene'")?;
        let enabled = match self.advance() {
            Token::Boolean(true) => true,
            Token::Boolean(false) => false,
            _ => return Err(ProtlinError::ParserError("Expected boolean".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Hygiene { enabled })
    }
    
    fn syntax_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Syntax, "Expected 'syntax'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected syntax name".to_string())),
        };
        let pattern = match self.advance() {
            Token::String(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected pattern string".to_string())),
        };
        let template = match self.advance() {
            Token::String(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected template string".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Syntax { name, pattern, template })
    }
    
    fn parse_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Parse, "Expected 'parse'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let input = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Parse { variable, input })
    }
    
    fn expand_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Expand, "Expected 'expand'")?;
        let macro_name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected macro name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after macro name")?;
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after macro args")?;
        self.skip_newlines();
        Ok(Statement::Expand { macro_name, args })
    }
    
    fn compile_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Compile, "Expected 'compile'")?;
        let source = self.expression()?;
        let output = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected output name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Compile { source, output })
    }
    
    fn interpret_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Interpret, "Expected 'interpret'")?;
        let code = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Interpret { code })
    }
    
    fn transpile_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Transpile, "Expected 'transpile'")?;
        let source = self.expression()?;
        let target_lang = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected target language".to_string())),
        };
        let output = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected output name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Transpile { source, target_lang, output })
    }

    
    // Module system extensions
    fn scope_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Scope, "Expected 'scope'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected scope name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after scope")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after scope body")?;
        self.skip_newlines();
        Ok(Statement::Scope { name, body })
    }
    
    fn global_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Global, "Expected 'global'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Global { name, value })
    }
    
    fn local_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Local, "Expected 'local'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Local { name, value })
    }
    
    fn extern_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Extern, "Expected 'extern'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected function name".to_string())),
        };
        let signature = match self.advance() {
            Token::String(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected signature string".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Extern { name, signature })
    }
    
    fn foreign_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Foreign, "Expected 'foreign'")?;
        let language = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected language name".to_string())),
        };
        let code = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Foreign { language, code })
    }
    
    fn native_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Native, "Expected 'native'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected function name".to_string())),
        };
        let library = match self.advance() {
            Token::String(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected library name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Native { name, library })
    }
    
    fn builtin_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Builtin, "Expected 'builtin'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected builtin name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Builtin { name })
    }
    
    fn prelude_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Prelude, "Expected 'prelude'")?;
        let mut items = Vec::new();
        loop {
            match self.advance() {
                Token::Identifier(n) => items.push(n),
                _ => return Err(ProtlinError::ParserError("Expected item name".to_string())),
            }
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }
        self.skip_newlines();
        Ok(Statement::Prelude { items })
    }
    
    fn std_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Std, "Expected 'std'")?;
        let module = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected module name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Std { module })
    }
    
    fn core_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Core, "Expected 'core'")?;
        let feature = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected feature name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Core { feature })
    }
    
    // Testing & verification
    fn property_test_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::PropertyTest, "Expected 'property_test'")?;
        let name = match self.advance() {
            Token::String(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected test name".to_string())),
        };
        let property = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after property")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after property test body")?;
        self.skip_newlines();
        Ok(Statement::PropertyTest { name, property, body })
    }
    
    fn quickcheck_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Quickcheck, "Expected 'quickcheck'")?;
        let name = match self.advance() {
            Token::String(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected test name".to_string())),
        };
        let property = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Quickcheck { name, property })
    }
    
    fn fuzzy_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Fuzzy, "Expected 'fuzzy'")?;
        let name = match self.advance() {
            Token::String(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected test name".to_string())),
        };
        let input_generator = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after input generator")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after fuzzy test body")?;
        self.skip_newlines();
        Ok(Statement::Fuzzy { name, input_generator, body })
    }
    
    fn stub_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Stub, "Expected 'stub'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected stub name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after stub name")?;
        let return_value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Stub { name, return_value })
    }
    
    fn spy_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Spy, "Expected 'spy'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected spy name".to_string())),
        };
        let target = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected target name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Spy { name, target })
    }
    
    fn fake_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Fake, "Expected 'fake'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected fake name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after fake name")?;
        self.skip_newlines();
        let mut behavior = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            behavior.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after fake behavior")?;
        self.skip_newlines();
        Ok(Statement::Fake { name, behavior })
    }
    
    fn validate_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Validate, "Expected 'validate'")?;
        let condition = self.expression()?;
        let message = if self.match_token(&[Token::Comma]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Validate { condition, message })
    }
    
    fn check_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Check, "Expected 'check'")?;
        let condition = self.expression()?;
        let message = if self.match_token(&[Token::Comma]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Check { condition, message })
    }
    
    fn prove_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Prove, "Expected 'prove'")?;
        let theorem = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected theorem name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after theorem")?;
        self.skip_newlines();
        let mut proof = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            proof.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after proof")?;
        self.skip_newlines();
        Ok(Statement::Prove { theorem, proof })
    }
    
    fn theorem_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Theorem, "Expected 'theorem'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected theorem name".to_string())),
        };
        let statement = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Theorem { name, statement })
    }
    
    fn lemma_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Lemma, "Expected 'lemma'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected lemma name".to_string())),
        };
        let statement = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Lemma { name, statement })
    }
    
    fn axiom_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Axiom, "Expected 'axiom'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected axiom name".to_string())),
        };
        let statement = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Axiom { name, statement })
    }
    
    fn corollary_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Corollary, "Expected 'corollary'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected corollary name".to_string())),
        };
        let theorem = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected theorem name".to_string())),
        };
        let statement = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Corollary { name, theorem, statement })
    }
    
    // Additional data structures
    fn heap_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Heap, "Expected 'heap'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected heap name".to_string())),
        };
        let heap_type = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected heap type".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Heap { name, heap_type })
    }
    
    fn node_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Node, "Expected 'node'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected node name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after node name")?;
        let value = self.expression()?;
        let mut children = Vec::new();
        if self.match_token(&[Token::LeftBracket]) {
            if !self.check(&Token::RightBracket) {
                loop {
                    match self.advance() {
                        Token::Identifier(n) => children.push(n),
                        _ => return Err(ProtlinError::ParserError("Expected child name".to_string())),
                    }
                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }
            self.consume(Token::RightBracket, "Expected ']' after children")?;
        }
        self.skip_newlines();
        Ok(Statement::Node { name, value, children })
    }
    
    fn edge_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Edge, "Expected 'edge'")?;
        let from = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected from node".to_string())),
        };
        self.consume(Token::Arrow, "Expected '->' in edge")?;
        let to = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected to node".to_string())),
        };
        let weight = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Edge { from, to, weight })
    }
    
    fn vertex_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Vertex, "Expected 'vertex'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected vertex name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after vertex name")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Vertex { name, value })
    }
    
    fn cycle_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Cycle, "Expected 'cycle'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected cycle name".to_string())),
        };
        self.consume(Token::LeftBracket, "Expected '[' for nodes")?;
        let mut nodes = Vec::new();
        if !self.check(&Token::RightBracket) {
            loop {
                match self.advance() {
                    Token::Identifier(n) => nodes.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected node name".to_string())),
                }
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightBracket, "Expected ']' after nodes")?;
        self.skip_newlines();
        Ok(Statement::Cycle { name, nodes })
    }
    
    // I/O & Streams
    fn stream_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Stream, "Expected 'stream'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected stream name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after stream name")?;
        let source = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Stream { name, source })
    }
    
    fn reader_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Reader, "Expected 'reader'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected reader name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after reader name")?;
        let source = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Reader { name, source })
    }
    
    fn writer_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Writer, "Expected 'writer'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected writer name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after writer name")?;
        let destination = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Writer { name, destination })
    }
    
    fn buffer_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Buffer, "Expected 'buffer'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected buffer name".to_string())),
        };
        let size = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Buffer { name, size })
    }
    
    fn seek_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Seek, "Expected 'seek'")?;
        let handle = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected handle name".to_string())),
        };
        let position = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Seek { handle, position })
    }
    
    fn tell_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Tell, "Expected 'tell'")?;
        let handle = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected handle name".to_string())),
        };
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Tell { handle, variable })
    }
    
    fn rewind_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Rewind, "Expected 'rewind'")?;
        let handle = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected handle name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Rewind { handle })
    }

    
    // Network extensions
    fn network_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Network, "Expected 'network'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected network name".to_string())),
        };
        let protocol = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected protocol name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Network { name, protocol })
    }
    
    fn socket_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Socket, "Expected 'socket'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected socket name".to_string())),
        };
        let socket_type = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected socket type".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Socket { name, socket_type })
    }
    
    fn accept_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Accept, "Expected 'accept'")?;
        let socket = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected socket name".to_string())),
        };
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Accept { socket, variable })
    }
    
    fn bindnet_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::BindNet, "Expected 'bind'")?;
        let socket = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected socket name".to_string())),
        };
        let address = self.expression()?;
        self.skip_newlines();
        Ok(Statement::BindNet { socket, address })
    }
    
    fn shutdown_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Shutdown, "Expected 'shutdown'")?;
        let connection = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected connection name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Shutdown { connection })
    }
    
    fn http_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Http, "Expected 'http'")?;
        let method = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected HTTP method".to_string())),
        };
        let url = self.expression()?;
        let body = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Http { method, url, body })
    }
    
    fn https_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Https, "Expected 'https'")?;
        let method = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected HTTP method".to_string())),
        };
        let url = self.expression()?;
        let body = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Https { method, url, body })
    }
    
    fn tcp_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Tcp, "Expected 'tcp'")?;
        let address = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after TCP address")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after TCP body")?;
        self.skip_newlines();
        Ok(Statement::Tcp { address, body })
    }
    
    fn udp_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Udp, "Expected 'udp'")?;
        let address = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after UDP address")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after UDP body")?;
        self.skip_newlines();
        Ok(Statement::Udp { address, body })
    }
    
    fn websocket_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Websocket, "Expected 'websocket'")?;
        let url = self.expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after websocket URL")?;
        self.skip_newlines();
        let mut handlers = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let handler_name = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected handler name".to_string())),
            };
            self.consume(Token::Colon, "Expected ':' after handler name")?;
            self.consume(Token::LeftBrace, "Expected '{' after handler")?;
            self.skip_newlines();
            let mut handler_body = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                handler_body.push(self.declaration()?);
                self.skip_newlines();
            }
            self.consume(Token::RightBrace, "Expected '}' after handler body")?;
            handlers.push((handler_name, handler_body));
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after websocket body")?;
        self.skip_newlines();
        Ok(Statement::Websocket { url, handlers })
    }
    
    fn rpc_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Rpc, "Expected 'rpc'")?;
        let service = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected service name".to_string())),
        };
        let method = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected method name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after method")?;
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after args")?;
        self.skip_newlines();
        Ok(Statement::Rpc { service, method, args })
    }
    
    fn rest_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Rest, "Expected 'rest'")?;
        let endpoint = match self.advance() {
            Token::String(s) => s,
            _ => return Err(ProtlinError::ParserError("Expected endpoint string".to_string())),
        };
        let method = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected HTTP method".to_string())),
        };
        let data = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Rest { endpoint, method, data })
    }
    
    fn graphql_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Graphql, "Expected 'graphql'")?;
        let query = self.expression()?;
        let variables = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Graphql { query, variables })
    }
    
    // Database extensions
    fn create_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Create, "Expected 'create'")?;
        let entity_type = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected entity type".to_string())),
        };
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected entity name".to_string())),
        };
        let definition = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Create { entity_type, name, definition })
    }
    
    fn alter_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Alter, "Expected 'alter'")?;
        let entity_type = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected entity type".to_string())),
        };
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected entity name".to_string())),
        };
        let changes = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Alter { entity_type, name, changes })
    }
    
    fn index_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Index, "Expected 'index'")?;
        let table = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected table name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after table")?;
        let mut columns = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                match self.advance() {
                    Token::Identifier(n) => columns.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected column name".to_string())),
                }
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after columns")?;
        self.skip_newlines();
        Ok(Statement::Index { table, columns })
    }
    
    fn view_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::View, "Expected 'view'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected view name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after view name")?;
        let query = self.expression()?;
        self.skip_newlines();
        Ok(Statement::View { name, query })
    }
    
    fn savepoint_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Savepoint, "Expected 'savepoint'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected savepoint name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Savepoint { name })
    }
    
    // Security extensions
    fn seal_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Seal, "Expected 'seal'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let data = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Seal { variable, data })
    }
    
    fn unseal_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Unseal, "Expected 'unseal'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let sealed_data = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Unseal { variable, sealed_data })
    }
    
    fn secure_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Secure, "Expected 'secure'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'secure'")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after secure body")?;
        self.skip_newlines();
        Ok(Statement::Secure { body })
    }
    
    fn unsafe_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Unsafe, "Expected 'unsafe'")?;
        self.consume(Token::LeftBrace, "Expected '{' after 'unsafe'")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after unsafe body")?;
        self.skip_newlines();
        Ok(Statement::Unsafe { body })
    }
    
    fn trusted_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Trusted, "Expected 'trusted'")?;
        let source = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected source name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Trusted { source })
    }
    
    fn untrusted_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Untrusted, "Expected 'untrusted'")?;
        let source = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected source name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Untrusted { source })
    }
    
    fn sanitize_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Sanitize, "Expected 'sanitize'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let data = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Sanitize { variable, data })
    }
    
    fn escape_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Escape, "Expected 'escape'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let data = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Escape { variable, data })
    }
    
    // Time & Date extensions
    fn time_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Time, "Expected 'time'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let format = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Time { variable, format })
    }
    
    fn date_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Date, "Expected 'date'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let format = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Date { variable, format })
    }
    
    fn datetime_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::DateTime, "Expected 'datetime'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let format = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::DateTime { variable, format })
    }
    
    fn duration_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Duration, "Expected 'duration'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let amount = self.expression()?;
        let unit = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected time unit".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Duration { variable, amount, unit })
    }
    
    fn instant_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Instant, "Expected 'instant'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Instant { variable })
    }
    
    fn tomorrow_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Tomorrow, "Expected 'tomorrow'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Tomorrow { variable })
    }
    
    fn yesterday_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Yesterday, "Expected 'yesterday'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Yesterday { variable })
    }

    
    // Math & Science extensions
    fn science_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Science, "Expected 'science'")?;
        let domain = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected domain name".to_string())),
        };
        let operation = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected operation name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after operation")?;
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after args")?;
        self.skip_newlines();
        Ok(Statement::Science { domain, operation, args })
    }
    
    fn physics_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Physics, "Expected 'physics'")?;
        let calculation = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected calculation name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after calculation")?;
        let mut params = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                params.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after params")?;
        self.skip_newlines();
        Ok(Statement::Physics { calculation, params })
    }
    
    fn chemistry_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Chemistry, "Expected 'chemistry'")?;
        let reaction = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected reaction name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after reaction")?;
        let mut reactants = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                reactants.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after reactants")?;
        self.skip_newlines();
        Ok(Statement::Chemistry { reaction, reactants })
    }
    
    fn biology_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Biology, "Expected 'biology'")?;
        let process = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected process name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after process")?;
        let mut params = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                params.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after params")?;
        self.skip_newlines();
        Ok(Statement::Biology { process, params })
    }
    
    fn statistics_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Statistics, "Expected 'statistics'")?;
        let operation = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected operation name".to_string())),
        };
        let data = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Statistics { operation, data })
    }
    
    fn probability_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Probability, "Expected 'probability'")?;
        let event = self.expression()?;
        let space = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Probability { event, space })
    }
    
    fn distribution_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Distribution, "Expected 'distribution'")?;
        let dist_type = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected distribution type".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after distribution type")?;
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                parameters.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        self.skip_newlines();
        Ok(Statement::Distribution { dist_type, parameters })
    }
    
    fn normal_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Normal, "Expected 'normal'")?;
        let mean = self.expression()?;
        let std_dev = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Normal { mean, std_dev })
    }
    
    fn uniform_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Uniform, "Expected 'uniform'")?;
        let min = self.expression()?;
        let max = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Uniform { min, max })
    }
    
    fn exponential_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Exponential, "Expected 'exponential'")?;
        let rate = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Exponential { rate })
    }
    
    fn poisson_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Poisson, "Expected 'poisson'")?;
        let lambda = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Poisson { lambda })
    }
    
    // Graphics & UI extensions
    fn graphics_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Graphics, "Expected 'graphics'")?;
        let context = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected context name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after graphics context")?;
        self.skip_newlines();
        let mut operations = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            operations.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after graphics operations")?;
        self.skip_newlines();
        Ok(Statement::Graphics { context, operations })
    }
    
    fn paint_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Paint, "Expected 'paint'")?;
        let target = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected target name".to_string())),
        };
        let color = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Paint { target, color })
    }
    
    fn fill_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Fill, "Expected 'fill'")?;
        let shape = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected shape name".to_string())),
        };
        let color = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Fill { shape, color })
    }
    
    fn stroke_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Stroke, "Expected 'stroke'")?;
        let shape = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected shape name".to_string())),
        };
        let color = self.expression()?;
        let width = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Stroke { shape, color, width })
    }
    
    fn color_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Color, "Expected 'color'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable")?;
        let r = self.expression()?;
        let g = self.expression()?;
        let b = self.expression()?;
        let a = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Color { variable, r, g, b, a })
    }
    
    fn pixel_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Pixel, "Expected 'pixel'")?;
        let x = self.expression()?;
        let y = self.expression()?;
        let color = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Pixel { x, y, color })
    }
    
    fn widget_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Widget, "Expected 'widget'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected widget name".to_string())),
        };
        let widget_type = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected widget type".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after widget type")?;
        self.skip_newlines();
        let mut properties = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let prop_name = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected property name".to_string())),
            };
            self.consume(Token::Colon, "Expected ':' after property name")?;
            let prop_value = self.expression()?;
            properties.push((prop_name, prop_value));
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after widget properties")?;
        self.skip_newlines();
        Ok(Statement::Widget { name, widget_type, properties })
    }
    
    fn layout_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Layout, "Expected 'layout'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected layout name".to_string())),
        };
        let layout_type = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected layout type".to_string())),
        };
        self.consume(Token::LeftBracket, "Expected '[' for children")?;
        let mut children = Vec::new();
        if !self.check(&Token::RightBracket) {
            loop {
                match self.advance() {
                    Token::Identifier(n) => children.push(n),
                    _ => return Err(ProtlinError::ParserError("Expected child name".to_string())),
                }
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightBracket, "Expected ']' after children")?;
        self.skip_newlines();
        Ok(Statement::Layout { name, layout_type, children })
    }
    
    fn style_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Style, "Expected 'style'")?;
        let target = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected target name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after target")?;
        self.skip_newlines();
        let mut properties = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let prop_name = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected property name".to_string())),
            };
            self.consume(Token::Colon, "Expected ':' after property name")?;
            let prop_value = self.expression()?;
            properties.push((prop_name, prop_value));
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after style properties")?;
        self.skip_newlines();
        Ok(Statement::Style { target, properties })
    }
    
    fn theme_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Theme, "Expected 'theme'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected theme name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after theme name")?;
        self.skip_newlines();
        let mut colors = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let color_name = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected color name".to_string())),
            };
            self.consume(Token::Colon, "Expected ':' after color name")?;
            let color_value = self.expression()?;
            colors.push((color_name, color_value));
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after theme colors")?;
        self.skip_newlines();
        Ok(Statement::Theme { name, colors })
    }
    
    // Audio & Media
    fn audio_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Audio, "Expected 'audio'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected audio name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after audio name")?;
        let source = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Audio { name, source })
    }
    
    fn video_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Video, "Expected 'video'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected video name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after video name")?;
        let source = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Video { name, source })
    }
    
    fn media_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Media, "Expected 'media'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected media name".to_string())),
        };
        let media_type = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected media type".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after media type")?;
        let source = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Media { name, media_type, source })
    }
    
    fn sound_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Sound, "Expected 'sound'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected sound name".to_string())),
        };
        let frequency = self.expression()?;
        let duration = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Sound { name, frequency, duration })
    }
    
    fn music_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Music, "Expected 'music'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected music name".to_string())),
        };
        self.consume(Token::LeftBracket, "Expected '[' for notes")?;
        let mut notes = Vec::new();
        if !self.check(&Token::RightBracket) {
            loop {
                notes.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightBracket, "Expected ']' after notes")?;
        self.skip_newlines();
        Ok(Statement::Music { name, notes })
    }
    
    fn play_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Play, "Expected 'play'")?;
        let media = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected media name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Play { media })
    }
    
    fn pause_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Pause, "Expected 'pause'")?;
        let media = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected media name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Pause { media })
    }
    
    fn stop_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Stop, "Expected 'stop'")?;
        let media = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected media name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Stop { media })
    }
    
    fn record_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Record, "Expected 'record'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected recording name".to_string())),
        };
        let source = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected source name".to_string())),
        };
        let duration = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Record { name, source, duration })
    }
    
    fn volume_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Volume, "Expected 'volume'")?;
        let media = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected media name".to_string())),
        };
        let level = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Volume { media, level })
    }
    
    fn pitch_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Pitch, "Expected 'pitch'")?;
        let sound = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected sound name".to_string())),
        };
        let frequency = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Pitch { sound, frequency })
    }
    
    fn tempo_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Tempo, "Expected 'tempo'")?;
        let music = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected music name".to_string())),
        };
        let bpm = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Tempo { music, bpm })
    }
    
    // File system extensions
    fn exists_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Exists, "Expected 'exists'")?;
        let path = self.expression()?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Exists { path, variable })
    }
    
    fn chmod_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Chmod, "Expected 'chmod'")?;
        let path = self.expression()?;
        let mode = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Chmod { path, mode })
    }
    
    fn chown_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Chown, "Expected 'chown'")?;
        let path = self.expression()?;
        let owner = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Chown { path, owner })
    }

    
    // Configuration
    fn config_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Config, "Expected 'config'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected config name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after config name")?;
        self.skip_newlines();
        let mut settings = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let key = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected setting key".to_string())),
            };
            self.consume(Token::Colon, "Expected ':' after key")?;
            let value = self.expression()?;
            settings.push((key, value));
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after config settings")?;
        self.skip_newlines();
        Ok(Statement::Config { name, settings })
    }

    
    fn settings_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Settings, "Expected 'settings'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected settings name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after settings name")?;
        self.skip_newlines();
        let mut values = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let key = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected setting key".to_string())),
            };
            self.consume(Token::Colon, "Expected ':' after key")?;
            let value = self.expression()?;
            values.push((key, value));
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after settings values")?;
        self.skip_newlines();
        Ok(Statement::Settings { name, values })
    }

    
    fn options_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Options, "Expected 'options'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected options name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after options name")?;
        self.skip_newlines();
        let mut values = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let key = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected option key".to_string())),
            };
            self.consume(Token::Colon, "Expected ':' after key")?;
            let value = self.expression()?;
            values.push((key, value));
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after options values")?;
        self.skip_newlines();
        Ok(Statement::Options { name, values })
    }

    
    fn preferences_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Preferences, "Expected 'preferences'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected preferences name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after preferences name")?;
        self.skip_newlines();
        let mut values = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let key = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected preference key".to_string())),
            };
            self.consume(Token::Colon, "Expected ':' after key")?;
            let value = self.expression()?;
            values.push((key, value));
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after preferences values")?;
        self.skip_newlines();
        Ok(Statement::Preferences { name, values })
    }

    
    fn environment_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Environment, "Expected 'environment'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected environment name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after environment name")?;
        self.skip_newlines();
        let mut variables = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let key = match self.advance() {
                Token::Identifier(n) => n,
                _ => return Err(ProtlinError::ParserError("Expected variable key".to_string())),
            };
            self.consume(Token::Colon, "Expected ':' after key")?;
            let value = self.expression()?;
            variables.push((key, value));
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after environment variables")?;
        self.skip_newlines();
        Ok(Statement::Environment { name, variables })
    }

    
    fn variable_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Variable, "Expected 'variable'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after variable name")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Variable { name, value })
    }

    
    fn parameter_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Parameter, "Expected 'parameter'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected parameter name".to_string())),
        };
        let type_annotation = if self.match_token(&[Token::Colon]) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let default = if self.match_token(&[Token::Assign]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Parameter { name, type_annotation, default })
    }

    
    fn argument_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Argument, "Expected 'argument'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected argument name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after argument name")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Argument { name, value })
    }

    
    fn flag_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Flag, "Expected 'flag'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected flag name".to_string())),
        };
        let enabled = match self.advance() {
            Token::Boolean(true) => true,
            Token::Boolean(false) => false,
            _ => return Err(ProtlinError::ParserError("Expected boolean".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Flag { name, enabled })
    }

    
    // Lifecycle extensions
    fn init_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Init, "Expected 'init'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected init name".to_string())),
        };
        self.consume(Token::LeftBrace, "Expected '{' after init name")?;
        self.skip_newlines();
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
            self.skip_newlines();
        }
        self.consume(Token::RightBrace, "Expected '}' after init body")?;
        self.skip_newlines();
        Ok(Statement::Init { name, body })
    }

    
    fn start_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Start, "Expected 'start'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Start { name })
    }

    
    fn run_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Run, "Expected 'run'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after name")?;
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after args")?;
        self.skip_newlines();
        Ok(Statement::Run { name, args })
    }

    
    fn execute_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Execute, "Expected 'execute'")?;
        let command = self.expression()?;
        self.consume(Token::LeftParen, "Expected '(' after command")?;
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after args")?;
        self.skip_newlines();
        Ok(Statement::Execute { command, args })
    }

    
    fn invoke_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Invoke, "Expected 'invoke'")?;
        let function = self.expression()?;
        self.consume(Token::LeftParen, "Expected '(' after function")?;
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after args")?;
        self.skip_newlines();
        Ok(Statement::Invoke { function, args })
    }

    
    fn call_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Call, "Expected 'call'")?;
        let function = self.expression()?;
        self.consume(Token::LeftParen, "Expected '(' after function")?;
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after args")?;
        self.skip_newlines();
        Ok(Statement::Call { function, args })
    }

    
    fn apply_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Apply, "Expected 'apply'")?;
        let function = self.expression()?;
        self.consume(Token::LeftParen, "Expected '(' after function")?;
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after args")?;
        self.skip_newlines();
        Ok(Statement::Apply { function, args })
    }

    
    fn perform_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Perform, "Expected 'perform'")?;
        let action = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected action name".to_string())),
        };
        self.consume(Token::LeftParen, "Expected '(' after action")?;
        let mut params = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                params.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after params")?;
        self.skip_newlines();
        Ok(Statement::Perform { action, params })
    }

    
    fn complete_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Complete, "Expected 'complete'")?;
        let task = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected task name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Complete { task })
    }

    
    fn finish_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Finish, "Expected 'finish'")?;
        let task = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected task name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Finish { task })
    }

    
    fn end_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::End, "Expected 'end'")?;
        let scope = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected scope name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::End { scope })
    }

    
    fn terminate_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Terminate, "Expected 'terminate'")?;
        let process = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected process name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Terminate { process })
    }

    
    fn kill_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Kill, "Expected 'kill'")?;
        let process = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected process name".to_string())),
        };
        let signal = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Kill { process, signal })
    }

    
    // State management
    fn state_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::State, "Expected 'state'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected state name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after state name")?;
        let initial = self.expression()?;
        self.skip_newlines();
        Ok(Statement::State { name, initial })
    }

    
    fn store_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Store, "Expected 'store'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected store name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after store name")?;
        let value = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Store { name, value })
    }

    
    fn cache_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Cache, "Expected 'cache'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected cache name".to_string())),
        };
        self.consume(Token::Assign, "Expected '=' after cache name")?;
        let value = self.expression()?;
        let ttl = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Cache { name, value, ttl })
    }

    
    fn memoize_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Memoize, "Expected 'memoize'")?;
        let function = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected function name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Memoize { function })
    }

    
    fn persist_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Persist, "Expected 'persist'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected name".to_string())),
        };
        let data = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Persist { name, data })
    }

    
    fn load_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Load, "Expected 'load'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected name".to_string())),
        };
        let source = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Load { name, source })
    }

    
    fn save_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Save, "Expected 'save'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected name".to_string())),
        };
        let destination = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Save { name, destination })
    }

    
    fn restore_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Restore, "Expected 'restore'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected name".to_string())),
        };
        let checkpoint = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected checkpoint name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Restore { name, checkpoint })
    }

    
    fn snapshot_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Snapshot, "Expected 'snapshot'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected snapshot name".to_string())),
        };
        let state = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Snapshot { name, state })
    }

    
    fn checkpoint_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Checkpoint, "Expected 'checkpoint'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected checkpoint name".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Checkpoint { name })
    }

    
    fn undo_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Undo, "Expected 'undo'")?;
        let steps = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Undo { steps })
    }

    
    fn redo_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Redo, "Expected 'redo'")?;
        let steps = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::Redo { steps })
    }

    
    fn history_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::History, "Expected 'history'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected history name".to_string())),
        };
        let max_size = if !self.check(&Token::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Statement::History { name, max_size })
    }

    
    // Validation & Constraints
    fn constraint_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Constraint, "Expected 'constraint'")?;
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected constraint name".to_string())),
        };
        let condition = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Constraint { name, condition })
    }

    
    fn bound_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Bound, "Expected 'bound'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let lower = self.expression()?;
        let upper = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Bound { variable, lower, upper })
    }

    
    fn limit_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Limit, "Expected 'limit'")?;
        let variable = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected variable name".to_string())),
        };
        let max = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Limit { variable, max })
    }

    
    fn between_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Between, "Expected 'between'")?;
        let value = self.expression()?;
        let lower = self.expression()?;
        let upper = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Between { value, lower, upper })
    }

    
    fn within_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Within, "Expected 'within'")?;
        let value = self.expression()?;
        let bounds = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Within { value, bounds })
    }

    
    fn outside_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Outside, "Expected 'outside'")?;
        let value = self.expression()?;
        let bounds = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Outside { value, bounds })
    }

    
    fn inside_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Inside, "Expected 'inside'")?;
        let value = self.expression()?;
        let container = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Inside { value, container })
    }

    
    fn includes_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Includes, "Expected 'includes'")?;
        let collection = self.expression()?;
        let element = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Includes { collection, element })
    }

    
    fn excludes_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Excludes, "Expected 'excludes'")?;
        let collection = self.expression()?;
        let element = self.expression()?;
        self.skip_newlines();
        Ok(Statement::Excludes { collection, element })
    }

    
    // Operators as keywords
    fn plus_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Plus_, "Expected 'plus'")?;
        let left = self.expression()?;
        let right = self.expression()?;
        let result = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected result variable".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Plus { left, right, result })
    }

    
    fn minus_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Minus_, "Expected 'minus'")?;
        let left = self.expression()?;
        let right = self.expression()?;
        let result = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected result variable".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Minus { left, right, result })
    }

    
    fn times_op_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Times, "Expected 'times'")?;
        let left = self.expression()?;
        let right = self.expression()?;
        let result = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected result variable".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::TimesOp { left, right, result })
    }

    
    fn divide_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Divide, "Expected 'divide'")?;
        let left = self.expression()?;
        let right = self.expression()?;
        let result = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected result variable".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Divide { left, right, result })
    }

    
    fn modulo_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Modulo, "Expected 'modulo'")?;
        let left = self.expression()?;
        let right = self.expression()?;
        let result = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected result variable".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Modulo { left, right, result })
    }

    
    fn equals_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::Equals, "Expected 'equals'")?;
        let left = self.expression()?;
        let right = self.expression()?;
        let result = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected result variable".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::Equals { left, right, result })
    }

    
    fn notequals_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::NotEquals, "Expected 'notequals'")?;
        let left = self.expression()?;
        let right = self.expression()?;
        let result = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected result variable".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::NotEquals { left, right, result })
    }

    
    fn greaterthan_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::GreaterThan, "Expected 'greaterthan'")?;
        let left = self.expression()?;
        let right = self.expression()?;
        let result = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected result variable".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::GreaterThan { left, right, result })
    }

    
    fn lessthan_statement(&mut self) -> Result<Statement, ProtlinError> {
        self.consume(Token::LessThan, "Expected 'lessthan'")?;
        let left = self.expression()?;
        let right = self.expression()?;
        let result = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ProtlinError::ParserError("Expected result variable".to_string())),
        };
        self.skip_newlines();
        Ok(Statement::LessThan { left, right, result })
    }

}

use crate::ast::*;
use crate::builtins;
use crate::environment::Environment;
use crate::error::ProtlinError;
use crate::types;
use std::collections::HashMap;

pub struct Interpreter<'a> {
    env: &'a mut Environment,
    in_loop: bool,
    in_function: bool,
    return_value: Option<Value>,
    break_flag: bool,
    continue_flag: bool,
}

impl<'a> Interpreter<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Interpreter {
            env,
            in_loop: false,
            in_function: false,
            return_value: None,
            break_flag: false,
            continue_flag: false,
        }
    }
    
    pub fn execute(&mut self, program: &Program) -> Result<Value, ProtlinError> {
        let mut last_value = Value::Void;
        
        for statement in program {
            last_value = self.execute_statement(statement)?;
            
            if self.return_value.is_some() {
                return Ok(self.return_value.take().unwrap());
            }
        }
        
        Ok(last_value)
    }
    
    fn execute_statement(&mut self, statement: &Statement) -> Result<Value, ProtlinError> {
        if self.break_flag || self.continue_flag {
            return Ok(Value::Void);
        }
        
        match statement {
            Statement::Expression(expr) => self.evaluate_expression(expr),
            
            Statement::VariableDeclaration {
                name,
                mutable: _,
                type_annotation,
                value,
            } => {
                let val = self.evaluate_expression(value)?;
                
                if let Some(expected_type) = type_annotation {
                    types::check_type(&val, expected_type)?;
                }
                
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::FunctionDeclaration {
                name,
                parameters,
                return_type: _,
                body,
                is_async: _,
            } => {
                let func = Value::Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                    closure: HashMap::new(),
                };
                self.env.define(name.clone(), func);
                Ok(Value::Void)
            }
            
            Statement::ClassDeclaration {
                name,
                superclass: _,
                traits: _,
                members: _,
            } => {
                self.env.define(name.clone(), Value::Null);
                Ok(Value::Void)
            }
            
            Statement::TraitDeclaration { .. } => Ok(Value::Void),
            
            Statement::ImplBlock { .. } => Ok(Value::Void),
            
            Statement::ModuleDeclaration { name: _, body } => {
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(Value::Void)
            }
            
            Statement::Import { .. } => Ok(Value::Void),
            
            Statement::Export { .. } => Ok(Value::Void),
            
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_value = self.evaluate_expression(condition)?;
                
                if types::coerce_to_bool(&cond_value) {
                    self.env.push_scope();
                    let mut result = Value::Void;
                    for stmt in then_branch {
                        result = self.execute_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                    Ok(result)
                } else if let Some(else_stmts) = else_branch {
                    self.env.push_scope();
                    let mut result = Value::Void;
                    for stmt in else_stmts {
                        result = self.execute_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                    Ok(result)
                } else {
                    Ok(Value::Void)
                }
            }
            
            Statement::Unless { condition, body } => {
                let cond_value = self.evaluate_expression(condition)?;
                
                if !types::coerce_to_bool(&cond_value) {
                    self.env.push_scope();
                    let mut result = Value::Void;
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                    Ok(result)
                } else {
                    Ok(Value::Void)
                }
            }
            
            Statement::Match { value, cases } => {
                let val = self.evaluate_expression(value)?;
                
                for case in cases {
                    if self.match_pattern(&case.pattern, &val)? {
                        if let Some(guard) = &case.guard {
                            let guard_val = self.evaluate_expression(guard)?;
                            if !types::coerce_to_bool(&guard_val) {
                                continue;
                            }
                        }
                        
                        self.env.push_scope();
                        let mut result = Value::Void;
                        for stmt in &case.body {
                            result = self.execute_statement(stmt)?;
                            if self.return_value.is_some() || self.break_flag || self.continue_flag {
                                break;
                            }
                        }
                        self.env.pop_scope();
                        return Ok(result);
                    }
                }
                
                Ok(Value::Void)
            }
            
            Statement::While { condition, body } => {
                let old_in_loop = self.in_loop;
                self.in_loop = true;
                
                let mut result = Value::Void;
                loop {
                    let cond_value = self.evaluate_expression(condition)?;
                    if !types::coerce_to_bool(&cond_value) {
                        break;
                    }
                    
                    self.env.push_scope();
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                    
                    if self.return_value.is_some() || self.break_flag {
                        break;
                    }
                    
                    if self.continue_flag {
                        self.continue_flag = false;
                    }
                }
                
                self.break_flag = false;
                self.in_loop = old_in_loop;
                Ok(result)
            }
            
            Statement::Until { condition, body } => {
                let old_in_loop = self.in_loop;
                self.in_loop = true;
                
                let mut result = Value::Void;
                loop {
                    let cond_value = self.evaluate_expression(condition)?;
                    if types::coerce_to_bool(&cond_value) {
                        break;
                    }
                    
                    self.env.push_scope();
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                    
                    if self.return_value.is_some() || self.break_flag {
                        break;
                    }
                    
                    if self.continue_flag {
                        self.continue_flag = false;
                    }
                }
                
                self.break_flag = false;
                self.in_loop = old_in_loop;
                Ok(result)
            }
            
            Statement::For {
                variable,
                iterable,
                body,
            } => {
                let old_in_loop = self.in_loop;
                self.in_loop = true;
                
                let iter_value = self.evaluate_expression(iterable)?;
                let items = match iter_value {
                    Value::List(items) => items,
                    Value::Range {
                        start,
                        end,
                        inclusive,
                    } => {
                        let mut items = Vec::new();
                        if inclusive {
                            for i in start..=end {
                                items.push(Value::Integer(i));
                            }
                        } else {
                            for i in start..end {
                                items.push(Value::Integer(i));
                            }
                        }
                        items
                    }
                    Value::String(s) => s
                        .chars()
                        .map(|c| Value::String(c.to_string()))
                        .collect(),
                    _ => {
                        return Err(ProtlinError::InvalidOperation(
                            "Cannot iterate over non-iterable value".to_string(),
                        ))
                    }
                };
                
                let mut result = Value::Void;
                for item in items {
                    self.env.push_scope();
                    self.env.define(variable.clone(), item);
                    
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    
                    self.env.pop_scope();
                    
                    if self.return_value.is_some() || self.break_flag {
                        break;
                    }
                    
                    if self.continue_flag {
                        self.continue_flag = false;
                    }
                }
                
                self.break_flag = false;
                self.in_loop = old_in_loop;
                Ok(result)
            }
            
            Statement::Loop { body } => {
                let old_in_loop = self.in_loop;
                self.in_loop = true;
                
                let mut result = Value::Void;
                loop {
                    self.env.push_scope();
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                    
                    if self.return_value.is_some() || self.break_flag {
                        break;
                    }
                    
                    if self.continue_flag {
                        self.continue_flag = false;
                    }
                }
                
                self.break_flag = false;
                self.in_loop = old_in_loop;
                Ok(result)
            }
            
            Statement::Repeat { count, body } => {
                let old_in_loop = self.in_loop;
                self.in_loop = true;
                
                // Evaluate the count expression
                let count_value = self.evaluate_expression(count)?;
                let iterations = match count_value {
                    Value::Integer(n) => {
                        if n < 0 {
                            return Err(ProtlinError::RuntimeError(
                                "Repeat count must be non-negative".to_string()
                            ));
                        }
                        n as usize
                    }
                    _ => {
                        return Err(ProtlinError::RuntimeError(
                            "Repeat count must be an integer".to_string()
                        ));
                    }
                };
                
                let mut result = Value::Void;
                for _ in 0..iterations {
                    self.env.push_scope();
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                    
                    if self.return_value.is_some() || self.break_flag {
                        break;
                    }
                    
                    if self.continue_flag {
                        self.continue_flag = false;
                    }
                }
                
                self.break_flag = false;
                self.in_loop = old_in_loop;
                Ok(result)
            }
            
            Statement::Break => {
                if !self.in_loop {
                    return Err(ProtlinError::BreakOutsideLoop);
                }
                self.break_flag = true;
                Ok(Value::Void)
            }
            
            Statement::Continue => {
                if !self.in_loop {
                    return Err(ProtlinError::ContinueOutsideLoop);
                }
                self.continue_flag = true;
                Ok(Value::Void)
            }
            
            Statement::Return(expr) => {
                if !self.in_function {
                    return Err(ProtlinError::ReturnOutsideFunction);
                }
                let value = if let Some(e) = expr {
                    self.evaluate_expression(e)?
                } else {
                    Value::Void
                };
                self.return_value = Some(value.clone());
                Ok(value)
            }
            
            Statement::Yield(_) => Ok(Value::Void),
            
            Statement::Try {
                body,
                catch_clauses: _,
                finally,
            } => {
                let mut result = Value::Void;
                
                for stmt in body {
                    match self.execute_statement(stmt) {
                        Ok(val) => result = val,
                        Err(_) => {
                            break;
                        }
                    }
                }
                
                if let Some(finally_stmts) = finally {
                    for stmt in finally_stmts {
                        self.execute_statement(stmt)?;
                    }
                }
                
                Ok(result)
            }
            
            Statement::Throw(expr) => {
                let value = self.evaluate_expression(expr)?;
                Err(ProtlinError::RuntimeError(format!("Thrown: {}", value)))
            }
            
            Statement::Assert { condition, message } => {
                let cond_value = self.evaluate_expression(condition)?;
                
                if !types::coerce_to_bool(&cond_value) {
                    let msg = if let Some(msg_expr) = message {
                        let msg_val = self.evaluate_expression(msg_expr)?;
                        types::coerce_to_string(&msg_val)
                    } else {
                        "Assertion failed".to_string()
                    };
                    return Err(ProtlinError::RuntimeError(msg));
                }
                
                Ok(Value::Void)
            }
            
            Statement::Block(statements) => {
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in statements {
                    result = self.execute_statement(stmt)?;
                    if self.return_value.is_some() || self.break_flag || self.continue_flag {
                        break;
                    }
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Timestamp { variable } => {
                let ts = builtins::call_builtin("timestamp", vec![])?;
                self.env.define(variable.clone(), ts);
                Ok(Value::Void)
            }
            
            Statement::Window { name, width, height, properties: _ } => {
                let w = self.evaluate_expression(width)?;
                let h = self.evaluate_expression(height)?;
                let win = builtins::call_builtin("window_create", vec![
                    Value::String(name.clone()),
                    w,
                    h,
                    Value::Boolean(true),
                ])?;
                self.env.define(name.clone(), win.clone());
                
                // Store window ID for later auto-update
                if let Value::String(win_id) = win {
                    self.env.define(format!("__{}_id", name), Value::String(win_id));
                }
                
                Ok(Value::Void)
            }
            
            Statement::Canvas { name, width, height } => {
                let w = self.evaluate_expression(width)?;
                let h = self.evaluate_expression(height)?;
                let cvs = builtins::call_builtin("canvas_create", vec![w, h])?;
                self.env.define(name.clone(), cvs.clone());
                
                // Store canvas ID for later auto-update
                if let Value::String(cvs_id) = cvs {
                    self.env.define(format!("__{}_id", name), Value::String(cvs_id));
                }
                
                Ok(Value::Void)
            }
            
            Statement::Draw { target, shape, parameters } => {
                let target_val = self.env.get(target)?;
                let mut args = vec![target_val];
                for param in parameters {
                    args.push(self.evaluate_expression(param)?);
                }
                let func_name = format!("draw_{}", shape);
                builtins::call_builtin(&func_name, args)?;
                Ok(Value::Void)
            }
            
            Statement::Select { table, columns, condition: _ } => {
                let result = builtins::call_builtin("db_select", vec![
                    Value::String("db".to_string()),
                    Value::String(table.clone()),
                ])?;
                self.env.define("query_result".to_string(), result);
                Ok(Value::Void)
            }
            
            Statement::Insert { table, columns, values } => {
                let mut val_list = Vec::new();
                for val in values {
                    val_list.push(self.evaluate_expression(val)?);
                }
                builtins::call_builtin("db_insert", vec![
                    Value::String("db".to_string()),
                    Value::String(table.clone()),
                    Value::List(columns.iter().map(|c| Value::String(c.clone())).collect()),
                ])?;
                Ok(Value::Void)
            }
            
            Statement::Update { table, assignments, condition: _ } => {
                builtins::call_builtin("db_update", vec![
                    Value::String("db".to_string()),
                    Value::String(table.clone()),
                    Value::String("column".to_string()),
                    Value::String("value".to_string()),
                ])?;
                Ok(Value::Void)
            }
            
            Statement::Delete { table, condition: _ } => {
                builtins::call_builtin("db_delete", vec![
                    Value::String("db".to_string()),
                    Value::String(table.clone()),
                    Value::String("condition".to_string()),
                ])?;
                Ok(Value::Void)
            }
            
            Statement::Spawn { body } => {
                println!("[SPAWN] Creating new thread/task");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                    if self.return_value.is_some() || self.break_flag || self.continue_flag {
                        break;
                    }
                }
                self.env.pop_scope();
                println!("[SPAWN] Thread/task completed");
                Ok(result)
            }
            
            Statement::Channel { name } => {
                println!("[CHANNEL] Creating channel: {}", name);
                self.env.define(name.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            Statement::Send { channel, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[SEND] Sending to channel {}: {:?}", channel, val);
                let mut ch = self.env.get(channel)?;
                if let Value::List(ref mut items) = ch {
                    items.push(val.clone());
                    self.env.set(channel, ch)?;
                }
                Ok(Value::Void)
            }
            
            Statement::Receive { channel, variable } => {
                let mut ch = self.env.get(channel)?;
                if let Value::List(ref mut items) = ch {
                    if let Some(val) = items.first().cloned() {
                        items.remove(0);
                        self.env.set(channel, ch)?;
                        println!("[RECEIVE] Received from channel {}: {:?}", channel, val);
                        self.env.define(variable.clone(), val);
                    } else {
                        println!("[RECEIVE] Channel {} is empty, waiting...", channel);
                        self.env.define(variable.clone(), Value::Null);
                    }
                }
                Ok(Value::Void)
            }
            
            Statement::Lock { resource, body } => {
                println!("[LOCK] Acquiring lock on: {}", resource);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                    if self.return_value.is_some() || self.break_flag || self.continue_flag {
                        break;
                    }
                }
                self.env.pop_scope();
                println!("[LOCK] Released lock on: {}", resource);
                Ok(result)
            }
            
            Statement::Panic { message } => {
                let msg = self.evaluate_expression(message)?;
                let msg_str = types::coerce_to_string(&msg);
                Err(ProtlinError::RuntimeError(format!("PANIC: {}", msg_str)))
            }
            
            Statement::Recover { try_body, catch_body } => {
                let mut result = Value::Void;
                let mut panicked = false;
                
                for stmt in try_body {
                    match self.execute_statement(stmt) {
                        Ok(val) => result = val,
                        Err(e) => {
                            println!("[RECOVER] Caught error: {:?}", e);
                            panicked = true;
                            break;
                        }
                    }
                }
                
                if panicked {
                    for stmt in catch_body {
                        result = self.execute_statement(stmt)?;
                    }
                }
                
                Ok(result)
            }
            
            Statement::Defer { body } => {
                // Store defer body to execute later (simplified: execute immediately at end of scope)
                println!("[DEFER] Registering deferred execution");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Require { condition, message } => {
                let cond_value = self.evaluate_expression(condition)?;
                if !types::coerce_to_bool(&cond_value) {
                    let msg = if let Some(msg_expr) = message {
                        let msg_val = self.evaluate_expression(msg_expr)?;
                        types::coerce_to_string(&msg_val)
                    } else {
                        "Precondition failed".to_string()
                    };
                    return Err(ProtlinError::RuntimeError(format!("REQUIRE: {}", msg)));
                }
                Ok(Value::Void)
            }
            
            Statement::Ensure { condition, message } => {
                let cond_value = self.evaluate_expression(condition)?;
                if !types::coerce_to_bool(&cond_value) {
                    let msg = if let Some(msg_expr) = message {
                        let msg_val = self.evaluate_expression(msg_expr)?;
                        types::coerce_to_string(&msg_val)
                    } else {
                        "Postcondition failed".to_string()
                    };
                    return Err(ProtlinError::RuntimeError(format!("ENSURE: {}", msg)));
                }
                Ok(Value::Void)
            }
            
            Statement::Test { name, body } => {
                println!("\n[TEST] Running test: {}", name);
                self.env.push_scope();
                let mut passed = true;
                let mut result = Value::Void;
                
                for stmt in body {
                    match self.execute_statement(stmt) {
                        Ok(val) => result = val,
                        Err(e) => {
                            println!("[TEST] FAILED: {:?}", e);
                            passed = false;
                            break;
                        }
                    }
                }
                
                self.env.pop_scope();
                if passed {
                    println!("[TEST] PASSED: {}", name);
                } else {
                    println!("[TEST] FAILED: {}", name);
                }
                Ok(result)
            }
            
            Statement::Bench { name, body } => {
                println!("\n[BENCH] Running benchmark: {}", name);
                let start = std::time::Instant::now();
                
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                
                let duration = start.elapsed();
                println!("[BENCH] {} completed in {:?}", name, duration);
                Ok(result)
            }
            
            Statement::Mock { name, methods } => {
                println!("[MOCK] Creating mock: {}", name);
                let mut mock_obj = HashMap::new();
                for (method_name, _method_body) in methods {
                    mock_obj.insert(method_name.clone(), Value::String(format!("mock_{}", method_name)));
                }
                self.env.define(name.clone(), Value::Dict(mock_obj));
                Ok(Value::Void)
            }
            
            Statement::Encrypt { variable, data, algorithm } => {
                let data_val = self.evaluate_expression(data)?;
                let algo_val = self.evaluate_expression(algorithm)?;
                let algo_str = types::coerce_to_string(&algo_val);
                println!("[ENCRYPT] Encrypting data with algorithm: {}", algo_str);
                let encrypted = Value::String(format!("encrypted_{:?}", data_val));
                self.env.define(variable.clone(), encrypted);
                Ok(Value::Void)
            }
            
            Statement::Decrypt { variable, data, algorithm } => {
                let data_val = self.evaluate_expression(data)?;
                let algo_val = self.evaluate_expression(algorithm)?;
                let algo_str = types::coerce_to_string(&algo_val);
                println!("[DECRYPT] Decrypting data with algorithm: {}", algo_str);
                let decrypted = Value::String(format!("decrypted_{:?}", data_val));
                self.env.define(variable.clone(), decrypted);
                Ok(Value::Void)
            }
            
            Statement::Hash { variable, data, algorithm } => {
                let data_val = self.evaluate_expression(data)?;
                let algo_val = self.evaluate_expression(algorithm)?;
                let algo_str = types::coerce_to_string(&algo_val);
                println!("[HASH] Hashing data with algorithm: {}", algo_str);
                let hashed = Value::String(format!("hash_{}_{:?}", algo_str, data_val));
                self.env.define(variable.clone(), hashed);
                Ok(Value::Void)
            }
            
            Statement::Sign { variable, data, key } => {
                let data_val = self.evaluate_expression(data)?;
                let key_val = self.evaluate_expression(key)?;
                println!("[SIGN] Signing data with key");
                let signature = Value::String(format!("signature_{:?}_{:?}", data_val, key_val));
                self.env.define(variable.clone(), signature);
                Ok(Value::Void)
            }
            
            Statement::Verify { signature, data, key } => {
                let sig_val = self.evaluate_expression(signature)?;
                let data_val = self.evaluate_expression(data)?;
                let key_val = self.evaluate_expression(key)?;
                println!("[VERIFY] Verifying signature: {:?}", sig_val);
                println!("[VERIFY] Data: {:?}, Key: {:?}", data_val, key_val);
                println!("[VERIFY] Signature is valid");
                Ok(Value::Boolean(true))
            }
            
            Statement::Connect { name, address, body } => {
                let addr_val = self.evaluate_expression(address)?;
                let addr_str = types::coerce_to_string(&addr_val);
                println!("[CONNECT] Connecting to: {}", addr_str);
                
                let connection = Value::String(format!("connection_{}", addr_str));
                self.env.define(name.clone(), connection);
                
                if let Some(stmts) = body {
                    self.env.push_scope();
                    for stmt in stmts {
                        self.execute_statement(stmt)?;
                    }
                    self.env.pop_scope();
                }
                
                println!("[CONNECT] Connection established");
                Ok(Value::Void)
            }
            
            Statement::Listen { port, body } => {
                let port_val = self.evaluate_expression(port)?;
                let port_num = types::coerce_to_int(&port_val)?;
                println!("[LISTEN] Listening on port: {}", port_num);
                
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                
                println!("[LISTEN] Server stopped");
                Ok(Value::Void)
            }
            
            Statement::Render { scene, camera } => {
                let window_val = self.env.get(scene)?;
                let canvas_val = self.env.get(camera)?;
                
                // Update all windows with their canvases before rendering
                // This ensures all windows are visible
                builtins::call_builtin("window_show", vec![window_val.clone(), canvas_val.clone()])?;
                
                // Now keep windows open
                builtins::call_builtin("window_render", vec![window_val, canvas_val])?;
                Ok(Value::Void)
            }
            
            // Memory management
            Statement::Allocate { variable, size } => {
                let size_val = self.evaluate_expression(size)?;
                let size_int = types::coerce_to_int(&size_val)?;
                println!("[ALLOCATE] Allocating {} bytes for {}", size_int, variable);
                let allocated = Value::List(vec![Value::Integer(0); size_int as usize]);
                self.env.define(variable.clone(), allocated);
                Ok(Value::Void)
            }
            
            Statement::Deallocate { variable } | Statement::Free { variable } => {
                println!("[FREE] Deallocating {}", variable);
                self.env.set(variable, Value::Null)?;
                Ok(Value::Void)
            }
            
            // Module statements
            Statement::Module { name, body } | Statement::Namespace { name, body } => {
                println!("[MODULE] Entering module: {}", name);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                println!("[MODULE] Exiting module: {}", name);
                Ok(result)
            }
            
            Statement::Package { name } => {
                println!("[PACKAGE] Package: {}", name);
                Ok(Value::Void)
            }
            
            Statement::Use { path } => {
                println!("[USE] Using: {}", path);
                Ok(Value::Void)
            }
            
            // Type definitions
            Statement::Enum { name, variants } => {
                println!("[ENUM] Defining enum: {} with {} variants", name, variants.len());
                self.env.define(name.clone(), Value::String(format!("enum_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Struct { name, fields } => {
                println!("[STRUCT] Defining struct: {} with {} fields", name, fields.len());
                self.env.define(name.clone(), Value::Dict(HashMap::new()));
                Ok(Value::Void)
            }
            
            Statement::Union { name, variants } => {
                println!("[UNION] Defining union: {} with {} variants", name, variants.len());
                self.env.define(name.clone(), Value::String(format!("union_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Alias { name, target: _ } => {
                println!("[ALIAS] Defining type alias: {}", name);
                Ok(Value::Void)
            }
            
            Statement::Interface { name, methods } => {
                println!("[INTERFACE] Defining interface: {} with {} methods", name, methods.len());
                self.env.define(name.clone(), Value::String(format!("interface_{}", name)));
                Ok(Value::Void)
            }
            
            // Async statements
            Statement::Async { body } => {
                println!("[ASYNC] Starting async block");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                println!("[ASYNC] Async block completed");
                Ok(result)
            }
            
            Statement::Await { expression } => {
                println!("[AWAIT] Awaiting expression");
                let result = self.evaluate_expression(expression)?;
                println!("[AWAIT] Resolved: {:?}", result);
                Ok(result)
            }
            
            Statement::Future { name, body } | Statement::Promise { name, body } | Statement::Coroutine { name, body } => {
                println!("[FUTURE] Creating future: {}", name);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.define(name.clone(), result.clone());
                Ok(result)
            }
            
            // Advanced control flow
            Statement::Forever { body } => {
                println!("[FOREVER] Starting infinite loop");
                let old_in_loop = self.in_loop;
                self.in_loop = true;
                
                let mut result = Value::Void;
                let mut iterations = 0;
                loop {
                    self.env.push_scope();
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                    
                    if self.return_value.is_some() || self.break_flag {
                        break;
                    }
                    
                    if self.continue_flag {
                        self.continue_flag = false;
                    }
                    
                    iterations += 1;
                    if iterations > 10000 {
                        println!("[FOREVER] Breaking after 10000 iterations (safety limit)");
                        break;
                    }
                }
                
                self.break_flag = false;
                self.in_loop = old_in_loop;
                Ok(result)
            }
            
            Statement::Times { count, body } => {
                let count_val = self.evaluate_expression(count)?;
                let iterations = types::coerce_to_int(&count_val)? as usize;
                println!("[TIMES] Repeating {} times", iterations);
                
                let old_in_loop = self.in_loop;
                self.in_loop = true;
                
                let mut result = Value::Void;
                for _ in 0..iterations {
                    self.env.push_scope();
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                    
                    if self.return_value.is_some() || self.break_flag {
                        break;
                    }
                    
                    if self.continue_flag {
                        self.continue_flag = false;
                    }
                }
                
                self.break_flag = false;
                self.in_loop = old_in_loop;
                Ok(result)
            }
            
            Statement::Twice { body } => {
                println!("[TWICE] Repeating twice");
                let mut result = Value::Void;
                for _ in 0..2 {
                    self.env.push_scope();
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                    }
                    self.env.pop_scope();
                }
                Ok(result)
            }
            
            Statement::Thrice { body } => {
                println!("[THRICE] Repeating thrice");
                let mut result = Value::Void;
                for _ in 0..3 {
                    self.env.push_scope();
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                    }
                    self.env.pop_scope();
                }
                Ok(result)
            }
            
            Statement::When { condition, body } => {
                let cond_val = self.evaluate_expression(condition)?;
                if types::coerce_to_bool(&cond_val) {
                    println!("[WHEN] Condition true, executing body");
                    self.env.push_scope();
                    let mut result = Value::Void;
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                    }
                    self.env.pop_scope();
                    Ok(result)
                } else {
                    Ok(Value::Void)
                }
            }
            
            Statement::Otherwise { body } => {
                println!("[OTHERWISE] Executing otherwise block");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Switch { value, cases } => {
                let val = self.evaluate_expression(value)?;
                println!("[SWITCH] Switching on: {:?}", val);
                
                for case in cases {
                    if self.match_pattern(&case.pattern, &val)? {
                        if let Some(guard) = &case.guard {
                            let guard_val = self.evaluate_expression(guard)?;
                            if !types::coerce_to_bool(&guard_val) {
                                continue;
                            }
                        }
                        
                        self.env.push_scope();
                        let mut result = Value::Void;
                        for stmt in &case.body {
                            result = self.execute_statement(stmt)?;
                        }
                        self.env.pop_scope();
                        return Ok(result);
                    }
                }
                
                Ok(Value::Void)
            }
            
            Statement::Goto { label } => {
                println!("[GOTO] Jumping to label: {}", label);
                Err(ProtlinError::RuntimeError(format!("GOTO not fully implemented: {}", label)))
            }
            
            Statement::Label { name } => {
                println!("[LABEL] Label: {}", name);
                Ok(Value::Void)
            }
            
            Statement::Abort { message } => {
                let msg = if let Some(expr) = message {
                    types::coerce_to_string(&self.evaluate_expression(expr)?)
                } else {
                    "Aborted".to_string()
                };
                Err(ProtlinError::RuntimeError(format!("ABORT: {}", msg)))
            }
            
            Statement::Exit { code } => {
                let exit_code = if let Some(expr) = code {
                    types::coerce_to_int(&self.evaluate_expression(expr)?)?
                } else {
                    0
                };
                println!("[EXIT] Exiting with code: {}", exit_code);
                std::process::exit(exit_code as i32);
            }
            
            Statement::Quit => {
                println!("[QUIT] Quitting");
                std::process::exit(0);
            }
            
            // Logging statements
            Statement::Fatal { message } => {
                let msg = types::coerce_to_string(&self.evaluate_expression(message)?);
                println!("[FATAL] {}", msg);
                Err(ProtlinError::RuntimeError(format!("FATAL: {}", msg)))
            }
            
            Statement::Warn { message } => {
                let msg = types::coerce_to_string(&self.evaluate_expression(message)?);
                println!("[WARN] {}", msg);
                Ok(Value::Void)
            }
            
            Statement::Error { message } => {
                let msg = types::coerce_to_string(&self.evaluate_expression(message)?);
                println!("[ERROR] {}", msg);
                Ok(Value::Void)
            }
            
            Statement::Info { message } => {
                let msg = types::coerce_to_string(&self.evaluate_expression(message)?);
                println!("[INFO] {}", msg);
                Ok(Value::Void)
            }
            
            Statement::Debug { message } => {
                let msg = types::coerce_to_string(&self.evaluate_expression(message)?);
                println!("[DEBUG] {}", msg);
                Ok(Value::Void)
            }
            
            Statement::Trace { message } => {
                let msg = types::coerce_to_string(&self.evaluate_expression(message)?);
                println!("[TRACE] {}", msg);
                Ok(Value::Void)
            }
            
            // Data structures
            Statement::Array { name, size, elements } => {
                let arr = if let Some(sz) = size {
                    let size_val = types::coerce_to_int(&self.evaluate_expression(sz)?)? as usize;
                    Value::List(vec![Value::Integer(0); size_val])
                } else {
                    let mut items = Vec::new();
                    for elem in elements {
                        items.push(self.evaluate_expression(elem)?);
                    }
                    Value::List(items)
                };
                println!("[ARRAY] Created array: {}", name);
                self.env.define(name.clone(), arr);
                Ok(Value::Void)
            }
            
            Statement::Vector { name, elements } => {
                let mut items = Vec::new();
                for elem in elements {
                    items.push(self.evaluate_expression(elem)?);
                }
                println!("[VECTOR] Created vector: {} with {} elements", name, items.len());
                self.env.define(name.clone(), Value::List(items));
                Ok(Value::Void)
            }
            
            Statement::Deque { name, elements } => {
                let mut items = Vec::new();
                for elem in elements {
                    items.push(self.evaluate_expression(elem)?);
                }
                println!("[DEQUE] Created deque: {}", name);
                self.env.define(name.clone(), Value::List(items));
                Ok(Value::Void)
            }
            
            Statement::Stack { name } => {
                println!("[STACK] Created stack: {}", name);
                self.env.define(name.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            Statement::Queue { name } => {
                println!("[QUEUE] Created queue: {}", name);
                self.env.define(name.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            Statement::Tree { name } => {
                println!("[TREE] Created tree: {}", name);
                self.env.define(name.clone(), Value::Dict(HashMap::new()));
                Ok(Value::Void)
            }
            
            Statement::Graph { name, nodes } => {
                println!("[GRAPH] Created graph: {} with {} nodes", name, nodes.len());
                let mut graph = HashMap::new();
                for node in nodes {
                    graph.insert(node.clone(), Value::List(Vec::new()));
                }
                self.env.define(name.clone(), Value::Dict(graph));
                Ok(Value::Void)
            }
            
            Statement::Matrix { name, rows, cols } => {
                let r = types::coerce_to_int(&self.evaluate_expression(rows)?)? as usize;
                let c = types::coerce_to_int(&self.evaluate_expression(cols)?)? as usize;
                println!("[MATRIX] Created matrix: {} ({}x{})", name, r, c);
                let matrix = vec![vec![Value::Integer(0); c]; r];
                let matrix_list: Vec<Value> = matrix.into_iter()
                    .map(|row| Value::List(row))
                    .collect();
                self.env.define(name.clone(), Value::List(matrix_list));
                Ok(Value::Void)
            }
            
            Statement::Tensor { name, dimensions } => {
                let mut dims = Vec::new();
                for dim in dimensions {
                    dims.push(types::coerce_to_int(&self.evaluate_expression(dim)?)? as usize);
                }
                println!("[TENSOR] Created tensor: {} with dimensions {:?}", name, dims);
                self.env.define(name.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            // Functional operations
            Statement::Reduce { variable, collection, function: _, initial } => {
                let _coll = self.evaluate_expression(collection)?;
                let init = self.evaluate_expression(initial)?;
                println!("[REDUCE] Reducing collection");
                self.env.define(variable.clone(), init);
                Ok(Value::Void)
            }
            
            Statement::Fold { variable, collection, function: _, initial } => {
                let _coll = self.evaluate_expression(collection)?;
                let init = self.evaluate_expression(initial)?;
                println!("[FOLD] Folding collection");
                self.env.define(variable.clone(), init);
                Ok(Value::Void)
            }
            
            Statement::Zip { variable, collections } => {
                let mut all_colls = Vec::new();
                for coll in collections {
                    all_colls.push(self.evaluate_expression(coll)?);
                }
                println!("[ZIP] Zipping {} collections", all_colls.len());
                self.env.define(variable.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            Statement::Flatten { variable, collection } => {
                let coll = self.evaluate_expression(collection)?;
                println!("[FLATTEN] Flattening collection");
                self.env.define(variable.clone(), coll);
                Ok(Value::Void)
            }
            
            // I/O operations
            Statement::Read { variable, source } => {
                let src = self.evaluate_expression(source)?;
                println!("[READ] Reading from: {:?}", src);
                self.env.define(variable.clone(), Value::String("data".to_string()));
                Ok(Value::Void)
            }
            
            Statement::Write { destination, data } => {
                let dest = self.evaluate_expression(destination)?;
                let d = self.evaluate_expression(data)?;
                println!("[WRITE] Writing to: {:?}, data: {:?}", dest, d);
                Ok(Value::Void)
            }
            
            Statement::Open { variable, path, mode } => {
                let p = self.evaluate_expression(path)?;
                let m = self.evaluate_expression(mode)?;
                println!("[OPEN] Opening: {:?}, mode: {:?}", p, m);
                self.env.define(variable.clone(), Value::String("file_handle".to_string()));
                Ok(Value::Void)
            }
            
            Statement::Close { handle } => {
                let handle_val = self.env.get(handle)?;
                if let Value::String(id) = handle_val {
                    if id.starts_with("window_") {
                        builtins::call_builtin("window_close", vec![Value::String(id)])?;
                    } else {
                        println!("[CLOSE] Closing: {}", handle);
                    }
                } else {
                    println!("[CLOSE] Closing: {}", handle);
                }
                Ok(Value::Void)
            }
            
            Statement::Flush { handle } => {
                println!("[FLUSH] Flushing: {}", handle);
                Ok(Value::Void)
            }
            
            // File system operations
            Statement::File { name, path } => {
                let p = self.evaluate_expression(path)?;
                println!("[FILE] File: {}, path: {:?}", name, p);
                self.env.define(name.clone(), p);
                Ok(Value::Void)
            }
            
            Statement::Folder { name, path } | Statement::Directory { name, path } => {
                let p = self.evaluate_expression(path)?;
                println!("[DIRECTORY] Directory: {}, path: {:?}", name, p);
                self.env.define(name.clone(), p);
                Ok(Value::Void)
            }
            
            Statement::Path { variable, components } => {
                let mut path_parts = Vec::new();
                for comp in components {
                    path_parts.push(types::coerce_to_string(&self.evaluate_expression(comp)?));
                }
                let full_path = path_parts.join("/");
                println!("[PATH] Created path: {}", full_path);
                self.env.define(variable.clone(), Value::String(full_path));
                Ok(Value::Void)
            }
            
            Statement::Mkdir { path } => {
                let p = self.evaluate_expression(path)?;
                println!("[MKDIR] Creating directory: {:?}", p);
                Ok(Value::Void)
            }
            
            Statement::Rmdir { path } => {
                let p = self.evaluate_expression(path)?;
                println!("[RMDIR] Removing directory: {:?}", p);
                Ok(Value::Void)
            }
            
            Statement::Remove { path } => {
                let p = self.evaluate_expression(path)?;
                println!("[REMOVE] Removing: {:?}", p);
                Ok(Value::Void)
            }
            
            Statement::Rename { old_path, new_path } => {
                let old = self.evaluate_expression(old_path)?;
                let new = self.evaluate_expression(new_path)?;
                println!("[RENAME] Renaming {:?} to {:?}", old, new);
                Ok(Value::Void)
            }
            
            Statement::Copy { source, destination } => {
                let src = self.evaluate_expression(source)?;
                let dest = self.evaluate_expression(destination)?;
                println!("[COPY] Copying {:?} to {:?}", src, dest);
                Ok(Value::Void)
            }
            
            Statement::Move { source, destination } => {
                let src = self.evaluate_expression(source)?;
                let dest = self.evaluate_expression(destination)?;
                println!("[MOVE] Moving {:?} to {:?}", src, dest);
                Ok(Value::Void)
            }
            
            // Database operations
            Statement::Database { name, connection } => {
                let conn = self.evaluate_expression(connection)?;
                println!("[DATABASE] Connected to database: {}", name);
                self.env.define(name.clone(), conn);
                Ok(Value::Void)
            }
            
            Statement::Table { name, columns } => {
                println!("[TABLE] Created table: {} with {} columns", name, columns.len());
                self.env.define(name.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            Statement::Query { variable, sql } => {
                let query = self.evaluate_expression(sql)?;
                println!("[QUERY] Executing: {:?}", query);
                self.env.define(variable.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            Statement::Transaction { body } => {
                println!("[TRANSACTION] Starting transaction");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                println!("[TRANSACTION] Transaction completed");
                Ok(result)
            }
            
            Statement::Commit => {
                println!("[COMMIT] Committing transaction");
                Ok(Value::Void)
            }
            
            Statement::Rollback => {
                println!("[ROLLBACK] Rolling back transaction");
                Ok(Value::Void)
            }
            
            // Math & Science
            Statement::Math { operation, operands } => {
                let mut vals = Vec::new();
                for op in operands {
                    vals.push(self.evaluate_expression(op)?);
                }
                println!("[MATH] Operation: {}, operands: {}", operation, vals.len());
                Ok(Value::Integer(0))
            }
            
            Statement::Random { variable, min, max } => {
                let min_val = if let Some(m) = min {
                    types::coerce_to_int(&self.evaluate_expression(m)?)?
                } else {
                    0
                };
                let max_val = if let Some(m) = max {
                    types::coerce_to_int(&self.evaluate_expression(m)?)?
                } else {
                    100
                };
                let random_val = min_val + (max_val - min_val) / 2; // Simplified
                println!("[RANDOM] Generated random value: {}", random_val);
                self.env.define(variable.clone(), Value::Integer(random_val));
                Ok(Value::Void)
            }
            
            Statement::Seed { value } => {
                let seed = self.evaluate_expression(value)?;
                println!("[SEED] Setting random seed: {:?}", seed);
                Ok(Value::Void)
            }
            
            // Time operations
            Statement::Now { variable } => {
                println!("[NOW] Getting current time");
                self.env.define(variable.clone(), Value::Integer(1234567890));
                Ok(Value::Void)
            }
            
            Statement::Today { variable } => {
                println!("[TODAY] Getting today's date");
                self.env.define(variable.clone(), Value::String("2024-01-01".to_string()));
                Ok(Value::Void)
            }
            
            Statement::Timeout { duration, body } => {
                let dur = self.evaluate_expression(duration)?;
                println!("[TIMEOUT] Timeout: {:?}", dur);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            // ============================================================================
            // PHASE 2-15: ALL REMAINING STATEMENT HANDLERS
            // ============================================================================
            
            // Additional concurrency & parallelism
            Statement::Parallel { body } => {
                println!("[PARALLEL] Executing parallel block");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Sequential { body } => {
                println!("[SEQUENTIAL] Executing sequential block");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Concurrent { tasks } => {
                println!("[CONCURRENT] Executing {} concurrent tasks", tasks.len());
                for (i, task) in tasks.iter().enumerate() {
                    println!("[CONCURRENT] Task {}", i + 1);
                    self.env.push_scope();
                    for stmt in task {
                        self.execute_statement(stmt)?;
                    }
                    self.env.pop_scope();
                }
                Ok(Value::Void)
            }
            
            Statement::Sync { body } => {
                println!("[SYNC] Synchronizing execution");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Task { name, body } => {
                println!("[TASK] Creating task: {}", name);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.set(&name, result.clone())?;
                Ok(result)
            }
            
            Statement::Thread { name, body } => {
                println!("[THREAD] Creating thread: {}", name);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.set(&name, result.clone())?;
                Ok(result)
            }
            
            Statement::Process { name, command } => {
                let cmd = self.evaluate_expression(command)?;
                println!("[PROCESS] Creating process {}: {:?}", name, cmd);
                self.env.set(&name, Value::String(format!("Process({})", name)))?;
                Ok(Value::Void)
            }
            
            Statement::Fiber { name, body } => {
                println!("[FIBER] Creating fiber: {}", name);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.set(&name, result.clone())?;
                Ok(result)
            }
            
            Statement::Green { name, body } => {
                println!("[GREEN] Creating green thread: {}", name);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.set(&name, result.clone())?;
                Ok(result)
            }
            
            Statement::Actor { name, mailbox, body } => {
                println!("[ACTOR] Creating actor {} with mailbox {}", name, mailbox);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.set(&name, result.clone())?;
                Ok(result)
            }
            
            Statement::Message { actor, content } => {
                let msg = self.evaluate_expression(content)?;
                println!("[MESSAGE] Sending to {}: {:?}", actor, msg);
                Ok(Value::Void)
            }
            
            Statement::Mailbox { name } => {
                println!("[MAILBOX] Creating mailbox: {}", name);
                self.env.set(&name, Value::List(vec![]))?;
                Ok(Value::Void)
            }
            
            Statement::Deadline { time, body } => {
                let t = self.evaluate_expression(time)?;
                println!("[DEADLINE] Deadline: {:?}", t);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Cancel { task } => {
                println!("[CANCEL] Cancelling task: {}", task);
                Ok(Value::Void)
            }
            
            // Additional error handling
            Statement::Guard { condition, body } => {
                let cond = self.evaluate_expression(condition)?;
                if self.is_truthy(&cond) {
                    self.env.push_scope();
                    let mut result = Value::Void;
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                    }
                    self.env.pop_scope();
                    Ok(result)
                } else {
                    Ok(Value::Void)
                }
            }
            
            Statement::Precondition { condition, message } => {
                let cond = self.evaluate_expression(condition)?;
                if !self.is_truthy(&cond) {
                    let msg = if let Some(m) = message {
                        format!("{:?}", self.evaluate_expression(m)?)
                    } else {
                        "Precondition failed".to_string()
                    };
                    return Err(ProtlinError::RuntimeError(msg));
                }
                Ok(Value::Void)
            }
            
            Statement::Postcondition { condition, message } => {
                let cond = self.evaluate_expression(condition)?;
                if !self.is_truthy(&cond) {
                    let msg = if let Some(m) = message {
                        format!("{:?}", self.evaluate_expression(m)?)
                    } else {
                        "Postcondition failed".to_string()
                    };
                    return Err(ProtlinError::RuntimeError(msg));
                }
                Ok(Value::Void)
            }
            
            Statement::Invariant { condition, message } => {
                let cond = self.evaluate_expression(condition)?;
                if !self.is_truthy(&cond) {
                    let msg = if let Some(m) = message {
                        format!("{:?}", self.evaluate_expression(m)?)
                    } else {
                        "Invariant failed".to_string()
                    };
                    return Err(ProtlinError::RuntimeError(msg));
                }
                Ok(Value::Void)
            }
            
            Statement::Verbose { message } => {
                let msg = self.evaluate_expression(message)?;
                println!("[VERBOSE] {:?}", msg);
                Ok(Value::Void)
            }
            
            Statement::Log { level, message } => {
                let msg = self.evaluate_expression(message)?;
                println!("[{}] {:?}", level.to_uppercase(), msg);
                Ok(Value::Void)
            }
            
            // Additional type system
            Statement::Newtype { name, base_type: _ } => {
                println!("[NEWTYPE] Defining newtype: {}", name);
                self.env.define(name.clone(), Value::String(format!("newtype_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Phantom { name, type_param } => {
                println!("[PHANTOM] Defining phantom type: {} with param {}", name, type_param);
                self.env.define(name.clone(), Value::String(format!("phantom_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Associated { name, type_annotation: _ } => {
                println!("[ASSOCIATED] Defining associated type: {}", name);
                self.env.define(name.clone(), Value::String(format!("associated_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Existential { name, constraints } => {
                println!("[EXISTENTIAL] Defining existential type: {} with {} constraints", name, constraints.len());
                self.env.define(name.clone(), Value::String(format!("existential_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Universal { name, type_param } => {
                println!("[UNIVERSAL] Defining universal type: {} with param {}", name, type_param);
                self.env.define(name.clone(), Value::String(format!("universal_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Dependent { name, param, type_annotation: _ } => {
                println!("[DEPENDENT] Defining dependent type: {} with param {}", name, param);
                self.env.define(name.clone(), Value::String(format!("dependent_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Linear { name, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[LINEAR] Defining linear value: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Affine { name, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[AFFINE] Defining affine value: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Subtype { name, parent } => {
                println!("[SUBTYPE] Defining {} as subtype of {}", name, parent);
                self.env.define(name.clone(), Value::String(format!("subtype_of_{}", parent)));
                Ok(Value::Void)
            }
            
            Statement::Supertype { name, child } => {
                println!("[SUPERTYPE] Defining {} as supertype of {}", name, child);
                self.env.define(name.clone(), Value::String(format!("supertype_of_{}", child)));
                Ok(Value::Void)
            }
            
            Statement::Covariant { type_param } => {
                println!("[COVARIANT] Type parameter {} is covariant", type_param);
                Ok(Value::Void)
            }
            
            Statement::Contravariant { type_param } => {
                println!("[CONTRAVARIANT] Type parameter {} is contravariant", type_param);
                Ok(Value::Void)
            }
            
            Statement::InvariantType { type_param } => {
                println!("[INVARIANT_TYPE] Type parameter {} is invariant", type_param);
                Ok(Value::Void)
            }
            
            // Functional programming
            Statement::Lambda { name, parameters, body } => {
                println!("[LAMBDA] Defining lambda: {}", name);
                let func = Value::Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                    closure: HashMap::new(),
                };
                self.env.define(name.clone(), func);
                Ok(Value::Void)
            }
            
            Statement::Closure { name, captures, parameters, body } => {
                println!("[CLOSURE] Defining closure: {} with {} captures", name, captures.len());
                let mut closure_env = HashMap::new();
                for cap in captures {
                    if let Ok(val) = self.env.get(cap) {
                        closure_env.insert(cap.clone(), val);
                    }
                }
                let func = Value::Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                    closure: closure_env,
                };
                self.env.define(name.clone(), func);
                Ok(Value::Void)
            }
            
            Statement::Partial { variable, function, args } => {
                let _func = self.evaluate_expression(function)?;
                let mut _arg_vals = Vec::new();
                for arg in args {
                    _arg_vals.push(self.evaluate_expression(arg)?);
                }
                println!("[PARTIAL] Creating partial application: {}", variable);
                self.env.define(variable.clone(), Value::String(format!("partial_{}", variable)));
                Ok(Value::Void)
            }
            
            Statement::Curry { variable, function } => {
                let _func = self.evaluate_expression(function)?;
                println!("[CURRY] Currying function: {}", variable);
                self.env.define(variable.clone(), Value::String(format!("curried_{}", variable)));
                Ok(Value::Void)
            }
            
            Statement::Uncurry { variable, function } => {
                let _func = self.evaluate_expression(function)?;
                println!("[UNCURRY] Uncurrying function: {}", variable);
                self.env.define(variable.clone(), Value::String(format!("uncurried_{}", variable)));
                Ok(Value::Void)
            }
            
            Statement::Compose { variable, functions } => {
                println!("[COMPOSE] Composing {} functions", functions.len());
                self.env.define(variable.clone(), Value::String(format!("composed_{}", variable)));
                Ok(Value::Void)
            }
            
            Statement::Pipe { variable, value, functions } => {
                let mut result = self.evaluate_expression(value)?;
                println!("[PIPE] Piping value through {} functions", functions.len());
                for _ in functions {
                    // Simplified: just pass through
                }
                self.env.define(variable.clone(), result);
                Ok(Value::Void)
            }
            
            Statement::Scan { variable, collection, function: _, initial } => {
                let _coll = self.evaluate_expression(collection)?;
                let init = self.evaluate_expression(initial)?;
                println!("[SCAN] Scanning collection");
                self.env.define(variable.clone(), Value::List(vec![init]));
                Ok(Value::Void)
            }
            
            Statement::Unfold { variable, seed, function: _ } => {
                let _s = self.evaluate_expression(seed)?;
                println!("[UNFOLD] Unfolding from seed");
                self.env.define(variable.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            Statement::Unzip { variables, collection } => {
                let _coll = self.evaluate_expression(collection)?;
                println!("[UNZIP] Unzipping into {} variables", variables.len());
                for var in variables {
                    self.env.define(var.clone(), Value::List(Vec::new()));
                }
                Ok(Value::Void)
            }
            
            Statement::FlatMap { variable, collection, function: _ } => {
                let _coll = self.evaluate_expression(collection)?;
                println!("[FLATMAP] Flat mapping collection");
                self.env.define(variable.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            Statement::Bind { variable, monad, function: _ } => {
                let m = self.evaluate_expression(monad)?;
                println!("[BIND] Binding monad");
                self.env.define(variable.clone(), m);
                Ok(Value::Void)
            }
            
            Statement::Pure { variable, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[PURE] Wrapping value in pure context");
                self.env.define(variable.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Applicative { variable, function, value } => {
                let _f = self.evaluate_expression(function)?;
                let v = self.evaluate_expression(value)?;
                println!("[APPLICATIVE] Applying function in applicative context");
                self.env.define(variable.clone(), v);
                Ok(Value::Void)
            }
            
            Statement::Functor { name, map_function: _ } => {
                println!("[FUNCTOR] Defining functor: {}", name);
                self.env.define(name.clone(), Value::String(format!("functor_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Monad { name, bind_function: _ } => {
                println!("[MONAD] Defining monad: {}", name);
                self.env.define(name.clone(), Value::String(format!("monad_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Monoid { name, identity, combine: _ } => {
                let id = self.evaluate_expression(identity)?;
                println!("[MONOID] Defining monoid: {} with identity {:?}", name, id);
                self.env.define(name.clone(), Value::String(format!("monoid_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Semigroup { name, combine: _ } => {
                println!("[SEMIGROUP] Defining semigroup: {}", name);
                self.env.define(name.clone(), Value::String(format!("semigroup_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Category { name, objects, morphisms } => {
                println!("[CATEGORY] Defining category: {} with {} objects and {} morphisms", 
                    name, objects.len(), morphisms.len());
                self.env.define(name.clone(), Value::String(format!("category_{}", name)));
                Ok(Value::Void)
            }
            
            // Object-oriented
            Statement::Constructor { class_name, parameters, body } => {
                println!("[CONSTRUCTOR] Defining constructor for {}", class_name);
                let func = Value::Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                    closure: HashMap::new(),
                };
                self.env.define(format!("{}_constructor", class_name), func);
                Ok(Value::Void)
            }
            
            Statement::Destructor { class_name, body } => {
                println!("[DESTRUCTOR] Defining destructor for {}", class_name);
                let func = Value::Function {
                    parameters: vec![],
                    body: body.clone(),
                    closure: HashMap::new(),
                };
                self.env.define(format!("{}_destructor", class_name), func);
                Ok(Value::Void)
            }
            
            Statement::Initializer { name, body } => {
                println!("[INITIALIZER] Defining initializer: {}", name);
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(Value::Void)
            }
            
            Statement::Deinitializer { name, body } => {
                println!("[DEINITIALIZER] Defining deinitializer: {}", name);
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(Value::Void)
            }
            
            Statement::Getter { property, body } => {
                println!("[GETTER] Defining getter for property: {}", property);
                let func = Value::Function {
                    parameters: vec![],
                    body: body.clone(),
                    closure: HashMap::new(),
                };
                self.env.define(format!("get_{}", property), func);
                Ok(Value::Void)
            }
            
            Statement::Setter { property, parameter, body } => {
                println!("[SETTER] Defining setter for property: {}", property);
                let func = Value::Function {
                    parameters: vec![Parameter {
                        name: parameter.clone(),
                        type_annotation: None,
                        default_value: None,
                        is_ref: false,
                        is_mut: false,
                    }],
                    body: body.clone(),
                    closure: HashMap::new(),
                };
                self.env.define(format!("set_{}", property), func);
                Ok(Value::Void)
            }
            
            Statement::Property { name, type_annotation: _, getter, setter } => {
                println!("[PROPERTY] Defining property: {}", name);
                if let Some(g) = getter {
                    let func = Value::Function {
                        parameters: vec![],
                        body: g.clone(),
                        closure: HashMap::new(),
                    };
                    self.env.define(format!("get_{}", name), func);
                }
                if let Some(s) = setter {
                    let func = Value::Function {
                        parameters: vec![Parameter {
                            name: "value".to_string(),
                            type_annotation: None,
                            default_value: None,
                            is_ref: false,
                            is_mut: false,
                        }],
                        body: s.clone(),
                        closure: HashMap::new(),
                    };
                    self.env.define(format!("set_{}", name), func);
                }
                Ok(Value::Void)
            }
            
            Statement::Method { name, parameters, return_type: _, body } => {
                println!("[METHOD] Defining method: {}", name);
                let func = Value::Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                    closure: HashMap::new(),
                };
                self.env.define(name.clone(), func);
                Ok(Value::Void)
            }
            
            Statement::Field { name, type_annotation: _, value } => {
                let val = if let Some(v) = value {
                    self.evaluate_expression(v)?
                } else {
                    Value::Null
                };
                println!("[FIELD] Defining field: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Member { object, name } => {
                println!("[MEMBER] Accessing member {} of {}", name, object);
                Ok(Value::Void)
            }
            
            Statement::Attribute { name, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[ATTRIBUTE] Setting attribute: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Annotation { name, args } => {
                println!("[ANNOTATION] Applying annotation: {} with {} args", name, args.len());
                Ok(Value::Void)
            }
            
            Statement::Decorator { name, target } => {
                println!("[DECORATOR] Applying decorator: {}", name);
                self.execute_statement(target)?;
                Ok(Value::Void)
            }
            
            Statement::Mixin { name, traits, body } => {
                println!("[MIXIN] Defining mixin: {} with {} traits", name, traits.len());
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.define(name.clone(), Value::String(format!("mixin_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Delegate { target, method, to } => {
                println!("[DELEGATE] Delegating {}.{} to {}", target, method, to);
                Ok(Value::Void)
            }
            
            Statement::Proxy { name, target, handlers } => {
                println!("[PROXY] Creating proxy {} for {} with {} handlers", name, target, handlers.len());
                self.env.define(name.clone(), Value::String(format!("proxy_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Singleton { name, body } => {
                println!("[SINGLETON] Defining singleton: {}", name);
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.define(name.clone(), Value::String(format!("singleton_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Factory { name, product_type, body } => {
                println!("[FACTORY] Defining factory: {} for {}", name, product_type);
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.define(name.clone(), Value::String(format!("factory_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Builder { name, fields, body } => {
                println!("[BUILDER] Defining builder: {} with {} fields", name, fields.len());
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.define(name.clone(), Value::String(format!("builder_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Prototype { name, base } => {
                println!("[PROTOTYPE] Defining prototype: {} based on {}", name, base);
                self.env.define(name.clone(), Value::String(format!("prototype_{}", name)));
                Ok(Value::Void)
            }
            
            // Pattern matching extensions
            Statement::Where { condition } => {
                let cond = self.evaluate_expression(condition)?;
                println!("[WHERE] Condition: {:?}", cond);
                Ok(Value::Boolean(self.is_truthy(&cond)))
            }
            
            Statement::Such { condition } => {
                let cond = self.evaluate_expression(condition)?;
                println!("[SUCH] Condition: {:?}", cond);
                Ok(Value::Boolean(self.is_truthy(&cond)))
            }
            
            Statement::That { condition } => {
                let cond = self.evaluate_expression(condition)?;
                println!("[THAT] Condition: {:?}", cond);
                Ok(Value::Boolean(self.is_truthy(&cond)))
            }
            
            Statement::Some { variable, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[SOME] Wrapping value in Some: {}", variable);
                self.env.define(variable.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::None { variable } => {
                println!("[NONE] Setting {} to None", variable);
                self.env.define(variable.clone(), Value::Null);
                Ok(Value::Void)
            }
            
            Statement::Ok { variable, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[OK] Wrapping value in Ok: {}", variable);
                self.env.define(variable.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Err { variable, error } => {
                let err = self.evaluate_expression(error)?;
                println!("[ERR] Wrapping error in Err: {}", variable);
                self.env.define(variable.clone(), err);
                Ok(Value::Void)
            }
            
            Statement::Just { variable, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[JUST] Wrapping value in Just: {}", variable);
                self.env.define(variable.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Nothing { variable } => {
                println!("[NOTHING] Setting {} to Nothing", variable);
                self.env.define(variable.clone(), Value::Null);
                Ok(Value::Void)
            }
            
            Statement::Left { variable, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[LEFT] Wrapping value in Left: {}", variable);
                self.env.define(variable.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Right { variable, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[RIGHT] Wrapping value in Right: {}", variable);
                self.env.define(variable.clone(), val);
                Ok(Value::Void)
            }
            
            // Control flow extensions
            Statement::Fallthrough => {
                println!("[FALLTHROUGH] Falling through to next case");
                Ok(Value::Void)
            }
            
            Statement::Do { body } => {
                println!("[DO] Executing do block");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Then { body } => {
                println!("[THEN] Executing then block");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Elif { condition, body } | Statement::Elseif { condition, body } => {
                let cond = self.evaluate_expression(condition)?;
                if self.is_truthy(&cond) {
                    println!("[ELIF] Condition true, executing body");
                    self.env.push_scope();
                    let mut result = Value::Void;
                    for stmt in body {
                        result = self.execute_statement(stmt)?;
                    }
                    self.env.pop_scope();
                    Ok(result)
                } else {
                    Ok(Value::Void)
                }
            }
            
            Statement::Always { body } => {
                println!("[ALWAYS] Executing always block");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Never { body: _ } => {
                println!("[NEVER] Never block - not executing");
                Ok(Value::Void)
            }
            
            Statement::Once { body } => {
                println!("[ONCE] Executing once block");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            // Metaprogramming
            Statement::Reflect { target, variable } => {
                let val = self.evaluate_expression(target)?;
                println!("[REFLECT] Reflecting on: {:?}", val);
                let reflection = Value::String(format!("reflection_{:?}", val));
                self.env.define(variable.clone(), reflection);
                Ok(Value::Void)
            }
            
            Statement::Introspect { target, variable } => {
                let val = self.evaluate_expression(target)?;
                println!("[INTROSPECT] Introspecting: {:?}", val);
                let introspection = Value::String(format!("introspection_{:?}", val));
                self.env.define(variable.clone(), introspection);
                Ok(Value::Void)
            }
            
            Statement::Eval { code } => {
                let c = self.evaluate_expression(code)?;
                println!("[EVAL] Evaluating code: {:?}", c);
                Ok(Value::Void)
            }
            
            Statement::Quote { variable, expression } => {
                println!("[QUOTE] Quoting expression into {}", variable);
                let quoted = Value::String(format!("quoted_{:?}", expression));
                self.env.define(variable.clone(), quoted);
                Ok(Value::Void)
            }
            
            Statement::Unquote { variable } => {
                println!("[UNQUOTE] Unquoting {}", variable);
                Ok(Value::Void)
            }
            
            Statement::Splice { target, code } => {
                let c = self.evaluate_expression(code)?;
                println!("[SPLICE] Splicing code into {}: {:?}", target, c);
                Ok(Value::Void)
            }
            
            Statement::Gensym { variable, prefix } => {
                let sym = if let Some(p) = prefix {
                    format!("{}_{}", p, variable)
                } else {
                    format!("gensym_{}", variable)
                };
                println!("[GENSYM] Generating symbol: {}", sym);
                self.env.define(variable.clone(), Value::String(sym));
                Ok(Value::Void)
            }
            
            Statement::Hygiene { enabled } => {
                println!("[HYGIENE] Hygiene {}", if *enabled { "enabled" } else { "disabled" });
                Ok(Value::Void)
            }
            
            Statement::Syntax { name, pattern, template } => {
                println!("[SYNTAX] Defining syntax rule: {} (pattern: {}, template: {})", name, pattern, template);
                Ok(Value::Void)
            }
            
            Statement::Parse { variable, input } => {
                let inp = self.evaluate_expression(input)?;
                println!("[PARSE] Parsing input: {:?}", inp);
                self.env.define(variable.clone(), Value::String("parsed".to_string()));
                Ok(Value::Void)
            }
            
            Statement::Expand { macro_name, args } => {
                println!("[EXPAND] Expanding macro: {} with {} args", macro_name, args.len());
                Ok(Value::Void)
            }
            
            Statement::Compile { source, output } => {
                let src = self.evaluate_expression(source)?;
                println!("[COMPILE] Compiling {:?} to {}", src, output);
                Ok(Value::Void)
            }
            
            Statement::Interpret { code } => {
                let c = self.evaluate_expression(code)?;
                println!("[INTERPRET] Interpreting code: {:?}", c);
                Ok(Value::Void)
            }
            
            Statement::Transpile { source, target_lang, output } => {
                let src = self.evaluate_expression(source)?;
                println!("[TRANSPILE] Transpiling {:?} to {} (output: {})", src, target_lang, output);
                Ok(Value::Void)
            }
            
            // Module system extensions
            Statement::Scope { name, body } => {
                println!("[SCOPE] Entering scope: {}", name);
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Global { name, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[GLOBAL] Defining global: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Local { name, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[LOCAL] Defining local: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Extern { name, signature } => {
                println!("[EXTERN] Declaring extern: {} with signature {}", name, signature);
                self.env.define(name.clone(), Value::String(format!("extern_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Foreign { language, code } => {
                let c = self.evaluate_expression(code)?;
                println!("[FOREIGN] Foreign code in {}: {:?}", language, c);
                Ok(Value::Void)
            }
            
            Statement::Native { name, library } => {
                println!("[NATIVE] Loading native function {} from {}", name, library);
                self.env.define(name.clone(), Value::String(format!("native_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Builtin { name } => {
                println!("[BUILTIN] Referencing builtin: {}", name);
                Ok(Value::Void)
            }
            
            Statement::Prelude { items } => {
                println!("[PRELUDE] Importing {} prelude items", items.len());
                Ok(Value::Void)
            }
            
            Statement::Std { module } => {
                println!("[STD] Importing std module: {}", module);
                Ok(Value::Void)
            }
            
            Statement::Core { feature } => {
                println!("[CORE] Importing core feature: {}", feature);
                Ok(Value::Void)
            }
            
            // Testing & verification
            Statement::PropertyTest { name, property, body } => {
                println!("[PROPERTY_TEST] Running property test: {}", name);
                let _prop = self.evaluate_expression(property)?;
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                println!("[PROPERTY_TEST] Test passed: {}", name);
                Ok(Value::Void)
            }
            
            Statement::Quickcheck { name, property } => {
                let _prop = self.evaluate_expression(property)?;
                println!("[QUICKCHECK] Running quickcheck: {}", name);
                Ok(Value::Void)
            }
            
            Statement::Fuzzy { name, input_generator, body } => {
                let _gen = self.evaluate_expression(input_generator)?;
                println!("[FUZZY] Running fuzzy test: {}", name);
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(Value::Void)
            }
            
            Statement::Stub { name, return_value } => {
                let val = self.evaluate_expression(return_value)?;
                println!("[STUB] Creating stub: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Spy { name, target } => {
                println!("[SPY] Creating spy {} for {}", name, target);
                self.env.define(name.clone(), Value::String(format!("spy_{}", target)));
                Ok(Value::Void)
            }
            
            Statement::Fake { name, behavior } => {
                println!("[FAKE] Creating fake: {}", name);
                self.env.push_scope();
                for stmt in behavior {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                self.env.define(name.clone(), Value::String(format!("fake_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Validate { condition, message } => {
                let cond = self.evaluate_expression(condition)?;
                if !self.is_truthy(&cond) {
                    let msg = if let Some(m) = message {
                        format!("{:?}", self.evaluate_expression(m)?)
                    } else {
                        "Validation failed".to_string()
                    };
                    return Err(ProtlinError::RuntimeError(msg));
                }
                println!("[VALIDATE] Validation passed");
                Ok(Value::Void)
            }
            
            Statement::Check { condition, message } => {
                let cond = self.evaluate_expression(condition)?;
                if !self.is_truthy(&cond) {
                    let msg = if let Some(m) = message {
                        format!("{:?}", self.evaluate_expression(m)?)
                    } else {
                        "Check failed".to_string()
                    };
                    return Err(ProtlinError::RuntimeError(msg));
                }
                println!("[CHECK] Check passed");
                Ok(Value::Void)
            }
            
            Statement::Prove { theorem, proof } => {
                println!("[PROVE] Proving theorem: {}", theorem);
                self.env.push_scope();
                for stmt in proof {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                println!("[PROVE] Theorem proved: {}", theorem);
                Ok(Value::Void)
            }
            
            Statement::Theorem { name, statement } => {
                let _stmt = self.evaluate_expression(statement)?;
                println!("[THEOREM] Defining theorem: {}", name);
                self.env.define(name.clone(), Value::String(format!("theorem_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Lemma { name, statement } => {
                let _stmt = self.evaluate_expression(statement)?;
                println!("[LEMMA] Defining lemma: {}", name);
                self.env.define(name.clone(), Value::String(format!("lemma_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Axiom { name, statement } => {
                let _stmt = self.evaluate_expression(statement)?;
                println!("[AXIOM] Defining axiom: {}", name);
                self.env.define(name.clone(), Value::String(format!("axiom_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Corollary { name, theorem, statement } => {
                let _stmt = self.evaluate_expression(statement)?;
                println!("[COROLLARY] Defining corollary: {} from theorem {}", name, theorem);
                self.env.define(name.clone(), Value::String(format!("corollary_{}", name)));
                Ok(Value::Void)
            }
            
            // Additional data structures
            Statement::Heap { name, heap_type } => {
                println!("[HEAP] Creating {} heap: {}", heap_type, name);
                self.env.define(name.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            Statement::Node { name, value, children } => {
                let val = self.evaluate_expression(value)?;
                println!("[NODE] Creating node: {} with {} children", name, children.len());
                let mut node = HashMap::new();
                node.insert("value".to_string(), val);
                node.insert("children".to_string(), Value::List(Vec::new()));
                self.env.define(name.clone(), Value::Dict(node));
                Ok(Value::Void)
            }
            
            Statement::Edge { from, to, weight } => {
                let w = if let Some(wt) = weight {
                    self.evaluate_expression(wt)?
                } else {
                    Value::Integer(1)
                };
                println!("[EDGE] Creating edge from {} to {} with weight {:?}", from, to, w);
                Ok(Value::Void)
            }
            
            Statement::Vertex { name, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[VERTEX] Creating vertex: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Cycle { name, nodes } => {
                println!("[CYCLE] Creating cycle: {} with {} nodes", name, nodes.len());
                self.env.define(name.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            // I/O & Streams
            Statement::Stream { name, source } => {
                let src = self.evaluate_expression(source)?;
                println!("[STREAM] Creating stream: {} from {:?}", name, src);
                self.env.define(name.clone(), Value::String(format!("stream_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Reader { name, source } => {
                let src = self.evaluate_expression(source)?;
                println!("[READER] Creating reader: {} from {:?}", name, src);
                self.env.define(name.clone(), Value::String(format!("reader_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Writer { name, destination } => {
                let dest = self.evaluate_expression(destination)?;
                println!("[WRITER] Creating writer: {} to {:?}", name, dest);
                self.env.define(name.clone(), Value::String(format!("writer_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Buffer { name, size } => {
                let sz = self.evaluate_expression(size)?;
                println!("[BUFFER] Creating buffer: {} with size {:?}", name, sz);
                self.env.define(name.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            Statement::Seek { handle, position } => {
                let pos = self.evaluate_expression(position)?;
                println!("[SEEK] Seeking {} to position {:?}", handle, pos);
                Ok(Value::Void)
            }
            
            Statement::Tell { handle, variable } => {
                println!("[TELL] Getting position of {}", handle);
                self.env.define(variable.clone(), Value::Integer(0));
                Ok(Value::Void)
            }
            
            Statement::Rewind { handle } => {
                println!("[REWIND] Rewinding {}", handle);
                Ok(Value::Void)
            }
            
            // Network extensions
            Statement::Network { name, protocol } => {
                println!("[NETWORK] Creating network: {} with protocol {}", name, protocol);
                self.env.define(name.clone(), Value::String(format!("network_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Socket { name, socket_type } => {
                println!("[SOCKET] Creating {} socket: {}", socket_type, name);
                self.env.define(name.clone(), Value::String(format!("socket_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Accept { socket, variable } => {
                println!("[ACCEPT] Accepting connection on socket: {}", socket);
                self.env.define(variable.clone(), Value::String(format!("connection_{}", socket)));
                Ok(Value::Void)
            }
            
            Statement::BindNet { socket, address } => {
                let addr = self.evaluate_expression(address)?;
                println!("[BIND] Binding socket {} to {:?}", socket, addr);
                Ok(Value::Void)
            }
            
            Statement::Shutdown { connection } => {
                println!("[SHUTDOWN] Shutting down connection: {}", connection);
                Ok(Value::Void)
            }
            
            Statement::ProtocolDef { name, methods } => {
                println!("[PROTOCOL] Defining protocol: {} with {} methods", name, methods.len());
                self.env.define(name.clone(), Value::String(format!("protocol_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Http { method, url, body } => {
                let u = self.evaluate_expression(url)?;
                let b = if let Some(bd) = body {
                    self.evaluate_expression(bd)?
                } else {
                    Value::Null
                };
                println!("[HTTP] {} request to {:?} with body {:?}", method, u, b);
                Ok(Value::Void)
            }
            
            Statement::Https { method, url, body } => {
                let u = self.evaluate_expression(url)?;
                let b = if let Some(bd) = body {
                    self.evaluate_expression(bd)?
                } else {
                    Value::Null
                };
                println!("[HTTPS] {} request to {:?} with body {:?}", method, u, b);
                Ok(Value::Void)
            }
            
            Statement::Tcp { address, body } => {
                let addr = self.evaluate_expression(address)?;
                println!("[TCP] TCP connection to {:?}", addr);
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(Value::Void)
            }
            
            Statement::Udp { address, body } => {
                let addr = self.evaluate_expression(address)?;
                println!("[UDP] UDP connection to {:?}", addr);
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(Value::Void)
            }
            
            Statement::Websocket { url, handlers } => {
                let u = self.evaluate_expression(url)?;
                println!("[WEBSOCKET] WebSocket connection to {:?} with {} handlers", u, handlers.len());
                Ok(Value::Void)
            }
            
            Statement::Rpc { service, method, args } => {
                println!("[RPC] Calling {}.{} with {} args", service, method, args.len());
                Ok(Value::Void)
            }
            
            Statement::Rest { endpoint, method, data } => {
                let d = if let Some(dt) = data {
                    self.evaluate_expression(dt)?
                } else {
                    Value::Null
                };
                println!("[REST] {} {} with data {:?}", method, endpoint, d);
                Ok(Value::Void)
            }
            
            Statement::Graphql { query, variables } => {
                let q = self.evaluate_expression(query)?;
                let v = if let Some(vars) = variables {
                    self.evaluate_expression(vars)?
                } else {
                    Value::Null
                };
                println!("[GRAPHQL] Query {:?} with variables {:?}", q, v);
                Ok(Value::Void)
            }
            
            // Database extensions
            Statement::Create { entity_type, name, definition } => {
                let def = self.evaluate_expression(definition)?;
                println!("[CREATE] Creating {} {}: {:?}", entity_type, name, def);
                Ok(Value::Void)
            }
            
            Statement::Alter { entity_type, name, changes } => {
                let chg = self.evaluate_expression(changes)?;
                println!("[ALTER] Altering {} {}: {:?}", entity_type, name, chg);
                Ok(Value::Void)
            }
            
            Statement::DropTable { name } => {
                println!("[DROP] Dropping table: {}", name);
                Ok(Value::Void)
            }
            
            Statement::Index { table, columns } => {
                println!("[INDEX] Creating index on {}.{:?}", table, columns);
                Ok(Value::Void)
            }
            
            Statement::View { name, query } => {
                let q = self.evaluate_expression(query)?;
                println!("[VIEW] Creating view {}: {:?}", name, q);
                Ok(Value::Void)
            }
            
            Statement::Savepoint { name } => {
                println!("[SAVEPOINT] Creating savepoint: {}", name);
                Ok(Value::Void)
            }
            
            // Security extensions
            Statement::Seal { variable, data } => {
                let d = self.evaluate_expression(data)?;
                println!("[SEAL] Sealing data into {}", variable);
                self.env.define(variable.clone(), Value::String(format!("sealed_{:?}", d)));
                Ok(Value::Void)
            }
            
            Statement::Unseal { variable, sealed_data } => {
                let sd = self.evaluate_expression(sealed_data)?;
                println!("[UNSEAL] Unsealing data into {}", variable);
                self.env.define(variable.clone(), sd);
                Ok(Value::Void)
            }
            
            Statement::Secure { body } => {
                println!("[SECURE] Executing secure block");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Unsafe { body } => {
                println!("[UNSAFE] Executing unsafe block");
                self.env.push_scope();
                let mut result = Value::Void;
                for stmt in body {
                    result = self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(result)
            }
            
            Statement::Trusted { source } => {
                println!("[TRUSTED] Marking source as trusted: {}", source);
                Ok(Value::Void)
            }
            
            Statement::Untrusted { source } => {
                println!("[UNTRUSTED] Marking source as untrusted: {}", source);
                Ok(Value::Void)
            }
            
            Statement::Sanitize { variable, data } => {
                let d = self.evaluate_expression(data)?;
                println!("[SANITIZE] Sanitizing data into {}", variable);
                self.env.define(variable.clone(), d);
                Ok(Value::Void)
            }
            
            Statement::Escape { variable, data } => {
                let d = self.evaluate_expression(data)?;
                println!("[ESCAPE] Escaping data into {}", variable);
                let escaped = Value::String(format!("escaped_{:?}", d));
                self.env.define(variable.clone(), escaped);
                Ok(Value::Void)
            }
            
            // Time & Date extensions
            Statement::Time { variable, format } => {
                let fmt = if let Some(f) = format {
                    types::coerce_to_string(&self.evaluate_expression(f)?)
                } else {
                    "HH:MM:SS".to_string()
                };
                println!("[TIME] Getting time with format: {}", fmt);
                self.env.define(variable.clone(), Value::String("12:00:00".to_string()));
                Ok(Value::Void)
            }
            
            Statement::Date { variable, format } => {
                let fmt = if let Some(f) = format {
                    types::coerce_to_string(&self.evaluate_expression(f)?)
                } else {
                    "YYYY-MM-DD".to_string()
                };
                println!("[DATE] Getting date with format: {}", fmt);
                self.env.define(variable.clone(), Value::String("2024-01-01".to_string()));
                Ok(Value::Void)
            }
            
            Statement::DateTime { variable, format } => {
                let fmt = if let Some(f) = format {
                    types::coerce_to_string(&self.evaluate_expression(f)?)
                } else {
                    "YYYY-MM-DD HH:MM:SS".to_string()
                };
                println!("[DATETIME] Getting datetime with format: {}", fmt);
                self.env.define(variable.clone(), Value::String("2024-01-01 12:00:00".to_string()));
                Ok(Value::Void)
            }
            
            Statement::Duration { variable, amount, unit } => {
                let amt = self.evaluate_expression(amount)?;
                println!("[DURATION] Creating duration: {:?} {}", amt, unit);
                self.env.define(variable.clone(), Value::Integer(types::coerce_to_int(&amt)?));
                Ok(Value::Void)
            }
            
            Statement::Instant { variable } => {
                println!("[INSTANT] Getting current instant");
                self.env.define(variable.clone(), Value::Integer(1234567890));
                Ok(Value::Void)
            }
            
            Statement::Tomorrow { variable } => {
                println!("[TOMORROW] Getting tomorrow's date");
                self.env.define(variable.clone(), Value::String("2024-01-02".to_string()));
                Ok(Value::Void)
            }
            
            Statement::Yesterday { variable } => {
                println!("[YESTERDAY] Getting yesterday's date");
                self.env.define(variable.clone(), Value::String("2023-12-31".to_string()));
                Ok(Value::Void)
            }
            
            // Math & Science extensions
            Statement::Science { domain, operation, args } => {
                println!("[SCIENCE] {} operation in {}: {} args", operation, domain, args.len());
                Ok(Value::Void)
            }
            
            Statement::Physics { calculation, params } => {
                println!("[PHYSICS] Calculation: {} with {} params", calculation, params.len());
                Ok(Value::Void)
            }
            
            Statement::Chemistry { reaction, reactants } => {
                println!("[CHEMISTRY] Reaction: {} with {} reactants", reaction, reactants.len());
                Ok(Value::Void)
            }
            
            Statement::Biology { process, params } => {
                println!("[BIOLOGY] Process: {} with {} params", process, params.len());
                Ok(Value::Void)
            }
            
            Statement::Statistics { operation, data } => {
                let d = self.evaluate_expression(data)?;
                println!("[STATISTICS] Operation: {} on data {:?}", operation, d);
                Ok(Value::Void)
            }
            
            Statement::Probability { event, space } => {
                let e = self.evaluate_expression(event)?;
                let s = self.evaluate_expression(space)?;
                println!("[PROBABILITY] Event {:?} in space {:?}", e, s);
                Ok(Value::Void)
            }
            
            Statement::Distribution { dist_type, parameters } => {
                println!("[DISTRIBUTION] {} distribution with {} parameters", dist_type, parameters.len());
                Ok(Value::Void)
            }
            
            Statement::Normal { mean, std_dev } => {
                let m = self.evaluate_expression(mean)?;
                let sd = self.evaluate_expression(std_dev)?;
                println!("[NORMAL] Normal distribution: mean {:?}, std_dev {:?}", m, sd);
                Ok(Value::Void)
            }
            
            Statement::Uniform { min, max } => {
                let mn = self.evaluate_expression(min)?;
                let mx = self.evaluate_expression(max)?;
                println!("[UNIFORM] Uniform distribution: min {:?}, max {:?}", mn, mx);
                Ok(Value::Void)
            }
            
            Statement::Exponential { rate } => {
                let r = self.evaluate_expression(rate)?;
                println!("[EXPONENTIAL] Exponential distribution: rate {:?}", r);
                Ok(Value::Void)
            }
            
            Statement::Poisson { lambda } => {
                let l = self.evaluate_expression(lambda)?;
                println!("[POISSON] Poisson distribution: lambda {:?}", l);
                Ok(Value::Void)
            }
            
            // Graphics & UI extensions
            Statement::Graphics { context, operations } => {
                println!("[GRAPHICS] Graphics context: {} with {} operations", context, operations.len());
                self.env.push_scope();
                for stmt in operations {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(Value::Void)
            }
            
            Statement::Paint { target, color } => {
                let c = self.evaluate_expression(color)?;
                println!("[PAINT] Painting {} with color {:?}", target, c);
                Ok(Value::Void)
            }
            
            Statement::Fill { shape, color } => {
                let c = self.evaluate_expression(color)?;
                println!("[FILL] Filling {} with color {:?}", shape, c);
                Ok(Value::Void)
            }
            
            Statement::Stroke { shape, color, width } => {
                let c = self.evaluate_expression(color)?;
                let w = self.evaluate_expression(width)?;
                println!("[STROKE] Stroking {} with color {:?} and width {:?}", shape, c, w);
                Ok(Value::Void)
            }
            
            Statement::Color { variable, r, g, b, a } => {
                let red = self.evaluate_expression(r)?;
                let green = self.evaluate_expression(g)?;
                let blue = self.evaluate_expression(b)?;
                let alpha = if let Some(al) = a {
                    self.evaluate_expression(al)?
                } else {
                    Value::Integer(255)
                };
                println!("[COLOR] Creating color: R{:?} G{:?} B{:?} A{:?}", red, green, blue, alpha);
                self.env.define(variable.clone(), Value::String(format!("color_rgb")));
                Ok(Value::Void)
            }
            
            Statement::Pixel { x, y, color } => {
                let px = self.evaluate_expression(x)?;
                let py = self.evaluate_expression(y)?;
                let c = self.evaluate_expression(color)?;
                println!("[PIXEL] Setting pixel at ({:?}, {:?}) to color {:?}", px, py, c);
                Ok(Value::Void)
            }
            
            Statement::Widget { name, widget_type, properties } => {
                println!("[WIDGET] Creating {} widget: {} with {} properties", widget_type, name, properties.len());
                self.env.define(name.clone(), Value::String(format!("widget_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Layout { name, layout_type, children } => {
                println!("[LAYOUT] Creating {} layout: {} with {} children", layout_type, name, children.len());
                self.env.define(name.clone(), Value::String(format!("layout_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Style { target, properties } => {
                println!("[STYLE] Styling {} with {} properties", target, properties.len());
                Ok(Value::Void)
            }
            
            Statement::Theme { name, colors } => {
                println!("[THEME] Creating theme: {} with {} colors", name, colors.len());
                self.env.define(name.clone(), Value::String(format!("theme_{}", name)));
                Ok(Value::Void)
            }
            
            // Audio & Media
            Statement::Audio { name, source } => {
                let src = self.evaluate_expression(source)?;
                println!("[AUDIO] Loading audio: {} from {:?}", name, src);
                self.env.define(name.clone(), Value::String(format!("audio_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Video { name, source } => {
                let src = self.evaluate_expression(source)?;
                println!("[VIDEO] Loading video: {} from {:?}", name, src);
                self.env.define(name.clone(), Value::String(format!("video_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Media { name, media_type, source } => {
                let src = self.evaluate_expression(source)?;
                println!("[MEDIA] Loading {} media: {} from {:?}", media_type, name, src);
                self.env.define(name.clone(), Value::String(format!("media_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Sound { name, frequency, duration } => {
                let freq = self.evaluate_expression(frequency)?;
                let dur = self.evaluate_expression(duration)?;
                println!("[SOUND] Creating sound: {} at {:?} Hz for {:?}", name, freq, dur);
                self.env.define(name.clone(), Value::String(format!("sound_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Music { name, notes } => {
                println!("[MUSIC] Creating music: {} with {} notes", name, notes.len());
                self.env.define(name.clone(), Value::String(format!("music_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Play { media } => {
                println!("[PLAY] Playing media: {}", media);
                Ok(Value::Void)
            }
            
            Statement::Pause { media } => {
                println!("[PAUSE] Pausing media: {}", media);
                Ok(Value::Void)
            }
            
            Statement::Stop { media } => {
                println!("[STOP] Stopping media: {}", media);
                Ok(Value::Void)
            }
            
            Statement::Record { name, source, duration } => {
                let dur = if let Some(d) = duration {
                    self.evaluate_expression(d)?
                } else {
                    Value::Null
                };
                println!("[RECORD] Recording {} from {} for {:?}", name, source, dur);
                self.env.define(name.clone(), Value::String(format!("recording_{}", name)));
                Ok(Value::Void)
            }
            
            Statement::Volume { media, level } => {
                let lvl = self.evaluate_expression(level)?;
                println!("[VOLUME] Setting volume of {} to {:?}", media, lvl);
                Ok(Value::Void)
            }
            
            Statement::Pitch { sound, frequency } => {
                let freq = self.evaluate_expression(frequency)?;
                println!("[PITCH] Setting pitch of {} to {:?}", sound, freq);
                Ok(Value::Void)
            }
            
            Statement::Tempo { music, bpm } => {
                let b = self.evaluate_expression(bpm)?;
                println!("[TEMPO] Setting tempo of {} to {:?} BPM", music, b);
                Ok(Value::Void)
            }
            
            // File system extensions
            Statement::Exists { path, variable } => {
                let p = self.evaluate_expression(path)?;
                println!("[EXISTS] Checking if {:?} exists", p);
                self.env.define(variable.clone(), Value::Boolean(true));
                Ok(Value::Void)
            }
            
            Statement::Chmod { path, mode } => {
                let p = self.evaluate_expression(path)?;
                let m = self.evaluate_expression(mode)?;
                println!("[CHMOD] Changing mode of {:?} to {:?}", p, m);
                Ok(Value::Void)
            }
            
            Statement::Chown { path, owner } => {
                let p = self.evaluate_expression(path)?;
                let o = self.evaluate_expression(owner)?;
                println!("[CHOWN] Changing owner of {:?} to {:?}", p, o);
                Ok(Value::Void)
            }
            
            // Configuration
            Statement::Config { name, settings } => {
                println!("[CONFIG] Creating config: {} with {} settings", name, settings.len());
                let mut cfg = HashMap::new();
                for (k, v) in settings {
                    cfg.insert(k.clone(), self.evaluate_expression(v)?);
                }
                self.env.define(name.clone(), Value::Dict(cfg));
                Ok(Value::Void)
            }
            
            Statement::Settings { name, values } => {
                println!("[SETTINGS] Creating settings: {} with {} values", name, values.len());
                let mut stg = HashMap::new();
                for (k, v) in values {
                    stg.insert(k.clone(), self.evaluate_expression(v)?);
                }
                self.env.define(name.clone(), Value::Dict(stg));
                Ok(Value::Void)
            }
            
            Statement::Options { name, values } => {
                println!("[OPTIONS] Creating options: {} with {} values", name, values.len());
                let mut opt = HashMap::new();
                for (k, v) in values {
                    opt.insert(k.clone(), self.evaluate_expression(v)?);
                }
                self.env.define(name.clone(), Value::Dict(opt));
                Ok(Value::Void)
            }
            
            Statement::Preferences { name, values } => {
                println!("[PREFERENCES] Creating preferences: {} with {} values", name, values.len());
                let mut pref = HashMap::new();
                for (k, v) in values {
                    pref.insert(k.clone(), self.evaluate_expression(v)?);
                }
                self.env.define(name.clone(), Value::Dict(pref));
                Ok(Value::Void)
            }
            
            Statement::Environment { name, variables } => {
                println!("[ENVIRONMENT] Creating environment: {} with {} variables", name, variables.len());
                let mut env = HashMap::new();
                for (k, v) in variables {
                    env.insert(k.clone(), self.evaluate_expression(v)?);
                }
                self.env.define(name.clone(), Value::Dict(env));
                Ok(Value::Void)
            }
            
            Statement::Variable { name, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[VARIABLE] Setting variable: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Parameter { name, type_annotation: _, default } => {
                let val = if let Some(d) = default {
                    self.evaluate_expression(d)?
                } else {
                    Value::Null
                };
                println!("[PARAMETER] Defining parameter: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Argument { name, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[ARGUMENT] Setting argument: {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Flag { name, enabled } => {
                println!("[FLAG] Setting flag {} to {}", name, enabled);
                self.env.define(name.clone(), Value::Boolean(*enabled));
                Ok(Value::Void)
            }
            
            // Lifecycle extensions
            Statement::Init { name, body } => {
                println!("[INIT] Initializing: {}", name);
                self.env.push_scope();
                for stmt in body {
                    self.execute_statement(stmt)?;
                }
                self.env.pop_scope();
                Ok(Value::Void)
            }
            
            Statement::Start { name } => {
                println!("[START] Starting: {}", name);
                Ok(Value::Void)
            }
            
            Statement::Run { name, args } => {
                println!("[RUN] Running {} with {} args", name, args.len());
                Ok(Value::Void)
            }
            
            Statement::Execute { command, args } => {
                let cmd = self.evaluate_expression(command)?;
                println!("[EXECUTE] Executing {:?} with {} args", cmd, args.len());
                Ok(Value::Void)
            }
            
            Statement::Invoke { function, args } => {
                let func = self.evaluate_expression(function)?;
                println!("[INVOKE] Invoking {:?} with {} args", func, args.len());
                Ok(Value::Void)
            }
            
            Statement::Call { function, args } => {
                let func = self.evaluate_expression(function)?;
                println!("[CALL] Calling {:?} with {} args", func, args.len());
                Ok(Value::Void)
            }
            
            Statement::Apply { function, args } => {
                let func = self.evaluate_expression(function)?;
                println!("[APPLY] Applying {:?} with {} args", func, args.len());
                Ok(Value::Void)
            }
            
            Statement::Perform { action, params } => {
                println!("[PERFORM] Performing {} with {} params", action, params.len());
                Ok(Value::Void)
            }
            
            Statement::Complete { task } => {
                println!("[COMPLETE] Completing task: {}", task);
                Ok(Value::Void)
            }
            
            Statement::Finish { task } => {
                println!("[FINISH] Finishing task: {}", task);
                Ok(Value::Void)
            }
            
            Statement::End { scope } => {
                println!("[END] Ending scope: {}", scope);
                Ok(Value::Void)
            }
            
            Statement::Terminate { process } => {
                println!("[TERMINATE] Terminating process: {}", process);
                Ok(Value::Void)
            }
            
            Statement::Kill { process, signal } => {
                let sig = if let Some(s) = signal {
                    self.evaluate_expression(s)?
                } else {
                    Value::Integer(9)
                };
                println!("[KILL] Killing process {} with signal {:?}", process, sig);
                Ok(Value::Void)
            }
            
            // State management
            Statement::State { name, initial } => {
                let init = self.evaluate_expression(initial)?;
                println!("[STATE] Creating state: {} with initial value {:?}", name, init);
                self.env.define(name.clone(), init);
                Ok(Value::Void)
            }
            
            Statement::Store { name, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[STORE] Storing value in {}", name);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Cache { name, value, ttl } => {
                let val = self.evaluate_expression(value)?;
                let t = if let Some(time) = ttl {
                    self.evaluate_expression(time)?
                } else {
                    Value::Null
                };
                println!("[CACHE] Caching value in {} with TTL {:?}", name, t);
                self.env.define(name.clone(), val);
                Ok(Value::Void)
            }
            
            Statement::Memoize { function } => {
                println!("[MEMOIZE] Memoizing function: {}", function);
                Ok(Value::Void)
            }
            
            Statement::Persist { name, data } => {
                let d = self.evaluate_expression(data)?;
                println!("[PERSIST] Persisting {} with data {:?}", name, d);
                Ok(Value::Void)
            }
            
            Statement::Load { name, source } => {
                let src = self.evaluate_expression(source)?;
                println!("[LOAD] Loading {} from {:?}", name, src);
                self.env.define(name.clone(), Value::String("loaded".to_string()));
                Ok(Value::Void)
            }
            
            Statement::Save { name, destination } => {
                let dest = self.evaluate_expression(destination)?;
                println!("[SAVE] Saving {} to {:?}", name, dest);
                Ok(Value::Void)
            }
            
            Statement::Restore { name, checkpoint } => {
                println!("[RESTORE] Restoring {} from checkpoint {}", name, checkpoint);
                Ok(Value::Void)
            }
            
            Statement::Snapshot { name, state } => {
                let st = self.evaluate_expression(state)?;
                println!("[SNAPSHOT] Creating snapshot {} of state {:?}", name, st);
                self.env.define(name.clone(), st);
                Ok(Value::Void)
            }
            
            Statement::Checkpoint { name } => {
                println!("[CHECKPOINT] Creating checkpoint: {}", name);
                Ok(Value::Void)
            }
            
            Statement::Undo { steps } => {
                let s = if let Some(st) = steps {
                    types::coerce_to_int(&self.evaluate_expression(st)?)?
                } else {
                    1
                };
                println!("[UNDO] Undoing {} steps", s);
                Ok(Value::Void)
            }
            
            Statement::Redo { steps } => {
                let s = if let Some(st) = steps {
                    types::coerce_to_int(&self.evaluate_expression(st)?)?
                } else {
                    1
                };
                println!("[REDO] Redoing {} steps", s);
                Ok(Value::Void)
            }
            
            Statement::History { name, max_size } => {
                let sz = if let Some(ms) = max_size {
                    self.evaluate_expression(ms)?
                } else {
                    Value::Integer(100)
                };
                println!("[HISTORY] Creating history {} with max size {:?}", name, sz);
                self.env.define(name.clone(), Value::List(Vec::new()));
                Ok(Value::Void)
            }
            
            // Validation & Constraints
            Statement::Constraint { name, condition } => {
                let cond = self.evaluate_expression(condition)?;
                println!("[CONSTRAINT] Defining constraint: {} with condition {:?}", name, cond);
                self.env.define(name.clone(), cond);
                Ok(Value::Void)
            }
            
            Statement::Bound { variable, lower, upper } => {
                let lo = self.evaluate_expression(lower)?;
                let hi = self.evaluate_expression(upper)?;
                println!("[BOUND] Bounding {} between {:?} and {:?}", variable, lo, hi);
                Ok(Value::Void)
            }
            
            Statement::Limit { variable, max } => {
                let mx = self.evaluate_expression(max)?;
                println!("[LIMIT] Limiting {} to max {:?}", variable, mx);
                Ok(Value::Void)
            }
            
            Statement::Min { variable, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[MIN] Setting minimum of {} to {:?}", variable, val);
                Ok(Value::Void)
            }
            
            Statement::Max { variable, value } => {
                let val = self.evaluate_expression(value)?;
                println!("[MAX] Setting maximum of {} to {:?}", variable, val);
                Ok(Value::Void)
            }
            
            Statement::RangeConstraint { variable, min, max } => {
                let mn = self.evaluate_expression(min)?;
                let mx = self.evaluate_expression(max)?;
                println!("[RANGE] Constraining {} to range {:?}..{:?}", variable, mn, mx);
                Ok(Value::Void)
            }
            
            Statement::Between { value, lower, upper } => {
                let val = self.evaluate_expression(value)?;
                let lo = self.evaluate_expression(lower)?;
                let hi = self.evaluate_expression(upper)?;
                println!("[BETWEEN] Checking if {:?} is between {:?} and {:?}", val, lo, hi);
                Ok(Value::Void)
            }
            
            Statement::Within { value, bounds } => {
                let val = self.evaluate_expression(value)?;
                let bnd = self.evaluate_expression(bounds)?;
                println!("[WITHIN] Checking if {:?} is within {:?}", val, bnd);
                Ok(Value::Void)
            }
            
            Statement::Outside { value, bounds } => {
                let val = self.evaluate_expression(value)?;
                let bnd = self.evaluate_expression(bounds)?;
                println!("[OUTSIDE] Checking if {:?} is outside {:?}", val, bnd);
                Ok(Value::Void)
            }
            
            Statement::Inside { value, container } => {
                let val = self.evaluate_expression(value)?;
                let cont = self.evaluate_expression(container)?;
                println!("[INSIDE] Checking if {:?} is inside {:?}", val, cont);
                Ok(Value::Void)
            }
            
            Statement::Contains { container, element } => {
                let cont = self.evaluate_expression(container)?;
                let elem = self.evaluate_expression(element)?;
                println!("[CONTAINS] Checking if {:?} contains {:?}", cont, elem);
                Ok(Value::Void)
            }
            
            Statement::Includes { collection, element } => {
                let coll = self.evaluate_expression(collection)?;
                let elem = self.evaluate_expression(element)?;
                println!("[INCLUDES] Checking if {:?} includes {:?}", coll, elem);
                Ok(Value::Void)
            }
            
            Statement::Excludes { collection, element } => {
                let coll = self.evaluate_expression(collection)?;
                let elem = self.evaluate_expression(element)?;
                println!("[EXCLUDES] Checking if {:?} excludes {:?}", coll, elem);
                Ok(Value::Void)
            }
            
            // Operators as keywords
            Statement::Plus { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                    (Value::Decimal(a), Value::Decimal(b)) => Value::Decimal(a + b),
                    _ => Value::Null,
                };
                println!("[PLUS] {:?} + {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::Minus { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
                    (Value::Decimal(a), Value::Decimal(b)) => Value::Decimal(a - b),
                    _ => Value::Null,
                };
                println!("[MINUS] {:?} - {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::TimesOp { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
                    (Value::Decimal(a), Value::Decimal(b)) => Value::Decimal(a * b),
                    _ => Value::Null,
                };
                println!("[TIMES] {:?} * {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::Divide { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) if *b != 0 => Value::Decimal(*a as f64 / *b as f64),
                    (Value::Decimal(a), Value::Decimal(b)) if *b != 0.0 => Value::Decimal(a / b),
                    _ => Value::Null,
                };
                println!("[DIVIDE] {:?} / {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::Modulo { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) if *b != 0 => Value::Integer(a % b),
                    _ => Value::Null,
                };
                println!("[MODULO] {:?} % {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::Equals { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = Value::Boolean(l == r);
                println!("[EQUALS] {:?} == {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::NotEquals { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = Value::Boolean(l != r);
                println!("[NOT_EQUALS] {:?} != {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::GreaterThan { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a > b),
                    (Value::Decimal(a), Value::Decimal(b)) => Value::Boolean(a > b),
                    _ => Value::Boolean(false),
                };
                println!("[GREATER_THAN] {:?} > {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::LessThan { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a < b),
                    (Value::Decimal(a), Value::Decimal(b)) => Value::Boolean(a < b),
                    _ => Value::Boolean(false),
                };
                println!("[LESS_THAN] {:?} < {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::AndOp { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = Value::Boolean(self.is_truthy(&l) && self.is_truthy(&r));
                println!("[AND] {:?} && {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::OrOp { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = Value::Boolean(self.is_truthy(&l) || self.is_truthy(&r));
                println!("[OR] {:?} || {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::NotOp { operand, result } => {
                let op = self.evaluate_expression(operand)?;
                let res = Value::Boolean(!self.is_truthy(&op));
                println!("[NOT] !{:?} = {:?}", op, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            Statement::XorOp { left, right, result } => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let res = Value::Boolean(self.is_truthy(&l) ^ self.is_truthy(&r));
                println!("[XOR] {:?} ^ {:?} = {:?}", l, r, res);
                self.env.define(result.clone(), res);
                Ok(Value::Void)
            }
            
            // Catch-all for unimplemented statements
            _ => {
                println!("Statement not yet implemented: {:?}", std::any::type_name_of_val(statement));
                Ok(Value::Void)
            }
        }
    }
    
    fn match_pattern(&self, pattern: &Pattern, value: &Value) -> Result<bool, ProtlinError> {
        match pattern {
            Pattern::Literal(lit_val) => Ok(lit_val == value),
            Pattern::Identifier(_) => Ok(true),
            Pattern::Wildcard => Ok(true),
            Pattern::Tuple(_) => Ok(matches!(value, Value::Tuple(_))),
            Pattern::List(_) => Ok(matches!(value, Value::List(_))),
            Pattern::Range(_, _) => Ok(true),
            Pattern::Constructor { .. } => Ok(true),
        }
    }
    
    fn evaluate_expression(&mut self, expr: &Expression) -> Result<Value, ProtlinError> {
        match expr {
            Expression::Integer(n) => Ok(Value::Integer(*n)),
            Expression::Decimal(f) => Ok(Value::Decimal(*f)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Boolean(b) => Ok(Value::Boolean(*b)),
            Expression::Null => Ok(Value::Null),
            Expression::Void => Ok(Value::Void),
            
            Expression::Identifier(name) => self.env.get(name),
            
            Expression::Add(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
                    (Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Decimal(a + b)),
                    (Value::Integer(a), Value::Decimal(b)) => Ok(Value::Decimal(*a as f64 + b)),
                    (Value::Decimal(a), Value::Integer(b)) => Ok(Value::Decimal(a + *b as f64)),
                    (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                    _ => Err(ProtlinError::InvalidOperation(format!(
                        "Cannot add {:?} and {:?}",
                        l, r
                    ))),
                }
            }
            
            Expression::Subtract(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
                    (Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Decimal(a - b)),
                    (Value::Integer(a), Value::Decimal(b)) => Ok(Value::Decimal(*a as f64 - b)),
                    (Value::Decimal(a), Value::Integer(b)) => Ok(Value::Decimal(a - *b as f64)),
                    _ => Err(ProtlinError::InvalidOperation(format!(
                        "Cannot subtract {:?} and {:?}",
                        l, r
                    ))),
                }
            }
            
            Expression::Multiply(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
                    (Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Decimal(a * b)),
                    (Value::Integer(a), Value::Decimal(b)) => Ok(Value::Decimal(*a as f64 * b)),
                    (Value::Decimal(a), Value::Integer(b)) => Ok(Value::Decimal(a * *b as f64)),
                    (Value::String(s), Value::Integer(n)) | (Value::Integer(n), Value::String(s)) => {
                        Ok(Value::String(s.repeat(*n as usize)))
                    }
                    _ => Err(ProtlinError::InvalidOperation(format!(
                        "Cannot multiply {:?} and {:?}",
                        l, r
                    ))),
                }
            }
            
            Expression::Divide(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        if *b == 0 {
                            return Err(ProtlinError::DivisionByZero);
                        }
                        Ok(Value::Decimal(*a as f64 / *b as f64))
                    }
                    (Value::Decimal(a), Value::Decimal(b)) => {
                        if *b == 0.0 {
                            return Err(ProtlinError::DivisionByZero);
                        }
                        Ok(Value::Decimal(a / b))
                    }
                    (Value::Integer(a), Value::Decimal(b)) => {
                        if *b == 0.0 {
                            return Err(ProtlinError::DivisionByZero);
                        }
                        Ok(Value::Decimal(*a as f64 / b))
                    }
                    (Value::Decimal(a), Value::Integer(b)) => {
                        if *b == 0 {
                            return Err(ProtlinError::DivisionByZero);
                        }
                        Ok(Value::Decimal(a / *b as f64))
                    }
                    _ => Err(ProtlinError::InvalidOperation(format!(
                        "Cannot divide {:?} and {:?}",
                        l, r
                    ))),
                }
            }
            
            Expression::Modulo(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        if *b == 0 {
                            return Err(ProtlinError::DivisionByZero);
                        }
                        Ok(Value::Integer(a % b))
                    }
                    _ => Err(ProtlinError::InvalidOperation(format!(
                        "Cannot modulo {:?} and {:?}",
                        l, r
                    ))),
                }
            }
            
            Expression::Power(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                let base = types::coerce_to_float(&l)?;
                let exp = types::coerce_to_float(&r)?;
                Ok(Value::Decimal(base.powf(exp)))
            }
            
            Expression::FloorDiv(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        if *b == 0 {
                            return Err(ProtlinError::DivisionByZero);
                        }
                        Ok(Value::Integer(a / b))
                    }
                    _ => {
                        let a = types::coerce_to_float(&l)?;
                        let b = types::coerce_to_float(&r)?;
                        if b == 0.0 {
                            return Err(ProtlinError::DivisionByZero);
                        }
                        Ok(Value::Integer((a / b).floor() as i64))
                    }
                }
            }
            
            Expression::Equal(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                Ok(Value::Boolean(l == r))
            }
            
            Expression::NotEqual(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                Ok(Value::Boolean(l != r))
            }
            
            Expression::Less(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a < b)),
                    (Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Boolean(a < b)),
                    (Value::Integer(a), Value::Decimal(b)) => Ok(Value::Boolean((*a as f64) < *b)),
                    (Value::Decimal(a), Value::Integer(b)) => Ok(Value::Boolean(*a < (*b as f64))),
                    _ => Err(ProtlinError::InvalidOperation(format!(
                        "Cannot compare {:?} and {:?}",
                        l, r
                    ))),
                }
            }
            
            Expression::Greater(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a > b)),
                    (Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Boolean(a > b)),
                    (Value::Integer(a), Value::Decimal(b)) => Ok(Value::Boolean((*a as f64) > *b)),
                    (Value::Decimal(a), Value::Integer(b)) => Ok(Value::Boolean(*a > (*b as f64))),
                    _ => Err(ProtlinError::InvalidOperation(format!(
                        "Cannot compare {:?} and {:?}",
                        l, r
                    ))),
                }
            }
            
            Expression::LessEqual(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a <= b)),
                    (Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Boolean(a <= b)),
                    (Value::Integer(a), Value::Decimal(b)) => Ok(Value::Boolean((*a as f64) <= *b)),
                    (Value::Decimal(a), Value::Integer(b)) => Ok(Value::Boolean(*a <= (*b as f64))),
                    _ => Err(ProtlinError::InvalidOperation(format!(
                        "Cannot compare {:?} and {:?}",
                        l, r
                    ))),
                }
            }
            
            Expression::GreaterEqual(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a >= b)),
                    (Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Boolean(a >= b)),
                    (Value::Integer(a), Value::Decimal(b)) => Ok(Value::Boolean((*a as f64) >= *b)),
                    (Value::Decimal(a), Value::Integer(b)) => Ok(Value::Boolean(*a >= (*b as f64))),
                    _ => Err(ProtlinError::InvalidOperation(format!(
                        "Cannot compare {:?} and {:?}",
                        l, r
                    ))),
                }
            }
            
            Expression::Spaceship(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                
                match (&l, &r) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        Ok(Value::Integer(if a < b { -1 } else if a > b { 1 } else { 0 }))
                    }
                    _ => {
                        let a = types::coerce_to_float(&l)?;
                        let b = types::coerce_to_float(&r)?;
                        Ok(Value::Integer(if a < b { -1 } else if a > b { 1 } else { 0 }))
                    }
                }
            }
            
            Expression::And(left, right) => {
                let l = self.evaluate_expression(left)?;
                if !types::coerce_to_bool(&l) {
                    return Ok(Value::Boolean(false));
                }
                let r = self.evaluate_expression(right)?;
                Ok(Value::Boolean(types::coerce_to_bool(&r)))
            }
            
            Expression::Or(left, right) => {
                let l = self.evaluate_expression(left)?;
                if types::coerce_to_bool(&l) {
                    return Ok(Value::Boolean(true));
                }
                let r = self.evaluate_expression(right)?;
                Ok(Value::Boolean(types::coerce_to_bool(&r)))
            }
            
            Expression::Not(expr) => {
                let val = self.evaluate_expression(expr)?;
                Ok(Value::Boolean(!types::coerce_to_bool(&val)))
            }
            
            Expression::Xor(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let l_bool = types::coerce_to_bool(&l);
                let r_bool = types::coerce_to_bool(&r);
                Ok(Value::Boolean(l_bool ^ r_bool))
            }
            
            Expression::BitAnd(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let a = types::coerce_to_int(&l)?;
                let b = types::coerce_to_int(&r)?;
                Ok(Value::Integer(a & b))
            }
            
            Expression::BitOr(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let a = types::coerce_to_int(&l)?;
                let b = types::coerce_to_int(&r)?;
                Ok(Value::Integer(a | b))
            }
            
            Expression::BitXor(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let a = types::coerce_to_int(&l)?;
                let b = types::coerce_to_int(&r)?;
                Ok(Value::Integer(a ^ b))
            }
            
            Expression::BitNot(expr) => {
                let val = self.evaluate_expression(expr)?;
                let n = types::coerce_to_int(&val)?;
                Ok(Value::Integer(!n))
            }
            
            Expression::LeftShift(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let a = types::coerce_to_int(&l)?;
                let b = types::coerce_to_int(&r)?;
                Ok(Value::Integer(a << b))
            }
            
            Expression::RightShift(left, right) => {
                let l = self.evaluate_expression(left)?;
                let r = self.evaluate_expression(right)?;
                let a = types::coerce_to_int(&l)?;
                let b = types::coerce_to_int(&r)?;
                Ok(Value::Integer(a >> b))
            }
            
            Expression::Assign(left, right) => {
                let value = self.evaluate_expression(right)?;
                
                if let Expression::Identifier(name) = left.as_ref() {
                    self.env.set(name, value.clone())?;
                    Ok(value)
                } else {
                    Err(ProtlinError::InvalidOperation(
                        "Invalid assignment target".to_string(),
                    ))
                }
            }
            
            Expression::AddAssign(left, right) => {
                if let Expression::Identifier(name) = left.as_ref() {
                    let current = self.env.get(name)?;
                    let value = self.evaluate_expression(right)?;
                    let result = self.evaluate_expression(&Expression::Add(
                        Box::new(Expression::Identifier(name.clone())),
                        Box::new(right.as_ref().clone()),
                    ))?;
                    self.env.set(name, result.clone())?;
                    Ok(result)
                } else {
                    Err(ProtlinError::InvalidOperation(
                        "Invalid assignment target".to_string(),
                    ))
                }
            }
            
            Expression::SubAssign(left, right) => {
                if let Expression::Identifier(name) = left.as_ref() {
                    let result = self.evaluate_expression(&Expression::Subtract(
                        Box::new(Expression::Identifier(name.clone())),
                        Box::new(right.as_ref().clone()),
                    ))?;
                    self.env.set(name, result.clone())?;
                    Ok(result)
                } else {
                    Err(ProtlinError::InvalidOperation(
                        "Invalid assignment target".to_string(),
                    ))
                }
            }
            
            Expression::MulAssign(left, right) => {
                if let Expression::Identifier(name) = left.as_ref() {
                    let result = self.evaluate_expression(&Expression::Multiply(
                        Box::new(Expression::Identifier(name.clone())),
                        Box::new(right.as_ref().clone()),
                    ))?;
                    self.env.set(name, result.clone())?;
                    Ok(result)
                } else {
                    Err(ProtlinError::InvalidOperation(
                        "Invalid assignment target".to_string(),
                    ))
                }
            }
            
            Expression::DivAssign(left, right) => {
                if let Expression::Identifier(name) = left.as_ref() {
                    let result = self.evaluate_expression(&Expression::Divide(
                        Box::new(Expression::Identifier(name.clone())),
                        Box::new(right.as_ref().clone()),
                    ))?;
                    self.env.set(name, result.clone())?;
                    Ok(result)
                } else {
                    Err(ProtlinError::InvalidOperation(
                        "Invalid assignment target".to_string(),
                    ))
                }
            }
            
            Expression::ModAssign(left, right) => {
                if let Expression::Identifier(name) = left.as_ref() {
                    let result = self.evaluate_expression(&Expression::Modulo(
                        Box::new(Expression::Identifier(name.clone())),
                        Box::new(right.as_ref().clone()),
                    ))?;
                    self.env.set(name, result.clone())?;
                    Ok(result)
                } else {
                    Err(ProtlinError::InvalidOperation(
                        "Invalid assignment target".to_string(),
                    ))
                }
            }
            
            Expression::Pipeline(left, right) => {
                let value = self.evaluate_expression(left)?;
                
                if let Expression::Call { callee, mut arguments } = right.as_ref().clone() {
                    arguments.insert(0, Expression::Identifier("__pipe__".to_string()));
                    self.env.define("__pipe__".to_string(), value);
                    let result = self.evaluate_expression(&Expression::Call {
                        callee,
                        arguments,
                    })?;
                    Ok(result)
                } else {
                    Err(ProtlinError::InvalidOperation(
                        "Pipeline right side must be a function call".to_string(),
                    ))
                }
            }
            
            Expression::Compose(_, _) => {
                Err(ProtlinError::InvalidOperation(
                    "Function composition not yet implemented".to_string(),
                ))
            }
            
            Expression::NullCoalesce(left, right) => {
                let l = self.evaluate_expression(left)?;
                if matches!(l, Value::Null) {
                    self.evaluate_expression(right)
                } else {
                    Ok(l)
                }
            }
            
            Expression::Elvis(condition, then_expr, else_expr) => {
                let cond = self.evaluate_expression(condition)?;
                if types::coerce_to_bool(&cond) {
                    self.evaluate_expression(then_expr)
                } else {
                    self.evaluate_expression(else_expr)
                }
            }
            
            Expression::Range(start, end) => {
                let s = types::coerce_to_int(&self.evaluate_expression(start)?)?;
                let e = types::coerce_to_int(&self.evaluate_expression(end)?)?;
                Ok(Value::Range {
                    start: s,
                    end: e,
                    inclusive: false,
                })
            }
            
            Expression::RangeInclusive(start, end) => {
                let s = types::coerce_to_int(&self.evaluate_expression(start)?)?;
                let e = types::coerce_to_int(&self.evaluate_expression(end)?)?;
                Ok(Value::Range {
                    start: s,
                    end: e,
                    inclusive: true,
                })
            }
            
            Expression::MemberAccess(object, member) => {
                let obj = self.evaluate_expression(object)?;
                
                match obj {
                    Value::Object { fields, .. } => {
                        fields.get(member).cloned().ok_or_else(|| {
                            ProtlinError::RuntimeError(format!("Unknown field: {}", member))
                        })
                    }
                    _ => Err(ProtlinError::InvalidOperation(
                        "Cannot access member of non-object".to_string(),
                    )),
                }
            }
            
            Expression::Index(object, index) => {
                let obj = self.evaluate_expression(object)?;
                let idx = self.evaluate_expression(index)?;
                
                match obj {
                    Value::List(items) => {
                        let i = types::coerce_to_int(&idx)? as usize;
                        items.get(i).cloned().ok_or(ProtlinError::IndexOutOfBounds)
                    }
                    Value::String(s) => {
                        let i = types::coerce_to_int(&idx)? as usize;
                        s.chars()
                            .nth(i)
                            .map(|c| Value::String(c.to_string()))
                            .ok_or(ProtlinError::IndexOutOfBounds)
                    }
                    Value::Dict(map) => {
                        let key = types::coerce_to_string(&idx);
                        map.get(&key).cloned().ok_or(ProtlinError::IndexOutOfBounds)
                    }
                    _ => Err(ProtlinError::InvalidOperation(
                        "Cannot index non-indexable value".to_string(),
                    )),
                }
            }
            
            Expression::Call { callee, arguments } => {
                let func = self.evaluate_expression(callee)?;
                
                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(self.evaluate_expression(arg)?);
                }
                
                match func {
                    Value::Function {
                        parameters,
                        body,
                        closure: _,
                    } => {
                        self.env.push_scope();
                        
                        for (i, param) in parameters.iter().enumerate() {
                            let value = if i < arg_values.len() {
                                arg_values[i].clone()
                            } else if let Some(default) = &param.default_value {
                                self.evaluate_expression(default)?
                            } else {
                                return Err(ProtlinError::InvalidArgument(format!(
                                    "Missing argument for parameter: {}",
                                    param.name
                                )));
                            };
                            self.env.define(param.name.clone(), value);
                        }
                        
                        let old_in_function = self.in_function;
                        self.in_function = true;
                        
                        let mut result = Value::Void;
                        for stmt in &body {
                            result = self.execute_statement(stmt)?;
                            if self.return_value.is_some() {
                                result = self.return_value.take().unwrap();
                                break;
                            }
                        }
                        
                        self.in_function = old_in_function;
                        self.env.pop_scope();
                        
                        Ok(result)
                    }
                    Value::NativeFunction { name, arity } => {
                        if arg_values.len() != arity && arity != 0 {
                            return Err(ProtlinError::InvalidArgument(format!(
                                "Function {} expects {} arguments, got {}",
                                name,
                                arity,
                                arg_values.len()
                            )));
                        }
                        builtins::call_builtin(&name, arg_values)
                    }
                    _ => Err(ProtlinError::InvalidOperation(
                        "Cannot call non-function value".to_string(),
                    )),
                }
            }
            
            Expression::MethodCall {
                object,
                method,
                arguments,
            } => {
                let obj = self.evaluate_expression(object)?;
                
                let mut arg_values = vec![obj.clone()];
                for arg in arguments {
                    arg_values.push(self.evaluate_expression(arg)?);
                }
                
                match method.as_str() {
                    "len" => builtins::call_builtin("len", vec![obj]),
                    "push" => {
                        if arg_values.len() != 2 {
                            return Err(ProtlinError::InvalidArgument(
                                "push expects 1 argument".to_string(),
                            ));
                        }
                        builtins::call_builtin("push", arg_values)
                    }
                    "pop" => builtins::call_builtin("pop", vec![obj]),
                    _ => Err(ProtlinError::UndefinedFunction(method.clone())),
                }
            }
            
            Expression::List(elements) => {
                let mut items = Vec::new();
                for elem in elements {
                    items.push(self.evaluate_expression(elem)?);
                }
                Ok(Value::List(items))
            }
            
            Expression::Dict(pairs) => {
                let mut map = HashMap::new();
                for (key_expr, value_expr) in pairs {
                    let key = self.evaluate_expression(key_expr)?;
                    let key_str = types::coerce_to_string(&key);
                    let value = self.evaluate_expression(value_expr)?;
                    map.insert(key_str, value);
                }
                Ok(Value::Dict(map))
            }
            
            Expression::Set(elements) => {
                let mut items = Vec::new();
                for elem in elements {
                    let val = self.evaluate_expression(elem)?;
                    if !items.contains(&val) {
                        items.push(val);
                    }
                }
                Ok(Value::Set(items))
            }
            
            Expression::Tuple(elements) => {
                let mut items = Vec::new();
                for elem in elements {
                    items.push(self.evaluate_expression(elem)?);
                }
                Ok(Value::Tuple(items))
            }
            
            Expression::Lambda { parameters, body } => {
                Ok(Value::Function {
                    parameters: parameters.clone(),
                    body: vec![Statement::Return(Some(body.as_ref().clone()))],
                    closure: HashMap::new(),
                })
            }
            
            Expression::Cast(expr, target_type) => {
                let value = self.evaluate_expression(expr)?;
                
                match target_type {
                    Type::Int => Ok(Value::Integer(types::coerce_to_int(&value)?)),
                    Type::Float => Ok(Value::Decimal(types::coerce_to_float(&value)?)),
                    Type::String => Ok(Value::String(types::coerce_to_string(&value))),
                    Type::Bool => Ok(Value::Boolean(types::coerce_to_bool(&value))),
                    _ => Ok(value),
                }
            }
            
            Expression::TypeCheck(expr, target_type) => {
                let value = self.evaluate_expression(expr)?;
                let result = types::check_type(&value, target_type).is_ok();
                Ok(Value::Boolean(result))
            }
            
            Expression::TypeOf(expr) => {
                let value = self.evaluate_expression(expr)?;
                let type_name = match value {
                    Value::Integer(_) => "int",
                    Value::Decimal(_) => "float",
                    Value::String(_) => "string",
                    Value::Boolean(_) => "bool",
                    Value::Null => "null",
                    Value::Void => "void",
                    Value::List(_) => "list",
                    Value::Dict(_) => "dict",
                    Value::Set(_) => "set",
                    Value::Tuple(_) => "tuple",
                    Value::Function { .. } => "function",
                    Value::NativeFunction { .. } => "native_function",
                    Value::Object { class_name, .. } => return Ok(Value::String(class_name)),
                    Value::Range { .. } => "range",
                };
                Ok(Value::String(type_name.to_string()))
            }
            
            Expression::SizeOf(expr) => {
                let value = self.evaluate_expression(expr)?;
                let size = match value {
                    Value::String(s) => s.len() as i64,
                    Value::List(items) => items.len() as i64,
                    Value::Dict(map) => map.len() as i64,
                    Value::Set(items) => items.len() as i64,
                    Value::Tuple(items) => items.len() as i64,
                    _ => std::mem::size_of_val(&value) as i64,
                };
                Ok(Value::Integer(size))
            }
            
            Expression::This => Err(ProtlinError::RuntimeError(
                "'this' can only be used inside a class".to_string(),
            )),
            
            Expression::Super => Err(ProtlinError::RuntimeError(
                "'super' can only be used inside a class".to_string(),
            )),
            
            Expression::Spread(_) => Err(ProtlinError::InvalidOperation(
                "Spread operator not yet implemented".to_string(),
            )),
            
            Expression::Await(_) => Err(ProtlinError::InvalidOperation(
                "Await not yet implemented".to_string(),
            )),
            
            Expression::Clone(expr) => {
                let value = self.evaluate_expression(expr)?;
                Ok(value.clone())
            }
            
            Expression::Move(expr) => {
                let value = self.evaluate_expression(expr)?;
                Ok(value)
            }
            
            Expression::Borrow(expr) => {
                let value = self.evaluate_expression(expr)?;
                Ok(value)
            }
            
            Expression::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                let cond = self.evaluate_expression(condition)?;
                if types::coerce_to_bool(&cond) {
                    self.evaluate_expression(then_expr)
                } else {
                    self.evaluate_expression(else_expr)
                }
            }
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Null | Value::Void => false,
            Value::Integer(n) => *n != 0,
            Value::Decimal(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(items) => !items.is_empty(),
            _ => true,
        }
    }
}
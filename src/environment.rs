use crate::ast::Value;
use crate::error::ProtlinError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            scopes: vec![HashMap::new()],
        };
        env.define_builtins();
        env
    }
    
    fn define_builtins(&mut self) {
        // Define built-in constants - Number system from 0 to myriad (9,999)
        // myriad = 9,999 (maximum base value)
        // Numbers >= 10,000 use myriad+ notation: myriad+1 = 10,000, myriad+100 = 10,099, etc.
        self.define("zero".to_string(), Value::Integer(0));
        self.define("one".to_string(), Value::Integer(1));
        self.define("two".to_string(), Value::Integer(2));
        self.define("three".to_string(), Value::Integer(3));
        self.define("four".to_string(), Value::Integer(4));
        self.define("five".to_string(), Value::Integer(5));
        self.define("six".to_string(), Value::Integer(6));
        self.define("seven".to_string(), Value::Integer(7));
        self.define("eight".to_string(), Value::Integer(8));
        self.define("nine".to_string(), Value::Integer(9));
        self.define("ten".to_string(), Value::Integer(10));
        self.define("hundred".to_string(), Value::Integer(100));
        self.define("thousand".to_string(), Value::Integer(1000));
        self.define("myriad".to_string(), Value::Integer(9999));  // Maximum base value
        
        // Mathematical constants
        self.define("pi".to_string(), Value::Decimal(std::f64::consts::PI));
        self.define("e".to_string(), Value::Decimal(std::f64::consts::E));
        self.define("tau".to_string(), Value::Decimal(std::f64::consts::TAU));
        self.define("phi".to_string(), Value::Decimal(1.618033988749895)); // Golden ratio
        
        // Basic I/O
        self.define("print".to_string(), Value::NativeFunction { name: "print".to_string(), arity: 1 });
        self.define("println".to_string(), Value::NativeFunction { name: "println".to_string(), arity: 1 });
        self.define("input".to_string(), Value::NativeFunction { name: "input".to_string(), arity: 0 });
        
        // Module system
        self.define("import".to_string(), Value::NativeFunction { name: "import".to_string(), arity: 0 });
        self.define("export".to_string(), Value::NativeFunction { name: "export".to_string(), arity: 2 });
        self.define("load".to_string(), Value::NativeFunction { name: "load".to_string(), arity: 0 });
        self.define("unload".to_string(), Value::NativeFunction { name: "unload".to_string(), arity: 0 });
        
        // Collection operations
        self.define("len".to_string(), Value::NativeFunction { name: "len".to_string(), arity: 1 });
        self.define("push".to_string(), Value::NativeFunction { name: "push".to_string(), arity: 2 });
        self.define("pop".to_string(), Value::NativeFunction { name: "pop".to_string(), arity: 1 });
        self.define("append".to_string(), Value::NativeFunction { name: "append".to_string(), arity: 2 });
        self.define("extend".to_string(), Value::NativeFunction { name: "extend".to_string(), arity: 2 });
        self.define("insert".to_string(), Value::NativeFunction { name: "insert".to_string(), arity: 3 });
        self.define("remove".to_string(), Value::NativeFunction { name: "remove".to_string(), arity: 2 });
        self.define("slice".to_string(), Value::NativeFunction { name: "slice".to_string(), arity: 3 });
        self.define("reverse_list".to_string(), Value::NativeFunction { name: "reverse_list".to_string(), arity: 1 });
        self.define("sort".to_string(), Value::NativeFunction { name: "sort".to_string(), arity: 1 });
        self.define("sum".to_string(), Value::NativeFunction { name: "sum".to_string(), arity: 1 });
        self.define("product".to_string(), Value::NativeFunction { name: "product".to_string(), arity: 1 });
        self.define("filter".to_string(), Value::NativeFunction { name: "filter".to_string(), arity: 2 });
        self.define("map".to_string(), Value::NativeFunction { name: "map".to_string(), arity: 2 });
        
        // Type conversion
        self.define("str".to_string(), Value::NativeFunction { name: "str".to_string(), arity: 1 });
        self.define("int".to_string(), Value::NativeFunction { name: "int".to_string(), arity: 1 });
        self.define("float".to_string(), Value::NativeFunction { name: "float".to_string(), arity: 1 });
        self.define("bool".to_string(), Value::NativeFunction { name: "bool".to_string(), arity: 1 });
        self.define("type".to_string(), Value::NativeFunction { name: "type".to_string(), arity: 1 });
        self.define("range".to_string(), Value::NativeFunction { name: "range".to_string(), arity: 2 });
        
        // Basic math
        self.define("abs".to_string(), Value::NativeFunction { name: "abs".to_string(), arity: 1 });
        self.define("min".to_string(), Value::NativeFunction { name: "min".to_string(), arity: 2 });
        self.define("max".to_string(), Value::NativeFunction { name: "max".to_string(), arity: 2 });
        self.define("pow".to_string(), Value::NativeFunction { name: "pow".to_string(), arity: 2 });
        self.define("sqrt".to_string(), Value::NativeFunction { name: "sqrt".to_string(), arity: 1 });
        self.define("floor".to_string(), Value::NativeFunction { name: "floor".to_string(), arity: 1 });
        self.define("ceil".to_string(), Value::NativeFunction { name: "ceil".to_string(), arity: 1 });
        self.define("round".to_string(), Value::NativeFunction { name: "round".to_string(), arity: 1 });
        
        // Trigonometry
        self.define("sin".to_string(), Value::NativeFunction { name: "sin".to_string(), arity: 1 });
        self.define("cos".to_string(), Value::NativeFunction { name: "cos".to_string(), arity: 1 });
        self.define("tan".to_string(), Value::NativeFunction { name: "tan".to_string(), arity: 1 });
        self.define("asin".to_string(), Value::NativeFunction { name: "asin".to_string(), arity: 1 });
        self.define("acos".to_string(), Value::NativeFunction { name: "acos".to_string(), arity: 1 });
        self.define("atan".to_string(), Value::NativeFunction { name: "atan".to_string(), arity: 1 });
        self.define("atan2".to_string(), Value::NativeFunction { name: "atan2".to_string(), arity: 2 });
        self.define("sinh".to_string(), Value::NativeFunction { name: "sinh".to_string(), arity: 1 });
        self.define("cosh".to_string(), Value::NativeFunction { name: "cosh".to_string(), arity: 1 });
        self.define("tanh".to_string(), Value::NativeFunction { name: "tanh".to_string(), arity: 1 });
        
        // Advanced math
        self.define("exp".to_string(), Value::NativeFunction { name: "exp".to_string(), arity: 1 });
        self.define("ln".to_string(), Value::NativeFunction { name: "ln".to_string(), arity: 1 });
        self.define("log".to_string(), Value::NativeFunction { name: "log".to_string(), arity: 2 });
        self.define("log10".to_string(), Value::NativeFunction { name: "log10".to_string(), arity: 1 });
        self.define("log2".to_string(), Value::NativeFunction { name: "log2".to_string(), arity: 1 });
        self.define("cbrt".to_string(), Value::NativeFunction { name: "cbrt".to_string(), arity: 1 });
        self.define("hypot".to_string(), Value::NativeFunction { name: "hypot".to_string(), arity: 2 });
        self.define("trunc".to_string(), Value::NativeFunction { name: "trunc".to_string(), arity: 1 });
        self.define("fract".to_string(), Value::NativeFunction { name: "fract".to_string(), arity: 1 });
        self.define("signum".to_string(), Value::NativeFunction { name: "signum".to_string(), arity: 1 });
        self.define("copysign".to_string(), Value::NativeFunction { name: "copysign".to_string(), arity: 2 });
        self.define("degrees".to_string(), Value::NativeFunction { name: "degrees".to_string(), arity: 1 });
        self.define("radians".to_string(), Value::NativeFunction { name: "radians".to_string(), arity: 1 });
        
        // String operations
        self.define("upper".to_string(), Value::NativeFunction { name: "upper".to_string(), arity: 1 });
        self.define("lower".to_string(), Value::NativeFunction { name: "lower".to_string(), arity: 1 });
        self.define("trim".to_string(), Value::NativeFunction { name: "trim".to_string(), arity: 1 });
        self.define("split".to_string(), Value::NativeFunction { name: "split".to_string(), arity: 2 });
        self.define("join".to_string(), Value::NativeFunction { name: "join".to_string(), arity: 2 });
        self.define("replace".to_string(), Value::NativeFunction { name: "replace".to_string(), arity: 3 });
        self.define("startswith".to_string(), Value::NativeFunction { name: "startswith".to_string(), arity: 2 });
        self.define("endswith".to_string(), Value::NativeFunction { name: "endswith".to_string(), arity: 2 });
        self.define("contains".to_string(), Value::NativeFunction { name: "contains".to_string(), arity: 2 });
        self.define("replicate".to_string(), Value::NativeFunction { name: "replicate".to_string(), arity: 2 });
        self.define("reverse".to_string(), Value::NativeFunction { name: "reverse".to_string(), arity: 1 });
        self.define("chars".to_string(), Value::NativeFunction { name: "chars".to_string(), arity: 1 });
        
        // Array/Vector operations (20+ functions)
        self.define("first".to_string(), Value::NativeFunction { name: "first".to_string(), arity: 1 });
        self.define("last".to_string(), Value::NativeFunction { name: "last".to_string(), arity: 1 });
        self.define("take".to_string(), Value::NativeFunction { name: "take".to_string(), arity: 2 });
        self.define("drop".to_string(), Value::NativeFunction { name: "drop".to_string(), arity: 2 });
        self.define("nth".to_string(), Value::NativeFunction { name: "nth".to_string(), arity: 2 });
        self.define("count".to_string(), Value::NativeFunction { name: "count".to_string(), arity: 2 });
        self.define("unique".to_string(), Value::NativeFunction { name: "unique".to_string(), arity: 1 });
        self.define("flatten_deep".to_string(), Value::NativeFunction { name: "flatten_deep".to_string(), arity: 1 });
        self.define("chunk".to_string(), Value::NativeFunction { name: "chunk".to_string(), arity: 2 });
        self.define("partition".to_string(), Value::NativeFunction { name: "partition".to_string(), arity: 2 });
        self.define("group_by".to_string(), Value::NativeFunction { name: "group_by".to_string(), arity: 2 });
        self.define("all".to_string(), Value::NativeFunction { name: "all".to_string(), arity: 2 });
        self.define("any".to_string(), Value::NativeFunction { name: "any".to_string(), arity: 2 });
        self.define("find".to_string(), Value::NativeFunction { name: "find".to_string(), arity: 2 });
        self.define("find_index".to_string(), Value::NativeFunction { name: "find_index".to_string(), arity: 2 });
        self.define("concat".to_string(), Value::NativeFunction { name: "concat".to_string(), arity: 2 });
        self.define("interleave".to_string(), Value::NativeFunction { name: "interleave".to_string(), arity: 2 });
        self.define("rotate".to_string(), Value::NativeFunction { name: "rotate".to_string(), arity: 2 });
        self.define("shuffle".to_string(), Value::NativeFunction { name: "shuffle".to_string(), arity: 1 });
        self.define("sample".to_string(), Value::NativeFunction { name: "sample".to_string(), arity: 2 });
        
        // Random & Statistics (12+ functions)
        self.define("rand".to_string(), Value::NativeFunction { name: "rand".to_string(), arity: 0 });
        self.define("randint".to_string(), Value::NativeFunction { name: "randint".to_string(), arity: 2 });
        self.define("choice".to_string(), Value::NativeFunction { name: "choice".to_string(), arity: 1 });
        self.define("mean".to_string(), Value::NativeFunction { name: "mean".to_string(), arity: 1 });
        self.define("median".to_string(), Value::NativeFunction { name: "median".to_string(), arity: 1 });
        self.define("mode".to_string(), Value::NativeFunction { name: "mode".to_string(), arity: 1 });
        self.define("variance".to_string(), Value::NativeFunction { name: "variance".to_string(), arity: 1 });
        self.define("stddev".to_string(), Value::NativeFunction { name: "stddev".to_string(), arity: 1 });
        self.define("quantile".to_string(), Value::NativeFunction { name: "quantile".to_string(), arity: 2 });
        self.define("percentile".to_string(), Value::NativeFunction { name: "percentile".to_string(), arity: 2 });
        self.define("correlation".to_string(), Value::NativeFunction { name: "correlation".to_string(), arity: 2 });
        self.define("covariance".to_string(), Value::NativeFunction { name: "covariance".to_string(), arity: 2 });
        
        // Date/Time (14+ functions)
        self.define("timestamp".to_string(), Value::NativeFunction { name: "timestamp".to_string(), arity: 0 });
        self.define("year".to_string(), Value::NativeFunction { name: "year".to_string(), arity: 0 });
        self.define("month".to_string(), Value::NativeFunction { name: "month".to_string(), arity: 0 });
        self.define("day".to_string(), Value::NativeFunction { name: "day".to_string(), arity: 0 });
        self.define("hour".to_string(), Value::NativeFunction { name: "hour".to_string(), arity: 0 });
        self.define("minute".to_string(), Value::NativeFunction { name: "minute".to_string(), arity: 0 });
        self.define("second".to_string(), Value::NativeFunction { name: "second".to_string(), arity: 0 });
        self.define("weekday".to_string(), Value::NativeFunction { name: "weekday".to_string(), arity: 0 });
        self.define("is_leap_year".to_string(), Value::NativeFunction { name: "is_leap_year".to_string(), arity: 1 });
        self.define("days_in_month".to_string(), Value::NativeFunction { name: "days_in_month".to_string(), arity: 2 });
        self.define("format_date".to_string(), Value::NativeFunction { name: "format_date".to_string(), arity: 1 });
        self.define("parse_date".to_string(), Value::NativeFunction { name: "parse_date".to_string(), arity: 1 });
        self.define("add_days".to_string(), Value::NativeFunction { name: "add_days".to_string(), arity: 2 });
        self.define("diff_days".to_string(), Value::NativeFunction { name: "diff_days".to_string(), arity: 2 });
        
        // Encoding/Decoding (12+ functions)
        self.define("base64_encode".to_string(), Value::NativeFunction { name: "base64_encode".to_string(), arity: 1 });
        self.define("base64_decode".to_string(), Value::NativeFunction { name: "base64_decode".to_string(), arity: 1 });
        self.define("hex_encode".to_string(), Value::NativeFunction { name: "hex_encode".to_string(), arity: 1 });
        self.define("hex_decode".to_string(), Value::NativeFunction { name: "hex_decode".to_string(), arity: 1 });
        self.define("url_encode".to_string(), Value::NativeFunction { name: "url_encode".to_string(), arity: 1 });
        self.define("url_decode".to_string(), Value::NativeFunction { name: "url_decode".to_string(), arity: 1 });
        self.define("json_encode".to_string(), Value::NativeFunction { name: "json_encode".to_string(), arity: 1 });
        self.define("json_decode".to_string(), Value::NativeFunction { name: "json_decode".to_string(), arity: 1 });
        self.define("md5".to_string(), Value::NativeFunction { name: "md5".to_string(), arity: 1 });
        self.define("sha1".to_string(), Value::NativeFunction { name: "sha1".to_string(), arity: 1 });
        self.define("sha256".to_string(), Value::NativeFunction { name: "sha256".to_string(), arity: 1 });
        self.define("sha512".to_string(), Value::NativeFunction { name: "sha512".to_string(), arity: 1 });
        
        // File System (13+ functions)
        self.define("file_exists".to_string(), Value::NativeFunction { name: "file_exists".to_string(), arity: 1 });
        self.define("file_size".to_string(), Value::NativeFunction { name: "file_size".to_string(), arity: 1 });
        self.define("file_read".to_string(), Value::NativeFunction { name: "file_read".to_string(), arity: 1 });
        self.define("file_write".to_string(), Value::NativeFunction { name: "file_write".to_string(), arity: 2 });
        self.define("file_append".to_string(), Value::NativeFunction { name: "file_append".to_string(), arity: 2 });
        self.define("file_delete".to_string(), Value::NativeFunction { name: "file_delete".to_string(), arity: 1 });
        self.define("dir_list".to_string(), Value::NativeFunction { name: "dir_list".to_string(), arity: 1 });
        self.define("dir_create".to_string(), Value::NativeFunction { name: "dir_create".to_string(), arity: 1 });
        self.define("dir_delete".to_string(), Value::NativeFunction { name: "dir_delete".to_string(), arity: 1 });
        self.define("path_join".to_string(), Value::NativeFunction { name: "path_join".to_string(), arity: 2 });
        self.define("path_basename".to_string(), Value::NativeFunction { name: "path_basename".to_string(), arity: 1 });
        self.define("path_dirname".to_string(), Value::NativeFunction { name: "path_dirname".to_string(), arity: 1 });
        self.define("path_extension".to_string(), Value::NativeFunction { name: "path_extension".to_string(), arity: 1 });
        
        // Additional math functions
        self.define("gcd".to_string(), Value::NativeFunction { name: "gcd".to_string(), arity: 2 });
        self.define("lcm".to_string(), Value::NativeFunction { name: "lcm".to_string(), arity: 2 });
        
        // Bit manipulation (10 functions)
        self.define("bit_and".to_string(), Value::NativeFunction { name: "bit_and".to_string(), arity: 2 });
        self.define("bit_or".to_string(), Value::NativeFunction { name: "bit_or".to_string(), arity: 2 });
        self.define("bit_xor".to_string(), Value::NativeFunction { name: "bit_xor".to_string(), arity: 2 });
        self.define("bit_not".to_string(), Value::NativeFunction { name: "bit_not".to_string(), arity: 1 });
        self.define("bit_shift_left".to_string(), Value::NativeFunction { name: "bit_shift_left".to_string(), arity: 2 });
        self.define("bit_shift_right".to_string(), Value::NativeFunction { name: "bit_shift_right".to_string(), arity: 2 });
        self.define("bit_count".to_string(), Value::NativeFunction { name: "bit_count".to_string(), arity: 1 });
        self.define("bit_set".to_string(), Value::NativeFunction { name: "bit_set".to_string(), arity: 2 });
        self.define("bit_clear".to_string(), Value::NativeFunction { name: "bit_clear".to_string(), arity: 2 });
        self.define("bit_test".to_string(), Value::NativeFunction { name: "bit_test".to_string(), arity: 2 });
        
        // Color/RGB (8 functions)
        self.define("rgb".to_string(), Value::NativeFunction { name: "rgb".to_string(), arity: 3 });
        self.define("rgba".to_string(), Value::NativeFunction { name: "rgba".to_string(), arity: 4 });
        self.define("hex_to_rgb".to_string(), Value::NativeFunction { name: "hex_to_rgb".to_string(), arity: 1 });
        self.define("rgb_to_hex".to_string(), Value::NativeFunction { name: "rgb_to_hex".to_string(), arity: 3 });
        self.define("lighten".to_string(), Value::NativeFunction { name: "lighten".to_string(), arity: 4 });
        self.define("darken".to_string(), Value::NativeFunction { name: "darken".to_string(), arity: 4 });
        self.define("invert_color".to_string(), Value::NativeFunction { name: "invert_color".to_string(), arity: 3 });
        self.define("luminance".to_string(), Value::NativeFunction { name: "luminance".to_string(), arity: 3 });
        
        // Geometry/Math (11 functions)
        self.define("distance".to_string(), Value::NativeFunction { name: "distance".to_string(), arity: 4 });
        self.define("midpoint".to_string(), Value::NativeFunction { name: "midpoint".to_string(), arity: 4 });
        self.define("slope".to_string(), Value::NativeFunction { name: "slope".to_string(), arity: 4 });
        self.define("lerp".to_string(), Value::NativeFunction { name: "lerp".to_string(), arity: 3 });
        self.define("clamp".to_string(), Value::NativeFunction { name: "clamp".to_string(), arity: 3 });
        self.define("factorial".to_string(), Value::NativeFunction { name: "factorial".to_string(), arity: 1 });
        self.define("fibonacci_n".to_string(), Value::NativeFunction { name: "fibonacci_n".to_string(), arity: 1 });
        self.define("is_prime".to_string(), Value::NativeFunction { name: "is_prime".to_string(), arity: 1 });
        self.define("magnitude".to_string(), Value::NativeFunction { name: "magnitude".to_string(), arity: 2 });
        self.define("normalize".to_string(), Value::NativeFunction { name: "normalize".to_string(), arity: 2 });
        self.define("dot_product".to_string(), Value::NativeFunction { name: "dot_product".to_string(), arity: 4 });
        
        // Text processing (12 functions)
        self.define("word_count".to_string(), Value::NativeFunction { name: "word_count".to_string(), arity: 1 });
        self.define("line_count".to_string(), Value::NativeFunction { name: "line_count".to_string(), arity: 1 });
        self.define("char_count".to_string(), Value::NativeFunction { name: "char_count".to_string(), arity: 1 });
        self.define("capitalize".to_string(), Value::NativeFunction { name: "capitalize".to_string(), arity: 1 });
        self.define("title_case".to_string(), Value::NativeFunction { name: "title_case".to_string(), arity: 1 });
        self.define("snake_case".to_string(), Value::NativeFunction { name: "snake_case".to_string(), arity: 1 });
        self.define("camel_case".to_string(), Value::NativeFunction { name: "camel_case".to_string(), arity: 1 });
        self.define("kebab_case".to_string(), Value::NativeFunction { name: "kebab_case".to_string(), arity: 1 });
        self.define("pad_left".to_string(), Value::NativeFunction { name: "pad_left".to_string(), arity: 3 });
        self.define("pad_right".to_string(), Value::NativeFunction { name: "pad_right".to_string(), arity: 3 });
        self.define("truncate".to_string(), Value::NativeFunction { name: "truncate".to_string(), arity: 2 });
        self.define("slug".to_string(), Value::NativeFunction { name: "slug".to_string(), arity: 1 });
        
        // Collection utilities (6 functions)
        self.define("zip_with".to_string(), Value::NativeFunction { name: "zip_with".to_string(), arity: 2 });
        self.define("frequencies".to_string(), Value::NativeFunction { name: "frequencies".to_string(), arity: 1 });
        self.define("pairwise".to_string(), Value::NativeFunction { name: "pairwise".to_string(), arity: 1 });
        self.define("compact".to_string(), Value::NativeFunction { name: "compact".to_string(), arity: 1 });
        self.define("flatten_once".to_string(), Value::NativeFunction { name: "flatten_once".to_string(), arity: 1 });
        self.define("transpose".to_string(), Value::NativeFunction { name: "transpose".to_string(), arity: 1 });
        
        // Validation (13 functions)
        self.define("is_email".to_string(), Value::NativeFunction { name: "is_email".to_string(), arity: 1 });
        self.define("is_url".to_string(), Value::NativeFunction { name: "is_url".to_string(), arity: 1 });
        self.define("is_alpha".to_string(), Value::NativeFunction { name: "is_alpha".to_string(), arity: 1 });
        self.define("is_numeric".to_string(), Value::NativeFunction { name: "is_numeric".to_string(), arity: 1 });
        self.define("is_alphanumeric".to_string(), Value::NativeFunction { name: "is_alphanumeric".to_string(), arity: 1 });
        self.define("is_lowercase".to_string(), Value::NativeFunction { name: "is_lowercase".to_string(), arity: 1 });
        self.define("is_uppercase".to_string(), Value::NativeFunction { name: "is_uppercase".to_string(), arity: 1 });
        self.define("is_palindrome".to_string(), Value::NativeFunction { name: "is_palindrome".to_string(), arity: 1 });
        self.define("is_empty".to_string(), Value::NativeFunction { name: "is_empty".to_string(), arity: 1 });
        self.define("is_even".to_string(), Value::NativeFunction { name: "is_even".to_string(), arity: 1 });
        self.define("is_odd".to_string(), Value::NativeFunction { name: "is_odd".to_string(), arity: 1 });
        self.define("is_positive".to_string(), Value::NativeFunction { name: "is_positive".to_string(), arity: 1 });
        self.define("is_negative".to_string(), Value::NativeFunction { name: "is_negative".to_string(), arity: 1 });
        
        // Conversion (8 functions)
        self.define("to_binary".to_string(), Value::NativeFunction { name: "to_binary".to_string(), arity: 1 });
        self.define("to_octal".to_string(), Value::NativeFunction { name: "to_octal".to_string(), arity: 1 });
        self.define("to_hex_num".to_string(), Value::NativeFunction { name: "to_hex_num".to_string(), arity: 1 });
        self.define("from_binary".to_string(), Value::NativeFunction { name: "from_binary".to_string(), arity: 1 });
        self.define("from_octal".to_string(), Value::NativeFunction { name: "from_octal".to_string(), arity: 1 });
        self.define("from_hex_num".to_string(), Value::NativeFunction { name: "from_hex_num".to_string(), arity: 1 });
        self.define("to_roman".to_string(), Value::NativeFunction { name: "to_roman".to_string(), arity: 1 });
        self.define("to_ordinal".to_string(), Value::NativeFunction { name: "to_ordinal".to_string(), arity: 1 });
        
        // Advanced String (11 functions)
        self.define("substring".to_string(), Value::NativeFunction { name: "substring".to_string(), arity: 3 });
        self.define("index_of".to_string(), Value::NativeFunction { name: "index_of".to_string(), arity: 2 });
        self.define("last_index_of".to_string(), Value::NativeFunction { name: "last_index_of".to_string(), arity: 2 });
        self.define("left_pad".to_string(), Value::NativeFunction { name: "left_pad".to_string(), arity: 2 });
        self.define("right_pad".to_string(), Value::NativeFunction { name: "right_pad".to_string(), arity: 2 });
        self.define("center".to_string(), Value::NativeFunction { name: "center".to_string(), arity: 2 });
        self.define("remove_prefix".to_string(), Value::NativeFunction { name: "remove_prefix".to_string(), arity: 2 });
        self.define("remove_suffix".to_string(), Value::NativeFunction { name: "remove_suffix".to_string(), arity: 2 });
        self.define("count_substr".to_string(), Value::NativeFunction { name: "count_substr".to_string(), arity: 2 });
        self.define("is_whitespace".to_string(), Value::NativeFunction { name: "is_whitespace".to_string(), arity: 1 });
        self.define("levenshtein".to_string(), Value::NativeFunction { name: "levenshtein".to_string(), arity: 2 });
        
        // Advanced Math (15 functions)
        self.define("nroot".to_string(), Value::NativeFunction { name: "nroot".to_string(), arity: 2 });
        self.define("mod_pow".to_string(), Value::NativeFunction { name: "mod_pow".to_string(), arity: 3 });
        self.define("binomial".to_string(), Value::NativeFunction { name: "binomial".to_string(), arity: 2 });
        self.define("permutation".to_string(), Value::NativeFunction { name: "permutation".to_string(), arity: 2 });
        self.define("combination".to_string(), Value::NativeFunction { name: "combination".to_string(), arity: 2 });
        self.define("is_perfect_square".to_string(), Value::NativeFunction { name: "is_perfect_square".to_string(), arity: 1 });
        self.define("is_power_of_two".to_string(), Value::NativeFunction { name: "is_power_of_two".to_string(), arity: 1 });
        self.define("next_prime".to_string(), Value::NativeFunction { name: "next_prime".to_string(), arity: 1 });
        self.define("prime_factors".to_string(), Value::NativeFunction { name: "prime_factors".to_string(), arity: 1 });
        self.define("divisors".to_string(), Value::NativeFunction { name: "divisors".to_string(), arity: 1 });
        self.define("sum_divisors".to_string(), Value::NativeFunction { name: "sum_divisors".to_string(), arity: 1 });
        self.define("is_perfect".to_string(), Value::NativeFunction { name: "is_perfect".to_string(), arity: 1 });
        self.define("digital_root".to_string(), Value::NativeFunction { name: "digital_root".to_string(), arity: 1 });
        self.define("reverse_number".to_string(), Value::NativeFunction { name: "reverse_number".to_string(), arity: 1 });
        self.define("is_palindrome_number".to_string(), Value::NativeFunction { name: "is_palindrome_number".to_string(), arity: 1 });
        
        // Advanced List (9 functions)
        self.define("cartesian_product".to_string(), Value::NativeFunction { name: "cartesian_product".to_string(), arity: 2 });
        self.define("permutations".to_string(), Value::NativeFunction { name: "permutations".to_string(), arity: 1 });
        self.define("sliding_window".to_string(), Value::NativeFunction { name: "sliding_window".to_string(), arity: 2 });
        self.define("cumsum".to_string(), Value::NativeFunction { name: "cumsum".to_string(), arity: 1 });
        self.define("diff".to_string(), Value::NativeFunction { name: "diff".to_string(), arity: 1 });
        self.define("running_max".to_string(), Value::NativeFunction { name: "running_max".to_string(), arity: 1 });
        self.define("running_min".to_string(), Value::NativeFunction { name: "running_min".to_string(), arity: 1 });
        self.define("argmax".to_string(), Value::NativeFunction { name: "argmax".to_string(), arity: 1 });
        self.define("argmin".to_string(), Value::NativeFunction { name: "argmin".to_string(), arity: 1 });
        
        // Matrix operations (6 functions)
        self.define("matrix_add".to_string(), Value::NativeFunction { name: "matrix_add".to_string(), arity: 2 });
        self.define("matrix_multiply".to_string(), Value::NativeFunction { name: "matrix_multiply".to_string(), arity: 2 });
        self.define("matrix_transpose".to_string(), Value::NativeFunction { name: "matrix_transpose".to_string(), arity: 1 });
        self.define("matrix_determinant".to_string(), Value::NativeFunction { name: "matrix_determinant".to_string(), arity: 1 });
        self.define("matrix_identity".to_string(), Value::NativeFunction { name: "matrix_identity".to_string(), arity: 1 });
        self.define("matrix_trace".to_string(), Value::NativeFunction { name: "matrix_trace".to_string(), arity: 1 });
        
        // Set operations (8 functions)
        self.define("set_union".to_string(), Value::NativeFunction { name: "set_union".to_string(), arity: 2 });
        self.define("set_intersection".to_string(), Value::NativeFunction { name: "set_intersection".to_string(), arity: 2 });
        self.define("set_difference".to_string(), Value::NativeFunction { name: "set_difference".to_string(), arity: 2 });
        self.define("set_symmetric_difference".to_string(), Value::NativeFunction { name: "set_symmetric_difference".to_string(), arity: 2 });
        self.define("is_subset".to_string(), Value::NativeFunction { name: "is_subset".to_string(), arity: 2 });
        self.define("is_superset".to_string(), Value::NativeFunction { name: "is_superset".to_string(), arity: 2 });
        self.define("is_disjoint".to_string(), Value::NativeFunction { name: "is_disjoint".to_string(), arity: 2 });
        self.define("powerset".to_string(), Value::NativeFunction { name: "powerset".to_string(), arity: 1 });
        
        // Functional programming (7 functions)
        self.define("reduce".to_string(), Value::NativeFunction { name: "reduce".to_string(), arity: 2 });
        self.define("fold_left".to_string(), Value::NativeFunction { name: "fold_left".to_string(), arity: 2 });
        self.define("scan".to_string(), Value::NativeFunction { name: "scan".to_string(), arity: 1 });
        self.define("take_while".to_string(), Value::NativeFunction { name: "take_while".to_string(), arity: 2 });
        self.define("drop_while".to_string(), Value::NativeFunction { name: "drop_while".to_string(), arity: 2 });
        self.define("partition_by".to_string(), Value::NativeFunction { name: "partition_by".to_string(), arity: 2 });
        self.define("group_consecutive".to_string(), Value::NativeFunction { name: "group_consecutive".to_string(), arity: 1 });
        
        // Number theory (8 functions)
        self.define("totient".to_string(), Value::NativeFunction { name: "totient".to_string(), arity: 1 });
        self.define("is_coprime".to_string(), Value::NativeFunction { name: "is_coprime".to_string(), arity: 2 });
        self.define("nth_fibonacci".to_string(), Value::NativeFunction { name: "nth_fibonacci".to_string(), arity: 1 });
        self.define("lucas_number".to_string(), Value::NativeFunction { name: "lucas_number".to_string(), arity: 1 });
        self.define("catalan_number".to_string(), Value::NativeFunction { name: "catalan_number".to_string(), arity: 1 });
        self.define("triangular_number".to_string(), Value::NativeFunction { name: "triangular_number".to_string(), arity: 1 });
        self.define("pentagonal_number".to_string(), Value::NativeFunction { name: "pentagonal_number".to_string(), arity: 1 });
        self.define("hexagonal_number".to_string(), Value::NativeFunction { name: "hexagonal_number".to_string(), arity: 1 });
        
        // Cryptography & Hashing (5 functions)
        self.define("hash_string".to_string(), Value::NativeFunction { name: "hash_string".to_string(), arity: 1 });
        self.define("checksum".to_string(), Value::NativeFunction { name: "checksum".to_string(), arity: 1 });
        self.define("crc32".to_string(), Value::NativeFunction { name: "crc32".to_string(), arity: 1 });
        self.define("adler32".to_string(), Value::NativeFunction { name: "adler32".to_string(), arity: 1 });
        self.define("fnv1a".to_string(), Value::NativeFunction { name: "fnv1a".to_string(), arity: 1 });
        
        // Graph & Tree operations (5 functions)
        self.define("graph_nodes".to_string(), Value::NativeFunction { name: "graph_nodes".to_string(), arity: 1 });
        self.define("graph_edges".to_string(), Value::NativeFunction { name: "graph_edges".to_string(), arity: 1 });
        self.define("tree_height".to_string(), Value::NativeFunction { name: "tree_height".to_string(), arity: 1 });
        self.define("tree_size".to_string(), Value::NativeFunction { name: "tree_size".to_string(), arity: 1 });
        self.define("tree_leaves".to_string(), Value::NativeFunction { name: "tree_leaves".to_string(), arity: 1 });
        
        // Sorting & Searching (5 functions)
        self.define("binary_search".to_string(), Value::NativeFunction { name: "binary_search".to_string(), arity: 2 });
        self.define("linear_search".to_string(), Value::NativeFunction { name: "linear_search".to_string(), arity: 2 });
        self.define("sort_ascending".to_string(), Value::NativeFunction { name: "sort_ascending".to_string(), arity: 1 });
        self.define("sort_descending".to_string(), Value::NativeFunction { name: "sort_descending".to_string(), arity: 1 });
        self.define("is_sorted".to_string(), Value::NativeFunction { name: "is_sorted".to_string(), arity: 1 });
        
        // Physics & Science (10 functions)
        self.define("velocity".to_string(), Value::NativeFunction { name: "velocity".to_string(), arity: 2 });
        self.define("acceleration".to_string(), Value::NativeFunction { name: "acceleration".to_string(), arity: 2 });
        self.define("force".to_string(), Value::NativeFunction { name: "force".to_string(), arity: 2 });
        self.define("kinetic_energy".to_string(), Value::NativeFunction { name: "kinetic_energy".to_string(), arity: 2 });
        self.define("potential_energy".to_string(), Value::NativeFunction { name: "potential_energy".to_string(), arity: 2 });
        self.define("momentum".to_string(), Value::NativeFunction { name: "momentum".to_string(), arity: 2 });
        self.define("work".to_string(), Value::NativeFunction { name: "work".to_string(), arity: 2 });
        self.define("power".to_string(), Value::NativeFunction { name: "power".to_string(), arity: 2 });
        self.define("pressure".to_string(), Value::NativeFunction { name: "pressure".to_string(), arity: 2 });
        self.define("density".to_string(), Value::NativeFunction { name: "density".to_string(), arity: 2 });
        
        // Financial & Economics (7 functions)
        self.define("simple_interest".to_string(), Value::NativeFunction { name: "simple_interest".to_string(), arity: 3 });
        self.define("compound_interest".to_string(), Value::NativeFunction { name: "compound_interest".to_string(), arity: 4 });
        self.define("future_value".to_string(), Value::NativeFunction { name: "future_value".to_string(), arity: 3 });
        self.define("present_value".to_string(), Value::NativeFunction { name: "present_value".to_string(), arity: 3 });
        self.define("loan_payment".to_string(), Value::NativeFunction { name: "loan_payment".to_string(), arity: 3 });
        self.define("roi".to_string(), Value::NativeFunction { name: "roi".to_string(), arity: 2 });
        self.define("profit_margin".to_string(), Value::NativeFunction { name: "profit_margin".to_string(), arity: 2 });
        
        // Geometry 3D (7 functions)
        self.define("distance3d".to_string(), Value::NativeFunction { name: "distance3d".to_string(), arity: 6 });
        self.define("sphere_volume".to_string(), Value::NativeFunction { name: "sphere_volume".to_string(), arity: 1 });
        self.define("sphere_surface".to_string(), Value::NativeFunction { name: "sphere_surface".to_string(), arity: 1 });
        self.define("cube_volume".to_string(), Value::NativeFunction { name: "cube_volume".to_string(), arity: 1 });
        self.define("cylinder_volume".to_string(), Value::NativeFunction { name: "cylinder_volume".to_string(), arity: 2 });
        self.define("cone_volume".to_string(), Value::NativeFunction { name: "cone_volume".to_string(), arity: 2 });
        self.define("pyramid_volume".to_string(), Value::NativeFunction { name: "pyramid_volume".to_string(), arity: 2 });
        
        // Trigonometry Advanced (6 functions)
        self.define("sec".to_string(), Value::NativeFunction { name: "sec".to_string(), arity: 1 });
        self.define("csc".to_string(), Value::NativeFunction { name: "csc".to_string(), arity: 1 });
        self.define("cot".to_string(), Value::NativeFunction { name: "cot".to_string(), arity: 1 });
        self.define("asec".to_string(), Value::NativeFunction { name: "asec".to_string(), arity: 1 });
        self.define("acsc".to_string(), Value::NativeFunction { name: "acsc".to_string(), arity: 1 });
        self.define("acot".to_string(), Value::NativeFunction { name: "acot".to_string(), arity: 1 });
        
        // Statistics Advanced (5 functions)
        self.define("range_stat".to_string(), Value::NativeFunction { name: "range_stat".to_string(), arity: 1 });
        self.define("iqr".to_string(), Value::NativeFunction { name: "iqr".to_string(), arity: 1 });
        self.define("skewness".to_string(), Value::NativeFunction { name: "skewness".to_string(), arity: 1 });
        self.define("kurtosis".to_string(), Value::NativeFunction { name: "kurtosis".to_string(), arity: 1 });
        self.define("zscore".to_string(), Value::NativeFunction { name: "zscore".to_string(), arity: 3 });
        
        // Probability (3 functions)
        self.define("probability".to_string(), Value::NativeFunction { name: "probability".to_string(), arity: 2 });
        self.define("odds".to_string(), Value::NativeFunction { name: "odds".to_string(), arity: 2 });
        self.define("expected_value".to_string(), Value::NativeFunction { name: "expected_value".to_string(), arity: 2 });
        
        // Chemistry (4 functions)
        self.define("molar_mass".to_string(), Value::NativeFunction { name: "molar_mass".to_string(), arity: 1 });
        self.define("molarity".to_string(), Value::NativeFunction { name: "molarity".to_string(), arity: 2 });
        self.define("ph_to_h".to_string(), Value::NativeFunction { name: "ph_to_h".to_string(), arity: 1 });
        self.define("h_to_ph".to_string(), Value::NativeFunction { name: "h_to_ph".to_string(), arity: 1 });
        
        // Computer Science (4 functions)
        self.define("hamming_distance".to_string(), Value::NativeFunction { name: "hamming_distance".to_string(), arity: 2 });
        self.define("edit_distance".to_string(), Value::NativeFunction { name: "edit_distance".to_string(), arity: 2 });
        self.define("lcs_length".to_string(), Value::NativeFunction { name: "lcs_length".to_string(), arity: 2 });
        self.define("is_anagram".to_string(), Value::NativeFunction { name: "is_anagram".to_string(), arity: 2 });
        
        // Data Structures (6 functions)
        self.define("stack_push".to_string(), Value::NativeFunction { name: "stack_push".to_string(), arity: 2 });
        self.define("stack_pop".to_string(), Value::NativeFunction { name: "stack_pop".to_string(), arity: 1 });
        self.define("stack_peek".to_string(), Value::NativeFunction { name: "stack_peek".to_string(), arity: 1 });
        self.define("queue_enqueue".to_string(), Value::NativeFunction { name: "queue_enqueue".to_string(), arity: 2 });
        self.define("queue_dequeue".to_string(), Value::NativeFunction { name: "queue_dequeue".to_string(), arity: 1 });
        self.define("priority_queue_insert".to_string(), Value::NativeFunction { name: "priority_queue_insert".to_string(), arity: 2 });
        
        // Algorithms (6 functions)
        self.define("bubble_sort".to_string(), Value::NativeFunction { name: "bubble_sort".to_string(), arity: 1 });
        self.define("selection_sort".to_string(), Value::NativeFunction { name: "selection_sort".to_string(), arity: 1 });
        self.define("insertion_sort".to_string(), Value::NativeFunction { name: "insertion_sort".to_string(), arity: 1 });
        self.define("merge_sort".to_string(), Value::NativeFunction { name: "merge_sort".to_string(), arity: 1 });
        self.define("quick_sort".to_string(), Value::NativeFunction { name: "quick_sort".to_string(), arity: 1 });
        self.define("heap_sort".to_string(), Value::NativeFunction { name: "heap_sort".to_string(), arity: 1 });
        
        // UI/Graphics/Rendering (40+ functions)
        self.define("window_create".to_string(), Value::NativeFunction { name: "window_create".to_string(), arity: 4 });
        self.define("window_close".to_string(), Value::NativeFunction { name: "window_close".to_string(), arity: 1 });
        self.define("window_show".to_string(), Value::NativeFunction { name: "window_show".to_string(), arity: 1 });
        self.define("window_hide".to_string(), Value::NativeFunction { name: "window_hide".to_string(), arity: 1 });
        self.define("window_resize".to_string(), Value::NativeFunction { name: "window_resize".to_string(), arity: 3 });
        self.define("window_move".to_string(), Value::NativeFunction { name: "window_move".to_string(), arity: 3 });
        self.define("window_title".to_string(), Value::NativeFunction { name: "window_title".to_string(), arity: 2 });
        self.define("window_set_theme".to_string(), Value::NativeFunction { name: "window_set_theme".to_string(), arity: 0 });
        self.define("canvas_create".to_string(), Value::NativeFunction { name: "canvas_create".to_string(), arity: 2 });
        self.define("canvas_create_themed".to_string(), Value::NativeFunction { name: "canvas_create_themed".to_string(), arity: 3 });
        self.define("canvas_set_theme".to_string(), Value::NativeFunction { name: "canvas_set_theme".to_string(), arity: 0 });
        self.define("canvas_set_alpha".to_string(), Value::NativeFunction { name: "canvas_set_alpha".to_string(), arity: 2 });
        self.define("canvas_clear_transparent".to_string(), Value::NativeFunction { name: "canvas_clear_transparent".to_string(), arity: 1 });
        self.define("canvas_clear".to_string(), Value::NativeFunction { name: "canvas_clear".to_string(), arity: 1 });
        self.define("draw_pixel".to_string(), Value::NativeFunction { name: "draw_pixel".to_string(), arity: 4 });
        self.define("draw_line".to_string(), Value::NativeFunction { name: "draw_line".to_string(), arity: 6 });
        self.define("draw_rect".to_string(), Value::NativeFunction { name: "draw_rect".to_string(), arity: 6 });
        self.define("draw_circle".to_string(), Value::NativeFunction { name: "draw_circle".to_string(), arity: 5 });
        self.define("draw_ellipse".to_string(), Value::NativeFunction { name: "draw_ellipse".to_string(), arity: 6 });
        self.define("draw_polygon".to_string(), Value::NativeFunction { name: "draw_polygon".to_string(), arity: 3 });
        self.define("draw_text".to_string(), Value::NativeFunction { name: "draw_text".to_string(), arity: 5 });
        self.define("draw_image".to_string(), Value::NativeFunction { name: "draw_image".to_string(), arity: 4 });
        self.define("set_color".to_string(), Value::NativeFunction { name: "set_color".to_string(), arity: 4 });
        self.define("set_font".to_string(), Value::NativeFunction { name: "set_font".to_string(), arity: 2 });
        self.define("set_line_width".to_string(), Value::NativeFunction { name: "set_line_width".to_string(), arity: 1 });
        self.define("fill_rect".to_string(), Value::NativeFunction { name: "fill_rect".to_string(), arity: 5 });
        self.define("fill_circle".to_string(), Value::NativeFunction { name: "fill_circle".to_string(), arity: 4 });
        self.define("fill_polygon".to_string(), Value::NativeFunction { name: "fill_polygon".to_string(), arity: 2 });
        self.define("gradient_linear".to_string(), Value::NativeFunction { name: "gradient_linear".to_string(), arity: 6 });
        self.define("gradient_radial".to_string(), Value::NativeFunction { name: "gradient_radial".to_string(), arity: 5 });
        self.define("rotate_canvas".to_string(), Value::NativeFunction { name: "rotate_canvas".to_string(), arity: 2 });
        self.define("scale_canvas".to_string(), Value::NativeFunction { name: "scale_canvas".to_string(), arity: 3 });
        self.define("translate_canvas".to_string(), Value::NativeFunction { name: "translate_canvas".to_string(), arity: 3 });
        self.define("save_canvas".to_string(), Value::NativeFunction { name: "save_canvas".to_string(), arity: 1 });
        self.define("restore_canvas".to_string(), Value::NativeFunction { name: "restore_canvas".to_string(), arity: 1 });
        self.define("clip_rect".to_string(), Value::NativeFunction { name: "clip_rect".to_string(), arity: 5 });
        self.define("button_create".to_string(), Value::NativeFunction { name: "button_create".to_string(), arity: 5 });
        self.define("label_create".to_string(), Value::NativeFunction { name: "label_create".to_string(), arity: 4 });
        self.define("textbox_create".to_string(), Value::NativeFunction { name: "textbox_create".to_string(), arity: 5 });
        self.define("checkbox_create".to_string(), Value::NativeFunction { name: "checkbox_create".to_string(), arity: 5 });
        self.define("slider_create".to_string(), Value::NativeFunction { name: "slider_create".to_string(), arity: 7 });
        self.define("dropdown_create".to_string(), Value::NativeFunction { name: "dropdown_create".to_string(), arity: 5 });
        self.define("menu_create".to_string(), Value::NativeFunction { name: "menu_create".to_string(), arity: 1 });
        self.define("menu_add_item".to_string(), Value::NativeFunction { name: "menu_add_item".to_string(), arity: 3 });
        self.define("dialog_open".to_string(), Value::NativeFunction { name: "dialog_open".to_string(), arity: 2 });
        self.define("dialog_save".to_string(), Value::NativeFunction { name: "dialog_save".to_string(), arity: 2 });
        self.define("dialog_message".to_string(), Value::NativeFunction { name: "dialog_message".to_string(), arity: 2 });
        self.define("event_poll".to_string(), Value::NativeFunction { name: "event_poll".to_string(), arity: 0 });
        self.define("event_wait".to_string(), Value::NativeFunction { name: "event_wait".to_string(), arity: 0 });
        
        // Audio/Sound (25+ functions)
        self.define("audio_init".to_string(), Value::NativeFunction { name: "audio_init".to_string(), arity: 0 });
        self.define("audio_load".to_string(), Value::NativeFunction { name: "audio_load".to_string(), arity: 1 });
        self.define("audio_play".to_string(), Value::NativeFunction { name: "audio_play".to_string(), arity: 1 });
        self.define("audio_pause".to_string(), Value::NativeFunction { name: "audio_pause".to_string(), arity: 1 });
        self.define("audio_stop".to_string(), Value::NativeFunction { name: "audio_stop".to_string(), arity: 1 });
        self.define("audio_volume".to_string(), Value::NativeFunction { name: "audio_volume".to_string(), arity: 2 });
        self.define("audio_loop".to_string(), Value::NativeFunction { name: "audio_loop".to_string(), arity: 2 });
        self.define("audio_position".to_string(), Value::NativeFunction { name: "audio_position".to_string(), arity: 1 });
        self.define("audio_duration".to_string(), Value::NativeFunction { name: "audio_duration".to_string(), arity: 1 });
        self.define("audio_seek".to_string(), Value::NativeFunction { name: "audio_seek".to_string(), arity: 2 });
        self.define("audio_fade_in".to_string(), Value::NativeFunction { name: "audio_fade_in".to_string(), arity: 2 });
        self.define("audio_fade_out".to_string(), Value::NativeFunction { name: "audio_fade_out".to_string(), arity: 2 });
        self.define("audio_pitch".to_string(), Value::NativeFunction { name: "audio_pitch".to_string(), arity: 2 });
        self.define("audio_speed".to_string(), Value::NativeFunction { name: "audio_speed".to_string(), arity: 2 });
        self.define("audio_pan".to_string(), Value::NativeFunction { name: "audio_pan".to_string(), arity: 2 });
        self.define("audio_reverb".to_string(), Value::NativeFunction { name: "audio_reverb".to_string(), arity: 2 });
        self.define("audio_echo".to_string(), Value::NativeFunction { name: "audio_echo".to_string(), arity: 3 });
        self.define("audio_equalizer".to_string(), Value::NativeFunction { name: "audio_equalizer".to_string(), arity: 2 });
        self.define("audio_record".to_string(), Value::NativeFunction { name: "audio_record".to_string(), arity: 1 });
        self.define("audio_record_stop".to_string(), Value::NativeFunction { name: "audio_record_stop".to_string(), arity: 0 });
        self.define("audio_mix".to_string(), Value::NativeFunction { name: "audio_mix".to_string(), arity: 2 });
        self.define("audio_generate_tone".to_string(), Value::NativeFunction { name: "audio_generate_tone".to_string(), arity: 3 });
        self.define("audio_generate_noise".to_string(), Value::NativeFunction { name: "audio_generate_noise".to_string(), arity: 1 });
        self.define("audio_fft".to_string(), Value::NativeFunction { name: "audio_fft".to_string(), arity: 1 });
        self.define("audio_spectrum".to_string(), Value::NativeFunction { name: "audio_spectrum".to_string(), arity: 1 });
        
        // Networking/HTTP (30+ functions)
        self.define("http_get".to_string(), Value::NativeFunction { name: "http_get".to_string(), arity: 1 });
        self.define("http_post".to_string(), Value::NativeFunction { name: "http_post".to_string(), arity: 2 });
        self.define("http_put".to_string(), Value::NativeFunction { name: "http_put".to_string(), arity: 2 });
        self.define("http_delete".to_string(), Value::NativeFunction { name: "http_delete".to_string(), arity: 1 });
        self.define("http_patch".to_string(), Value::NativeFunction { name: "http_patch".to_string(), arity: 2 });
        self.define("http_head".to_string(), Value::NativeFunction { name: "http_head".to_string(), arity: 1 });
        self.define("http_options".to_string(), Value::NativeFunction { name: "http_options".to_string(), arity: 1 });
        self.define("http_set_header".to_string(), Value::NativeFunction { name: "http_set_header".to_string(), arity: 2 });
        self.define("http_set_timeout".to_string(), Value::NativeFunction { name: "http_set_timeout".to_string(), arity: 1 });
        self.define("http_download".to_string(), Value::NativeFunction { name: "http_download".to_string(), arity: 2 });
        self.define("http_upload".to_string(), Value::NativeFunction { name: "http_upload".to_string(), arity: 2 });
        self.define("socket_create".to_string(), Value::NativeFunction { name: "socket_create".to_string(), arity: 2 });
        self.define("socket_connect".to_string(), Value::NativeFunction { name: "socket_connect".to_string(), arity: 1 });
        self.define("socket_send".to_string(), Value::NativeFunction { name: "socket_send".to_string(), arity: 2 });
        self.define("socket_receive".to_string(), Value::NativeFunction { name: "socket_receive".to_string(), arity: 1 });
        self.define("socket_close".to_string(), Value::NativeFunction { name: "socket_close".to_string(), arity: 1 });
        self.define("socket_listen".to_string(), Value::NativeFunction { name: "socket_listen".to_string(), arity: 2 });
        self.define("socket_accept".to_string(), Value::NativeFunction { name: "socket_accept".to_string(), arity: 1 });
        self.define("websocket_connect".to_string(), Value::NativeFunction { name: "websocket_connect".to_string(), arity: 1 });
        self.define("websocket_send".to_string(), Value::NativeFunction { name: "websocket_send".to_string(), arity: 2 });
        self.define("websocket_receive".to_string(), Value::NativeFunction { name: "websocket_receive".to_string(), arity: 1 });
        self.define("websocket_close".to_string(), Value::NativeFunction { name: "websocket_close".to_string(), arity: 1 });
        self.define("ftp_connect".to_string(), Value::NativeFunction { name: "ftp_connect".to_string(), arity: 3 });
        self.define("ftp_upload".to_string(), Value::NativeFunction { name: "ftp_upload".to_string(), arity: 3 });
        self.define("ftp_download".to_string(), Value::NativeFunction { name: "ftp_download".to_string(), arity: 3 });
        self.define("ftp_list".to_string(), Value::NativeFunction { name: "ftp_list".to_string(), arity: 2 });
        self.define("ftp_delete".to_string(), Value::NativeFunction { name: "ftp_delete".to_string(), arity: 2 });
        self.define("smtp_send_email".to_string(), Value::NativeFunction { name: "smtp_send_email".to_string(), arity: 5 });
        self.define("dns_lookup".to_string(), Value::NativeFunction { name: "dns_lookup".to_string(), arity: 1 });
        self.define("ping".to_string(), Value::NativeFunction { name: "ping".to_string(), arity: 1 });
        
        // Database (20+ functions)
        self.define("db_connect".to_string(), Value::NativeFunction { name: "db_connect".to_string(), arity: 1 });
        self.define("db_close".to_string(), Value::NativeFunction { name: "db_close".to_string(), arity: 1 });
        self.define("db_query".to_string(), Value::NativeFunction { name: "db_query".to_string(), arity: 2 });
        self.define("db_execute".to_string(), Value::NativeFunction { name: "db_execute".to_string(), arity: 2 });
        self.define("db_insert".to_string(), Value::NativeFunction { name: "db_insert".to_string(), arity: 3 });
        self.define("db_update".to_string(), Value::NativeFunction { name: "db_update".to_string(), arity: 4 });
        self.define("db_delete".to_string(), Value::NativeFunction { name: "db_delete".to_string(), arity: 3 });
        self.define("db_select".to_string(), Value::NativeFunction { name: "db_select".to_string(), arity: 2 });
        self.define("db_create_table".to_string(), Value::NativeFunction { name: "db_create_table".to_string(), arity: 3 });
        self.define("db_drop_table".to_string(), Value::NativeFunction { name: "db_drop_table".to_string(), arity: 2 });
        self.define("db_begin_transaction".to_string(), Value::NativeFunction { name: "db_begin_transaction".to_string(), arity: 1 });
        self.define("db_commit".to_string(), Value::NativeFunction { name: "db_commit".to_string(), arity: 1 });
        self.define("db_rollback".to_string(), Value::NativeFunction { name: "db_rollback".to_string(), arity: 1 });
        self.define("db_prepare".to_string(), Value::NativeFunction { name: "db_prepare".to_string(), arity: 2 });
        self.define("db_bind".to_string(), Value::NativeFunction { name: "db_bind".to_string(), arity: 3 });
        self.define("db_fetch_one".to_string(), Value::NativeFunction { name: "db_fetch_one".to_string(), arity: 2 });
        self.define("db_fetch_all".to_string(), Value::NativeFunction { name: "db_fetch_all".to_string(), arity: 2 });
        self.define("db_count".to_string(), Value::NativeFunction { name: "db_count".to_string(), arity: 2 });
        self.define("db_exists".to_string(), Value::NativeFunction { name: "db_exists".to_string(), arity: 3 });
        self.define("db_last_insert_id".to_string(), Value::NativeFunction { name: "db_last_insert_id".to_string(), arity: 1 });
        
        // System/OS (25+ functions)
        self.define("sys_exec".to_string(), Value::NativeFunction { name: "sys_exec".to_string(), arity: 1 });
        self.define("sys_spawn".to_string(), Value::NativeFunction { name: "sys_spawn".to_string(), arity: 1 });
        self.define("sys_kill".to_string(), Value::NativeFunction { name: "sys_kill".to_string(), arity: 1 });
        self.define("sys_getenv".to_string(), Value::NativeFunction { name: "sys_getenv".to_string(), arity: 1 });
        self.define("sys_setenv".to_string(), Value::NativeFunction { name: "sys_setenv".to_string(), arity: 2 });
        self.define("sys_platform".to_string(), Value::NativeFunction { name: "sys_platform".to_string(), arity: 0 });
        self.define("sys_arch".to_string(), Value::NativeFunction { name: "sys_arch".to_string(), arity: 0 });
        self.define("sys_hostname".to_string(), Value::NativeFunction { name: "sys_hostname".to_string(), arity: 0 });
        self.define("sys_username".to_string(), Value::NativeFunction { name: "sys_username".to_string(), arity: 0 });
        self.define("sys_pid".to_string(), Value::NativeFunction { name: "sys_pid".to_string(), arity: 0 });
        self.define("sys_uptime".to_string(), Value::NativeFunction { name: "sys_uptime".to_string(), arity: 0 });
        self.define("sys_cpu_count".to_string(), Value::NativeFunction { name: "sys_cpu_count".to_string(), arity: 0 });
        self.define("sys_cpu_usage".to_string(), Value::NativeFunction { name: "sys_cpu_usage".to_string(), arity: 0 });
        self.define("sys_memory_total".to_string(), Value::NativeFunction { name: "sys_memory_total".to_string(), arity: 0 });
        self.define("sys_memory_used".to_string(), Value::NativeFunction { name: "sys_memory_used".to_string(), arity: 0 });
        self.define("sys_memory_free".to_string(), Value::NativeFunction { name: "sys_memory_free".to_string(), arity: 0 });
        self.define("sys_disk_total".to_string(), Value::NativeFunction { name: "sys_disk_total".to_string(), arity: 1 });
        self.define("sys_disk_used".to_string(), Value::NativeFunction { name: "sys_disk_used".to_string(), arity: 1 });
        self.define("sys_disk_free".to_string(), Value::NativeFunction { name: "sys_disk_free".to_string(), arity: 1 });
        self.define("sys_network_interfaces".to_string(), Value::NativeFunction { name: "sys_network_interfaces".to_string(), arity: 0 });
        self.define("sys_battery_level".to_string(), Value::NativeFunction { name: "sys_battery_level".to_string(), arity: 0 });
        self.define("sys_battery_charging".to_string(), Value::NativeFunction { name: "sys_battery_charging".to_string(), arity: 0 });
        self.define("sys_clipboard_get".to_string(), Value::NativeFunction { name: "sys_clipboard_get".to_string(), arity: 0 });
        self.define("sys_clipboard_set".to_string(), Value::NativeFunction { name: "sys_clipboard_set".to_string(), arity: 1 });
        self.define("sys_beep".to_string(), Value::NativeFunction { name: "sys_beep".to_string(), arity: 0 });
        
        // Image Processing (30+ functions)
        self.define("image_load".to_string(), Value::NativeFunction { name: "image_load".to_string(), arity: 1 });
        self.define("image_save".to_string(), Value::NativeFunction { name: "image_save".to_string(), arity: 2 });
        self.define("image_create".to_string(), Value::NativeFunction { name: "image_create".to_string(), arity: 2 });
        self.define("image_width".to_string(), Value::NativeFunction { name: "image_width".to_string(), arity: 1 });
        self.define("image_height".to_string(), Value::NativeFunction { name: "image_height".to_string(), arity: 1 });
        self.define("image_resize".to_string(), Value::NativeFunction { name: "image_resize".to_string(), arity: 3 });
        self.define("image_crop".to_string(), Value::NativeFunction { name: "image_crop".to_string(), arity: 5 });
        self.define("image_rotate".to_string(), Value::NativeFunction { name: "image_rotate".to_string(), arity: 2 });
        self.define("image_flip_h".to_string(), Value::NativeFunction { name: "image_flip_h".to_string(), arity: 1 });
        self.define("image_flip_v".to_string(), Value::NativeFunction { name: "image_flip_v".to_string(), arity: 1 });
        self.define("image_grayscale".to_string(), Value::NativeFunction { name: "image_grayscale".to_string(), arity: 1 });
        self.define("image_blur".to_string(), Value::NativeFunction { name: "image_blur".to_string(), arity: 2 });
        self.define("image_sharpen".to_string(), Value::NativeFunction { name: "image_sharpen".to_string(), arity: 1 });
        self.define("image_brightness".to_string(), Value::NativeFunction { name: "image_brightness".to_string(), arity: 2 });
        self.define("image_contrast".to_string(), Value::NativeFunction { name: "image_contrast".to_string(), arity: 2 });
        self.define("image_saturation".to_string(), Value::NativeFunction { name: "image_saturation".to_string(), arity: 2 });
        self.define("image_hue".to_string(), Value::NativeFunction { name: "image_hue".to_string(), arity: 2 });
        self.define("image_invert".to_string(), Value::NativeFunction { name: "image_invert".to_string(), arity: 1 });
        self.define("image_sepia".to_string(), Value::NativeFunction { name: "image_sepia".to_string(), arity: 1 });
        self.define("image_threshold".to_string(), Value::NativeFunction { name: "image_threshold".to_string(), arity: 2 });
        self.define("image_edge_detect".to_string(), Value::NativeFunction { name: "image_edge_detect".to_string(), arity: 1 });
        self.define("image_emboss".to_string(), Value::NativeFunction { name: "image_emboss".to_string(), arity: 1 });
        self.define("image_pixelate".to_string(), Value::NativeFunction { name: "image_pixelate".to_string(), arity: 2 });
        self.define("image_get_pixel".to_string(), Value::NativeFunction { name: "image_get_pixel".to_string(), arity: 3 });
        self.define("image_set_pixel".to_string(), Value::NativeFunction { name: "image_set_pixel".to_string(), arity: 5 });
        self.define("image_blend".to_string(), Value::NativeFunction { name: "image_blend".to_string(), arity: 3 });
        self.define("image_overlay".to_string(), Value::NativeFunction { name: "image_overlay".to_string(), arity: 4 });
        self.define("image_histogram".to_string(), Value::NativeFunction { name: "image_histogram".to_string(), arity: 1 });
        self.define("image_equalize".to_string(), Value::NativeFunction { name: "image_equalize".to_string(), arity: 1 });
        self.define("image_denoise".to_string(), Value::NativeFunction { name: "image_denoise".to_string(), arity: 1 });
        
        // Video Processing (15+ functions)
        self.define("video_load".to_string(), Value::NativeFunction { name: "video_load".to_string(), arity: 1 });
        self.define("video_save".to_string(), Value::NativeFunction { name: "video_save".to_string(), arity: 2 });
        self.define("video_duration".to_string(), Value::NativeFunction { name: "video_duration".to_string(), arity: 1 });
        self.define("video_fps".to_string(), Value::NativeFunction { name: "video_fps".to_string(), arity: 1 });
        self.define("video_frame_count".to_string(), Value::NativeFunction { name: "video_frame_count".to_string(), arity: 1 });
        self.define("video_get_frame".to_string(), Value::NativeFunction { name: "video_get_frame".to_string(), arity: 2 });
        self.define("video_set_frame".to_string(), Value::NativeFunction { name: "video_set_frame".to_string(), arity: 3 });
        self.define("video_extract_audio".to_string(), Value::NativeFunction { name: "video_extract_audio".to_string(), arity: 1 });
        self.define("video_add_audio".to_string(), Value::NativeFunction { name: "video_add_audio".to_string(), arity: 2 });
        self.define("video_trim".to_string(), Value::NativeFunction { name: "video_trim".to_string(), arity: 3 });
        self.define("video_concat".to_string(), Value::NativeFunction { name: "video_concat".to_string(), arity: 2 });
        self.define("video_resize".to_string(), Value::NativeFunction { name: "video_resize".to_string(), arity: 3 });
        self.define("video_rotate".to_string(), Value::NativeFunction { name: "video_rotate".to_string(), arity: 2 });
        self.define("video_speed".to_string(), Value::NativeFunction { name: "video_speed".to_string(), arity: 2 });
        self.define("video_reverse".to_string(), Value::NativeFunction { name: "video_reverse".to_string(), arity: 1 });
        
        // Animation (15+ functions)
        self.define("anim_create".to_string(), Value::NativeFunction { name: "anim_create".to_string(), arity: 2 });
        self.define("anim_add_frame".to_string(), Value::NativeFunction { name: "anim_add_frame".to_string(), arity: 2 });
        self.define("anim_play".to_string(), Value::NativeFunction { name: "anim_play".to_string(), arity: 1 });
        self.define("anim_pause".to_string(), Value::NativeFunction { name: "anim_pause".to_string(), arity: 1 });
        self.define("anim_stop".to_string(), Value::NativeFunction { name: "anim_stop".to_string(), arity: 1 });
        self.define("anim_loop".to_string(), Value::NativeFunction { name: "anim_loop".to_string(), arity: 2 });
        self.define("anim_speed".to_string(), Value::NativeFunction { name: "anim_speed".to_string(), arity: 2 });
        self.define("tween_linear".to_string(), Value::NativeFunction { name: "tween_linear".to_string(), arity: 3 });
        self.define("tween_ease_in".to_string(), Value::NativeFunction { name: "tween_ease_in".to_string(), arity: 3 });
        self.define("tween_ease_out".to_string(), Value::NativeFunction { name: "tween_ease_out".to_string(), arity: 3 });
        self.define("tween_ease_in_out".to_string(), Value::NativeFunction { name: "tween_ease_in_out".to_string(), arity: 3 });
        self.define("tween_bounce".to_string(), Value::NativeFunction { name: "tween_bounce".to_string(), arity: 3 });
        self.define("tween_elastic".to_string(), Value::NativeFunction { name: "tween_elastic".to_string(), arity: 3 });
        self.define("sprite_create".to_string(), Value::NativeFunction { name: "sprite_create".to_string(), arity: 2 });
        self.define("sprite_animate".to_string(), Value::NativeFunction { name: "sprite_animate".to_string(), arity: 2 });
        
        // JSON/XML/Data Formats (20+ functions)
        self.define("json_parse".to_string(), Value::NativeFunction { name: "json_parse".to_string(), arity: 1 });
        self.define("json_stringify".to_string(), Value::NativeFunction { name: "json_stringify".to_string(), arity: 1 });
        self.define("json_get".to_string(), Value::NativeFunction { name: "json_get".to_string(), arity: 2 });
        self.define("json_set".to_string(), Value::NativeFunction { name: "json_set".to_string(), arity: 3 });
        self.define("json_has".to_string(), Value::NativeFunction { name: "json_has".to_string(), arity: 2 });
        self.define("json_keys".to_string(), Value::NativeFunction { name: "json_keys".to_string(), arity: 1 });
        self.define("json_values".to_string(), Value::NativeFunction { name: "json_values".to_string(), arity: 1 });
        self.define("xml_parse".to_string(), Value::NativeFunction { name: "xml_parse".to_string(), arity: 1 });
        self.define("xml_stringify".to_string(), Value::NativeFunction { name: "xml_stringify".to_string(), arity: 1 });
        self.define("xml_get".to_string(), Value::NativeFunction { name: "xml_get".to_string(), arity: 2 });
        self.define("xml_set".to_string(), Value::NativeFunction { name: "xml_set".to_string(), arity: 3 });
        self.define("yaml_parse".to_string(), Value::NativeFunction { name: "yaml_parse".to_string(), arity: 1 });
        self.define("yaml_stringify".to_string(), Value::NativeFunction { name: "yaml_stringify".to_string(), arity: 1 });
        self.define("csv_parse".to_string(), Value::NativeFunction { name: "csv_parse".to_string(), arity: 1 });
        self.define("csv_stringify".to_string(), Value::NativeFunction { name: "csv_stringify".to_string(), arity: 1 });
        self.define("toml_parse".to_string(), Value::NativeFunction { name: "toml_parse".to_string(), arity: 1 });
        self.define("toml_stringify".to_string(), Value::NativeFunction { name: "toml_stringify".to_string(), arity: 1 });
        self.define("ini_parse".to_string(), Value::NativeFunction { name: "ini_parse".to_string(), arity: 1 });
        self.define("ini_stringify".to_string(), Value::NativeFunction { name: "ini_stringify".to_string(), arity: 1 });
        self.define("msgpack_encode".to_string(), Value::NativeFunction { name: "msgpack_encode".to_string(), arity: 1 });
        self.define("msgpack_decode".to_string(), Value::NativeFunction { name: "msgpack_decode".to_string(), arity: 1 });
        
        // Compression (12+ functions)
        self.define("compress_gzip".to_string(), Value::NativeFunction { name: "compress_gzip".to_string(), arity: 1 });
        self.define("decompress_gzip".to_string(), Value::NativeFunction { name: "decompress_gzip".to_string(), arity: 1 });
        self.define("compress_zlib".to_string(), Value::NativeFunction { name: "compress_zlib".to_string(), arity: 1 });
        self.define("decompress_zlib".to_string(), Value::NativeFunction { name: "decompress_zlib".to_string(), arity: 1 });
        self.define("compress_bzip2".to_string(), Value::NativeFunction { name: "compress_bzip2".to_string(), arity: 1 });
        self.define("decompress_bzip2".to_string(), Value::NativeFunction { name: "decompress_bzip2".to_string(), arity: 1 });
        self.define("zip_create".to_string(), Value::NativeFunction { name: "zip_create".to_string(), arity: 1 });
        self.define("zip_add".to_string(), Value::NativeFunction { name: "zip_add".to_string(), arity: 3 });
        self.define("zip_extract".to_string(), Value::NativeFunction { name: "zip_extract".to_string(), arity: 2 });
        self.define("zip_list".to_string(), Value::NativeFunction { name: "zip_list".to_string(), arity: 1 });
        self.define("tar_create".to_string(), Value::NativeFunction { name: "tar_create".to_string(), arity: 1 });
        self.define("tar_extract".to_string(), Value::NativeFunction { name: "tar_extract".to_string(), arity: 2 });
        
        // Threading/Concurrency (15+ functions)
        self.define("thread_create".to_string(), Value::NativeFunction { name: "thread_create".to_string(), arity: 1 });
        self.define("thread_join".to_string(), Value::NativeFunction { name: "thread_join".to_string(), arity: 1 });
        self.define("thread_sleep".to_string(), Value::NativeFunction { name: "thread_sleep".to_string(), arity: 1 });
        self.define("thread_id".to_string(), Value::NativeFunction { name: "thread_id".to_string(), arity: 0 });
        self.define("mutex_create".to_string(), Value::NativeFunction { name: "mutex_create".to_string(), arity: 0 });
        self.define("mutex_lock".to_string(), Value::NativeFunction { name: "mutex_lock".to_string(), arity: 1 });
        self.define("mutex_unlock".to_string(), Value::NativeFunction { name: "mutex_unlock".to_string(), arity: 1 });
        self.define("semaphore_create".to_string(), Value::NativeFunction { name: "semaphore_create".to_string(), arity: 1 });
        self.define("semaphore_wait".to_string(), Value::NativeFunction { name: "semaphore_wait".to_string(), arity: 1 });
        self.define("semaphore_signal".to_string(), Value::NativeFunction { name: "semaphore_signal".to_string(), arity: 1 });
        self.define("atomic_add".to_string(), Value::NativeFunction { name: "atomic_add".to_string(), arity: 2 });
        self.define("atomic_sub".to_string(), Value::NativeFunction { name: "atomic_sub".to_string(), arity: 2 });
        self.define("atomic_get".to_string(), Value::NativeFunction { name: "atomic_get".to_string(), arity: 1 });
        self.define("atomic_set".to_string(), Value::NativeFunction { name: "atomic_set".to_string(), arity: 2 });
        self.define("channel_create".to_string(), Value::NativeFunction { name: "channel_create".to_string(), arity: 0 });
        self.define("channel_send".to_string(), Value::NativeFunction { name: "channel_send".to_string(), arity: 2 });
        self.define("channel_receive".to_string(), Value::NativeFunction { name: "channel_receive".to_string(), arity: 1 });
        
        // Regex (10+ functions)
        self.define("regex_match".to_string(), Value::NativeFunction { name: "regex_match".to_string(), arity: 2 });
        self.define("regex_find".to_string(), Value::NativeFunction { name: "regex_find".to_string(), arity: 2 });
        self.define("regex_find_all".to_string(), Value::NativeFunction { name: "regex_find_all".to_string(), arity: 2 });
        self.define("regex_replace".to_string(), Value::NativeFunction { name: "regex_replace".to_string(), arity: 3 });
        self.define("regex_replace_all".to_string(), Value::NativeFunction { name: "regex_replace_all".to_string(), arity: 3 });
        self.define("regex_split".to_string(), Value::NativeFunction { name: "regex_split".to_string(), arity: 2 });
        self.define("regex_escape".to_string(), Value::NativeFunction { name: "regex_escape".to_string(), arity: 1 });
        self.define("regex_groups".to_string(), Value::NativeFunction { name: "regex_groups".to_string(), arity: 2 });
        self.define("regex_is_valid".to_string(), Value::NativeFunction { name: "regex_is_valid".to_string(), arity: 1 });
        self.define("regex_count".to_string(), Value::NativeFunction { name: "regex_count".to_string(), arity: 2 });
    }

    
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    
    pub fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }
    
    pub fn get(&self, name: &str) -> Result<Value, ProtlinError> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(ProtlinError::UndefinedVariable(name.to_string()))
    }
    
    pub fn set(&mut self, name: &str, value: Value) -> Result<(), ProtlinError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(ProtlinError::UndefinedVariable(name.to_string()))
    }
    
    pub fn exists(&self, name: &str) -> bool {
        self.scopes.iter().any(|scope| scope.contains_key(name))
    }
}

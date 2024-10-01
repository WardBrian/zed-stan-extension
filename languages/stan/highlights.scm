; these probably shouldn't be modules
(functions "functions" @constructor)
(data "data" @constructor)
(transformed_data "transformed data" @constructor)
(parameters "parameters" @constructor)
(transformed_parameters "transformed parameters" @constructor)
(model "model" @constructor)
(generated_quantities "generated quantities" @constructor)

(comment) @comment
(string_literal) @string

[
    "||"
    "&&"
    "=="
    "!="
    "<"
    "<="
    ">"
    ">="
    "+"
    "-"
    "*"
    "/"
    "%"
    "\\"
    ".^"
    "%/%"
    ".*"
    "./"
    "!"
    "-"
    "+"
    "^"
    "'"
    "~"
    "="
    (assignment_op)
] @operator

[
    "("
    ")"
    "["
    "]"
    "{"
    "}"
    "<"
    ">"
] @punctuation.bracket

[
    ","
    "|"
    ";"
] @punctuation.delimiter

(function_declarator
name: (identifier) @function
)

(function_expression
name: (identifier) @function.call
)

(function_statement
name: (identifier) @function.call
)

(distr_expression
name: (identifier) @function.call
)

(sampling_statement
name: (identifier) @function.call
)

(print_statement
"print" @function.call)

(reject_statement
"reject" @function.call)

(fatal_error_statement
"fatal_error" @function.call)

[
    "data"
    "int"
    "real"
    "complex"
    "array"
    "tuple"
    "vector"
    "simplex"
    "unit_vector"
    "ordered"
    "positive_ordered"
    "row_vector"
    "matrix"
    "complex_vector"
    "complex_matrix"
    "complex_row_vector"
    "corr_matrix"
    "cov_matrix"
    "cholesky_factor_cov"
    "cholesky_factor_corr"
    "void"
]@type

[
    (integer_literal)
    (real_literal)
    (imag_literal)
] @number

[
    "break"
    "continue"
    "while"
    "for"
    "if"
    "else"
    "return"
] @keyword.control

[
    "lower"
    "upper"
    "offset"
    "multiplier"
] @property

(preproc_include
    directive: "#include" @keyword
    file: (preproc_file) @string.special
)


(profile_statement "profile" @keyword)


(for_statement
loopvar: (identifier) @variable.parameter
)
(parameter_declaration
parameter: (identifier) @variable.parameter
)

(identifier) @variable

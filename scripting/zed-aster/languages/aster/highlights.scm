; Structure

(section name: (identifier) @function)
(header_param) @attribute
(header_entry) @string.special

; Dialog

(dialog) @markup.raw
(text_text_fragment) @markup.raw
(text_text_fragment) @spell
(text_escape_fragment) @tag

; Expressions
(symbol_ref) @variable
((symbol_ref) @constant (#match? @constant "^\\$[A-Z][A-Z_]+$"))
(command name: (identifier) @function)

(integer) @number
(float) @number
(boolean) @boolean

["if" "unless" "then"] @keyword.conditional
["loop" "restart" "while" "until" "do"] @keyword.loop
["end" "break"] @keyword.return

; Punctuation

[
    "(" ")" "[" "]" "{" "}"
] @punctuation.bracket

[
    "---"
    "$"
] @punctuation

; ERROR

(ERROR) @error

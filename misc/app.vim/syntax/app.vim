if exists("b:current_syntax")
	finish
endif

syntax keyword appKeyword
	\ proc
	\ len
    \ syscall
    \ int

syntax match appNumber "\v<\d+>"
syntax match appNumber "\v<(\d+_+)+\d+(\.\d+(_+\d+)*)?>"
syntax match appNumber "\v<\d+\.\d+>"
syntax match appNumber "\v<\d*\.?\d+([Ee]-?)?\d+>"
syntax match appNumber "\v<0x[[:xdigit:]_]+([Pp]-?)?\x+>"
syntax match appNumber "\v<0b[01_]+>"
syntax match appNumber "\v<0o[0-7_]+>"

syntax region appString start=/"/ end=/"/
syntax match  appComment "\v;.*$"

syntax match appProcCall /\<\K\k*\ze\s*(/
syntax match appProc      "\.\@<=\<\D\w*\>\ze("

syntax match appOperator "\v\+"
syntax match appOperator "\v\-"
syntax match appOperator "\v\*"
syntax match appOperator "\v\/"
syntax match appOperator "\v\!"
syntax match appOperator "\v\,"
syntax match appOperator "\v\="

hi default link appNumber   Number
hi default link appString   String
hi default link appComment  Comment
hi default link appKeyword  Keyword
hi default link appOperator Operator
hi default link appProc     Function
hi default link appProcCall Function

let b:current_syntax = "a++"

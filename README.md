# i_v
"ivy language" parser/generator/vm : A programming language/VM I'm designing/writing in rust as a learning experience

Currently doesn't do a whole lot

Parses function headers and variables - more grammar interpretation to come soon

Function declarations look like

fn function_name : return_type (arg_name : arg_return_type, arg_name : arg_return_type) { }

Variable declarations look like

var var_name : return_type = expression;

Has a register based instruction set ready to be generated and run via the virtual machine

Lexer successfully generates almost all of the necessary tokens from source, minus some of the standard library features that will be going in at a later date, when the compiler/run time are closer to being fully functional

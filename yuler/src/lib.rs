mod literal;
pub use literal::*;

// mod builtin;
// pub use builtin::*;

mod object;
pub use object::*;

mod value;
pub use value::*;

mod variable;
pub use variable::*;

mod for_loop;
pub use for_loop::*;

mod if_stmt;
pub use if_stmt::*;

mod switch;
pub use switch::*;

mod block;
pub use block::*;

mod function_declare;
pub use function_declare::*;

mod statement;
pub use statement::*;

mod ident;
pub use ident::*;

mod writer;
pub use writer::*;

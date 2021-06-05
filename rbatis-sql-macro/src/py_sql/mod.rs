pub mod string_node;
pub mod if_node;
pub mod trim_node;
pub mod foreach_node;
pub mod choose_node;
pub mod otherwise_node;
pub mod when_node;
pub mod bind_node;
pub mod set_node;
pub mod where_node;
pub mod print_node;
pub mod impl_node;
pub mod error;

use crate::py_sql::string_node::StringNode;
use crate::py_sql::if_node::IfNode;
use crate::py_sql::trim_node::TrimNode;
use crate::py_sql::foreach_node::ForEachNode;
use crate::py_sql::choose_node::ChooseNode;
use crate::py_sql::otherwise_node::OtherwiseNode;
use crate::py_sql::when_node::WhenNode;
use crate::py_sql::bind_node::BindNode;
use crate::py_sql::set_node::SetNode;
use crate::py_sql::where_node::WhereNode;
use crate::py_sql::print_node::PrintNode;


#[derive(Clone, Debug)]
pub enum NodeType {
    NString(StringNode),
    NIf(IfNode),
    NTrim(TrimNode),
    NForEach(ForEachNode),
    NChoose(ChooseNode),
    NOtherwise(OtherwiseNode),
    NWhen(WhenNode),
    NBind(BindNode),
    NSet(SetNode),
    NWhere(WhereNode),
    NPrint(PrintNode),
}


pub trait Name {
    fn name() -> &'static str;
}

pub trait DefName {
    fn def_name() -> &'static str;
}

pub trait ParsePySql {
    fn parse(arg: &str) -> Result<Vec<NodeType>, crate::py_sql::error::Error>;
}
use std::fmt;
use std::str;

use keywords::escape_if_keyword;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub schema: Option<String>,
    pub name: String,
    pub alias: Option<String>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref schema) = self.schema {
            write!(f, "{}.{}", escape_if_keyword(schema), escape_if_keyword(&self.name))?;
        } else {
            write!(f, "{}", escape_if_keyword(&self.name))?;
        }
        if let Some(ref alias) = self.alias {
            write!(f, " AS {}", escape_if_keyword(alias))?;
        }
        Ok(())
    }
}

impl<'a> From<&'a str> for Table {
    fn from(t: &str) -> Table {
        Table {
            schema: None,
            name: String::from(t),
            alias: None,
        }
    }
}


#[macro_export]
macro_rules! json {
    ( { $($args:tt)* } ) => {
        "{".to_string() + &json!($($args)*) + "}"
    };
    ( { $($args:tt)* } , $($rest:tt)* ) => {
        json!($($args)*) + "," + &json!($($rest)*)
    };
    ( [ $($args:tt)* ] ) => {
        "[".to_string() + &json!($($args)*) + "]"
    };
    ( [ $($args:tt)* ] , $($rest:tt)* ) => {
        json!($($args)*) + "," + &json!($($rest)*)
    };

    ( $first:expr => $($rest:tt)+ ) => {
        $first.serialize() + ":" + &json!($($rest)+)
    };
    ( $first:expr ,  $($rest:tt)+ ) => {
        $first.serialize() + "," + &json!($($rest)+)
    };

    ( $e:expr ) => {
        $e.serialize()
    };
}

pub trait Json {
    fn serialize(&self) -> String;
}

impl Json for String {
    fn serialize(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl<'a> Json for &'a str {
    fn serialize(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl Json for bool {
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl Json for i32 {
    fn serialize(&self) -> String {
        self.to_string()
    }
}

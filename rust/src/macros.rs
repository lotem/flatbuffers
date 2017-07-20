//! Macros to generate code for Flatbuffer objects.

#[macro_export]
macro_rules! table_fn {
    (get_u8,   $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_u8($slot, $default)   };
    (get_i8,   $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_i8($slot, $default)   };
    (get_u16,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_u16($slot, $default)  };
    (get_i16,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_i16($slot, $default)  };
    (get_u32,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_u32($slot, $default)  };
    (get_i32,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_i32($slot, $default)  };
    (get_u64,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_u64($slot, $default)  };
    (get_i64,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_i64($slot, $default)  };
    (get_f32,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_f32($slot, $default)  };
    (get_f64,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_f64($slot, $default)  };
    (get_bool, $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_bool($slot, $default) };
    (get_str,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_slot_str($slot)  };
    (get_struct, $s:ident, $slot:expr, $ty:ty)      => { ($s.0).get_slot_struct::<$ty>($slot)  };
    (vector,     $s:ident, $slot:expr, $ty:ty)      => { ($s.0).get_slot_vector::<$ty>($slot)  };
}

#[macro_export]
macro_rules! struct_fn {
    (get_u8,   $s:ident, $slot:expr, $default:expr) => { ($s.0).get_u8($slot)   };
    (get_i8,   $s:ident, $slot:expr, $default:expr) => { ($s.0).get_i8($slot)   };
    (get_u16,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_u16($slot)  };
    (get_i16,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_i16($slot)  };
    (get_u32,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_u32($slot)  };
    (get_i32,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_i32($slot)  };
    (get_u64,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_u64($slot)  };
    (get_i64,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_i64($slot)  };
    (get_f32,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_f32($slot)  };
    (get_f64,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_f64($slot)  };
    (get_bool, $s:ident, $slot:expr, $default:expr) => { ($s.0).get_bool($slot) };
    (get_str,  $s:ident, $slot:expr, $default:expr) => { ($s.0).get_str($slot)  };
    (get_struct,  $s:ident, $slot:expr, $ty:ty)     => { ($s.0).get_struct::<$ty>($slot)  };
}


#[macro_export]
macro_rules! table_get_fn {
    (($name:ident, get_struct, $ty:ty, $slot:expr)) => {
        pub fn $name(&self) -> Option<$ty> {
            table_fn!(get_struct, self, $slot, $ty)
        }
    };
    (($name:ident, vector, $ty:ty, $slot:expr)) => {
        pub fn $name(&self) -> flatbuffers::Iterator<$ty> {
            table_fn!(vector, self, $slot, $ty)
        }
    };
    (($name:ident, simple_enum, $n:ident, $ty:ty, $enum_mod:ident, $slot:expr, $default:expr)) => {
        pub fn $name(&self) -> Option<$enum_mod> {
            let v = table_fn!($n, self, $slot, $default);
            $enum_mod::from(v)
        }
    };
    (($name:ident, union, $ty_fn:ident, $ty:ty, $enum_mod:ident, $slot:expr, $default:expr)) => {
        pub fn $name(&self) -> Option<$ty> {
            let offset = self.0.field_offset($slot);
            if offset != 0 {
                let ty = self.$ty_fn();
                let table = self.0.get_root(offset);
                return $enum_mod::new(table, ty);
            }
            None
        }
    };
    (($name:ident, $n:ident, $ty:ty, $slot:expr, $default:expr)) => {
        pub fn $name(&self) -> $ty {
            table_fn!($n, self, $slot, $default)
        }
    }
}

#[macro_export]
macro_rules! struct_get_fn {
    (($name:ident, get_struct, $ty:ty, $slot:expr)) => {
        pub fn $name(&self) -> $ty {
            struct_fn!(get_struct, self, $slot, $ty)
        }
    };
    (($name:ident, simple_enum, $n:ident, $ty:ty, $enum_mod:ident, $slot:expr, $default:expr)) => {
        pub fn $name(&self) -> Option<$enum_mod> {
            let v = struct_fn!($n, self, $slot, $default);
            $enum_mod::from(v)
        }
    };
    (($name:ident, $n:ident, $ty:ty, $slot:expr, $default:expr)) => {
        pub fn $name(&self) -> $ty {
            struct_fn!($n, self, $slot, $default)
        }
    }
}

#[macro_export]
macro_rules! basic_struct_def {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name($crate::Table);

        impl $name {
            pub fn new(table: $crate::Table) -> $name {
                $name ( table )
            }
        }

        impl From<$crate::Table> for $name {
            fn from(table: $crate::Table) -> $name {
                $name(table)
            }
        }
    }
}

#[macro_export]
macro_rules! table_object_trait {
    ($name:ident, $indirect:expr, $inline_size:expr) => {
        impl $crate::TableObject for $name {
            fn is_struct() -> bool {
                $indirect
            }

            fn inline_size() -> usize {
                $inline_size
            }
        }
    }
}

#[macro_export]
macro_rules! table_object {
    ($name:ident, $inline_size:expr, [ $( $f:tt ),* ]) => {

        basic_struct_def!{$name}
        impl $name {
            $( table_get_fn!{$f} )*
        }
        table_object_trait!{ $name, false, $inline_size } 
    }
}

#[macro_export]
macro_rules! struct_object {
    ($name:ident, $inline_size:expr, [ $( $f:tt ),* ]) => {

        basic_struct_def!{$name}
        impl $name {
            $( struct_get_fn!{$f} )*
        }
        table_object_trait!{ $name, true, $inline_size }        
    }
}

#[macro_export]
macro_rules! simple_enum {
    ($type_name:ident, $repr:ident,
     [ $( ($e_name:ident, $value:expr) ),+ ]) => {

        #[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
        #[repr($repr)]
        pub enum $type_name {
            $( $e_name = $value ),+
        }

        impl $type_name {
            pub fn from(value: $repr) -> Option<$type_name> {
                match value {
                    $( $value => Some($type_name::$e_name) ),+,
                    _ => None
                }
            }
        }
    }   
}

#[macro_export]
macro_rules! union {
    ($name:ident,
     $type_name:ident,
     $repr:ident,
     [ $( ($e_name:ident, $value:expr, $ty:ty) ),+ ]) => {

        #[derive(Debug)]
        pub enum $name {
            None,
            $( $e_name( $ty ) ),+
        }

        impl $name {
            pub fn new(table: $crate::Table, utype: Option<$type_name>) -> Option<$name> {
                match utype {
                    $( Some($type_name::$e_name) => Some( $name::$e_name( table.into() ) ), )*
                    _ => None
                }
            }
        }

        simple_enum!($type_name, $repr, [ $( ($e_name, $value) ),+ ]);
    };
}

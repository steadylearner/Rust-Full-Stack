// https://github.com/rust-lang/rust/blob/master/src/libstd/macros.rs
// https://www.google.com/search?client=firefox-b-d&q=what+is+token+tree+in+rust
// https://doc.rust-lang.org/rust-by-example/trait/derive.html
// https://www.google.com/search?client=firefox-b-d&q=nested macro not allowed in rust
// https://www.google.com/search?client=firefox-b-d&q=struct inheritance vs composition rust

macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)+ }
        __nested_macro!($);
    }
}

// Include public
// https://stackoverflow.com/questions/34373169/how-do-i-create-a-rust-macro-with-optional-parameters-using-repetitions
// https://users.rust-lang.org/t/best-way-to-make-a-macro-with-required-and-optional-arguments/27514
// (You can use optional.)
// https://doc.rust-lang.org/reference/macros-by-example.html#metavariables

// https://doc.rust-lang.org/reference/macros-by-example.html
// vis - a possibly empty Visibility qualifier, You don't need a optional parameter ? with this.
// vis may only be followed by one of: , an identifier other than a non-raw priv, any token that can begin a type, or a metavariable with a ident, ty, or path fragment specifier.
macro_rules! public_struct {
    (pub struct $basestruct:ident { $( $commonfieldpub:vis $commonfield:ident: $commonty:ty ),+ $(,)* }) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $basestruct {
                    () => {
                        pub struct $basestruct {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };
                    (#[derive($s($arg:tt)+)]) => {
                        #[derive($s($arg)+)]
                        pub struct $basestruct {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };

                    (pub struct $name:ident { $s( $pub:vis $field:ident: $ty:ty ),+ $s(,)* }) => {
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                            $s( $pub $field: $ty ),+
                        }
                    };
                    (#[derive($s($arg:tt)+)] pub struct $name:ident { $s( $pub:vis $field:ident: $ty:ty ),+ $s(,)* }) => {
                        #[derive($s($arg)+)]
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                            $s( $pub $field: $ty ),+
                        }
                    };

                    (pub struct $name:ident) => {
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };
                    (#[derive($s($arg:tt)+)] pub struct $name:ident) => {
                        #[derive($s($arg)+)]
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };
                }
            }
        }
    };
}

// public by default. You should use pub before every struct keyord.
// pub before fields is optional.
public_struct!(
    // pub is required here before struct
    pub struct MessageBase {
        pub text: String
        // pub text: String // , here is not required?
    }
);

MessageBase!(
    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        // read: bool, // pub is optional to the fields
        pub read: bool,
    }
);

impl Message {
    fn update_text(&mut self, new_message: String) {
        self.text = new_message
    }
    fn read(&mut self) {
        if self.read == false {
            self.read = true;
        }
    }
}

MessageBase!(
    #[derive(Debug, Clone, PartialEq)]
    pub struct MessageCreateRequest
);

MessageBase!(
    // #[derive(Debug, Clone, PartialEq)]
    pub struct MessageUpdateRequest
);

fn main() {
    let message_create_request = MessageCreateRequest {
        text: "I have ghosted twice by two clients and almost broke.".into(),
    };

    let mut message = Message {
        text: message_create_request.text,
        read: false,
    };
    println!("{:#?}", &message);

    assert_eq!(message, message.clone());

    let message_update_request = MessageUpdateRequest {
        text: "Help me! to find work with programming. Plase, fellows.".into(),
    };

    message.update_text(message_update_request.text);
    println!("{:#?}", &message);

    message.read();
    println!("{:#?}", &message);
}

// macro_rules! private_struct {
//     (struct $basestruct:tt { $( $commonfield:ident: $commonty:ty ),+ $(,)* }) => {
//         nested_macro! {
//             ($s:tt) => {
//                 macro_rules! $basestruct {
//                     () => {
//                         struct $basestruct {
//                             $( $commonfield: $commonty, )+
//                         }
//                     };
//                     (#[derive($s($arg:tt)+)]) => {
//                         #[derive($s($arg)+)]
//                         struct $basestruct {
//                             $( $commonfield: $commonty, )+
//                         }
//                     };

//                     (struct $name:ident { $s( $field:ident: $ty:ty ),+ $s(,)* }) => {
//                         pub struct $name {
//                             $( $commonfieldpub $commonfield: $commonty, )+
//                             $s( $field: $ty ),*
//                         }
//                     };
//                     (#[derive($s($arg:tt)+)] struct $name:ident { $s( $field:ident: $ty:ty ),+ $s(,)* }) => {
//                         #[derive($s($arg)+)]
//                         struct $name {
//                             $( $commonfield: $commonty, )+
//                             $s( $field: $ty ),+
//                         }
//                     };

//                     (struct $name:ident) => {
//                         struct $name {
//                             $( $commonfield: $commonty, )+
//                         }
//                     };
//                     (#[derive($s($arg:tt)+)] struct $name:ident) => {
//                         #[derive($s($arg)+)]
//                         struct $name {
//                             $( $commonfield: $commonty, )+
//                         }
//                     };
//                 }
//             }
//         }
//     };
// }

// make_private!(
//     struct PrivateMessageBase {
//         text: String,
//     }
// );

// PrivateMessageBase!(
//     #[derive(Debug, Clone, PartialEq)]
//     struct PrivateMessage {
//         read: bool,
//     }
// );

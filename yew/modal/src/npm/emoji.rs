use stdweb::Value;
use stdweb::unstable::TryInto;

// #[derive(Default)]
// pub struct CcxtService(Option<Value>);

// When you want to use unpkg, verfiy that there is window.<package> in the source code
// Otherwise, use browserify pattern

// For example,
// <script type="text/javascript" src="https://unpkg.com/ccxt"></script> has
// window.ccxt = require ('./ccxt') in it

// The reason you use service is not to repeat the same code and make wrappers for rust codes
// You can do the same with js! relevant codes

#[derive(Default)]
pub struct EmojiService(Option<Value>);

impl EmojiService {
    pub fn new() -> Self {
        let lib = js! {
            return emoji;
        };
        EmojiService(Some(lib))
    }

    pub fn emojify(&mut self, message: String) -> String {
        let lib = self.0.as_ref().expect("node-emoji library object lost");
        let v: Value = js! {
            const emoji = @{lib};
            // console.log(emoji);
            return emoji.emojify(@{message});
        };
        let v: String = v.try_into().expect("can't convert to emoji");
        v
    }
}

use nvim_oxi::{
    self as oxi,
    serde::{Deserializer, Serializer},
    Dictionary, Function, Object,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Browser(#[from] anyhow::Error),
    #[error("Couldn't lock")]
    Lock,
}

impl<E> From<std::sync::PoisonError<E>> for Error {
    fn from(_: std::sync::PoisonError<E>) -> Self {
        Error::Lock
    }
}

#[derive(Serialize, Deserialize)]
struct Browser {
    index: usize,
}

impl oxi::lua::Pushable for Browser {
    unsafe fn push(
        self,
        lstate: *mut oxi::lua::ffi::lua_State,
    ) -> Result<std::ffi::c_int, oxi::lua::Error> {
        self.serialize(Serializer::new())
            .map_err(oxi::lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}

impl oxi::lua::Poppable for Browser {
    unsafe fn pop(lstate: *mut oxi::lua::ffi::lua_State) -> Result<Self, oxi::lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::deserialize(Deserializer::new(obj))
            .map_err(oxi::lua::Error::pop_error_from_err::<Self, _>)
    }
}

#[derive(Deserialize)]
struct Args {
    is_app: bool,
    url: String,
}

impl oxi::lua::Poppable for Args {
    unsafe fn pop(lstate: *mut oxi::lua::ffi::lua_State) -> Result<Self, oxi::lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::deserialize(Deserializer::new(obj))
            .map_err(oxi::lua::Error::pop_error_from_err::<Self, _>)
    }
}

struct BrowserManager {
    opened: Vec<headless_chrome::Browser>,
}

impl BrowserManager {
    fn new() -> BrowserManager {
        Self { opened: vec![] }
    }
    // fn get(&self, browser: Browser) -> headless_chrome::Browser {
    //     self.opened[browser.index].clone()
    // }
    fn open(&mut self, args: Args) -> Result<Browser, Error> {
        let url_opt = if args.is_app {
            std::ffi::OsString::from("--app=".to_string() + args.url.as_str())
        } else {
            std::ffi::OsString::from(&args.url)
        };
        let opts = headless_chrome::LaunchOptions::default_builder()
            .headless(false)
            .disable_default_args(true)
            .ignore_certificate_errors(false)
            // strange workaround for not-closing connection
            .idle_browser_timeout(std::time::Duration::new(u64::MAX, 0))
            .args(vec![
                &url_opt, /*OsStr::new("--enable-experimental-ui-automation")*/
            ])
            .build()
            .expect("Could not find chrome-executable");
        let browser = headless_chrome::Browser::new(opts)?;
        // browser.new_tab_with_options(headless_chrome::protocol::cdp::Target::CreateTarget {
        //     url: args.url,
        //     width: None,
        //     height: None,
        //     browser_context_id: None,
        //     enable_begin_frame_control: None,
        //     new_window: None,
        //     background: None,
        // })?;
        self.opened.push(browser);
        Ok(Browser {
            index: self.opened.len() - 1,
        })
    }
    // fn get_tabs(&self, browser: Browser) -> Result<Vec<String>, Error> {
    //     Ok(self
    //         .get(browser)
    //         .get_tabs()
    //         .lock()?
    //         .iter()
    //         .map(|t| t.get_url())
    //         .collect())
    // }
    // fn refresh(&self, browser: Browser) -> Result<(), Error> {
    //     self.get(browser).get_tabs().lock()?
    // }
}

#[oxi::module]
fn prochrome_internals() -> oxi::Result<Dictionary> {
    let mut bm = BrowserManager::new();

    let open = Function::from_fn_mut(move |args: Args| bm.open(args));

    Ok(Dictionary::from_iter([("open", open)]))
}

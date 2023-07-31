use std::sync::Arc;

use headless_chrome::protocol::cdp::Target::CreateTarget;
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

impl oxi::lua::Poppable for Args {
    unsafe fn pop(lstate: *mut oxi::lua::ffi::lua_State) -> Result<Self, oxi::lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::deserialize(Deserializer::new(obj))
            .map_err(oxi::lua::Error::pop_error_from_err::<Self, _>)
    }
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
impl oxi::lua::Pushable for Tab {
    unsafe fn push(
        self,
        lstate: *mut oxi::lua::ffi::lua_State,
    ) -> Result<std::ffi::c_int, oxi::lua::Error> {
        self.serialize(Serializer::new())
            .map_err(oxi::lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}
impl oxi::lua::Poppable for Tab {
    unsafe fn pop(lstate: *mut oxi::lua::ffi::lua_State) -> Result<Self, oxi::lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::deserialize(Deserializer::new(obj))
            .map_err(oxi::lua::Error::pop_error_from_err::<Self, _>)
    }
}

#[derive(Serialize, Deserialize)]
struct Browser {
    get_tabs: oxi::Function<Self, Vec<Tab>>,
    new_tab: oxi::Function<(Self, String), Tab>,
}
impl Browser {
    fn from_headless(browser: &headless_chrome::Browser) -> Self {
        let b1 = browser.clone();
        let b2 = browser.clone();
        Self {
            get_tabs: oxi::Function::from_fn(move |_: Browser| {
                let tabs = b1
                    .get_tabs()
                    .lock()?
                    .iter()
                    .map(Tab::from_headless)
                    .collect();
                Ok::<_, Error>(tabs)
            }),
            new_tab: oxi::Function::from_fn(move |(_, url): (Browser, String)| {
                let tab = b2.new_tab_with_options(CreateTarget {
                    url,
                    width: None,
                    height: None,
                    browser_context_id: None,
                    enable_begin_frame_control: None,
                    new_window: Some(false),
                    background: None,
                })?;
                Ok::<_, Error>(Tab::from_headless(&tab))
            }),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Tab {
    url: String,
    title: Option<String>,
    refresh: oxi::Function<Self, ()>,
    navigate_to: oxi::Function<(Self, String), ()>,
    find_elements: oxi::Function<(Self, String), Vec<String>>,
}

impl Tab {
    fn from_headless(tab: &Arc<headless_chrome::Tab>) -> Self {
        let tab1 = tab.clone();
        let tab2 = tab.clone();
        let tab3 = tab.clone();
        Self {
            url: tab.get_url(),
            title: tab.get_title().ok(),
            refresh: oxi::Function::from_fn(move |_: Tab| {
                tab1.reload(true, None)?;
                Ok::<_, Error>(())
            }),
            navigate_to: oxi::Function::from_fn(move |(_, url): (Tab, String)| {
                tab2.navigate_to(&url)?;
                Ok::<_, Error>(())
            }),
            find_elements: oxi::Function::from_fn(move |(_, query): (Tab, String)| {
                // TODO Make element struct with methods
                let els = tab3
                    .find_elements(&query)?
                    .iter()
                    .filter_map(|x| x.get_inner_text().ok())
                    .collect::<Vec<_>>();
                Ok::<_, Error>(els)
            }),
        }
    }
}

#[derive(Deserialize)]
struct Args {
    is_app: bool,
    url: String,
}

#[derive(Clone)]
struct BrowserManager {
    opened: Vec<headless_chrome::Browser>,
}

impl BrowserManager {
    fn new() -> BrowserManager {
        Self { opened: vec![] }
    }
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
        self.opened.push(browser);
        Ok(Browser::from_headless(self.opened.last().unwrap()))
    }
}

#[oxi::module]
fn prochrome_internals() -> oxi::Result<Dictionary> {
    let mut bm = BrowserManager::new();

    let open = Function::from_fn_mut(move |args: Args| bm.open(args));

    Ok(Dictionary::from_iter([("open", open)]))
}

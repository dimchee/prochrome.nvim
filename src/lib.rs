use std::sync::{Arc, Mutex};

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
    #[error(transparent)]
    Lua(#[from] oxi::lua::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl<E> From<std::sync::TryLockError<E>> for Error {
    fn from(_: std::sync::TryLockError<E>) -> Self {
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
impl oxi::lua::Pushable for Element {
    unsafe fn push(
        self,
        lstate: *mut oxi::lua::ffi::lua_State,
    ) -> Result<std::ffi::c_int, oxi::lua::Error> {
        self.serialize(Serializer::new())
            .map_err(oxi::lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}

#[derive(Serialize, Deserialize)]
struct Browser {
    get_tabs: oxi::Function<Self, Vec<Tab>>,
    new_tab: oxi::Function<(Self, String), Tab>,
}
impl Browser {
    fn from_headless(browser: &headless_chrome::Browser, on_refresh: Option<Cmd>) -> Self {
        let b1 = browser.clone();
        let b2 = browser.clone();
        let or1 = on_refresh.clone();
        Self {
            get_tabs: oxi::Function::from_fn(move |_: Browser| {
                let tabs = b1
                    .get_tabs()
                    .try_lock()?
                    .iter()
                    .map(|t| Tab::from_headless(t, or1.clone()))
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
                Ok::<_, Error>(Tab::from_headless(&tab, on_refresh.clone()))
            }),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Tab {
    url: String,
    title: Option<String>,
    on_refresh: Option<Cmd>,
    refresh: oxi::Function<Self, ()>,
    close: oxi::Function<Self, ()>,
    navigate_to: oxi::Function<(Self, String), ()>,
    find_element: oxi::Function<(Self, String), Element>,
}

fn exec((cmd, args): Cmd) -> Result<(), Error> {
    std::process::Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    Ok(())
}

impl Tab {
    fn from_headless(tab: &Arc<headless_chrome::Tab>, on_refresh: Option<Cmd>) -> Self {
        let tab1 = tab.clone();
        let tab2 = tab.clone();
        let tab3 = tab.clone();
        let tab4 = tab.clone();
        Self {
            url: tab.get_url(),
            title: tab.get_title().ok(),
            on_refresh,
            refresh: oxi::Function::from_fn(move |t: Tab| {
                if let Some(on_refresh) = t.on_refresh {
                    exec(on_refresh)?; // TODO error on `function() print'test' end`
                }
                tab1.reload(true, None)?;
                Ok::<_, Error>(())
            }),
            close: oxi::Function::from_fn(move |_: Tab| {
                tab4.close(false)?;
                Ok::<_, Error>(())
            }),
            navigate_to: oxi::Function::from_fn(move |(_, url): (Tab, String)| {
                tab2.navigate_to(&url)?;
                Ok::<_, Error>(())
            }),
            find_element: oxi::Function::from_fn(move |(_, query): (Tab, String)| {
                // TODO slow, async?
                let el = tab3.find_element(&query)?;
                Ok::<_, Error>(Element {
                    inner_text: el.get_inner_text().ok(),
                    content: el.get_content().ok(),
                })
            }),
        }
    }
}
// TODO lua callbacks for on_start and on_refresh
// type Callback = oxi::Function<(), oxi::Object>;
type Cmd = (String, Vec<String>);

#[derive(Serialize)]
struct Element {
    inner_text: Option<String>,
    content: Option<String>,
}

#[derive(Deserialize)]
struct Args {
    is_app: Option<bool>,
    url: String,
    on_start: Option<Cmd>,
    on_refresh: Option<Cmd>,
}

#[derive(Clone)]
struct BrowserManager {
    opened: Arc<Mutex<Vec<headless_chrome::Browser>>>,
}

impl BrowserManager {
    fn new() -> BrowserManager {
        Self {
            opened: Arc::new(Mutex::new(vec![])),
        }
    }
    fn open(&mut self, args: Args) -> Result<Browser, Error> {
        let is_app = if args.is_app.unwrap_or(false) {
            std::ffi::OsString::from("--app=".to_string() + &args.url)
        } else {
            std::ffi::OsString::from(args.url)
        };
        //&url_opt, /*OsStr::new("--enable-experimental-ui-automation")*/
        let chrome_args = vec![is_app]; //is_app.into_iter().collect::<Vec<_>>();
        let opts = headless_chrome::LaunchOptions::default_builder()
            .headless(false)
            .disable_default_args(true)
            .ignore_certificate_errors(false)
            // strange workaround for not-closing connection
            .idle_browser_timeout(std::time::Duration::new(u64::MAX, 0))
            .args(chrome_args.iter().map(|x| x.as_os_str()).collect())
            .build()
            .expect("Could not find chrome-executable");
        let browser = headless_chrome::Browser::new(opts)?;
        if let Some(on_start) = args.on_start {
            exec(on_start)?;
        }
        self.opened.try_lock()?.push(browser);
        Ok(Browser::from_headless(
            self.opened.try_lock()?.last().unwrap(), // always has at least one element
            args.on_refresh,
        ))
    }
}

#[oxi::module]
fn prochrome_internals() -> oxi::Result<Dictionary> {
    let mut bm = BrowserManager::new();
    let bm1 = bm.clone();

    let open = Function::from_fn_mut(move |args: Args| bm.open(args));
    let get_opened = Function::from_fn_mut(move |_: ()| {
        Ok::<_, Error>(
            bm1.opened
                .try_lock()?
                .iter()
                .map(|b| Browser::from_headless(b, None))
                .collect::<Vec<_>>(),
        )
    });

    Ok(Dictionary::from_iter([
        ("open", oxi::Object::from(open)),
        ("get_opened", oxi::Object::from(get_opened)),
    ]))
}

#[oxi::test]
fn call_lua_function() {}

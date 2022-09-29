use async_trait::async_trait;
use async_std::io::Stdout;
use async_std;
use nvim_rs::{create::async_std as create, Handler, Value, Neovim};
use std::{ffi::OsString, sync::{Arc, Mutex}, time::Duration};

use headless_chrome::{LaunchOptions, protocol::cdp::Target::CreateTarget};

#[derive(Clone)]
struct Browser {
    browser: Option<Arc<headless_chrome::Browser>>,
}

impl Browser {
    fn new() -> Browser {
        Self { browser: None }
    }
    fn refresh(&self) -> Result<(), String> {
        self.browser.as_ref()
            .ok_or("No browser")?
            .get_tabs().lock().ok().ok_or("Could not get tabs")?
            .first().ok_or("No opened tabs available")?
            .reload(true, None).map_err(|err| err.to_string())?;
        Ok(())
    }
    fn status(&self) -> String {
        if self.browser.is_some() {
            "Chrome is running, everything good :D"
        } else {
            "No chrome running :P"
        } .to_string()
    }
    fn get_tabs(&self) -> Result<Vec<String>, String> {
        Ok(self.browser.as_ref()
            .ok_or("No browser")?
            .get_tabs().lock().ok().ok_or("Could not get tabs")?.iter()
            .map(|t| t.get_url()).collect::<Vec<String>>())
    }

    fn new_app(&mut self, url: &str) -> Result<(), String> {
        let mut app = OsString::from("--app=");
        app.push(url);
        let mut opts = LaunchOptions::default_builder()
            .headless(false)
            // strange workaround for not-closing connection
            .idle_browser_timeout(Duration::new(u64::MAX, 0))
            .build()
            .expect("Could not find chrome-executable");
        opts.args.push(&app);
        opts.disable_default_args = true;
        opts.ignore_certificate_errors = false;
        // opts.args.push(OsStr::new("--enable-experimental-ui-automation"));
        self.browser = Some(headless_chrome::Browser::new(opts)
            .map(|x| Arc::new(x))
            .map_err(|_| "chrome browser couldn't start")?);
        Ok(())
    }
    fn new_chrome(&mut self, url: &str) -> Result<(), String> {
        let mut opts = LaunchOptions::default_builder()
            .headless(false)
            // strange workaround for not-closing connection
            .idle_browser_timeout(Duration::new(u64::MAX, 0))
            .build()
            .expect("Could not find chrome-executable");
        let url_str = OsString::from(url);
        opts.args.push(&url_str);
        opts.disable_default_args = true;
        opts.ignore_certificate_errors = false;
        // opts.args.push(OsStr::new("--enable-experimental-ui-automation"));
        self.browser = Some(headless_chrome::Browser::new(opts)
            .map(|x| Arc::new(x))
            .map_err(|_| "chrome browser couldn't start")?);
        Ok(())
    }
    fn new_tab(&mut self) -> Result<(), String> {
        let _new_tab = self.browser.as_ref()
            .ok_or("No browser")?
            .new_tab_with_options(CreateTarget {
                url: "chrome://version".to_string(),
                width: None,
                height: None, 
                browser_context_id: None,
                enable_begin_frame_control: None,
                new_window: Some(false),
                background: None
            });
        Ok(())
    }
    fn navigate_to(&mut self, url: &str) -> Result<(), String> {
        self.browser.as_ref()
            .ok_or("No browser")?
            .get_tabs().lock().ok().ok_or("Could not get tabs")?
            .first().ok_or("No opened tabs available")?
            .navigate_to(url).ok().ok_or("Could not navigate to url")?;
        Ok(())
    }
    fn find_elements(&mut self, q: &str) -> Result<Vec<String>, String> {
        let tabs = self.browser.as_ref()
            .ok_or("No browser")?
            .get_tabs().lock().ok().ok_or("Could not get tabs")?;
        let tab = tabs.first().ok_or("No opened tabs available")?;
        let els = tab.find_elements(q).ok().ok_or("Couldn't find elements")?;
        Ok(els.iter().filter_map(|x| x.get_inner_text().ok()).collect::<Vec<_>>())
    }
}

#[derive(Clone)]
struct EventHandler {
    chrome: Arc<Mutex<Browser>>
}

impl EventHandler {
    fn new() -> EventHandler {
        let chrome = Browser::new();
        EventHandler { chrome: Arc::new(Mutex::new(chrome)) }
    }
}

#[async_trait]
impl Handler for EventHandler {
    type Writer = Stdout;

    async fn handle_request(&self, name: String, args: Vec<Value>, _neovim: Neovim<Stdout>) -> Result<Value, Value> {
        match name.as_ref() {
            "refresh" => {
                self.chrome
                    .try_lock().map_err(|_| Value::from("Could not lock browser :P"))?
                    .refresh()?;
                Ok(Value::from("Refreshed!"))
            }
            "new_app" => {
                if let [Value::String(raw_url)] = &args[..] {
                    let url = raw_url.as_str().ok_or(Value::from("Argument is not valid string!"))?;
                    self.chrome
                        .try_lock().map_err(|_| Value::from("Could not lock browser :P"))?
                        .new_app(url)?;
                    Ok(Value::from("Opened New App"))
                } else {
                    Err(Value::from("Wrong args to new_app, usage: new_app <url>"))
                }
            },
            "new_chrome" => {
                if let [Value::String(raw_url)] = &args[..] {
                    let url = raw_url.as_str().ok_or(Value::from("Argument is not valid string!"))?;
                    self.chrome
                        .try_lock().map_err(|_| Value::from("Could not lock browser :P"))?
                        .new_chrome(url)?;
                    Ok(Value::from("Opened New Chrome"))
                } else {
                    Err(Value::from("Wrong args to new_chrome, usage: new_app <url>"))
                }
            },
            "status" => {
                Ok(Value::from(self.chrome
                    .try_lock().map_err(|_| Value::from("Could not lock browser :P"))?
                    .status()
                ))
            }
            "get_tabs" => {
                Ok(Value::from(self.chrome
                    .try_lock().map_err(|_| Value::from("Could not lock browser :P"))?
                    .get_tabs()?.into_iter().map(|x| Value::from(x)).collect::<Vec<_>>()
                ))
            }
            "new_tab" => {
                self.chrome
                    .try_lock().map_err(|_| Value::from("Could not lock browser :P"))?
                    .new_tab()?;
                Ok(Value::from("Opened new tab!"))
            }
            "navigate_to" => {
                if let [Value::String(raw_url)] = &args[..] {
                    let url = raw_url.as_str().ok_or(Value::from("Argument is not valid string!"))?;
                    self.chrome
                        .try_lock().map_err(|_| Value::from("Could not lock browser :P"))?
                        .navigate_to(url)?;
                    Ok(Value::from("Navigated to".to_owned() + url))
                } else {
                    Err(Value::from("Wrong args to new_chrome, usage: new_app <url>"))
                }
            },
            "find_elements" => {
                if let [Value::String(raw_q)] = &args[..] {
                    let q = raw_q.as_str().ok_or(Value::from("Argument is not valid string!"))?;
                    let mut tab = self.chrome
                        .try_lock().map_err(|_| Value::from("Could not lock browser :P"))?;
                    let elems = tab.find_elements(q)?;
                    Ok(Value::from(
                        elems.iter()
                            .map(|x| Value::from(x.to_string()))
                            .collect::<Vec<_>>()
                    ))
                } else {
                    Err(Value::from("Wrong args to new_chrome, usage: find_elements <query>"))
                }
            }
            _ => {
                Ok(Value::from("Unknown command!"))
            }
        }
    }
}

#[async_std::main]
async fn main() {
    let handler = EventHandler::new();
    let (nvim, io_handler) = create::new_parent(handler).await;

    // Any error should probably be logged, as stderr is not visible to users.
    match io_handler.await {
        Err(err) => {
            if !err.is_reader_error() {
                // One last try, since there wasn't an error with writing to the
                // stream
                nvim
                    .err_writeln(&format!("Error: '{}'", err))
                    .await
                    .unwrap_or_else(|e| {
                        // We could inspect this error to see what was happening, and
                        // maybe retry, but at this point it's probably best
                        // to assume the worst and print a friendly and
                        // supportive message to our users
                        eprintln!("Well, dang... '{}'", e);
                    });
            }

            if !err.is_channel_closed() {
                // Closed channel usually means neovim quit itself, or this plugin was
                // told to quit by closing the channel, so it's not always an error
                // condition.
                eprintln!("Error: '{}'", err);

                // let mut source = err.source();
                //
                // while let Some(e) = source {
                //     eprintln!("Caused by: '{}'", e);
                //     source = e.source();
                // }
            }
        }
        Ok(()) => {}
    }
}

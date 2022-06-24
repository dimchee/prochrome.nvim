use async_trait::async_trait;
use async_std::io::Stdout;
use async_std;
use nvim_rs::{create::async_std as create, Handler, Value, Neovim};
use std::{ffi::OsString, sync::{Arc, Mutex}, time::Duration};

use headless_chrome::LaunchOptions;

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

    fn new_app(&mut self, link: &str) -> Result<(), String> {
        let mut app = OsString::from("--app=");
        app.push(link);
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
            "newapp" => {
                if let [Value::String(raw_link)] = &args[..] {
                    let link = raw_link.as_str().ok_or(Value::from("Argument is not valid string!"))?;
                    self.chrome
                        .try_lock().map_err(|_| Value::from("Could not lock browser :P"))?
                        .new_app(link)?;
                    Ok(Value::from("Opened New App"))
                } else {
                    Err(Value::from("Wrong args to newapp, usage: newapp <link>"))
                }
            },
            "status" => {
                Ok(Value::from("good :D"))
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

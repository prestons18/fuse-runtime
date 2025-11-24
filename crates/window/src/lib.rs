use web_view::*;

pub struct Window {
    view: WebView<'static, ()>,
}

pub struct WindowOptions {
    pub title: String,
    pub html: String,
    pub width: i32,
    pub height: i32,
    pub resizable: bool,
    pub debug: bool,
}

impl Window {
    pub fn new(opts: WindowOptions) -> anyhow::Result<Self> {
        let view = web_view::builder()
            .title(Box::leak(opts.title.into_boxed_str()))
            .content(Content::Html(Box::leak(opts.html.into_boxed_str())))
            .size(opts.width, opts.height)
            .resizable(opts.resizable)
            .debug(opts.debug)
            .user_data(())
            .invoke_handler(|_webview, _arg| Ok(()))
            .build()?;

        Ok(Self { view })
    }

    pub fn run(self) -> anyhow::Result<()> {
        self.view.run()?;
        Ok(())
    }

    pub fn eval(&mut self, js: &str) -> anyhow::Result<()> {
        self.view.eval(js)?;
        Ok(())
    }
}
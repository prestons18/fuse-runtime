use web_view::*;

pub struct Window<'a> {
    view: WebView<'a, ()>,
}

pub struct WindowOptions<'a> {
    pub title: &'a str,
    pub html: &'a str,
    pub width: i32,
    pub height: i32,
    pub resizable: bool,
    pub debug: bool,
}

impl<'a> Window<'a> {
    pub fn new(opts: WindowOptions<'a>) -> anyhow::Result<Self> {
        let view = web_view::builder()
            .title(opts.title)
            .content(Content::Html(opts.html))
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
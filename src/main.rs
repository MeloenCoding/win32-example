use example_app::ExampleApp;

mod example_app;
mod window;

fn main() {
    // create an app
    let mut app: ExampleApp = example_app::ExampleApp::create();

    // launch the app
    app.launch();

    // print the exit codes
    app.window.print_exit_codes();
}


use std::sync::{Arc, Mutex};

pub fn pretty_code_fmt(content: &mut String) {
    let err_msg_arc = Arc::new(Mutex::new(String::new()));

    // set a custom panic hook to capture the panic information
    set_custom_panic_hook(Arc::clone(&err_msg_arc));

    // attempt to pretty print the content, which may panic.
    let pretty_content = attempt_pretty_print(content);

    // replace original content with the pretty string
    // in case of panic, error message is logged, content remain unchanged
    handle_pretty_print_result(content, pretty_content, err_msg_arc);
}

fn set_custom_panic_hook(err_msg_arc: Arc<Mutex<String>>) {
    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let err_msg = format_panic_message(panic_info);
        *err_msg_arc.lock().unwrap() = err_msg;
    }));
    // restore the original panic hook after setting.
    std::panic::set_hook(old_hook);
}

fn format_panic_message(panic_info: &std::panic::PanicInfo<'_>) -> String {
    let payload = panic_info.payload();
    let msg = if let Some(s) = payload.downcast_ref::<&str>() {
        *s
    } else if let Some(s) = payload.downcast_ref::<String>() {
        &**s
    } else {
        "Box<Any>"
    };
    let location = panic_info.location().map_or_else(
        || String::from("unknown location"),
        |location| format!("at '{}' line {}", location.file(), location.line()),
    );
    format!("panicked with '{}' {}", msg, location)
}

fn attempt_pretty_print(
    content: &str,
) -> Result<String, Box<dyn std::any::Any + Send>> {
    std::panic::catch_unwind(|| {
        let token_stream =
            syn::parse_str::<proc_macro2::TokenStream>(content).unwrap();
        let syntax_tree: syn::File = syn::parse2(token_stream).unwrap();
        prettyplease::unparse(&syntax_tree) // Return the pretty string.
    })
}

fn handle_pretty_print_result(
    content: &mut String,
    result: Result<String, Box<dyn std::any::Any + Send>>,
    err_msg_arc: Arc<Mutex<String>>,
) {
    if let Err(_) = &result {
        log::warn!("{}\n>>>\n{}<<<", *err_msg_arc.lock().unwrap(), content);
    } else if let Ok(pretty) = result {
        *content = pretty;
    }
}

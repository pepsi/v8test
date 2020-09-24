use rusty_v8 as v8;
use v8::MapFnTo;
mod kfs;
mod kstd;
pub fn compile_module<'a>(scope: &mut v8::HandleScope<'a>, code: String, name: String) ->Option<v8::Local<'a, v8::Module>>{
    // Register functions into object
    let mut funcs: Vec<(v8::Local<v8::String>, v8::Local<v8::Function>)> = vec![];

    funcs.push((
        v8::String::new(scope, "print").unwrap(),
        v8::Function::new(scope, kstd::print).unwrap(),
    ));
    funcs.push((
        v8::String::new(scope, "println").unwrap(),
        v8::Function::new(scope, kstd::println).unwrap(),
    ));
    funcs.push((
        v8::String::new(scope, "assert").unwrap(),
        v8::Function::new(scope, kstd::assert).unwrap(),
    ));
    funcs.push((
        v8::String::new(scope, "read").unwrap(),
        v8::Function::new(scope, kfs::read).unwrap(),
    ));

    let global_std_obj = v8::Object::new(scope);
    for funcs in funcs {
        global_std_obj
            .set(scope, funcs.0.into(), funcs.1.into())
            .unwrap();
    }

    let k = v8::String::new(scope, "$").unwrap().into();
    // Set global `std` to refer to our object that has print objects
    let global = scope.get_current_context().global(scope);
    global.set(scope, k, global_std_obj.into());
    //TODO: Change in.js to detect what file is being run (future)
    let script_origin_resource_name = v8::String::new(scope, &name).unwrap().into();
    let script_origin_line_offset = v8::Integer::new(scope, 0).into();
    let script_origin_column_offset = v8::Integer::new(scope, 0).into();
    let script_origin_is_cross_origin = v8::Boolean::new(scope, true).into();
    let script_origin_script_id = v8::Integer::new(scope, 123);
    let script_origin_sourcemap_url = v8::String::new(scope, "").unwrap().into();
    let script_origin_opaque = v8::Boolean::new(scope, true);
    let script_origin_is_wasm = v8::Boolean::new(scope, false);
    let script_origin_is_es6_module = v8::Boolean::new(scope, true);
    let script_origin = v8::ScriptOrigin::new(
        script_origin_resource_name,
        script_origin_line_offset,
        script_origin_column_offset,
        script_origin_is_cross_origin,
        script_origin_script_id,
        script_origin_sourcemap_url,
        script_origin_opaque,
        script_origin_is_wasm,
        script_origin_is_es6_module,
    );
    let v8str_code: v8::Local<v8::String> = v8::String::new(scope, &code).unwrap();
    let script_source = v8::script_compiler::Source::new(v8str_code, &script_origin);
    let /* mut*/  module = v8::script_compiler::compile_module(scope, script_source).unwrap();
    let _im: Option<bool> = module.instantiate_module(scope, resolver);
    let _result = module.evaluate(scope).unwrap();
    return Some(module)
}
pub fn resolver<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    _referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    unsafe{
        //TODO: Get this actually working. 
        /* TODO: 
            Calculate `cwd/specifier`
            Seems to not be working but shouldn't be that hard to fix
        */
        let mut scope = &mut v8::CallbackScope::new(context);
        let r = specifier.to_rust_string_lossy(scope);
        let cwd_s = &std::env::current_dir().unwrap().into_os_string();
        let cwd = std::path::Path::new(cwd_s);

        let path = cwd.join(std::path::Path::new(&r));
        println!("path = {}", path.to_string_lossy());

        let code_input = std::fs::read(path).unwrap();
        let module = compile_module(scope, String::from_utf8(code_input).unwrap(), r);
        return Some(module.unwrap())
     
    }
    
}

fn main() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());
    // TODO Implement dynamic module imports
    // isolate.set_host_import_module_dynamically_callback(resolver);
    let scope = &mut v8::HandleScope::new(isolate);

    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);
    //TODO: Support different file names
    let code_input = std::fs::read("example/in.js").unwrap();
    let module = compile_module(scope,String::from_utf8( code_input).unwrap(), "example/in.js".into());
    match module{
        Some(m) => {

            // m.

        }
        None => {}
    }
}
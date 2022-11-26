# Bug passing js function to rust async function

Reproduction repository

## Try

```sh
make run
```

or

```sh
wasm-pack build --release -t web
deno run --allow-read run.js
```

## Result

```
[Function: report]
info! hello sync
call_fn_sync result
undefined
TypeError: Cannot read properties of undefined (reading 'call')
    at file:///home/mariano/src/rust/experiments/wasmbindgen_bug_async_fn_arg/pkg/wasmbindgen_bug_async_fn_arg.js:197:36
    at handleError (file:///home/mariano/src/rust/experiments/wasmbindgen_bug_async_fn_arg/pkg/wasmbindgen_bug_async_fn_arg.js:134:18)
    at imports.wbg.__wbg_call_168da88779e35f61 (file:///home/mariano/src/rust/experiments/wasmbindgen_bug_async_fn_arg/pkg/wasmbindgen_bug_async_fn_arg.js:196:67)
    at <anonymous> (file:///home/mariano/src/rust/experiments/wasmbindgen_bug_async_fn_arg/pkg/wasmbindgen_bug_async_fn_arg_bg.wasm:1:18727)
    at <anonymous> (file:///home/mariano/src/rust/experiments/wasmbindgen_bug_async_fn_arg/pkg/wasmbindgen_bug_async_fn_arg_bg.wasm:1:10612)
    at <anonymous> (file:///home/mariano/src/rust/experiments/wasmbindgen_bug_async_fn_arg/pkg/wasmbindgen_bug_async_fn_arg_bg.wasm:1:15565)
    at <anonymous> (file:///home/mariano/src/rust/experiments/wasmbindgen_bug_async_fn_arg/pkg/wasmbindgen_bug_async_fn_arg_bg.wasm:1:11637)
    at <anonymous> (file:///home/mariano/src/rust/experiments/wasmbindgen_bug_async_fn_arg/pkg/wasmbindgen_bug_async_fn_arg_bg.wasm:1:19954)
    at __wbg_adapter_10 (file:///home/mariano/src/rust/experiments/wasmbindgen_bug_async_fn_arg/pkg/wasmbindgen_bug_async_fn_arg.js:75:10)
    at real (file:///home/mariano/src/rust/experiments/wasmbindgen_bug_async_fn_arg/pkg/wasmbindgen_bug_async_fn_arg.js:60:20)
call_fn_async result
```

## Expected Result

```
[Function: report]
info! hello sync
call_fn_sync result
[Function: report]
info! hello async
call_fn_async result
```

import init, {call_fn_sync, call_fn_async} from './pkg/wasmbindgen_bug_async_fn_arg.js';

async function main() {
  await init();
  function report(info) {
    console.log('info!', info);
  }
  console.log('call_fn_sync', call_fn_sync(report));
  console.log('call_fn_async', await call_fn_async(report));
}

await main();

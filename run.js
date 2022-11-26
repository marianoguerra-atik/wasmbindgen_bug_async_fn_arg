import init, {call_fn_sync, call_fn_async} from './pkg/wasmbindgen_bug_async_fn_arg.js';

function report(info) {
    console.log('info!', info);
}

const reportArrow = info => {
    console.log('info arrow!', info);
}

async function main() {
    await init();

    console.log('call_fn_sync', call_fn_sync(report));
    console.log('call_fn_async', await call_fn_async(report));

    console.log('call_fn_sync arrow', call_fn_sync(reportArrow));
    console.log('call_fn_async arrow', await call_fn_async(reportArrow));
}

await main();

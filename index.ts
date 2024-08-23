import { CoreCrypto, CoreCryptoContext } from './pkg/core_crypto.js';


async function test() {
    var cc = new CoreCrypto();

    await cc.transaction({
        execute: async (ctx: CoreCryptoContext) => {
            let utf8 = new TextEncoder();
            console.log(await ctx.decrypt("conv1", utf8.encode("Hello World!")));
            try {
                await ctx.decrypt("id", utf8.encode("Will Fail"));
            } catch (error) {
                console.log("This is supposed to happen: ", error);
            }
        }
    })

}

await test();

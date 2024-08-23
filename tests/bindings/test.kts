import uniffi.core_crypto.*

val cc = CoreCrypto()

kotlinx.coroutines.runBlocking {
    var context: CoreCryptoContext? = null
    cc.transaction(
        object : CoreCryptoCommand {
            override suspend fun execute(ctx: CoreCryptoContext) {
                context = ctx
                ctx.decrypt("conv1", "Hello World!".toByteArray())
                var failed = false
                try {
                    ctx.decrypt("id", "Hello World!".toByteArray())
                } catch (e: Exception) {
                    println("Error ${e.message}")
                    failed = true
                }
                assert(failed)
            }
        },
    )
    // we can't block this on rust side, so we have to implement protections agasint it
    context!!.decrypt("conv1", "should fail miserably, but it won't".toByteArray())
}

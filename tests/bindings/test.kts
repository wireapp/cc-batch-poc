import uniffi.core_crypto.*

val cc = CoreCrypto()

suspend inline fun <R> CoreCrypto.niceTransaction(crossinline block: suspend (context: CoreCryptoContext) -> R): R? {
    var result: R? = null
    this.transaction(object : CoreCryptoCommand {
        override suspend fun execute(ctx: CoreCryptoContext) {
            result = block(ctx)
        }
    })
    return result
}

kotlinx.coroutines.runBlocking {
    var context: CoreCryptoContext? = null
    val result = cc.niceTransaction({ ctx: CoreCryptoContext ->
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
        5
    })
    println("result: $result")

    // we can't block this on rust side, so we have to implement protections against it
    context!!.decrypt("conv1", "should fail miserably, but it won't".toByteArray())
}
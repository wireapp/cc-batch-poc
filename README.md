# POC of creating batch transactions on Core Crypto

This is a POC to test out how to provide a batch API in [Core Crypto](https://github.com/wireapp/core-crypto).
The main goal is evaluating how to expose the API through the FFI into Kotlin
and TypeScript.

## Running Kotlin

To run the Kotlin example first download these 2 jars:

```bash
curl -o kotlinx-coroutines-core-jvm.jar https://repo1.maven.org/maven2/org/jetbrains/kotlinx/kotlinx-coroutines-core-jvm/1.6.4/kotlinx-coroutines-core-jvm-1.6.4.jar
curl -o jna.jar https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar
```

Then run:

```bash
CLASSPATH="kotlinx-coroutines-core-jvm.jar:jna.jar" cargo test -- --nocapture
```

## Running TypeScript

To run the TypeScript example ensure you have installed:

```bash
cargo install wasm-bindgen-cli
```

And build the Wasm binary:

```bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen-cli --out-dir pkg --target deno ./target/wasm32-unknown-unknown/release/core_crypto.wasm

```

Lastly run:

```bash
deno run --allow-read index.ts
```

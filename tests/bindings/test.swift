import core_crypto
import Foundation

var task = DispatchGroup()
var cc = CoreCrypto()
task.enter()
Task {
    if let result = try! await cc.niceTransaction({ context in
        print(try await context.decrypt(convId: "conv1", msg: Data("hello world".utf8)))
        var failed = false
        do {
            print(try await context.decrypt(convId: "id", msg: Data("hello world".utf8)))
        } catch {
            failed = true
            print("Expected error: \(error)")
        }
        assert(failed)
        return 5
    }) {
        print("result: \(result)")
    }
    task.leave()
}
task.wait()

class TransactionExecutor<Result>: CoreCryptoCommand {
    let block: (_ context: CoreCryptoContext) async throws -> Result
    var result: Result?

    init(_ block: @escaping (_ context: CoreCryptoContext) async throws -> Result) {
        self.block = block
    }

    func execute(context: CoreCryptoContext) async throws {
        result = try await block(context)
    }
}

extension CoreCrypto {
    func niceTransaction<Result>(_ block: @escaping (_ context: CoreCryptoContext) async throws -> Result) async throws -> Result? {
        let transactionExecutor = TransactionExecutor<Result>(block)
        try await transaction(command: transactionExecutor)
        return transactionExecutor.result
    }
}

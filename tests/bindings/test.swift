import core_crypto
import Foundation

var task = DispatchGroup()
var cc = CoreCrypto()
task.enter()
Task {
    class TestTransaction: CoreCryptoCommand {
        func execute(context: CoreCryptoContext) async throws {
            print(try await context.decrypt(convId: "conv1", msg: Data("hello world".utf8)))
            var failed = false
            do {
                print(try await context.decrypt(convId: "id", msg: Data("hello world".utf8)))
            } catch {
                failed = true
                print("Expected error: \(error)")
            }
            assert(failed)
        }
    }
    try! await cc.transaction(command: TestTransaction())
    task.leave()
}
task.wait()

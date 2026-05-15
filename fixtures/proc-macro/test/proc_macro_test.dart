import 'dart:typed_data';

import 'package:test/test.dart';
import '../proc_macro.dart';

class DartOtherCallback implements OtherCallbackInterface {
  @override
  int multiply(int a, int b) => a * b;
}

class DartTestCallback implements TestCallbackInterface {
  var didNothing = false;

  @override
  void doNothing() {
    didNothing = true;
  }

  @override
  int add(int a, int b) => a + b;

  @override
  int optional(int? a) => a ?? -1;

  @override
  Uint8List withBytes(RecordWithBytes rwb) {
    return Uint8List.fromList(rwb.someBytes.reversed.toList());
  }

  @override
  int tryParseInt(String value) {
    if (value == 'bad') {
      throw InvalidInputBasicException();
    }
    return int.parse(value);
  }

  @override
  int callbackHandler(Object o) {
    return o.isHeavy() == MaybeBool.uncertain ? 42 : -1;
  }

  @override
  OtherCallbackInterface getOtherCallbackInterface() => DartOtherCallback();
}

void main() {
  group('ProcMacro', () {
    test(
      'UDL and proc-macro interoperability',
      () {
        // This massive test will fail until comprehensive proc-macro support is implemented
        // Expected functionality:
        // - UDL types used in proc-macros (Zero -> make_zero())
        // - Proc-macro types used in UDL (One, MaybeBool, Object, etc.)
        // - Traits with foreign implementations
        // - Records, enums, objects, errors all working together

        // Basic record operations
        // final one = makeOne(42);
        // expect(one.inner, equals(42));

        // final two = Two(a: "hello");
        // expect(takeTwo(two), equals("hello"));

        // HashMap operations
        // final map = makeHashmap(1, 100);
        // expect(map[1], equals(100));

        // Enum operations
        // final obj = Object();
        // expect(obj.isHeavy(), equals(MaybeBool.uncertain));

        // Error handling
        // expect(() => alwaysFails(), throwsA(isA<BasicError>()));

        // UDL-defined functions using proc-macro types
        // final retrieved = getOne(null);
        // expect(retrieved.inner, equals(0));

        // final bool = getBool(null);
        // expect(bool, equals(MaybeBool.uncertain));
      },
      skip:
          'Blocked by comprehensive proc-macro support: '
          '#[derive(uniffi::Record)], #[derive(uniffi::Object)], #[derive(uniffi::Enum)], '
          '#[derive(uniffi::Error)], #[uniffi::export], #[uniffi::export(with_foreign)], '
          'trait definitions, HashMap support, and UDL/proc-macro interoperability',
    );

    test('callback interfaces', () {
      final callback = DartTestCallback();

      callbackDoNothing(callback: callback);
      expect(callback.didNothing, isTrue);
      expect(callbackAdd(callback: callback, a: 2, b: 3), 5);
      expect(callbackOptional(callback: callback, value: null), -1);
      expect(callbackOptional(callback: callback, value: 9), 9);
      expect(
        callbackWithBytes(
          callback: callback,
          bytes: Uint8List.fromList([1, 2, 3]),
        ),
        [3, 2, 1],
      );
      expect(callbackTryParseInt(callback: callback, value: '42'), 42);
      expect(callbackHandler(callback: callback), 42);
      expect(callbackGetOtherMultiply(callback: callback, a: 6, b: 7), 42);
    });

    test('callback interfaces preserve expected errors', () {
      final callback = DartTestCallback();

      expect(
        () => callbackTryParseInt(callback: callback, value: 'bad'),
        throwsA(isA<InvalidInputBasicException>()),
      );
    });

    test(
      'trait objects',
      () {
        // This test will fail until trait object support is implemented
        // Expected functionality:
        // - Trait definitions with #[uniffi::export]
        // - Trait implementations
        // - Trait objects as parameters and return values
      },
      skip:
          'Blocked by trait object support: #[uniffi::export] traits and Arc<dyn Trait>',
    );
  });
}

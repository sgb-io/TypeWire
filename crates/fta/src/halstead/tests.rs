#[cfg(test)]
mod tests {
    use crate::halstead::analyze_module;
    use crate::parse::parse_module;
    use crate::structs::HalsteadMetrics;
    use swc_ecma_ast::Module;

    fn parse(ts_code: &str) -> Module {
        let (parsed_module, _line_count) = parse_module(ts_code, true);

        if let Ok(parsed_module) = parsed_module {
            parsed_module
        } else {
            panic!("failed");
        }
    }

    fn analyze(module: &Module) -> HalsteadMetrics {
        let metrics = analyze_module(module);
        metrics
    }

    #[test]
    fn test_empty_module() {
        let ts_code = r#"
            /* Empty TypeScript code */
        "#;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 0,
            uniq_operands: 0,
            total_operators: 0,
            total_operands: 0,
            program_length: 0,
            vocabulary_size: 0,
            volume: 0.0,
            difficulty: 0.0,
            effort: 0.0,
            time: 0.0,
            bugs: 0.0,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_switch_case() {
        let ts_code = r#"
            switch (x) {
                case 0:
                    console.log("x is 0");
                    break;
                case 1:
                    console.log("x is 1");
                    break;
                default:
                    console.log("x is not 0 or 1");
            }
        "#;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 3,
            uniq_operands: 8,
            total_operators: 9,
            total_operands: 12,
            program_length: 11,
            vocabulary_size: 21,
            volume: 48.315491650566365,
            difficulty: 2.6666666666666665,
            effort: 128.84131106817696,
            time: 7.15785061489872,
            bugs: 0.016105163883522122,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_complex_case_a() {
        let ts_code = r##"
        import { React, useState } from 'react';
        import { asyncOperation } from './asyncOperation';

        let staticFoo = true;

        function displayThing(thing: string) {
          return `thing: ${thing}`;
        }

        export default function DummyComponent() {
          const [thing, setThing] = useState(null);

          const thingForDisplay = displayThing(thing) as string;

          const interact = async () => {
            const result = await asyncOperation();
            setThing(result);
            staticFoo = false;

            if (typeof thing === 'object' && thing?.foo?.bar) {
              console.log('This should not happen');
            }
          }

          const baz = staticFoo ? 32 : 42;

          return (
            <>
              <div>
                <h1>Hello World</h1>
              </div>
              <div>
                <h2>This is a test. {thingForDisplay} {baz}</h2>
                <button onClick={interact}>Click me</button>
              </div>
            </>
          )
        }
      "##;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 21,
            uniq_operands: 26,
            total_operators: 43,
            total_operands: 47,
            program_length: 47,
            vocabulary_size: 90,
            volume: 305.1170955274947,
            difficulty: 11.617021276595745,
            effort: 3544.5517905960023,
            time: 196.91954392200012,
            bugs: 0.1017056985091649,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_complex_case_c() {
        let ts_code = r##"
        let a, b, c = 3;
        a = 1;
        b = 2;
        let myArray = [a, b, c];

        myArray = [...myArray, ...myArray, 8, 9, 10];

        const myObject = {
          foo: 'bar'
        }

        const myOtherObject = {
          ...myObject,
          bar: 'baz'
        }

        class Foo {
          constructor() {
            this.foo = 'some value';
          }

          getFoo() {
            return this.foo!;
          }

          isFooCool() {
            const myRegex = /cool/;
            return myRegex.test(this.foo);
          }
        }

        const myFoo = new Foo();

        export { myFoo, myOtherObject };
      "##;

        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 10,
            uniq_operands: 25,
            total_operators: 31,
            total_operands: 44,
            program_length: 35,
            vocabulary_size: 75,
            volume: 218.00865416735581,
            difficulty: 8.522727272727273,
            effort: 1858.0283025626918,
            time: 103.22379458681621,
            bugs: 0.07266955138911861,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_complex_case_d() {
        let ts_code = r##"
        // Covers 'visit_export_decl'
        export declare const foo = 42;

        // Covers 'visit_tpl'
        const tpl = `result is ${binResult}`;

        // Covers 'visit_ts_mapped_type'
        type MappedType = { [P in keyof any]: P };

        // Covers 'visit_ts_indexed_access_type'
        type AccessType = MappedType["key"];

        // Covers 'visit_ts_type_operator'
        type NewType = keyof any;

        // Covers 'visit_tpl'
        const person = "Mike";
        const age = 28;
        function myTag(strings, personExp, ageExp) {
          const str0 = strings[0]; // "That "
          const str1 = strings[1]; // " is a "
          const str2 = strings[2]; // "."

          const ageStr = ageExp > 99 ? "centenarian" : "youngster";

          // We can even return a string built using a template literal
          return `${str0}${personExp}${str1}${ageStr}${str2}`;
        }
        const output = myTag`That ${person} is a ${age}.`;
        "##;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 15,
            uniq_operands: 27,
            total_operators: 39,
            total_operands: 41,
            program_length: 42,
            vocabulary_size: 80,
            volume: 265.5209799852692,
            difficulty: 12.512195121951219,
            effort: 3322.2503105473925,
            time: 184.56946169707737,
            bugs: 0.08850699332842307,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_complex_case_e() {
        let ts_code = r##"
        // visit_bin_expr
        let a = 5 + 3;
        
        // visit_unary_expr
        let b = !true;
        
        // visit_assign_expr
        let c = 10;
        c += a;
        
        // visit_update_expr
        c++;
        
        // visit_call_expr
        console.log(c);
        
        // visit_new_expr
        let obj = new Date();
        
        // visit_lit
        let str = "test";
        let num = 1;
        let bool = true;
        let reg = /ab+c/;
        let nullLit = null;
        
        // visit_arrow_expr
        let add = (x: number, y: number) => x + y;
        
        // visit_tagged_tpl
        let person = "John";
        let greeting = `Hello ${person}`;
        
        // visit_spread_element
        let arr1 = [1, 2, 3];
        let arr2 = [...arr1, 4, 5];
        
        // visit_ts_non_null_expr
        let maybeString: string | null = "Hello";
        let str2 = maybeString!;
        
        // visit_ts_type_assertion
        let someValue: unknown = "this is a string";
        let strLength: number = (someValue as string).length;
        
        // visit_ts_as_expr
        let anotherValue: unknown = "this is another string";
        
        // visit_ts_qualified_name 
        namespace A {
          export namespace B {
            export const message = "Hello, TypeScript!";
          }
        }
        console.log(A.B.message);
        
        // visit_cond_expr
        let condition = true ? "truthy" : "falsy";
        
        // visit_await_expr
        async function foo() {
          let result = await Promise.resolve("Hello, world!");
          console.log(result);
        }
        foo();
        
        // visit_yield_expr
        function* generator() {
          yield 'yielding a value';
        }
        
        // visit_meta_prop_expr
        function check() {
          if (new.target) {
            console.log('Function was called with "new" keyword');
          } else {
            console.log('Function was not called with "new" keyword');
          }
        }
        check();
        
        // visit_seq_expr
        let seq = (console.log('first'), console.log('second'), 'third');

        let a = 5; // visit_assign_expr
        let b = -a; // visit_unary_expr
        let c = a + b; // visit_bin_expr
        let d = ++c; // visit_update_expr
        let e = Math.sqrt(d); // visit_call_expr
        let f = new String(e); // visit_new_expr
        let g = "hello"; // visit_lit
        let h = (x: number) => x * 2; // visit_arrow_expr
        let arr = [...h]; // visit_spread_element
        let j: number! = 5; // visit_ts_non_null_expr
        let cond = (a > b) ? a : b; // visit_cond_expr
        async function asyncFunc() {
            let result = await Promise.resolve(true); // visit_await_expr
            return result;
        }
        function* generatorFunc() {
            yield 'hello'; // visit_yield_expr
            yield* arr; // visit_yield_expr
        }
        const meta = new.target; // visit_meta_prop_expr
        const seq = (1, 2, 3, 4, 5); // visit_seq_expr
        "##;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 28,
            uniq_operands: 75,
            total_operators: 130,
            total_operands: 139,
            program_length: 103,
            vocabulary_size: 269,
            volume: 831.3606233433322,
            difficulty: 35.07194244604317,
            effort: 29157.43193380392,
            time: 1619.8573296557734,
            bugs: 0.27712020778111074,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_complex_case_f() {
        let ts_code = r##"
      const obj = {
        prop1: 123,
        prop2: "hello",
        prop3: () => {
          console.log("Method prop");
        },
      };
      
      const fn: () => void = obj.prop3;
      
      const jsxElement = (
        <div>
          <h1>Hello</h1>
        </div>
      );
      "##;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 7,
            uniq_operands: 13,
            total_operators: 13,
            total_operands: 17,
            program_length: 20,
            vocabulary_size: 30,
            volume: 98.13781191217038,
            difficulty: 4.588235294117647,
            effort: 450.27937230289933,
            time: 25.015520683494408,
            bugs: 0.03271260397072346,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_complex_case_g() {
        let ts_code = r##"
        const value: any = "123";
        const result = value as number;
        const obj: MyNamespace.MyClass = new MyNamespace.MyClass();

        const obj = {
          prop1: {
            nested: {
              value: 42,
            },
          },
          prop2: [1, 2, 3],
        };
        console.log(obj.prop1.nested.value);
        console.log(obj.prop2[0]);
      "##;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 6,
            uniq_operands: 16,
            total_operators: 22,
            total_operands: 27,
            program_length: 22,
            vocabulary_size: 49,
            volume: 123.52361657053459,
            difficulty: 6.518518518518518,
            effort: 805.1909820894106,
            time: 44.73283233830059,
            bugs: 0.04117453885684486,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_complex_case_h() {
        let ts_code = r##"
        const obj = {
          prop1: "value1",
          prop2: {
            nested: "value2",
          },
          prop3() {
            return "value3";
          },
          prop4: 42,
          prop5,
          prop6: {
            nestedMethod() {
              return "nestedValue";
            },
          },
          prop7: "value7",
          prop8 = "value8"
        };
        
        const prop5 = "value5";
        
        console.log(obj.prop1);
        console.log(obj.prop2.nested);
        console.log(obj.prop3());
        console.log(obj.prop4);
        console.log(obj.prop5);
        console.log(obj.prop6.nestedMethod());
      "##;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 6,
            uniq_operands: 21,
            total_operators: 41,
            total_operands: 46,
            program_length: 27,
            vocabulary_size: 87,
            volume: 173.95947438791566,
            difficulty: 9.130434782608695,
            effort: 1588.3256357157516,
            time: 88.24031309531954,
            bugs: 0.057986491462638554,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_complex_case_i() {
        let ts_code = r##"
        let obj = {
          ['computed' + 'Property']: 'value'
        };
        
        class MyClass {
          [Symbol.iterator]() {}
        }

        class MyClassTwo {
          #privateField = 'value';

          getPrivateField() {
            return this.#privateField;
          }
        }
      "##;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 6,
            uniq_operands: 11,
            total_operators: 11,
            total_operands: 13,
            program_length: 17,
            vocabulary_size: 24,
            volume: 77.94436251225966,
            difficulty: 4.230769230769231,
            effort: 329.7646106287909,
            time: 18.32025614604394,
            bugs: 0.025981454170753218,
        };
        assert_eq!(analyze(&module), expected);
    }
}

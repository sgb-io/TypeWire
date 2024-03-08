#[cfg(test)]
mod tests {
    use crate::halstead::analyze_module;
    use crate::parse::parse_module;
    use crate::structs::HalsteadMetrics;
    use swc_ecma_ast::Module;

    fn parse(ts_code: &str) -> Module {
        let (parsed_module, _line_count) = parse_module(ts_code, true, false);

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
            program_length: 21,
            vocabulary_size: 11,
            volume: 72.64806399138324,
            difficulty: 1.5,
            effort: 108.97209598707485,
            time: 6.05400533261527,
            bugs: 0.02421602133046108,
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
            program_length: 90,
            vocabulary_size: 47,
            volume: 499.9129966509874,
            difficulty: 18.076923076923077,
            effort: 9036.888785614003,
            time: 502.0493769785557,
            bugs: 0.16663766555032913,
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
            program_length: 75,
            vocabulary_size: 35,
            volume: 384.6962262708725,
            difficulty: 8.8,
            effort: 3385.3267911836783,
            time: 188.07371062131546,
            bugs: 0.12823207542362416,
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
            program_length: 80,
            vocabulary_size: 42,
            volume: 431.38539382230084,
            difficulty: 10.62962962962963,
            effort: 4585.466963962976,
            time: 254.74816466460976,
            bugs: 0.1437951312741003,
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
            program_length: 269,
            vocabulary_size: 103,
            volume: 1798.6686418122858,
            difficulty: 25.946666666666665,
            effort: 46669.45569288944,
            time: 2592.7475384938575,
            bugs: 0.5995562139374286,
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
            program_length: 30,
            vocabulary_size: 20,
            volume: 129.65784284662087,
            difficulty: 3.923076923076923,
            effort: 508.6576911675126,
            time: 28.258760620417366,
            bugs: 0.043219280948873624,
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
            program_length: 49,
            vocabulary_size: 22,
            volume: 218.51214931322758,
            difficulty: 5.0625,
            effort: 1106.2177558982146,
            time: 61.45654199434526,
            bugs: 0.0728373831044092,
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
            program_length: 87,
            vocabulary_size: 27,
            volume: 413.67521268822173,
            difficulty: 6.571428571428571,
            effort: 2718.437111951171,
            time: 151.02428399728728,
            bugs: 0.1378917375627406,
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
            program_length: 24,
            vocabulary_size: 17,
            volume: 98.09910819000814,
            difficulty: 3.5454545454545454,
            effort: 347.80592903730155,
            time: 19.32255161318342,
            bugs: 0.032699702730002715,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn comments_have_no_impact_on_metrics() {
        let uncommented_code = r##"
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
        let commented_code = r##"
        // Define an object with a computed property
        let obj = {
            // The property name is the result of concatenating 'computed' and 'Property'
            ['computed' + 'Property']: 'value' // The value of the property is 'value'
        };
        
        // Define a class named MyClass
        class MyClass {
            /*
            *  Define a method with a computed name
            *  In this case, the method name is Symbol.iterator, which is a built-in symbol
            */ 
            [Symbol.iterator]() {} // The method is currently empty
        }
        
        // Define a class named MyClassTwo
        class MyClassTwo {
            // Define a private field named #privateField
            // The # syntax is used to denote private fields in JavaScript
            #privateField = 'value'; // The initial value of the field is 'value'
        
            // Define a method named getPrivateField
            getPrivateField() {
                // Return the value of the private field #privateField
                return this.#privateField;
            }
        }
      "##;
        let commented_code_module = parse(commented_code);
        let uncommented_code_module = parse(uncommented_code);
        assert_eq!(
            analyze(&commented_code_module),
            analyze(&uncommented_code_module)
        );
    }
}

import {greet, age_comparator} from './pkg/rust_wasm_package'

console.log(greet(`mani`))
console.log(age_comparator(18))
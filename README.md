[![HitCount](http://hits.dwyl.com/pepsi/v8test.svg)](http://hits.dwyl.com/pepsi/v8test)
![Lines of Code](https://tokei.rs/b1/github/pepsi/v8test?category=code)

`v8test` (Future `key`) is an experiment im doing with rust. Trying to make a small time node/deno "clone".
If you have any code improvements, or general ideas feel  free to make a pull request/issue respectively.


Current plans for std

STD
===
println(...data: Any[]) => void
print(...data: Any[]) => void
assert(assertion, message) => void
open() => File
exists(filename) => Promise<boolean>
File
====
read()           => Promise<String>
write(content)   => Promise<void>
append(content)  => Promise<void>

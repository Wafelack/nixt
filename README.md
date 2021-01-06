<div align="center">

  Nixt
  ---
  Nixt is an interpreted lisp inspired programming language written in Rust

</div>

# Index

- [About](#about)
- [Installation](#installation)
- [Build](#build)
- [TODO](#todo)

# About

Nixt goal is to provide a fast and easy to learn scripting language that provides productivity due to its easy syntax and correct performances


# Examples

FizzBuzz : 
```lisp
(let fizzbuzz (func (n) {
  (let i 1)
  (while (<= i n ) {
    (if (= (% i 15) 0) ((print "FizzBuzz") 
      (if (= (% i 5) 0)  (
        (print "Buzz")
        (if (= (% i 3) 0) (
          (print "Fizz")
          (print i)
        )))
    )))
  })  
}))
```

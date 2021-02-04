## mut 关键字放哪怎么放

定义变量是否可变

```
let mut a = ... 
```

以及可变引用

```
let a = &mut b
```

是两种使用关键字`mut`的地方。下面是关于mut关键字的详细解释

```rust
#![allow(unused)]

fn two_muts() {
    let mut a = String::from("hello");
    let mut other = String::from("world");
    let mut b = &mut a;

    // Works because `b` is a `&mut`.  Truncates `a`
    b.truncate(0);

    // Works because `b` itself is mutable (`let mut b ...`)
    b = &mut other;
}

fn mut_ref() {
    let mut a = String::from("hello");
    let mut other = String::from("world");
    let b = &mut a;

    // Works because b is a `&mut`.  Truncates `a`
    b.truncate(0);

    // Error: cannot assign twice to immutable variable `b`
    // I.e. the lack of `mut b` means we can't reassign to `b`
    // b = &mut other;
}

fn mut_b() {
    let mut a = String::from("hello");
    let mut other = String::from("world");
    let mut b = &a;

    // Error: `b` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // I.e. the lack of `&mut` means we can't mutate through `b`
    // b.truncate(0);

    // Works because `b` itself is mutable (`let mut b ...`)
    b = &other;
}

fn no_mut() {
    let mut a = String::from("hello");
    let mut other = String::from("world");
    let b = &a;

    // Error: `b` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // I.e. the lack of `&mut` means we can't mutate through `b`
    // b.truncate(0);

    // Error: cannot assign twice to immutable variable `b`
    // I.e. the lack of `mut b` means we can't reassign to `b`
    // b = &other;
}
```


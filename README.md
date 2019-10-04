# rs_taskflow
A task runner with dependencies for rust and a half-hearted attempt at recreating https://github.com/cpp-taskflow/cpp-taskflow from a rust beginner. 

Example: 
```
let a = Flow::new_task(DefaultResolveable::new(&(|| -> i32 {1})));
let b = Flow::new_task(DefaultResolveable::new(&(|| -> i32 {2})));
let c = Flow::new_task(DefaultResolveable::new(&(|| -> i32 {3})));

Flow::dep(&a, &b);
Flow::dep(&b, &c);
let mut flow = Flow::build(vec![&a, &b, &c]);

flow.start();
```

# graph_rs

## Usage

To graph some demo equations:
```
cargo run -- --demo
```


To enter custom equations:
```
cargo run
```
- exponents must be integers

Can be compiled to Web Assembly and accessed in a browser at localhost:8000.
```
./serve.sh
```

![graph_rs_2](https://github.com/user-attachments/assets/7859fb34-b5bb-42c8-8ea6-66cfb95e900d)


-----------------------

#### TODO
- graph cubics with cubic bezier curve
- highlight points hovered
- don't always graph full x domain if it's off-screen in the y
- integrate egui
- docker image caching

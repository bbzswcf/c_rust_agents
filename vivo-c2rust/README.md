make clean
bear -- make all
../c2rust/target/debug/c2rust transpile --output-dir ./output ./compile_commands.json 
去alloc_test
加extern c和use pub
单独封装测试函数
trie的两个static函数去掉
//use std::collections::HashMap;
 // --
//use std::{cmp::Ordering, io};
//use std::cmp::Ordering;
//use std::io;
 // --
use std::io::{self, Write};
//use std::io;
//use std::io::Write;
 // --
use std::collections::*; // Так подключаются все элементы по пути std::collections
// Чаще всего * используется при тестировании для подключения всего что есть в модуле tests
// Также используем * для импорта prelude - набора самах используемых инструментов какого либо модуля
 // --
// Так можно использовать use, эффективнее расходуя место, выделяя общую часть

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
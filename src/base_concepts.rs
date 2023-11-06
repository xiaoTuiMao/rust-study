use rand::Rng;
use std::io;

/**
 * 变量可变性示例
 */
fn practice_1 () {
	let mut y = 1;
	y = 2;
	let mut x = 3;
	let x = 'a';
	println!("y is {}, x is {}", y, x);
}

/**
 * ”隐藏“示例
 */
fn practice_2 () {
	let x = 5;
	// 重新创建变量 x，其值为第一个 x 的值 +1，即为 6
	let x = x + 1;

	{
		// 当前这个块级作用域下定义的 x 会遮蔽上面定义的第二个 x 变量，此时定义的 x 变量值为 12
			let x = x * 2;
			println!("The value of x in the inner scope is: {x}");
	}
// 块级作用域结束，第二个 x 遮蔽结束，输出 6
	println!("The value of x is: {x}");
}

/**
 * 元组示例
 */
fn practice_3 () {
	let tup = (500, 'a', "a");
	let (x, y, z) = tup;
	println!("x is {}, y is {}, z is {}", x,y,z);
	// 还可以通过索引直接访问
	println!("x is {}, y is {}, z is {}", tup.0, tup.1, tup.2);

	let tup1 = (500, 'a', "a");
	// 报错，tup1 不可修改
	tup1.0 = 200;

	let mut tup2 = (500, 'a', "a");
	tup2.0 = 1;
	// 报错，不可修改类型
	tup2.1 = 2;
}

/**
 * if 示例
 */
fn practice_4 () {
	let a = 1;
	// 需要注意这里不可以使用 return 返回，只能使用表达式返回。同时返回的类型必须要一致
	let b = if a==1 { 3 } else { 4 };
	// 会报错，因为前后返回的值类型不一致
	let c = if b==1 { 3 } else { 'x' };
}

/**
 * loop 示例
 */
fn practice_5 () {
	let mut count = 0;
	'counting_up: loop {
		println!("count = {count}");
		let mut remaining = 10;
		loop {
			println!("remaining = {remaining}");
			if remaining == 9 {
				// 跳出当前这个 loop 循环
				break;
			}
			if count == 2 {
				// 跳出第一个 loop 循环
				break 'counting_up;
			}
			remaining -= 1;
		}
		count += 1;
	}
	println!("End count = {count}");
}

/**
 * while for 循环示例
 */
fn practice_6 () {
	let a = [10, 20, 30, 40, 50];
	let mut index = 0;

	while index < 5 {
		println!("the value is: {}", a[index]);
		index += 1;
	}
	for element in a {
		println!("the value is: {element}");
	}
}

/**
 * 练习题
 * 实现一个猜字游戏，利用[rand](https://crates.io/crates/rand)生成一个0-100的随机数，
 * 读取用户的输入，根据用户的输入反馈是否符合要求。如果输入过大，则输出“输入值过大”，等待用户再次输入；输入过小，
 * 则输出“输入值过小”，等待用户再次输入；当输入值匹配时，输出“恭喜！回答正确！”，并结束程序。
 */
pub fn base_concepts_main () {
	let target: i32 = rand::thread_rng().gen_range(0..=100);

	loop {
		println!("请输入 0-100 数字");
		// 读取用户输入并赋值给 guess
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("Failed to read line");
		let guess: i32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("输入字符非法！请重新输入");
				continue;
			},
		};

		if guess == target {
			println!("恭喜你，猜对了！");
			break;
		}

		if guess < target {
			println!("猜小了，请重新输入");
			continue;
		}

		if guess > target {
			println!("猜大了，请重新输入");
			continue;
		}
	}
}


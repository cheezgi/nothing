//Copyleft 2014 Zack Hixon
//nothing.rs - does absolutely nothing.

fn main() {
	let mut nothing = 0;
	let mut zilch = false;
	if check_if_nothing(nothing) {
		zilch = true;
	}
	while zilch || nothing <= 10 {
		nothing += 1;
		if nothing == 10 {
			zilch = false;
			break;
		}
	}
}

fn check_if_nothing(nada: int) -> bool {
	if nada == 0 {
		return true;
	}
	else { return false; }
}

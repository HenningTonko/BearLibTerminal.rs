#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeyCode {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	Row1,
	Row2,
	Row3,
	Row4,
	Row5,
	Row6,
	Row7,
	Row8,
	Row9,
	Row0,
	Grave,
	Minus,
	Equals,
	LeftBracket,
	RightBracket,
	Backslash,
	Semicolon,
	Apostrophe,
	Comma,
	Period,
	Slash,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	Enter,
	Escape,
	Backspace,
	Tab,
	Space,
	Pause,
	Insert,
	Home,
	PageUp,
	Delete,
	End,
	PageDown,
	Right,
	Left,
	Down,
	Up,
	NumDivide,
	NumMultiply,
	NumMinus,
	NumPlus,
	NumEnter,
	NumPeriod,
	Num1,
	Num2,
	Num3,
	Num4,
	Num5,
	Num6,
	Num7,
	Num8,
	Num9,
	Num0,
	MouseLeft,
	MouseRight,
	MouseMiddle,
	MouseFourth,
	MouseFifth,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Event {
	Close,
	Resize,
	MouseMove{x: i32, y: i32},
	MouseScroll{delta: i32},
	KeyPressed{key: KeyCode, ctrl: bool, shift: bool},
	KeyReleased{key: KeyCode, ctrl: bool, shift: bool},
}

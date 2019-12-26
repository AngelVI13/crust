use std::collections::HashMap;

// Name of the engine
pub const NAME: str = "Crust 1.0";
// The total number of squares in the board representation
pub const BOARD_SQUARE_NUM: usize = 120;
// The number of rows/cols on the board
pub const ROW_SIZE: usize = 8;
// The number of squares for a normal chess board i.e. 8x8
pub const INNER_SQUARE_NUM: usize = ROW_SIZE * ROW_SIZE;
// Opening book filename
pub const BOOK_FILENAME: str = "board/book.txt";
// maximum number halfmoves allowed
pub const MAX_GAME_MOVES: u16 = 2048;

pub enum PieceType {
    Empty,
	WhitePawn,
	WhiteKnight,
	WhiteBishop,
	WhiteRook,
	WhiteQueen,
	WhiteKing,
	BlackPawn,
	BlackKnight,
	BlackBishop,
	BlackRook,
	BlackQueen,
	BlackKing,
}

pub enum Ranks {
    Rank1,
	Rank2,
	Rank3,
	Rank4,
	Rank5,
	Rank6,
	Rank7,
	Rank8,
	RankNone,
}

pub enum Files {
    FileA,
	FileB,
	FileC,
	FileD,
	FileE,
	FileF,
	FileG,
	FileH,
	FileNone,
}

pub enum Colours {
    White,
    Black,
    Both,
}

// Enum for board square indexes
pub enum Squares {
	// Rank 1
	A1 = 21,            // iota = 0, A1 = 21
	B1,                 // iota = 1
	C1,                 // iota = 2
	D1,                 // iota = 3
	E1,                 // iota = 4
	F1,                 // iota = 5
	G1,                 // iota = 6
	H1,                 // iota = 7
	// Rank 2
	A2 = 31,            // iota = 8
	B2,                 // iota = 9
	C2,                 // iota = 10
	D2,                 // iota = 11
	E2,                 // iota = 12
	F2,                 // iota = 13
	G2,                 // iota = 14
	H2,                 // iota = 15
	// Rank 3
	A3 = 41,            // iota = 16
	B3,                 // iota = 17
	C3,                 // iota = 18
	D3,                 // iota = 19
	E3,                 // iota = 20
	F3,                 // iota = 21
	G3,                 // iota = 22
	H3,                 // iota = 23
	// Rank 4
	A4 = 51,            // 51
	B4,
	C4,
	D4,
	E4,
	F4,
	G4,
	H4,
	// Rank 5
	A5 = 61,            // 61
	B5,
	C5,
	D5,
	E5,
	F5,
	G5,
	H5,
	// Rank 6
	A6 = 71,            // 71
	B6,
	C6,
	D6,
	E6,
	F6,
	G6,
	H6,
	// Rank 7
	A7 = 81,            // 81
	B7,
	C7,
	D7,
	E7,
	F7,
	G7,
	H7,
	// Rank 8
	A8 = 91,            // 91
	B8,
	C8,
	D8,
	E8,
	F8,
	G8,
	H8,
	// No square
	NoSquare, // 99
	OffBoard, // 100
}

// Defines for castling rights
// The values are such that they each represent a bit from a 4 bit int value
// for example if white can castle kingside and black can castle queenside
// the 4 bit int value is going to be 1001
pub const WHITE_KING_CASTLING:  u8 = 1;
pub const WHITE_QUEEN_CASTLING: u8 = 2;
pub const BLACK_KING_CASTLING:  u8 = 4;
pub const BLACK_QUEEN_CASTLING: u8 = 8;

pub struct Undo {
    move_: u32,
    castlePermissions: u32,
    enPassantSquare: u8,
    fiftyMove: u32, // todo type ?
    posKey: u64,
}

// Sq120ToSq64 would return the index of 120 mapped to a 64 square board
pub let mut Sq120ToSq64: [u8; BOARD_SQUARE_NUM];
// Sq64ToSq120 would return the index of 64 mapped to a 120 square board
pub let mut Sq64ToSq120: [u8; INNER_SQUARE_NUM];

// FileRankToSquare converts give file and rank to a square index
pub fn FileRankToSquare(file, rank: u8) -> u8 {
    return (21 + file) + (rank * 10);
}

// PieceKeys hashkeys for each piece for each possible position for the key
// todo is this correct ? : original - [13][BoardSquareNum]uint64
pub let mut PieceKeys: [[u64; BoardSquareNum]; 13];

// SideKey the hashkey associated with the current side
pub let mut SideKey: u64;

// CastleKeys haskeys associated with castling rights
// - castling value ranges from 0-15 -> we need 16 hashkeys
pub let mut CastleKeys: [u64; 16] 

// StartFen starting position in fen notation
pub const StartFen: str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

// PieceNotationMap maps piece notations (i.e. 'p', 'N') to piece values (i.e. 'BlackPawn', 'WhiteKnight')
pub let mut PieceNotationMap = HashMap::new();
PieceNotationMap.insert("p", PieceType.BlackPawn);
PieceNotationMap.insert("r", PieceType.BlackRook);
PieceNotationMap.insert("n", PieceType.BlackKnight);
PieceNotationMap.insert("b", PieceType.BlackBishop);
PieceNotationMap.insert("k", PieceType.BlackKing);
PieceNotationMap.insert("q", PieceType.BlackQueen);
PieceNotationMap.insert("P", PieceType.WhitePawn);
PieceNotationMap.insert("R", PieceType.WhiteRook);
PieceNotationMap.insert("N", PieceType.WhiteKnight);
PieceNotationMap.insert("B", PieceType.WhiteBishop);
PieceNotationMap.insert("K", PieceType.WhiteKing);
PieceNotationMap.insert("Q", PieceType.WhiteQueen);

// FileNotationMap maps file notations (i.e. 'a', 'h') to file values (i.e. 'FileA', 'FileH')
pub let mut FileNotationMap = HashMap::new();
FileNotationMap.insert("a", Files.FileA);
FileNotationMap.insert("b", Files.FileB);
FileNotationMap.insert("c", Files.FileC);
FileNotationMap.insert("d", Files.FileD);
FileNotationMap.insert("e", Files.FileE);
FileNotationMap.insert("f", Files.FileF);
FileNotationMap.insert("g", Files.FileG);
FileNotationMap.insert("h", Files.FileH);

// FilesBoard an array that returns which file a particular square is on
pub let mut FilesBoard: [i8; BOARD_SQUARE_NUM]

// RanksBoard an array that returns which file a particular square is on
pub let mut RanksBoard: [i8; BOARD_SQUARE_NUM]

/* Game move - information stored in the move int from type Move
   | |-P|-|||Ca-||---To--||-From-|
0000 0000 0000 0000 0000 0111 1111 -> From - 0x7F
0000 0000 0000 0011 1111 1000 0000 -> To - >> 7, 0x7F
0000 0000 0011 1100 0000 0000 0000 -> Captured - >> 14, 0xF
0000 0000 0100 0000 0000 0000 0000 -> En passant capt - 0x40000
0000 0000 1000 0000 0000 0000 0000 -> PawnStart - 0x80000
0000 1111 0000 0000 0000 0000 0000 -> Promotion to what piece - >> 20, 0xF
0001 0000 0000 0000 0000 0000 0000 -> Castle - 0x1000000
*/

// FromSq - macro that returns the 'from' bits from the move int
pub fn FromSq(move: u32) -> u32 {
	return move & 0x7f;
}

// ToSq - macro that returns the 'to' bits from the move int
pub fn ToSq(move: u32) -> u32 {
	return (move >> 7) & 0x7f;
}

// Captured - macro that returns the 'Captured' bits from the move int
pub fn Captured(move: u32) -> u32 {
	return (move >> 14) & 0xf;
}

// Promoted - macro that returns the 'Promoted' bits from the move int
pub fn Promoted(move: u32) -> u32 {
	return (move >> 20) & 0xf;
}

// MoveFlagEnPass move flag that denotes if the capture was an enpass
pub const MOVE_FLAG_ENPASSANT: u32 = 0x40000;
// MoveFlagPawnStart move flag that denotes if move was pawn start (2x)
pub const MOVE_FLAG_PAWN_START: u32 = 0x80000;
// MoveFlagCastle move flag that denotes if move was castling
pub const MOVE_FLAG_CASTLE: u32 = 0x1000000;

// MaxPositionMoves maximum number of possible moves for a given position
pub const MaxPositionMoves: u16 = 256;

// MoveList a structure to hold all generated moves
pub struct MoveList {
	Moves: [u32; MaxPositionMoves],
	Count: int, // number of moves on the moves list
}

// NoMove signifies no move
pub const NO_MOVE: u8 = 0;

// SearchInfo struct to hold search related information
// todo uncomment this
// pub struct SearchInfo {
// 	StartTime time.Time
// 	StopTime  int
// 	TimeSet   bool
// 	movesToGo int
// 	infinite  bool // if this is true, do not stop search based on time but when the gui sends the stop command

// 	nodes uint64 // count of all positions that the engine visits in the search tree

// 	Quit    bool // if interrupt is sent -> quit
// 	Stopped bool

// 	GameMode     int  // see consts below
// 	PostThinking bool // if true, engine posts its thinking to the gui
// }

// Game Modes
pub enum GameModes {
    // UciMode mode using the UCI protocol
	UciMode,
	// ConsoleMode mode using the console for input
	ConsoleMode,
}

// PieceChar map from piece value to string representation
pub let mut PieceChar = HashMap::new();
PieceChar.insert(PieceType.Empty, ".");
PieceChar.insert(PieceType.WhitePawn, "P");
PieceChar.insert(PieceType.WhiteKnight, "N");
PieceChar.insert(PieceType.WhiteBishop, "B");
PieceChar.insert(PieceType.WhiteRook, "R");
PieceChar.insert(PieceType.WhiteQueen, "Q");
PieceChar.insert(PieceType.WhiteKing, "K");
PieceChar.insert(PieceType.BlackPawn, "p");
PieceChar.insert(PieceType.BlackKnight, "n");
PieceChar.insert(PieceType.BlackBishop, "b");
PieceChar.insert(PieceType.BlackRook, "r");
PieceChar.insert(PieceType.BlackQueen, "q");
PieceChar.insert(PieceType.BlackKing, "k");

// SideChar string with side characters
pub const SideChar: str = "wb-";

// PieceColour A map used to identify a piece's colour
pub let mut PieceColour = HashMap::new();
PieceColour.insert(PieceType.Empty, Colours.Both);
PieceColour.insert(PieceType.WhitePawn, Colours.White);
PieceColour.insert(PieceType.WhiteKnight, Colours.White);
PieceColour.insert(PieceType.WhiteBishop, Colours.White);
PieceColour.insert(PieceType.WhiteRook, Colours.White);
PieceColour.insert(PieceType.WhiteQueen, Colours.White);
PieceColour.insert(PieceType.WhiteKing, Colours.White);
PieceColour.insert(PieceType.BlackPawn, Colours.Black);
PieceColour.insert(PieceType.BlackKnight, Colours.Black);
PieceColour.insert(PieceType.BlackBishop, Colours.Black);
PieceColour.insert(PieceType.BlackRook, Colours.Black);
PieceColour.insert(PieceType.BlackQueen, Colours.Black);
PieceColour.insert(PieceType.BlackKing, Colours.Black);

// IsPieceKnight holds information if a given piece is a knight
pub let mut IsPieceKnight = HashMap::new();
IsPieceKnight.insert(PieceType.Empty, false);
IsPieceKnight.insert(PieceType.WhitePawn, false);
IsPieceKnight.insert(PieceType.WhiteKnight, true);
IsPieceKnight.insert(PieceType.WhiteBishop, false);
IsPieceKnight.insert(PieceType.WhiteRook, false);
IsPieceKnight.insert(PieceType.WhiteQueen, false);
IsPieceKnight.insert(PieceType.WhiteKing, false);
IsPieceKnight.insert(PieceType.BlackPawn, false);
IsPieceKnight.insert(PieceType.BlackKnight, true);
IsPieceKnight.insert(PieceType.BlackBishop, false);
IsPieceKnight.insert(PieceType.BlackRook, false);
IsPieceKnight.insert(PieceType.BlackQueen, false);
IsPieceKnight.insert(PieceType.BlackKing, false);

// IsPieceKing holds information if a given piece is a king
pub let mut IsPieceKing = HashMap::new();
IsPieceKing.insert(PieceType.Empty, false);
IsPieceKing.insert(PieceType.WhitePawn, false);
IsPieceKing.insert(PieceType.WhiteKnight, false);
IsPieceKing.insert(PieceType.WhiteBishop, false);
IsPieceKing.insert(PieceType.WhiteRook, false);
IsPieceKing.insert(PieceType.WhiteQueen, false);
IsPieceKing.insert(PieceType.WhiteKing, true);
IsPieceKing.insert(PieceType.BlackPawn, false);
IsPieceKing.insert(PieceType.BlackKnight, false);
IsPieceKing.insert(PieceType.BlackBishop, false);
IsPieceKing.insert(PieceType.BlackRook, false);
IsPieceKing.insert(PieceType.BlackQueen, false);
IsPieceKing.insert(PieceType.BlackKing, true);

// IsPieceRookQueen holds information if a given piece is a rook or queen
pub let mut IsPieceRookQueen = HashMap::new();
IsPieceRookQueen.insert(PieceType.Empty, false);
IsPieceRookQueen.insert(PieceType.WhitePawn, false);
IsPieceRookQueen.insert(PieceType.WhiteKnight, false);
IsPieceRookQueen.insert(PieceType.WhiteBishop, false);
IsPieceRookQueen.insert(PieceType.WhiteRook, true);
IsPieceRookQueen.insert(PieceType.WhiteQueen, true);
IsPieceRookQueen.insert(PieceType.WhiteKing, false);
IsPieceRookQueen.insert(PieceType.BlackPawn, false);
IsPieceRookQueen.insert(PieceType.BlackKnight, false);
IsPieceRookQueen.insert(PieceType.BlackBishop, false);
IsPieceRookQueen.insert(PieceType.BlackRook, true);
IsPieceRookQueen.insert(PieceType.BlackQueen, true);
IsPieceRookQueen.insert(PieceType.BlackKing, false);

// IsPieceBishopQueen holds information if a given piece is a bishop or queen
pub let mut IsPieceBishopQueen = HashMap::new();
IsPieceBishopQueen.insert(PieceType.Empty, false);
IsPieceBishopQueen.insert(PieceType.WhitePawn, false);
IsPieceBishopQueen.insert(PieceType.WhiteKnight, false);
IsPieceBishopQueen.insert(PieceType.WhiteBishop, true);
IsPieceBishopQueen.insert(PieceType.WhiteRook, false);
IsPieceBishopQueen.insert(PieceType.WhiteQueen, true);
IsPieceBishopQueen.insert(PieceType.WhiteKing, false);
IsPieceBishopQueen.insert(PieceType.BlackPawn, false);
IsPieceBishopQueen.insert(PieceType.BlackKnight, false);
IsPieceBishopQueen.insert(PieceType.BlackBishop, true);
IsPieceBishopQueen.insert(PieceType.BlackRook, false);
IsPieceBishopQueen.insert(PieceType.BlackQueen, true);
IsPieceBishopQueen.insert(PieceType.BlackKing, false);

// IsPiecePawn holds information if a given piece is a pawn
pub let mut IsPiecePawn = HashMap::new();
IsPiecePawn.insert(PieceType.Empty, false);
IsPiecePawn.insert(PieceType.WhitePawn, true);
IsPiecePawn.insert(PieceType.WhiteKnight, false);
IsPiecePawn.insert(PieceType.WhiteBishop, false);
IsPiecePawn.insert(PieceType.WhiteRook, false);
IsPiecePawn.insert(PieceType.WhiteQueen, false);
IsPiecePawn.insert(PieceType.WhiteKing, false);
IsPiecePawn.insert(PieceType.BlackPawn, true);
IsPiecePawn.insert(PieceType.BlackKnight, false);
IsPiecePawn.insert(PieceType.BlackBishop, false);
IsPiecePawn.insert(PieceType.BlackRook, false);
IsPiecePawn.insert(PieceType.BlackQueen, false);
IsPiecePawn.insert(PieceType.BlackKing, false);

// PieceDir squares increment for each direction
pub let mut PieceDir = HashMap::new();
PieceDir.insert(PieceType.Empty, [0, 0, 0, 0, 0, 0, 0]);
PieceDir.insert(PieceType.WhitePawn, [0, 0, 0, 0, 0, 0, 0]);
PieceDir.insert(PieceType.WhiteKnight, [-8, -19, -21, -12, 8, 19, 21, 12]);
PieceDir.insert(PieceType.WhiteBishop, [-9, -11, 11, 9, 0, 0, 0, 0]);
PieceDir.insert(PieceType.WhiteRook, [-1, -10, 1, 10, 0, 0, 0, 0]);
PieceDir.insert(PieceType.WhiteQueen, [-1, -10, 1, 10, -9, -11, 11, 9]);
PieceDir.insert(PieceType.WhiteKing, [-1, -10, 1, 10, -9, -11, 11, 9]);
PieceDir.insert(PieceType.BlackPawn, [0, 0, 0, 0, 0, 0, 0]);
PieceDir.insert(PieceType.BlackKnight, [-8, -19, -21, -12, 8, 19, 21, 12]);
PieceDir.insert(PieceType.BlackBishop, [-9, -11, 11, 9, 0, 0, 0, 0]);
PieceDir.insert(PieceType.BlackRook, [-1, -10, 1, 10, 0, 0, 0, 0]);
PieceDir.insert(PieceType.BlackQueen, [-1, -10, 1, 10, -9, -11, 11, 9]);
PieceDir.insert(PieceType.BlackKing, [-1, -10, 1, 10, -9, -11, 11, 9]);

// NumberOfDir number of directions in which each piece can move
pub let mut NumberOfDir = HashMap::new();
NumberOfDir.insert(PieceType.Empty, 0);
NumberOfDir.insert(PieceType.WhitePawn, 0);
NumberOfDir.insert(PieceType.WhiteKnight, 8);
NumberOfDir.insert(PieceType.WhiteBishop, 4);
NumberOfDir.insert(PieceType.WhiteRook, 4);
NumberOfDir.insert(PieceType.WhiteQueen, 8);
NumberOfDir.insert(PieceType.WhiteKing, 8);
NumberOfDir.insert(PieceType.BlackPawn, 0);
NumberOfDir.insert(PieceType.BlackKnight, 8);
NumberOfDir.insert(PieceType.BlackBishop, 4);
NumberOfDir.insert(PieceType.BlackRook, 4);
NumberOfDir.insert(PieceType.BlackQueen, 8);
NumberOfDir.insert(PieceType.BlackKing, 8);

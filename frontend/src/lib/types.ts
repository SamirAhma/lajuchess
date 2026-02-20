export type Uuid = string;

export interface CreateGameRequest {
	side: string;
}

export interface CreateGameResponse {
	id: Uuid;
}

export interface GameState {
	fen: string;
	pgn: string;
	creator_color: string;
}

export type ClientMessage =
	| {
		type: "Move", payload: {
			uci: string;
		}
	};

export type ServerMessage =
	| { type: "State", payload: GameState }
	| { type: "Error", payload: string };

/*
 * Copyright 2018 The boardgame.io Authors.
 *
 * Use of this source code is governed by a MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

import React from 'react';
import { Lobby } from 'boardgame.io/react';
import { default as BoardChess } from '../games/chess/board';
import { default as GameChess } from '../games/chess/game';
import './lobby.css';

GameChess.minPlayers = GameChess.maxPlayers = 2;

const hostname = window.location.hostname;
const importedGames = [
  { game: GameChess, board: BoardChess },
];

const LobbyView = () => (
  <div style={{ padding: 50 }}>
    <h1>Lobby</h1>

    <Lobby
      gameServer={`http://${hostname}:8000`}
      lobbyServer={`http://${hostname}:8000`}
      gameComponents={importedGames}
    />
  </div>
);

export default LobbyView;

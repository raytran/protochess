/*
 * Copyright 2017 The boardgame.io Authors
 *
 * Use of this source code is governed by a MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

import { Server } from 'boardgame.io/server';
import Chess from './games/chess/game';

const PORT = process.env.PORT || 8000;
const server = Server({ games: [Chess] });
server.run(PORT, () => {
  console.log(`Serving at: http://localhost:${PORT}/`);
});

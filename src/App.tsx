import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

const TILE_SIZE = 10;

function App() {
  const [game, setGame] = useState();
  // const [generatedMap, setGeneratedMap] = useState([]);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  // Setup event listener for game state updates
  useEffect(() => {
    let unlisten;

    const setupListener = async () => {
      unlisten = await listen("game_state_updated", (event) => {
        console.log("Game state updated:", event.payload);
        setGame(event.payload);
      });
    };

    setupListener();

    // Cleanup listener on unmount
    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  useEffect(() => {
    if (game) return;
    invoke("initialize_game")
      .then((x) => setGame(x))
      .catch((err) => console.log(err));
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext("2d");
    if (!ctx || !game) return;

    game.map?.forEach((tile) => {
      ctx.fillStyle =
        tile.owner === "Player"
          ? `rgba(63,113,212,${tile.value / 400})`
          : `rgba(255,85,0,${tile.value / 400})`;
      ctx.fillRect(
        tile.x * TILE_SIZE,
        tile.y * TILE_SIZE,
        TILE_SIZE,
        TILE_SIZE,
      );
    });

    return () => ctx.clearRect(0, 0, canvas?.width, canvas?.height);
  }, [game]);

  const handleCanvasClick = (event) => {
    const canvas = canvasRef.current;

    if (!canvas) return;

    const elemLeft = canvas.offsetLeft;
    const elemTop = canvas.offsetTop;
    const context = canvas.getContext("2d");

    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - elemLeft;
    const y = event.clientY - elemTop;

    // Przelicz współrzędne na tile
    const tileX = Math.floor(x / TILE_SIZE);
    const tileY = Math.floor(y / TILE_SIZE);

    console.log(tileX, tileY);
    console.log(x, y);
    console.log(event.clientX, elemLeft);
    console.log(event.clientY, elemTop);

    // Znajdź tile po współrzędnych
    const clickedTile = game.map?.find(
      (tile) => tile.x === tileX && tile.y === tileY,
    );

    if (clickedTile) {
      console.log("Clicked tile:", clickedTile);

      // Wywołaj buy_map_tile
      invoke("buy_map_tile_command", { tileId: clickedTile.id }).catch(
        (err) => {
          console.error("Failed to buy tile:", err);
        },
      );
    }
  };

  console.log(game);

  return (
    <main className="container">
      <canvas
        onClick={handleCanvasClick}
        ref={canvasRef}
        id="canvas"
        width="200"
        height="200"
      />
      <button onClick={() => invoke("end_turn")}>End turn</button>
    </main>
  );
}

export default App;

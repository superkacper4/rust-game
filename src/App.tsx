import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { getTileColour, getTileImage } from "./helpers";
import { SmallTileManagement } from "./components/SmallTileManagement";
import { TurnManagement } from "./components/TurnManagement";

const TILE_SIZE = 64;

function App() {
  const [game, setGame] = useState<Game | undefined>();
  const [selectedTile, setSelectedTile] = useState();
  const [hoveredTile, setHoveredTile] = useState();
  const baseCanvasRef = useRef<HTMLCanvasElement | null>(null);
  const overlayCanvasRef = useRef<HTMLCanvasElement | null>(null);

  // Setup event listener for game state updates
  useEffect(() => {
    let unlisten;

    const setupListener = async () => {
      unlisten = await listen("game_state_updated", (event) => {
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
    if (game) return;
    invoke("initialize_game")
      .then((x) => setGame(x))
      .catch((err) => console.log(err));
  }, []);

  // Draw base map ONLY when game changes
  useEffect(() => {
    const canvas = baseCanvasRef.current;
    const ctx = canvas?.getContext("2d");
    if (!ctx || !game) return;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    game.map?.forEach((tile) => {
      const image = new Image(TILE_SIZE, TILE_SIZE);
      image.onload = () => {
        ctx.drawImage(
          image,
          tile.x * TILE_SIZE,
          tile.y * TILE_SIZE,
          TILE_SIZE,
          TILE_SIZE,
        );
      };
      image.src = getTileImage(tile);
    });
  }, [game]);

  // Draw highlights ONLY when hover/selection changes
  useEffect(() => {
    const canvas = overlayCanvasRef.current;
    const ctx = canvas?.getContext("2d");
    if (!ctx || !game) return;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    if (hoveredTile) {
      ctx.strokeStyle = "yellow";
      ctx.lineWidth = 2;
      ctx.strokeRect(
        hoveredTile.x * TILE_SIZE,
        hoveredTile.y * TILE_SIZE,
        TILE_SIZE,
        TILE_SIZE,
      );
    }

    if (selectedTile) {
      ctx.strokeStyle = "red";
      ctx.lineWidth = 2;
      ctx.strokeRect(
        selectedTile.x * TILE_SIZE,
        selectedTile.y * TILE_SIZE,
        TILE_SIZE,
        TILE_SIZE,
      );
    }
  }, [hoveredTile, selectedTile, game]);

  const handleCanvasClick = async (event) => {
    const canvas = overlayCanvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;

    const x = (event.clientX - rect.left) * scaleX;
    const y = (event.clientY - rect.top) * scaleY;

    const tileX = Math.floor(x / TILE_SIZE);
    const tileY = Math.floor(y / TILE_SIZE);

    const clickedTile = game.map?.find(
      (tile) => tile.x === tileX && tile.y === tileY,
    );

    if (clickedTile) {
      console.log("Clicked tile: ", clickedTile.id);
      const tileObject = { tileId: clickedTile.id };

      const result = await invoke("check_if_player_owns_tile", tileObject);

      setSelectedTile({ ...tileObject, owned: result, x: tileX, y: tileY });
    }
  };

  const handleCanvasHover = (event) => {
    const canvas = overlayCanvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;

    const x = (event.clientX - rect.left) * scaleX;
    const y = (event.clientY - rect.top) * scaleY;

    const tileX = Math.floor(x / TILE_SIZE);
    const tileY = Math.floor(y / TILE_SIZE);

    setHoveredTile({ x: tileX, y: tileY });
  };

  console.log(game);

  return (
    <main className="Container">
      <canvas
        ref={baseCanvasRef}
        id="base-canvas"
        width={TILE_SIZE * 20}
        height={TILE_SIZE * 20}
        style={{ position: "absolute", top: 0, left: 0 }}
      />
      <canvas
        ref={overlayCanvasRef}
        id="overlay-canvas"
        width={TILE_SIZE * 20}
        height={TILE_SIZE * 20}
        onClick={handleCanvasClick}
        onMouseMove={handleCanvasHover}
        style={{ position: "absolute", top: 0, left: 0 }}
      />
      <SmallTileManagement
        setSelectedTile={setSelectedTile}
        selectedTile={selectedTile}
      />
      <TurnManagement game={game} />
    </main>
  );
}

export default App;

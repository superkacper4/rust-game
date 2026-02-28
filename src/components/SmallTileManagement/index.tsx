import { invoke } from "@tauri-apps/api/core";
import "./index.css";

export const SmallTileManagement = ({ setSelectedTile, selectedTile }) => {
  console.log(selectedTile);
  if (!selectedTile) return null;

  const handleClick = (onClick: () => void) => {
    onClick();
    setSelectedTile(null);
  };

  return (
    <div className="SmallTileManagement">
      {selectedTile.owned ? (
        <div>
          <button>sprzedaj</button>
          <button
            onClick={() =>
              handleClick(() =>
                invoke("change_building_for_tile", selectedTile),
              )
            }
          >
            apartamenty
          </button>
        </div>
      ) : (
        <div>
          <button
            onClick={() =>
              handleClick(() => invoke("buy_map_tile_command", selectedTile))
            }
          >
            Kup
          </button>
        </div>
      )}
    </div>
  );
};

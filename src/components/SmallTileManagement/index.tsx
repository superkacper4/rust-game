import { invoke } from "@tauri-apps/api/core";
import "./index.css";

export const SmallTileManagement = ({ setSelectedTile, selectedTile }) => {
  if (!selectedTile) return null;

  const handleClick = (onClick: () => void) => {
    onClick();
    setSelectedTile(null);
  };

  console.log(selectedTile);

  return (
    <div className="SmallTileManagement">
      {selectedTile.owned ? (
        <div>
          <button
            onClick={() =>
              handleClick(() => invoke("sell_map_tile_command", selectedTile))
            }
          >
            sprzedaj
          </button>
          <button
            onClick={() =>
              handleClick(() =>
                invoke("change_building_for_tile", {
                  buildingKind: "APARTMENT",
                  tileId: selectedTile.tileId,
                }),
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

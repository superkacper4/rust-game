import { invoke } from "@tauri-apps/api/core";
import "./index.css";

export const TurnManagement = ({ game }) => {
  return (
    <div className="TurnManagement">
      <button onClick={() => invoke("end_turn")}>End turn</button>
      <div className="PlayerStats">
        {game?.player.name} | {game?.player.cash} cent $ |{" "}
        {game?.player.actions_left_in_turn}{" "}
      </div>
    </div>
  );
};

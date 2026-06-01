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
      <div className="PlayerStats">
        <span>wood: {game?.player.materials.wood}</span>
        <span>stone: {game?.player.materials.stone}</span>
        <span>iron: {game?.player.materials.iron}</span>
        <span>grain: {game?.player.materials.grain}</span>
        <span>clay: {game?.player.materials.clay}</span>
      </div>
    </div>
  );
};

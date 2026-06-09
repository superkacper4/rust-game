import { invoke } from "@tauri-apps/api/core";
import "./index.css";
import { useEffect, useState } from "react";

export const TurnManagement = ({ game }) => {
  const [player, setPlayer] = useState(null);
  useEffect(() => {
    invoke("get_current_player")
      .then((x) => setPlayer(x))
      .catch((err) => console.log(err));
  }, [game?.current_player_turn]);
  console.log(game?.current_player_turn, player);
  return (
    <div className="TurnManagement">
      <button onClick={() => invoke("end_turn")}>End turn</button>
      <div className="PlayerStats">
        {player?.name} | {player?.cash} cent $ |{" "}
        {player?.actions_left_in_turn}{" "}
      </div>
      <div className="PlayerStats">
        <span>wood: {player?.materials.wood}</span>
        <span>stone: {player?.materials.stone}</span>
        <span>iron: {player?.materials.iron}</span>
        <span>grain: {player?.materials.grain}</span>
        <span>clay: {player?.materials.clay}</span>
      </div>
    </div>
  );
};

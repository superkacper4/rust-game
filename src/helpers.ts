import apartmentImg from "./assets/1.png";
import factoryImg from "./assets/2.png";
import farmImg from "./assets/3.png";
import houseImg from "./assets/4.png";
import officeImg from "./assets/5.png";
import shopImg from "./assets/6.png";
import grassImg1 from "./assets/14.png";
import grassImg2 from "./assets/15.png";

export const getTileColour = (tile) => {
  switch (tile.building) {
    case "APARTMENT":
      return `rgba(170,241,250,${tile.value / 400})`;
    case "FACTORY":
      return `rgba(255,231,92,${tile.value / 400})`;
    case "FARM":
      return `rgba(158,255,89,${tile.value / 400})`;
    case "HOUSE":
      return `rgba(77,103,255,${tile.value / 400})`;
    case "OFFICE":
      return `rgba(89,178,255,${tile.value / 400})`;
    case "SHOP":
      return `rgba(252,114,197,${tile.value / 400})`;
    default:
      return `rgba(255,85,0,${tile.value / 400})`;
  }
};

export const getTileImage = (tile) => {
  switch (tile.building) {
    case "APARTMENT":
      return apartmentImg;
    case "FACTORY":
      return factoryImg;
    case "FARM":
      return farmImg;
    case "HOUSE":
      return houseImg;
    case "OFFICE":
      return officeImg;
    case "SHOP":
      return shopImg;
    default:
      return [grassImg1, grassImg2][Math.round(Math.random())];
  }
};
